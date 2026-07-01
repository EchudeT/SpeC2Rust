#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Mbszero {
    bytes: Vec<u8>,
}

impl Mbszero {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.bytes.fill(0);
    }

    pub fn clear(&mut self) {
        self.bytes.clear();
    }

    pub fn is_initial(&self) -> bool {
        self.bytes.iter().all(|&b| b == 0)
    }

    pub fn from_len(len: usize) -> Self {
        Self {
            bytes: vec![0; len],
        }
    }

    pub fn resize(&mut self, len: usize) {
        self.bytes.resize(len, 0);
        self.reset();
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}
