pub struct Mbszero;

impl Mbszero {
    pub fn reset<State: Default>(state: &mut State) {
        *state = State::default();
    }

    pub fn take_reset<State: Default>(state: State) -> State {
        let _ = state;
        State::default()
    }

    pub fn initial_state<State: Default>() -> State {
        State::default()
    }
}
