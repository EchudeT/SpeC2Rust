#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Mbrtoc32 {
    pending: [u8; 4],
    pending_len: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mbrtoc32Result {
    Complete { ch: u32, bytes: usize },
    Null { bytes: usize },
    Incomplete,
    Invalid,
}

impl Mbrtoc32 {
    pub fn mbrtoc_32(
        output: Option<&mut u32>,
        input: Option<&[u8]>,
        state: Option<&mut Mbrtoc32>,
    ) -> Mbrtoc32Result {
        match state {
            Some(state) => Self::decode_into(output, input, state),
            None => Mbrtoc32Result::Invalid,
        }
    }

    pub fn run_10() -> bool {
        let mut state = Mbrtoc32::default();
        let mut out = 0u32;

        if Self::mbrtoc_32(Some(&mut out), Some(b"A"), Some(&mut state))
            != Mbrtoc32Result::Complete {
                ch: 'A' as u32,
                bytes: 1,
            }
        {
            return false;
        }

        if Self::mbrtoc_32(Some(&mut out), Some(&[0]), Some(&mut state))
            != Mbrtoc32Result::Null { bytes: 0 }
        {
            return false;
        }

        if Self::mbrtoc_32(Some(&mut out), Some(&[0xE2, 0x82]), Some(&mut state))
            != Mbrtoc32Result::Incomplete
        {
            return false;
        }

        if Self::mbrtoc_32(Some(&mut out), Some(&[0xAC]), Some(&mut state))
            != Mbrtoc32Result::Complete {
                ch: 0x20AC,
                bytes: 1,
            }
        {
            return false;
        }

        let mut invalid_state = Mbrtoc32::default();
        Self::mbrtoc_32(Some(&mut out), Some(&[0xFF]), Some(&mut invalid_state))
            == Mbrtoc32Result::Invalid
    }

    fn decode_into(
        mut output: Option<&mut u32>,
        input: Option<&[u8]>,
        state: &mut Mbrtoc32,
    ) -> Mbrtoc32Result {
        let bytes = match input {
            Some(bytes) => bytes,
            None => b"\0".as_slice(),
        };

        if bytes.is_empty() {
            return Mbrtoc32Result::Incomplete;
        }

        let mut combined = [0u8; 4];
        let carried = state.pending_len;
        combined[..carried].copy_from_slice(&state.pending[..carried]);

        let available = (4 - carried).min(bytes.len());
        combined[carried..carried + available].copy_from_slice(&bytes[..available]);
        let total = carried + available;

        match decode_utf8_prefix(&combined[..total]) {
            Utf8Step::Complete { ch, len } => {
                if carried >= len.max(1) {
                    return Mbrtoc32Result::Invalid;
                }

                state.reset();

                let consumed_now = len.saturating_sub(carried);
                if ch == 0 {
                    if let Some(slot) = output.as_deref_mut() {
                        *slot = 0;
                    }
                    Mbrtoc32Result::Null { bytes: 0 }
                } else {
                    if let Some(slot) = output.as_deref_mut() {
                        *slot = ch;
                    }
                    Mbrtoc32Result::Complete {
                        ch,
                        bytes: consumed_now,
                    }
                }
            }
            Utf8Step::Incomplete { expected } => {
                let to_store = expected.min(total);
                state.pending[..to_store].copy_from_slice(&combined[..to_store]);
                state.pending_len = to_store;
                Mbrtoc32Result::Incomplete
            }
            Utf8Step::Invalid => Mbrtoc32Result::Invalid,
        }
    }

    fn reset(&mut self) {
        self.pending = [0; 4];
        self.pending_len = 0;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Utf8Step {
    Complete { ch: u32, len: usize },
    Incomplete { expected: usize },
    Invalid,
}


fn decode_utf8_prefix(bytes: &[u8]) -> Utf8Step {
    let first = match bytes.first().copied() {
        Some(b) => b,
        None => return Utf8Step::Incomplete { expected: 1 },
    };

    if first < 0x80 {
        return Utf8Step::Complete {
            ch: first as u32,
            len: 1,
        };
    }

    let (expected, min_code_point, mut code): (usize, u32, u32) = match first {
        0xC2..=0xDF => (2, 0x80, (first & 0x1F) as u32),
        0xE0..=0xEF => (3, 0x800, (first & 0x0F) as u32),
        0xF0..=0xF4 => (4, 0x10000, (first & 0x07) as u32),
        _ => return Utf8Step::Invalid,
    };

    if bytes.len() < expected {
        for &b in &bytes[1..] {
            if (b & 0xC0) != 0x80 {
                return Utf8Step::Invalid;
            }
        }
        return Utf8Step::Incomplete { expected };
    }

    for &b in &bytes[1..expected] {
        if (b & 0xC0) != 0x80 {
            return Utf8Step::Invalid;
        }
        code = (code << 6) | u32::from(b & 0x3F);
    }

    if code < min_code_point || code > 0x10FFFF || (0xD800..=0xDFFF).contains(&code) {
        return Utf8Step::Invalid;
    }

    Utf8Step::Complete {
        ch: code,
        len: expected,
    }
}
