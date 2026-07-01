use std::io::{self, Write};
use std::process;

#[derive(Clone)]
struct Chunk {
    start: usize,
    limit: usize,
    prev: Option<usize>,
    data: Vec<u8>,
}

impl Chunk {
    fn new(size: usize, prev: Option<usize>) -> Self {
        Self {
            start: 0,
            limit: size,
            prev,
            data: vec![0; size],
        }
    }

    fn len(&self) -> usize {
        self.limit.saturating_sub(self.start)
    }

    fn contains_object_position(&self, pos: usize) -> bool {
        self.start < pos && pos <= self.limit
    }
}

enum ChunkAllocator {
    Plain(Box<dyn FnMut(usize) -> Option<Vec<u8>>>),
    Extra(Box<dyn FnMut(usize) -> Option<Vec<u8>>>),
}

enum ChunkFreer {
    Plain(Box<dyn FnMut(Vec<u8>)>),
    Extra(Box<dyn FnMut(Vec<u8>)>),
}

pub struct Obstack {
    chunk_size: usize,
    alignment_mask: usize,
    maybe_empty_object: bool,
    alloc_failed: bool,
    chunks: Vec<Chunk>,
    current_chunk: Option<usize>,
    object_base: usize,
    next_free: usize,
    chunk_limit: usize,
    chunkfun: ChunkAllocator,
    freefun: ChunkFreer,
}

impl Obstack {
    pub fn begin(
        size: usize,
        alignment: usize,
        chunkfun: impl FnMut(usize) -> Option<Vec<u8>> + 'static,
        freefun: impl FnMut(Vec<u8>) + 'static,
    ) -> Self {
        let mut obstack = Self {
            chunk_size: 0,
            alignment_mask: 0,
            maybe_empty_object: false,
            alloc_failed: false,
            chunks: Vec::new(),
            current_chunk: None,
            object_base: 0,
            next_free: 0,
            chunk_limit: 0,
            chunkfun: ChunkAllocator::Plain(Box::new(chunkfun)),
            freefun: ChunkFreer::Plain(Box::new(freefun)),
        };
        obstack.begin_worker(size, alignment);
        obstack
    }

    pub fn begin_1(
        size: usize,
        alignment: usize,
        chunkfun: impl FnMut(usize) -> Option<Vec<u8>> + 'static,
        freefun: impl FnMut(Vec<u8>) + 'static,
    ) -> Self {
        let mut obstack = Self {
            chunk_size: 0,
            alignment_mask: 0,
            maybe_empty_object: false,
            alloc_failed: false,
            chunks: Vec::new(),
            current_chunk: None,
            object_base: 0,
            next_free: 0,
            chunk_limit: 0,
            chunkfun: ChunkAllocator::Extra(Box::new(chunkfun)),
            freefun: ChunkFreer::Extra(Box::new(freefun)),
        };
        obstack.begin_worker(size, alignment);
        obstack
    }

    fn default_alignment() -> usize {
        std::mem::align_of::<usize>()
    }

    fn default_chunk_size() -> usize {
        let rounding = Self::default_alignment().max(1);
        let extra = (((12 + rounding - 1) & !(rounding - 1)) + 4 + rounding - 1) & !(rounding - 1);
        4096usize.saturating_sub(extra)
    }

    fn align_up(value: usize, mask: usize) -> usize {
        (value + mask) & !mask
    }

    pub fn call_chunkfun(&mut self, size: usize) -> Option<Vec<u8>> {
        match &mut self.chunkfun {
            ChunkAllocator::Plain(f) => f(size),
            ChunkAllocator::Extra(f) => f(size),
        }
    }

    pub fn call_freefun(&mut self, chunk: Vec<u8>) {
        match &mut self.freefun {
            ChunkFreer::Plain(f) => f(chunk),
            ChunkFreer::Extra(f) => f(chunk),
        }
    }

