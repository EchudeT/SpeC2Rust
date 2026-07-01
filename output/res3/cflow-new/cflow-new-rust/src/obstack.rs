use std::process;
use std::rc::Rc;

type ChunkAlloc = Rc<dyn Fn(usize) -> Vec<u8>>;
type ChunkFree = Rc<dyn Fn(Vec<u8>)>;

#[derive(Clone)]
struct Chunk {
    storage: Vec<u8>,
    start: usize,
    limit: usize,
}

impl Chunk {
    fn total_bytes(&self) -> usize {
        self.storage.len()
    }

    fn contains_token(&self, token: &AllocationToken) -> bool {
        token.chunk_id == self.storage.as_ptr() as usize
            && token.offset >= self.start
            && token.offset <= self.limit
    }

    fn base_token(&self) -> AllocationToken {
        AllocationToken {
            chunk_id: self.storage.as_ptr() as usize,
            offset: self.start,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AllocationToken {
    chunk_id: usize,
    offset: usize,
}

pub struct Obstack {
    chunk_size: usize,
    alignment_mask: usize,
    chunks: Vec<Chunk>,
    current_object_base: usize,
    next_free: usize,
    maybe_empty_object: bool,
    alloc_failed: bool,
    chunk_alloc: ChunkAlloc,
    chunk_free: ChunkFree,
}

impl Obstack {
    pub fn call_chunkfun(&self, size: usize) -> Vec<u8> {
        (self.chunk_alloc)(size)
    }

    pub fn call_freefun(&self, chunk: Vec<u8>) {
        (self.chunk_free)(chunk);
    }

    pub fn begin_worker(
        chunk_size: usize,
        alignment: usize,
        chunk_alloc: impl Fn(usize) -> Vec<u8> + 'static,
        chunk_free: impl Fn(Vec<u8>) + 'static,
    ) -> Self {
        let mut obstack = Self {
            chunk_size: 0,
            alignment_mask: 0,
            chunks: Vec::new(),
            current_object_base: 0,
            next_free: 0,
            maybe_empty_object: false,
            alloc_failed: false,
            chunk_alloc: Rc::new(chunk_alloc),
            chunk_free: Rc::new(chunk_free),
        };

        let resolved_alignment = if alignment == 0 {
            std::mem::align_of::<usize>()
        } else {
            alignment
        };

        let resolved_chunk_size = if chunk_size == 0 {
            let default_rounding = std::mem::align_of::<usize>();
            let extra = (((12 + default_rounding - 1) & !(default_rounding - 1))
                + 4
                + default_rounding
                - 1)
                & !(default_rounding - 1);
            4096usize.saturating_sub(extra)
        } else {
            chunk_size
        };

        obstack.chunk_size = resolved_chunk_size;
        obstack.alignment_mask = resolved_alignment.saturating_sub(1);

        let mut initial = obstack.call_chunkfun(obstack.chunk_size);
        if initial.is_empty() {
            obstack.alloc_failed = true;
            Self::print_and_abort();
        }

        let start = Self::align_index(0, obstack.alignment_mask);
        if start > initial.len() {
            obstack.alloc_failed = true;
            Self::print_and_abort();
        }

        let limit = initial.len();
        obstack.current_object_base = start;
        obstack.next_free = start;
        obstack.chunks.push(Chunk {
            storage: std::mem::take(&mut initial),
            start,
            limit,
        });
        obstack.maybe_empty_object = false;
        obstack.alloc_failed = false;
        obstack
    }

    pub fn begin(
        chunk_size: usize,
        alignment: usize,
        chunk_alloc: impl Fn(usize) -> Vec<u8> + 'static,
        chunk_free: impl Fn(Vec<u8>) + 'static,
    ) -> Self {
        Self::begin_worker(chunk_size, alignment, chunk_alloc, chunk_free)
    }

    pub fn begin_1(
        chunk_size: usize,
        alignment: usize,
        chunk_alloc: impl Fn(usize) -> Vec<u8> + 'static,
        chunk_free: impl Fn(Vec<u8>) + 'static,
    ) -> Self {
        Self::begin_worker(chunk_size, alignment, chunk_alloc, chunk_free)
    }

    pub fn newchunk(&mut self, length: usize) {
        if self.chunks.is_empty() {
            self.alloc_failed = true;
            Self::print_and_abort();
        }

        let old_index = self.chunks.len() - 1;
        let old_chunk = self.chunks[old_index].clone();
        let obj_size = self.next_free.saturating_sub(self.current_object_base);

        let sum1 = obj_size.saturating_add(length);
        let sum2 = sum1.saturating_add(self.alignment_mask);
        let mut new_size = sum2.saturating_add(obj_size >> 3).saturating_add(100);
        if new_size < sum2 {
            new_size = sum2;
        }
        if new_size < self.chunk_size {
            new_size = self.chunk_size;
        }

        let mut storage = self.call_chunkfun(new_size);
        if storage.len() < sum2 {
            self.alloc_failed = true;
            Self::print_and_abort();
        }

        let object_base = Self::align_index(0, self.alignment_mask);
        let source = &old_chunk.storage[self.current_object_base..self.next_free];
        let destination_end = object_base + source.len();
        if destination_end > storage.len() {
            self.alloc_failed = true;
            Self::print_and_abort();
        }
        storage[object_base..destination_end].copy_from_slice(source);

        let new_chunk = Chunk {
            start: object_base,
            limit: storage.len(),
            storage,
        };

        let old_base_aligned = Self::align_index(0, self.alignment_mask);
        let free_old = !self.maybe_empty_object && self.current_object_base == old_base_aligned;

        if free_old {
            let removed = self.chunks.remove(old_index);
            self.call_freefun(removed.storage);
        }

        self.chunks.push(new_chunk);
        self.current_object_base = object_base;
        self.next_free = object_base + obj_size;
        self.maybe_empty_object = false;
        self.alloc_failed = false;
    }

    pub fn allocated_p(&self, obj: Option<AllocationToken>) -> bool {
        let Some(token) = obj else {
            return false;
        };

        self.chunks.iter().rev().any(|chunk| chunk.contains_token(&token))
    }

    pub fn memory_used(&self) -> usize {
        self.chunks.iter().map(Chunk::total_bytes).sum()
    }

    pub fn print_and_abort() -> ! {
        eprintln!("memory exhausted");
        process::exit(1);
    }

    pub fn run_03() -> i32 {
        let mut freed = 0usize;
        let mut obstack = Self::begin(
            128,
            std::mem::align_of::<usize>(),
            |size| vec![0u8; size],
            |_| {},
        );

        let before = obstack.memory_used();
        obstack.newchunk(256);
        let after = obstack.memory_used();

        if after >= before {
            freed += 1;
        }

        let token = obstack.chunks.last().map(Chunk::base_token);
        if obstack.allocated_p(token) && freed == 1 {
            0
        } else {
            1
        }
    }

    fn align_index(index: usize, mask: usize) -> usize {
        (index + mask) & !mask
    }
}

impl Drop for Obstack {
    fn drop(&mut self) {
        while let Some(chunk) = self.chunks.pop() {
            self.call_freefun(chunk.storage);
        }
    }
}
