#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Mbrtoc32 {
    pending: [u8; 4],
    pending_len: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Mbrtoc32Status {
    Complete { ch: char, consumed: usize },
    Null { consumed: usize },
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
        &mut self,
        input: Option<&[u8]>,
        output: Option<&mut u32>,
    ) -> Mbrtoc32Status {
        let bytes = input.unwrap_or(&[0]);

        if bytes.is_empty() {
            return Mbrtoc32Status::Incomplete;
        }

        let first = if self.pending_len == 0 {
            bytes[0]
        } else {
            self.pending[0]
        };

        let expected_len = match utf8_expected_len(first) {
            Some(len) => len,
            None => {
                self.pending_len = 0;
                return Mbrtoc32Status::Invalid;
            }
        };

        let mut seq = [0u8; 4];
        let mut seq_len = 0usize;

        for &b in &self.pending[..self.pending_len] {
            seq[seq_len] = b;
            seq_len += 1;
        }

        let mut consumed_from_input = 0usize;
        while seq_len < expected_len && consumed_from_input < bytes.len() {
            seq[seq_len] = bytes[consumed_from_input];
            seq_len += 1;
            consumed_from_input += 1;
        }

        if self.pending_len == 0 && consumed_from_input == 0 {
            seq[0] = bytes[0];
            seq_len = 1;
            consumed_from_input = 1;
            while seq_len < expected_len && consumed_from_input < bytes.len() {
                seq[seq_len] = bytes[consumed_from_input];
                seq_len += 1;
                consumed_from_input += 1;
            }
        }

        if seq_len < expected_len {
            self.pending[..seq_len].copy_from_slice(&seq[..seq_len]);
            self.pending_len = seq_len;
            return Mbrtoc32Status::Incomplete;
        }

        if !utf8_sequence_is_valid(&seq[..expected_len]) {
            self.pending_len = 0;
            return Mbrtoc32Status::Invalid;
        }

        let ch = match std::str::from_utf8(&seq[..expected_len])
            .ok()
            .and_then(|s| s.chars().next())
        {
            Some(ch) => ch,
            None => {
                self.pending_len = 0;
                return Mbrtoc32Status::Invalid;
            }
        };

        self.pending_len = 0;

        if let Some(slot) = output {
            *slot = ch as u32;
        }

        if ch == '\0' {
            Mbrtoc32Status::Null {
                consumed: expected_len,
            }
        } else {
            Mbrtoc32Status::Complete {
                ch,
                consumed: expected_len,
            }
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

fn utf8_sequence_is_valid(seq: &[u8]) -> bool {
    match seq.len() {
        1 => seq[0] <= 0x7f,
        2 => {
            (0xc2..=0xdf).contains(&seq[0]) && is_continuation(seq[1])
        }
        3 => match seq[0] {
            0xe0 => (0xa0..=0xbf).contains(&seq[1]) && is_continuation(seq[2]),
            0xe1..=0xec | 0xee..=0xef => {
                is_continuation(seq[1]) && is_continuation(seq[2])
            }
            0xed => (0x80..=0x9f).contains(&seq[1]) && is_continuation(seq[2]),
            _ => false,
        },
        4 => match seq[0] {
            0xf0 => {
                (0x90..=0xbf).contains(&seq[1])
                    && is_continuation(seq[2])
                    && is_continuation(seq[3])
            }
            0xf1..=0xf3 => {
                is_continuation(seq[1])
                    && is_continuation(seq[2])
                    && is_continuation(seq[3])
            }
            0xf4 => {
                (0x80..=0x8f).contains(&seq[1])
                    && is_continuation(seq[2])
                    && is_continuation(seq[3])
            }
            _ => false,
        },
        _ => false,
    }
}

fn is_continuation(byte: u8) -> bool {
    (0x80..=0xbf).contains(&byte)
}
