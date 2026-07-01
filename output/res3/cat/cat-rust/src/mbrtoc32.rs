#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Mbrtoc32;

#[derive(Clone, Debug, Eq, PartialEq)]
struct DecodeState {
    pending: [u8; 4],
    pending_len: usize,
}

impl Default for DecodeState {
    fn default() -> Self {
        Self {
            pending: [0; 4],
            pending_len: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mbrtoc32Result {
    Complete { bytes_read: usize, ch: u32 },
    CompleteNul,
    Incomplete,
    Invalid,
}

impl Mbrtoc32 {
    pub fn mbrtoc_32(
        output: Option<&mut u32>,
        input: Option<&[u8]>,
        state: Option<&mut Vec<u8>>,
    ) -> Mbrtoc32Result {
        let mut internal_state = Vec::new();
        let state_buf = match state {
            Some(existing) => existing,
            None => &mut internal_state,
        };

        let effective_input = match input {
            Some(bytes) => bytes,
            None => b"\0".as_slice(),
        };

        if effective_input.is_empty() {
            return Mbrtoc32Result::Incomplete;
        }

        let mut decoded_state = Self::decode_state_from_bytes(state_buf);
        let mut scratch = [0u8; 4];
        let mut total_len = 0usize;

        for &b in decoded_state.pending[..decoded_state.pending_len].iter() {
            scratch[total_len] = b;
            total_len += 1;
        }

        let appended = effective_input.len().min(4usize.saturating_sub(total_len));
        for &b in &effective_input[..appended] {
            scratch[total_len] = b;
            total_len += 1;
        }

        let result = Self::decode_one_utf8(&scratch[..total_len]);

        match result {
            Utf8Decode::Complete { ch, len } => {
                if decoded_state.pending_len >= len.max(1) {
                    return Mbrtoc32Result::Invalid;
                }

                let consumed_from_new = len.saturating_sub(decoded_state.pending_len);
                decoded_state.pending_len = 0;
                Self::store_state(state_buf, &decoded_state);

                if let Some(slot) = output {
                    *slot = ch;
                }

                if ch == 0 {
                    Mbrtoc32Result::CompleteNul
                } else {
                    Mbrtoc32Result::Complete {
                        bytes_read: consumed_from_new,
                        ch,
                    }
                }
            }
            Utf8Decode::Incomplete { needed_prefix_len } => {
                let to_store = total_len.min(needed_prefix_len).min(4);
                decoded_state.pending[..to_store].copy_from_slice(&scratch[..to_store]);
                decoded_state.pending_len = to_store;
                Self::store_state(state_buf, &decoded_state);
                Mbrtoc32Result::Incomplete
            }
            Utf8Decode::Invalid => Mbrtoc32Result::Invalid,
        }
    }

    fn decode_state_from_bytes(bytes: &[u8]) -> DecodeState {
        if bytes.is_empty() {
            return DecodeState::default();
        }

        let pending_len = bytes[0] as usize;
        if pending_len == 0 || pending_len > 4 || bytes.len() < pending_len + 1 {
            return DecodeState::default();
        }

        let mut pending = [0u8; 4];
        pending[..pending_len].copy_from_slice(&bytes[1..=pending_len]);
        DecodeState {
            pending,
            pending_len,
        }
    }

    fn store_state(buf: &mut Vec<u8>, state: &DecodeState) {
        buf.clear();
        buf.push(state.pending_len as u8);
        buf.extend_from_slice(&state.pending[..state.pending_len]);
    }

    fn decode_one_utf8(bytes: &[u8]) -> Utf8Decode {
        if bytes.is_empty() {
            return Utf8Decode::Incomplete {
                needed_prefix_len: 1,
            };
        }

        let b0 = bytes[0];
        if b0 <= 0x7f {
            return Utf8Decode::Complete {
                ch: b0 as u32,
                len: 1,
            };
        }

        if (0xc2..=0xdf).contains(&b0) {
            if bytes.len() < 2 {
                return Utf8Decode::Incomplete {
                    needed_prefix_len: 2,
                };
            }
            let b1 = bytes[1];
            if !Self::is_continuation(b1) {
                return Utf8Decode::Invalid;
            }
            let ch = ((b0 as u32 & 0x1f) << 6) | (b1 as u32 & 0x3f);
            return Utf8Decode::Complete { ch, len: 2 };
        }

        if b0 == 0xe0 {
            if bytes.len() < 3 {
                return Utf8Decode::Incomplete {
                    needed_prefix_len: 3,
                };
            }
            let b1 = bytes[1];
            let b2 = bytes[2];
            if !(0xa0..=0xbf).contains(&b1) || !Self::is_continuation(b2) {
                return Utf8Decode::Invalid;
            }
            let ch = ((b0 as u32 & 0x0f) << 12) | ((b1 as u32 & 0x3f) << 6) | (b2 as u32 & 0x3f);
            return Utf8Decode::Complete { ch, len: 3 };
        }

        if (0xe1..=0xec).contains(&b0) || (0xee..=0xef).contains(&b0) {
            if bytes.len() < 3 {
                return Utf8Decode::Incomplete {
                    needed_prefix_len: 3,
                };
            }
            let b1 = bytes[1];
            let b2 = bytes[2];
            if !Self::is_continuation(b1) || !Self::is_continuation(b2) {
                return Utf8Decode::Invalid;
            }
            let ch = ((b0 as u32 & 0x0f) << 12) | ((b1 as u32 & 0x3f) << 6) | (b2 as u32 & 0x3f);
            return Utf8Decode::Complete { ch, len: 3 };
        }

        if b0 == 0xed {
            if bytes.len() < 3 {
                return Utf8Decode::Incomplete {
                    needed_prefix_len: 3,
                };
            }
            let b1 = bytes[1];
            let b2 = bytes[2];
            if !(0x80..=0x9f).contains(&b1) || !Self::is_continuation(b2) {
                return Utf8Decode::Invalid;
            }
            let ch = ((b0 as u32 & 0x0f) << 12) | ((b1 as u32 & 0x3f) << 6) | (b2 as u32 & 0x3f);
            return Utf8Decode::Complete { ch, len: 3 };
        }

        if b0 == 0xf0 {
            if bytes.len() < 4 {
                return Utf8Decode::Incomplete {
                    needed_prefix_len: 4,
                };
            }
            let b1 = bytes[1];
            let b2 = bytes[2];
            let b3 = bytes[3];
            if !(0x90..=0xbf).contains(&b1)
                || !Self::is_continuation(b2)
                || !Self::is_continuation(b3)
            {
                return Utf8Decode::Invalid;
            }
            let ch = ((b0 as u32 & 0x07) << 18)
                | ((b1 as u32 & 0x3f) << 12)
                | ((b2 as u32 & 0x3f) << 6)
                | (b3 as u32 & 0x3f);
            return Utf8Decode::Complete { ch, len: 4 };
        }

        if (0xf1..=0xf3).contains(&b0) {
            if bytes.len() < 4 {
                return Utf8Decode::Incomplete {
                    needed_prefix_len: 4,
                };
            }
            let b1 = bytes[1];
            let b2 = bytes[2];
            let b3 = bytes[3];
            if !Self::is_continuation(b1)
                || !Self::is_continuation(b2)
                || !Self::is_continuation(b3)
            {
                return Utf8Decode::Invalid;
            }
            let ch = ((b0 as u32 & 0x07) << 18)
                | ((b1 as u32 & 0x3f) << 12)
                | ((b2 as u32 & 0x3f) << 6)
                | (b3 as u32 & 0x3f);
            return Utf8Decode::Complete { ch, len: 4 };
        }

        if b0 == 0xf4 {
            if bytes.len() < 4 {
                return Utf8Decode::Incomplete {
                    needed_prefix_len: 4,
                };
            }
            let b1 = bytes[1];
            let b2 = bytes[2];
            let b3 = bytes[3];
            if !(0x80..=0x8f).contains(&b1)
                || !Self::is_continuation(b2)
                || !Self::is_continuation(b3)
            {
                return Utf8Decode::Invalid;
            }
            let ch = ((b0 as u32 & 0x07) << 18)
                | ((b1 as u32 & 0x3f) << 12)
                | ((b2 as u32 & 0x3f) << 6)
                | (b3 as u32 & 0x3f);
            return Utf8Decode::Complete { ch, len: 4 };
        }

        Utf8Decode::Invalid
    }

    fn is_continuation(byte: u8) -> bool {
        (byte & 0xc0) == 0x80
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Utf8Decode {
    Complete { ch: u32, len: usize },
    Incomplete { needed_prefix_len: usize },
    Invalid,
}
