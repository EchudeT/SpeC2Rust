use std::mem;

pub struct Mbszero;

impl Mbszero {
    pub fn reset(state: &mut MbstateT) {
        *state = MbstateT::default();
    }

    pub fn new_state() -> MbstateT {
        MbstateT::default()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MbstateT {
    bytes: [u8; 16],
}
