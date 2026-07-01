pub struct Depmap {
    nrows: usize,
    rowlen: usize,
    rows: Vec<usize>,
}

impl Depmap {
    pub fn new(count: usize) -> Self {
        let bits_per_word = usize::BITS as usize;
        let rowlen = if count == 0 {
            0
        } else {
            count.div_ceil(bits_per_word)
        };

        Self {
            nrows: count,
            rowlen,
            rows: vec![0; count.saturating_mul(rowlen)],
        }
    }

    pub fn rowptr(&self, row: usize) -> &[usize] {
        assert!(row < self.nrows, "row index out of bounds");
        let start = row * self.rowlen;
        let end = start + self.rowlen;
        &self.rows[start..end]
    }

    fn rowptr_mut(&mut self, row: usize) -> &mut [usize] {
        assert!(row < self.nrows, "row index out of bounds");
        let start = row * self.rowlen;
        let end = start + self.rowlen;
        &mut self.rows[start..end]
    }

    pub fn set(&mut self, row: usize, col: usize) {
        assert!(row < self.nrows, "row index out of bounds");
        assert!(col < self.nrows, "column index out of bounds");

        let bits_per_word = usize::BITS as usize;
        let word = col / bits_per_word;
        let bit = col % bits_per_word;
        let mask = 1usize << bit;
        let rowptr = self.rowptr_mut(row);
        rowptr[word] |= mask;
    }

    pub fn is_set(&self, row: usize, col: usize) -> bool {
        assert!(row < self.nrows, "row index out of bounds");
        assert!(col < self.nrows, "column index out of bounds");

        let bits_per_word = usize::BITS as usize;
        let word = col / bits_per_word;
        let bit = col % bits_per_word;
        let mask = 1usize << bit;
        let rowptr = self.rowptr(row);
        (rowptr[word] & mask) != 0
    }

    pub fn transitive_closure(&mut self) {
        let n = self.nrows;
        if n == 0 || self.rowlen == 0 {
            return;
        }

        let rowlen = self.rowlen;

        for i in 0..n {
            for j in 0..n {
                if self.is_set(i, j) {
                    let src_start = i * rowlen;
                    let dst_start = j * rowlen;

                    let src_row = self.rows[src_start..src_start + rowlen].to_vec();
                    let dst_row = &mut self.rows[dst_start..dst_start + rowlen];

                    for (dst, src) in dst_row.iter_mut().zip(src_row.iter()) {
                        *dst |= *src;
                    }
                }
            }
        }
    }

    pub fn tc(&mut self) {
        self.transitive_closure();
    }

    pub fn obstack_chunk_alloc(size: usize) -> Vec<u8> {
        vec![0; size]
    }

    pub fn obstack_chunk_free<T>(_chunk: T) {}

    pub fn linked_list_head<T>(list: Option<&[T]>) -> Option<&T> {
        list.and_then(|items| items.first())
    }
}
