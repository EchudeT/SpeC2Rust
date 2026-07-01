pub struct Malloca;

#[derive(Debug, Default)]
pub struct MallocaBuffer {
    data: Vec<u8>,
}

impl MallocaBuffer {
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Malloca {
    pub fn allocate(n: usize) -> Option<MallocaBuffer> {
        let data = if n == 0 {
            Vec::new()
        } else {
            let mut data = Vec::new();
            if data.try_reserve_exact(n).is_err() {
                return None;
            }
            data.resize(n, 0);
            data
        };

        Some(MallocaBuffer { data })
    }

    pub fn free(buffer: Option<MallocaBuffer>) {
        drop(buffer);
    }
}
