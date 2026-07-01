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

impl Default for Mbrtoc32Status {
    fn default() -> Self {
        Self::Incomplete
    }
}

impl Mbrtoc32 {
    pub fn mbrtoc_32(
        output: Option<&mut u32>,
        input: Option<&[u8]>,
        n: usize,
        state: Option<&mut Mbrtoc32>,
    ) -> Mbrtoc32Status {
        let mut internal_state = Mbrtoc32::default();
        let state = match state {
            Some(state) => state,
            None => &mut internal_state,
        };

        let (effective_output, source, limit) = match input {
            Some(bytes) => (output, bytes, n.min(bytes.len())),
            None => (None, b"".as_slice(), 1),
        };

        if limit == 0 {
            return Mbrtoc32Status::Incomplete;
        }

        let mut combined = Vec::with_capacity(state.pending.len() + limit.min(4));
        combined.extend_from_slice(&state.pending);
        combined.extend_from_slice(&source[..limit.min(4)]);

        match decode_one_utf8(&combined) {
            DecodeOne::Complete { len, ch } => {
                if len <= state.pending.len() {
                    state.pending.clear();
                    return Mbrtoc32Status::Invalid;
                }

                let consumed_from_input = len - state.pending.len();
                state.pending.clear();

                if let Some(slot) = effective_output {
                    *slot = ch;
                }

                if ch == 0 {
                    Mbrtoc32Status::Null {
                        bytes: consumed_from_input,
                    }
                } else {
                    Mbrtoc32Status::Complete {
                        bytes: consumed_from_input,
                        ch,
                    }
                }
            }
            DecodeOne::Incomplete => {
                state.pending = combined;
                Mbrtoc32Status::Incomplete
            }
            DecodeOne::Invalid => {
                state.pending.clear();
                Mbrtoc32Status::Invalid
            }
        }
    }

    pub fn run_10() -> bool {
        let mut state = Mbrtoc32::default();

        let mut out = 0;
        match Self::mbrtoc_32(Some(&mut out), Some(b"A"), 1, Some(&mut state)) {
            Mbrtoc32Status::Complete { bytes, ch } => bytes == 1 && ch == 0x41 && out == 0x41,
            _ => false,
        }
    }
}

enum DecodeOne {
    Complete { len: usize, ch: u32 },
    Incomplete,
    Invalid,
}

fn decode_one_utf8(bytes: &[u8]) -> DecodeOne {
    if bytes.is_empty() {
        return DecodeOne::Incomplete;
    }

    let first = bytes[0];

    if first <= 0x7f {
        return DecodeOne::Complete {
            len: 1,
            ch: first as u32,
        };
    }

    let (expected_len, min_cp, mut codepoint) = match first {
        0xc2..=0xdf => (2, 0x80, (first & 0x1f) as u32),
        0xe0..=0xef => (3, 0x800, (first & 0x0f) as u32),
        0xf0..=0xf4 => (4, 0x10000, (first & 0x07) as u32),
        _ => return DecodeOne::Invalid,
    };

    if bytes.len() < expected_len {
        return DecodeOne::Incomplete;
    }

    for &byte in &bytes[1..expected_len] {
        if (byte & 0xc0) != 0x80 {
            return DecodeOne::Invalid;
        }
        codepoint = (codepoint << 6) | (byte & 0x3f) as u32;
    }

    if codepoint < min_cp || codepoint > 0x10ffff || (0xd800..=0xdfff).contains(&codepoint) {
        return DecodeOne::Invalid;
    }

    DecodeOne::Complete {
        len: expected_len,
        ch: codepoint,
    }
}
