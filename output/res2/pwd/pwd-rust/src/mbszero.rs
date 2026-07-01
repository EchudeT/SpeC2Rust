pub struct Mbszero;

impl Mbszero {
    pub fn new_state() -> std::mem::MaybeUninit<std::mem::MbState> {
        let mut state = std::mem::MaybeUninit::<std::mem::MbState>::uninit();
        state.write(Default::default());
        state
    }
}
