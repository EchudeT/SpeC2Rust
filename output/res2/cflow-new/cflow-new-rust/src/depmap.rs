pub struct Depmap {
    nrows: usize,
    rowlen: usize,
    rows: Vec<u32>,
}

impl Depmap {
    pub fn new(count: usize) -> Self {
        let bits_per_word = u32::BITS as usize;
        let rowlen = count.div_ceil(bits_per_word);
        let words = count
            .checked_mul(rowlen)
            .expect("dependency map size overflow");
        Self {
            nrows: count,
            rowlen,
            rows: vec![0; words],
        }
    }

    pub fn rowptr(&self, row: usize) -> &[u32] {
        assert!(row < self.nrows, "row out of bounds");
        let start = row * self.rowlen;
        let end = start + self.rowlen;
        &self.rows[start..end]
    }

    pub fn set(&mut self, row: usize, col: usize) {
        assert!(row < self.nrows, "row out of bounds");
        assert!(col < self.nrows, "column out of bounds");
        let bits_per_word = u32::BITS as usize;
        let word_index = col / bits_per_word;
        let bit_index = col % bits_per_word;
        let start = row * self.rowlen;
        self.rows[start + word_index] |= 1u32 << bit_index;
    }

    pub fn is_set(&self, row: usize, col: usize) -> bool {
        assert!(row < self.nrows, "row out of bounds");
        assert!(col < self.nrows, "column out of bounds");
        let bits_per_word = u32::BITS as usize;
        let word_index = col / bits_per_word;
        let bit_index = col % bits_per_word;
        let start = row * self.rowlen;
        (self.rows[start + word_index] & (1u32 << bit_index)) != 0
    }

    pub fn tc(&mut self) {
        self.transitive_closure();
    }

    pub fn transitive_closure(&mut self) {
        let mut cword = 0usize;
        let mut mask = 1u32;
        let mut rowi = 0usize;

        while rowi < self.nrows {
            let mut ccol = cword;
            let mut rowj = 0usize;

            while rowj < self.nrows {
                let ccol_index = rowj * self.rowlen + ccol;
                if (self.rows[ccol_index] & mask) != 0 {
                    let src_start = rowi * self.rowlen;
                    let dst_start = rowj * self.rowlen;
                    for offset in 0..self.rowlen {
                        let value = self.rows[src_start + offset];
                        self.rows[dst_start + offset] |= value;
                    }
                }
                rowj += 1;
            }

            mask = mask.wrapping_shl(1);
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
