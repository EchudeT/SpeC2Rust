#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct DecodeState {
    partial: [u8; 4],
    len: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Mbrtoc32Result {
    Complete { ch: u32, bytes: usize },
    Null { bytes: usize },
    Incomplete,
    Invalid,
}

pub struct Mbrtoc32 {
    partial: [u8; 4],
    len: usize,
}

impl Mbrtoc32 {
    pub fn mbrtoc_32(
        output: Option<&mut u32>,
        input: Option<&[u8]>,
        n: usize,
        state: Option<&mut Mbrtoc32>,
    ) -> Mbrtoc32Result {
        let (effective_input, limit) = match input {
            Some(bytes) => (bytes, n.min(bytes.len())),
            None => (&b""[..], 1),
        };

        let state_ref = match state {
            Some(state) => state,
            None => return Mbrtoc32Result::Invalid,
        };

        Self::decode_one(output, effective_input, limit, state_ref)
    }

    pub fn run_09() -> i32 {
        let mut state = Mbrtoc32::default();
        let mut out = 0_u32;

        match Self::mbrtoc_32(Some(&mut out), Some(b"A"), 1, Some(&mut state)) {
            Mbrtoc32Result::Complete { ch, bytes } if ch == u32::from(b'A') && bytes == 1 => 0,
            _ => 1,
        }
    }

    fn decode_one(
        output: Option<&mut u32>,
        input: &[u8],
        n: usize,
        state: &mut Mbrtoc32,
    ) -> Mbrtoc32Result {
        if n == 0 {
            return Mbrtoc32Result::Incomplete;
        }

        let mut combined = [0u8; 4];
        let mut combined_len = state.len;

        if combined_len > combined.len() {
            state.reset();
            return Mbrtoc32Result::Invalid;
        }

        combined[..combined_len].copy_from_slice(&state.partial[..combined_len]);

        let to_take = (combined.len().saturating_sub(combined_len)).min(n);
        combined[combined_len..combined_len + to_take].copy_from_slice(&input[..to_take]);
        combined_len += to_take;

        let first = combined[0];
        let expected = match utf8_expected_len(first) {
            Some(v) => v,
            None => {
                state.reset();
                return Mbrtoc32Result::Invalid;
            }
        };

        if expected == 0 {
            state.reset();
            if let Some(slot) = output {
                *slot = 0;
            }
            return Mbrtoc32Result::Null { bytes: 0 };
        }

        if state.len == 0 && n < expected {
            state.partial[..n].copy_from_slice(&input[..n]);
            state.len = n;
            return Mbrtoc32Result::Incomplete;
        }

        if combined_len < expected {
            state.partial[..combined_len].copy_from_slice(&combined[..combined_len]);
            state.len = combined_len;
            return Mbrtoc32Result::Incomplete;
        }

        let seq = &combined[..expected];
        let ch = match decode_utf8_exact(seq) {
            Some(ch) => ch,
            None => {
                state.reset();
                return Mbrtoc32Result::Invalid;
            }
        };

        state.reset();

        if let Some(slot) = output {
            *slot = ch;
        }

        if ch == 0 {
            Mbrtoc32Result::Null { bytes: 0 }
        } else {
            let consumed_from_input = expected.saturating_sub(state_consumed_prefix_len(expected, n, seq, input));
            Mbrtoc32Result::Complete {
                ch,
                bytes: consumed_from_input,
            }
        }
    }

    fn reset(&mut self) {
        self.partial = [0; 4];
        self.len = 0;
    }

}

impl Default for Mbrtoc32 {
    fn default() -> Self {
        Self {
            partial: [0; 4],
            len: 0,
        }
    }
}


fn utf8_expected_len(first: u8) -> Option<usize> {
    match first {
        0x00 => Some(0),
        0x01..=0x7f => Some(1),
        0xc2..=0xdf => Some(2),
        0xe0..=0xef => Some(3),
        0xf0..=0xf4 => Some(4),
        _ => None,
    }
}

fn decode_utf8_exact(bytes: &[u8]) -> Option<u32> {
    match bytes.len() {
        1 => Some(bytes[0] as u32),
        2 => {
            let b0 = bytes[0];
            let b1 = bytes[1];
            if (b1 & 0xc0) != 0x80 || !(0xc2..=0xdf).contains(&b0) {
                return None;
            }
            Some((((b0 & 0x1f) as u32) << 6) | ((b1 & 0x3f) as u32))
        }
        3 => {
            let b0 = bytes[0];
            let b1 = bytes[1];
            let b2 = bytes[2];
            if (b1 & 0xc0) != 0x80 || (b2 & 0xc0) != 0x80 {
                return None;
            }
            match b0 {
                0xe0 if !(0xa0..=0xbf).contains(&b1) => return None,
                0xe1..=0xec | 0xee..=0xef if !(0x80..=0xbf).contains(&b1) => return None,
                0xed if !(0x80..=0x9f).contains(&b1) => return None,
                _ if !(0xe0..=0xef).contains(&b0) => return None,
                _ => {}
            }
            Some(
                (((b0 & 0x0f) as u32) << 12)
                    | (((b1 & 0x3f) as u32) << 6)
                    | ((b2 & 0x3f) as u32),
            )
        }
        4 => {
            let b0 = bytes[0];
            let b1 = bytes[1];
            let b2 = bytes[2];
            let b3 = bytes[3];
            if (b1 & 0xc0) != 0x80 || (b2 & 0xc0) != 0x80 || (b3 & 0xc0) != 0x80 {
                return None;
            }
            match b0 {
                0xf0 if !(0x90..=0xbf).contains(&b1) => return None,
                0xf1..=0xf3 if !(0x80..=0xbf).contains(&b1) => return None,
                0xf4 if !(0x80..=0x8f).contains(&b1) => return None,
                _ if !(0xf0..=0xf4).contains(&b0) => return None,
                _ => {}
            }
            Some(
                (((b0 & 0x07) as u32) << 18)
                    | (((b1 & 0x3f) as u32) << 12)
                    | (((b2 & 0x3f) as u32) << 6)
                    | ((b3 & 0x3f) as u32),
            )
        }
        _ => None,
    }
}

fn state_consumed_prefix_len(expected: usize, _n: usize, seq: &[u8], input: &[u8]) -> usize {
    let mut prefix = 0usize;
    let max_prefix = expected.min(seq.len());
    while prefix < max_prefix && prefix < input.len() && seq[prefix] == input[prefix] {
        prefix += 1;
    }
    expected.saturating_sub(prefix)
}
