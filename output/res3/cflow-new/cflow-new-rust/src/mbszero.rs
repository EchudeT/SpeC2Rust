use std::mem;

pub struct Mbszero;

impl Mbszero {
    pub fn new_state() -> MbstateT {
        MbstateT::default()
    }

    pub fn reset_state(state: &mut MbstateT) {
        *state = MbstateT::default();
    }

    pub fn is_initial(state: &MbstateT) -> bool {
        state == &MbstateT::default()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MbstateT {
    bytes: [u8; mem::size_of::<usize>() * 2],
}
