pub struct Mbszero;

impl Mbszero {
    pub fn reset(state: &mut std::mbchar::MbState) {
        *state = std::mbchar::MbState::new();
    }
}
