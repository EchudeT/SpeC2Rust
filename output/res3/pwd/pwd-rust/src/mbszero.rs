#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Mbszero {
    pending: Vec<u8>,
}

impl Mbszero {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.pending.clear();
    }

    pub fn is_initial(&self) -> bool {
        self.pending.is_empty()
    }
}