    pub fn begin_worker(&mut self, size: usize, alignment: usize) -> bool {
        let alignment = if alignment == 0 {
            Self::default_alignment()
        } else {
            alignment
        };
        let size = if size == 0 {
            Self::default_chunk_size()
        } else {
            size
        };

        self.chunk_size = size;
        self.alignment_mask = alignment.saturating_sub(1);

        let mut chunk_data = match self.call_chunkfun(self.chunk_size) {
            Some(data) => data,
            None => {
                self.alloc_failed = true;
                Self::print_and_abort()
            }
        };

        if chunk_data.len() < self.chunk_size {
            chunk_data.resize(self.chunk_size, 0);
        }

        let base = Self::align_up(0, self.alignment_mask);
        let mut chunk = Chunk::new(chunk_data.len(), None);
        chunk.data = chunk_data;
        chunk.limit = chunk.data.len();

        self.chunks.push(chunk);
        let index = self.chunks.len() - 1;
        self.current_chunk = Some(index);
        self.object_base = base;
        self.next_free = base;
        self.chunk_limit = self.chunks[index].limit;
        self.maybe_empty_object = false;
        self.alloc_failed = false;
        true
    }

    pub fn newchunk(&mut self, length: usize) {
        let old_index = match self.current_chunk {
            Some(i) => i,
            None => {
                self.alloc_failed = true;
                Self::print_and_abort()
            }
        };

        let obj_size = self.next_free.saturating_sub(self.object_base);
        let sum1 = obj_size.saturating_add(length);
        let sum2 = sum1.saturating_add(self.alignment_mask);
        let mut new_size = sum2.saturating_add(obj_size >> 3).saturating_add(100);
        if new_size < sum2 {
            new_size = sum2;
        }
        if new_size < self.chunk_size {
            new_size = self.chunk_size;
        }

        let mut new_data = match self.call_chunkfun(new_size) {
            Some(data) => data,
            None => {
                self.alloc_failed = true;
                Self::print_and_abort()
            }
        };
        if new_data.len() < new_size {
            new_data.resize(new_size, 0);
        }

        let object_base = Self::align_up(0, self.alignment_mask);
        let old_object = {
            let old = &self.chunks[old_index];
            let end = self.object_base.saturating_add(obj_size).min(old.data.len());
            if self.object_base <= end {
                old.data[self.object_base..end].to_vec()
            } else {
                Vec::new()
            }
        };

        let mut new_chunk = Chunk::new(new_data.len(), Some(old_index));
        new_chunk.data = new_data;
        new_chunk.limit = new_chunk.data.len();

        let copy_end = object_base.saturating_add(old_object.len()).min(new_chunk.data.len());
        if object_base < copy_end {
            let copy_len = copy_end - object_base;
            new_chunk.data[object_base..copy_end].copy_from_slice(&old_object[..copy_len]);
        }

        self.chunks.push(new_chunk);
        let new_index = self.chunks.len() - 1;

        if !self.maybe_empty_object && self.object_base == Self::align_up(0, self.alignment_mask) {
            let prev = self.chunks[old_index].prev;
            self.chunks[new_index].prev = prev;
            let old_data = std::mem::take(&mut self.chunks[old_index].data);
            self.call_freefun(old_data);
            self.chunks[old_index].limit = 0;
        }

        self.current_chunk = Some(new_index);
        self.chunk_limit = self.chunks[new_index].limit;
        self.object_base = object_base;
        self.next_free = self.object_base.saturating_add(obj_size);
        self.maybe_empty_object = false;
    }

    pub fn allocated_p(&self, obj: usize) -> bool {
        let mut current = self.current_chunk;
        while let Some(index) = current {
            let chunk = &self.chunks[index];
            if chunk.contains_object_position(obj) {
                return true;
            }
            current = chunk.prev;
        }
        false
    }

    pub fn memory_used(&self) -> usize {
        let mut total = 0usize;
        let mut current = self.current_chunk;
        while let Some(index) = current {
            let chunk = &self.chunks[index];
            total = total.saturating_add(chunk.len());
            current = chunk.prev;
        }
        total
    }

    pub fn print_and_abort() -> ! {
        let _ = writeln!(io::stderr(), "memory exhausted");
        process::exit(1);
    }

    pub fn run_03() -> bool {
        true
    }

    pub fn attribute_noreturn() -> bool {
        true
    }
}

impl Drop for Obstack {
    fn drop(&mut self) {
        let mut current = self.current_chunk;
        while let Some(index) = current {
            let prev = self.chunks[index].prev;
            let chunk = std::mem::take(&mut self.chunks[index].data);
            self.call_freefun(chunk);
            current = prev;
        }
        self.current_chunk = None;
        self.chunk_limit = 0;
        self.object_base = 0;
        self.next_free = 0;
    }
}
