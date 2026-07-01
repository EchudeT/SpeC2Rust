#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Mbrtoc32 {
    pending: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Mbrtoc32Status {
    Complete { bytes: usize, ch: u32 },
    Null { bytes: usize },
    Incomplete,
    Invalid,
}

impl Mbrtoc32 {
    pub fn mbrtoc_32(
        output: Option<&mut u32>,
        input: Option<&[u8]>,
        state: Option<&mut Self>,
    ) -> Mbrtoc32Status {
        match state {
            Some(state) => Self::decode_with_state(output, input, state),
            None => {
                let mut internal = Self::default();
                Self::decode_with_state(output, input, &mut internal)
            }
        }
    }

    pub fn run_10() -> i32 {
        let mut state = Self::default();
        let mut out = 0_u32;

        match Self::mbrtoc_32(Some(&mut out), Some(&[b'A']), Some(&mut state)) {
            Mbrtoc32Status::Complete { bytes, ch } if bytes == 1 && ch == u32::from(b'A') => 0,
            _ => 1,
        }
    }

    fn decode_with_state(
        output: Option<&mut u32>,
        input: Option<&[u8]>,
        state: &mut Self,
    ) -> Mbrtoc32Status {
        let bytes = match input {
            Some(slice) => slice,
            None => &[0],
        };

        if bytes.is_empty() {
            return Mbrtoc32Status::Incomplete;
        }

        if state.pending.is_empty() {
            return Self::decode_fresh(output, bytes, state);
        }

        Self::decode_from_pending(output, bytes, state)
    }

    fn decode_fresh(
        output: Option<&mut u32>,
        bytes: &[u8],
        state: &mut Self,
    ) -> Mbrtoc32Status {
        let first = bytes[0];

        if first == 0 {
            state.pending.clear();
            if let Some(out) = output {
                *out = 0;
            }
            return Mbrtoc32Status::Null { bytes: 0 };
        }

        if first < 0x80 {
            state.pending.clear();
            if let Some(out) = output {
                *out = u32::from(first);
            }
            return Mbrtoc32Status::Complete {
                bytes: 1,
                ch: u32::from(first),
            };
        }

        let expected = match Self::utf8_expected_len(first) {
            Some(len) => len,
            None => {
                state.pending.clear();
                return Mbrtoc32Status::Invalid;
            }
        };

        if bytes.len() < expected {
            state.pending.clear();
            state.pending.extend_from_slice(bytes);
            return Mbrtoc32Status::Incomplete;
        }

        match core::str::from_utf8(&bytes[..expected]) {
            Ok(s) => {
                let ch = s.chars().next().map(|c| c as u32).unwrap_or(0);
                state.pending.clear();
                if ch == 0 {
                    if let Some(out) = output {
                        *out = 0;
                    }
                    Mbrtoc32Status::Null { bytes: 0 }
                } else {
                    if let Some(out) = output {
                        *out = ch;
                    }
                    Mbrtoc32Status::Complete {
                        bytes: expected,
                        ch,
                    }
                }
            }
            Err(_) => {
                state.pending.clear();
                Mbrtoc32Status::Invalid
            }
        }
    }

    fn decode_from_pending(
        output: Option<&mut u32>,
        bytes: &[u8],
        state: &mut Self,
    ) -> Mbrtoc32Status {
        let Some(expected) = state
            .pending
            .first()
            .and_then(|b| Self::utf8_expected_len(*b))
        else {
            state.pending.clear();
            return Mbrtoc32Status::Invalid;
        };

        let missing = expected.saturating_sub(state.pending.len());
        let to_take = missing.min(bytes.len());

        let mut combined = state.pending.clone();
        combined.extend_from_slice(&bytes[..to_take]);

        if combined.len() < expected {
            state.pending = combined;
            return Mbrtoc32Status::Incomplete;
        }

        match core::str::from_utf8(&combined[..expected]) {
            Ok(s) => {
                let ch = s.chars().next().map(|c| c as u32).unwrap_or(0);
                state.pending.clear();
                if ch == 0 {
                    if let Some(out) = output {
                        *out = 0;
                    }
                    Mbrtoc32Status::Null { bytes: 0 }
                } else {
                    if let Some(out) = output {
                        *out = ch;
                    }
                    Mbrtoc32Status::Complete {
                        bytes: to_take,
                        ch,
                    }
                }
            }
            Err(_) => {
                state.pending.clear();
                Mbrtoc32Status::Invalid
            }
        }
    }

    fn utf8_expected_len(first: u8) -> Option<usize> {
        match first {
            0x00..=0x7f => Some(1),
            0xc2..=0xdf => Some(2),
            0xe0..=0xef => Some(3),
            0xf0..=0xf4 => Some(4),
            _ => None,
        }
    }
}
