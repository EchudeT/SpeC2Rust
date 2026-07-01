pub struct Depmap {
    nrows: usize,
    rowlen: usize,
    r: Vec<u32>,
}

impl Depmap {
    pub fn new(count: usize) -> Self {
        let bits_per_word = u32::BITS as usize;
        let rowlen = count.div_ceil(bits_per_word);
        let total_words = count
            .checked_mul(rowlen)
            .expect("dependency map size overflow");

        Self {
            nrows: count,
            rowlen,
            r: vec![0; total_words],
        }
    }

    pub fn rowptr(&self, row: usize) -> &[u32] {
        assert!(row < self.nrows, "row index out of bounds");
        let start = row * self.rowlen;
        let end = start + self.rowlen;
        &self.r[start..end]
    }

    pub fn set(&mut self, row: usize, col: usize) {
        assert!(row < self.nrows, "row index out of bounds");
        assert!(col < self.nrows, "column index out of bounds");

        let bits_per_word = u32::BITS as usize;
        let word = col / bits_per_word;
        let bit = col % bits_per_word;
        let start = row * self.rowlen;
        self.r[start + word] |= 1u32 << bit;
    }

    pub fn is_set(&self, row: usize, col: usize) -> bool {
        assert!(row < self.nrows, "row index out of bounds");
        assert!(col < self.nrows, "column index out of bounds");

        let bits_per_word = u32::BITS as usize;
        let word = col / bits_per_word;
        let bit = col % bits_per_word;
        let start = row * self.rowlen;
        (self.r[start + word] & (1u32 << bit)) != 0
    }

    pub fn tc(&mut self) {
        self.transitive_closure();
    }

    pub fn transitive_closure(&mut self) {
        let rowsize = self.rowlen;
        let n = self.nrows;
        if n == 0 || rowsize == 0 {
            return;
        }

        let mut cword = 0usize;
        let mut mask = 1u32;
        let mut rowi = 0usize;

        while rowi < n {
            let mut ccol = cword;
            let mut rowj = 0usize;

            while rowj < n {
                if (self.r[ccol] & mask) != 0 {
                    for w in 0..rowsize {
                        let src = self.r[rowi * rowsize + w];
                        let dst = &mut self.r[rowj * rowsize + w];
                        *dst |= src;
                    }
                }

                rowj += 1;
                ccol += rowsize;
            }

            mask <<= 1;
            if mask == 0 {
                mask = 1;
                cword += 1;
            }
            rowi += 1;
        }
    }

    pub fn obstack_chunk_alloc(size: usize) -> Vec<u8> {
        vec![0; size]
    }

    pub fn obstack_chunk_free<T>(_chunk: T) {}

    pub fn linked_list_head<T>(list: Option<&[T]>) -> Option<&T> {
        list.and_then(|items| items.first())
    }
}
