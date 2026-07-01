use std::mem;

pub struct Mbszero;

impl Mbszero {
    pub fn reset<T: Default>(state: &mut T) {
        *state = T::default();
    }

    pub fn take_reset<T: Default>(mut state: T) -> T {
        mem::take(&mut state)
    }

    pub fn is_initial<T>(state: &T) -> bool
    where
        T: Default + PartialEq,
    {
        *state == T::default()
    }
}
