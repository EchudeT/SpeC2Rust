use crate::getprogname::Getprogname;
use std::cmp;

type ChunkAlloc = Box<dyn FnMut(usize) -> Vec<u8>>;
type ChunkFree = Box<dyn FnMut(Vec<u8>)>;

enum ChunkAllocator {
    Plain(ChunkAlloc),
    Extra {
        alloc: ChunkAlloc,
        free: ChunkFree,
    },
}

struct Chunk {
    prev: Option<Box<Chunk>>,
    data: Vec<u8>,
    object_base: usize,
    next_free: usize,
    maybe_empty_object: bool,
}

impl Chunk {
    fn capacity(&self) -> usize {
        self.data.len()
    }

    fn contains_object_position(&self, position: usize) -> bool {
        position > 0 && position <= self.capacity()
    }
}

pub struct Obstack {
    allocator: ChunkAllocator,
    chunk_size: usize,
    alignment_mask: usize,
    chunk: Option<Box<Chunk>>,
    alloc_failed: bool,
}

impl Obstack {
    fn default_alignment() -> usize {
        std::mem::align_of::<usize>()
    }

    fn normalize_alignment(alignment: usize) -> usize {
        let alignment = if alignment == 0 {
            Self::default_alignment()
        } else {
            alignment
        };
        alignment.next_power_of_two()
    }

    fn default_chunk_size() -> usize {
        let rounding = Self::default_alignment();
        let extra = (((12 + rounding - 1) & !(rounding - 1)) + 4 + rounding - 1) & !(rounding - 1);
        4096usize.saturating_sub(extra)
    }

    fn align_up(value: usize, mask: usize) -> usize {
        (value + mask) & !mask
    }

    pub fn call_chunkfun(&mut self, size: usize) -> Vec<u8> {
        match &mut self.allocator {
            ChunkAllocator::Plain(alloc) => alloc(size),
            ChunkAllocator::Extra { alloc, .. } => alloc(size),
        }
    }

    pub fn call_freefun(&mut self, old_chunk: Vec<u8>) {
        match &mut self.allocator {
            ChunkAllocator::Plain(_) => drop(old_chunk),
            ChunkAllocator::Extra { free, .. } => free(old_chunk),
        }
    }

    pub fn begin_worker(
        allocator: ChunkAllocator,
        size: usize,
        alignment: usize,
    ) -> Result<Self, String> {
        let alignment = Self::normalize_alignment(alignment);
        let chunk_size = if size == 0 {
            Self::default_chunk_size()
        } else {
            size
        };

        let mut obstack = Self {
            allocator,
            chunk_size,
            alignment_mask: alignment - 1,
            chunk: None,
            alloc_failed: false,
        };

        let data = obstack.call_chunkfun(chunk_size);
        if data.is_empty() && chunk_size != 0 {
            obstack.alloc_failed = true;
            Self::print_and_abort();
        }

        let object_base = Self::align_up(0, obstack.alignment_mask);
        if object_base > data.len() {
            obstack.alloc_failed = true;
            Self::print_and_abort();
        }

        obstack.chunk = Some(Box::new(Chunk {
            prev: None,
            data,
            object_base,
            next_free: object_base,
            maybe_empty_object: false,
        }));

        Ok(obstack)
    }

    pub fn begin<FAlloc>(size: usize, alignment: usize, chunkfun: FAlloc) -> Result<Self, String>
    where
        FAlloc: FnMut(usize) -> Vec<u8> + 'static,
    {
        Self::begin_worker(ChunkAllocator::Plain(Box::new(chunkfun)), size, alignment)
    }

    pub fn begin_1<FAlloc, FFree>(
        size: usize,
        alignment: usize,
        chunkfun: FAlloc,
        freefun: FFree,
    ) -> Result<Self, String>
    where
        FAlloc: FnMut(usize) -> Vec<u8> + 'static,
        FFree: FnMut(Vec<u8>) + 'static,
    {
        Self::begin_worker(
            ChunkAllocator::Extra {
                alloc: Box::new(chunkfun),
                free: Box::new(freefun),
            },
            size,
            alignment,
        )
    }

    pub fn newchunk(&mut self, length: usize) {
        let old_chunk = self
            .chunk
            .take()
            .expect("obstack newchunk requires an initialized current chunk");

        let obj_size = old_chunk.next_free.saturating_sub(old_chunk.object_base);
        let sum1 = obj_size.saturating_add(length);
        let sum2 = sum1.saturating_add(self.alignment_mask);
        let mut new_size = sum2.saturating_add(obj_size >> 3).saturating_add(100);

        if new_size < sum2 {
            new_size = sum2;
        }
        if new_size < self.chunk_size {
            new_size = self.chunk_size;
        }

        let mut new_data = self.call_chunkfun(new_size);
        if new_data.is_empty() && new_size != 0 {
            self.alloc_failed = true;
            Self::print_and_abort();
        }

        let object_base = Self::align_up(0, self.alignment_mask);
        if object_base + obj_size > new_data.len() {
            self.alloc_failed = true;
            Self::print_and_abort();
        }

        new_data[object_base..object_base + obj_size]
            .copy_from_slice(&old_chunk.data[old_chunk.object_base..old_chunk.next_free]);

        let old_chunk_aligned_base = Self::align_up(0, self.alignment_mask);
        let free_old = !old_chunk.maybe_empty_object && old_chunk.object_base == old_chunk_aligned_base;

        let prev = if free_old {
            let mut old_chunk = old_chunk;
            let prev = old_chunk.prev.take();
            self.call_freefun(old_chunk.data);
            prev
        } else {
            Some(old_chunk)
        };

        self.chunk = Some(Box::new(Chunk {
            prev,
            data: new_data,
            object_base,
            next_free: object_base + obj_size,
            maybe_empty_object: false,
        }));
    }

    pub fn allocated_p(&self, obj: usize) -> bool {
        let mut current = self.chunk.as_deref();
        while let Some(chunk) = current {
            if chunk.contains_object_position(obj) {
                return true;
            }
            current = chunk.prev.as_deref();
        }
        false
    }

    pub fn memory_used(&self) -> usize {
        let mut nbytes = 0usize;
        let mut current = self.chunk.as_deref();
        while let Some(chunk) = current {
            nbytes = nbytes.saturating_add(chunk.capacity());
            current = chunk.prev.as_deref();
        }
        nbytes
    }

    pub fn print_and_abort() -> ! {
        let program = Getprogname::program_name().unwrap_or_else(|| "obstack".to_string());
        eprintln!("{program}: memory exhausted");
        std::process::abort()
    }

    pub fn chunk(&self) -> Option<&[u8]> {
        self.chunk.as_deref().map(|chunk| chunk.data.as_slice())
    }
}

impl Drop for Obstack {
    fn drop(&mut self) {
        let mut current = self.chunk.take();
        while let Some(mut chunk) = current {
            current = chunk.prev.take();
            let data = std::mem::take(&mut chunk.data);
            match &mut self.allocator {
                ChunkAllocator::Plain(_) => {}
                ChunkAllocator::Extra { free, .. } => free(data),
            }
        }
    }
}
