#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Mbszero;

impl Mbszero {
    pub fn new() -> Self {
        Self
    }

    pub fn reset<T: Default>(state: &mut T) {
        *state = T::default();
    }

    pub fn initial_state<T: Default>() -> T {
        T::default()
    }

    pub fn is_initial<T: Default + PartialEq>(state: &T) -> bool {
        *state == T::default()
    }
}
