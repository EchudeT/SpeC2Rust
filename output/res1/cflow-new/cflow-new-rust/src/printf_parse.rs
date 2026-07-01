use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrintfDirective {
    pub start: usize,
    pub end: usize,
    pub position: Option<usize>,
    pub flags: String,
    pub width: Option<usize>,
    pub width_from_arg: Option<usize>,
    pub precision: Option<usize>,
    pub precision_from_arg: Option<usize>,
    pub length: Option<String>,
    pub conversion: char,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrintfArgKind {
    Int,
    UInt,
    Float,
    Char,
    String,
    Pointer,
    Count,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrintfArgSpec {
    pub index: usize,
    pub kind: PrintfArgKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrintfParse {
    pub directives: Vec<PrintfDirective>,
    pub arguments: Vec<PrintfArgSpec>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrintfParseError {
    message: String,
    offset: usize,
}

impl PrintfParseError {
    pub fn new(message: impl Into<String>, offset: usize) -> Self {
        Self {
            message: message.into(),
            offset,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}

impl fmt::Display for PrintfParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at byte {}", self.message, self.offset)
    }
}

impl std::error::Error for PrintfParseError {}

impl PrintfParse {
    pub fn parse(format: &str) -> Result<Self, PrintfParseError> {
        let mut parser = ParserState::new(format);
        parser.parse()
    }
}

struct ParserState<'a> {
    format: &'a str,
    directives: Vec<PrintfDirective>,
    arguments: Vec<Option<PrintfArgKind>>,
    next_arg_index: usize,
}

impl<'a> ParserState<'a> {
    fn new(format: &'a str) -> Self {
        Self {
            format,
            directives: Vec::new(),
            arguments: Vec::new(),
            next_arg_index: 0,
        }
    }

    fn parse(&mut self) -> Result<PrintfParse, PrintfParseError> {
        let bytes = self.format.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            if bytes[i] != b'%' {
                i += 1;
                continue;
            }

            if i + 1 < bytes.len() && bytes[i + 1] == b'%' {
                i += 2;
                continue;
            }

            let start = i;
            i += 1;

            let position = self.parse_explicit_position(&mut i)?;
            let flags = self.parse_flags(&mut i);
            let width_from_arg;
            let width;
            (width, width_from_arg) = self.parse_width(&mut i)?;
            let precision_from_arg;
            let precision;
            (precision, precision_from_arg) = self.parse_precision(&mut i)?;
            let length = self.parse_length(&mut i);
            let conversion = self.parse_conversion(&mut i)?;

            let kind = Self::arg_kind_for_conversion(conversion).ok_or_else(|| {
                PrintfParseError::new(
                    format!("unsupported conversion '{}'", conversion),
                    i.saturating_sub(1),
                )
            })?;

            let main_index = match position {
                Some(index) => index,
                None => self.take_next_arg(),
            };

            self.register_argument(main_index, kind.clone())?;

            if let Some(idx) = width_from_arg {
                self.register_argument(idx, PrintfArgKind::Int)?;
            }
            if let Some(idx) = precision_from_arg {
                self.register_argument(idx, PrintfArgKind::Int)?;
            }

            self.directives.push(PrintfDirective {
                start,
                end: i,
                position,
                flags,
                width,
                width_from_arg,
                precision,
                precision_from_arg,
                length,
                conversion,
            });
        }

        let arguments = self
            .arguments
            .iter()
            .enumerate()
            .filter_map(|(index, kind)| {
                kind.clone().map(|kind| PrintfArgSpec { index, kind })
            })
            .collect();

        Ok(PrintfParse {
            directives: std::mem::take(&mut self.directives),
            arguments,
        })
    }

    fn parse_explicit_position(&mut self, i: &mut usize) -> Result<Option<usize>, PrintfParseError> {
        let start = *i;
        let value = self.parse_number(i);

        if let Some(n) = value {
            if *i < self.format.len() && self.format.as_bytes()[*i] == b'$' {
                *i += 1;
                if n == 0 {
                    return Err(PrintfParseError::new(
                        "argument positions are 1-based",
                        start,
                    ));
                }
                return Ok(Some(n - 1));
            }
        }

        *i = start;
        Ok(None)
    }

    fn parse_flags(&self, i: &mut usize) -> String {
        let mut out = String::new();
        while *i < self.format.len() {
            let c = self.format.as_bytes()[*i] as char;
            match c {
                '\'' | '-' | '+' | ' ' | '#' | '0' => {
                    out.push(c);
                    *i += 1;
                }
                _ => break,
            }
        }
        out
    }

    fn parse_width(
        &mut self,
        i: &mut usize,
    ) -> Result<(Option<usize>, Option<usize>), PrintfParseError> {
        if *i >= self.format.len() {
            return Ok((None, None));
        }

        if self.format.as_bytes()[*i] == b'*' {
            *i += 1;
            let position = self.parse_star_position(i)?;
            let idx = position.unwrap_or_else(|| self.take_next_arg());
            return Ok((None, Some(idx)));
        }

        Ok((self.parse_number(i), None))
    }

    fn parse_precision(
        &mut self,
        i: &mut usize,
    ) -> Result<(Option<usize>, Option<usize>), PrintfParseError> {
        if *i >= self.format.len() || self.format.as_bytes()[*i] != b'.' {
            return Ok((None, None));
        }

        *i += 1;

        if *i < self.format.len() && self.format.as_bytes()[*i] == b'*' {
            *i += 1;
            let position = self.parse_star_position(i)?;
            let idx = position.unwrap_or_else(|| self.take_next_arg());
            return Ok((None, Some(idx)));
        }

        Ok((Some(self.parse_number(i).unwrap_or(0)), None))
    }

    fn parse_star_position(
        &mut self,
        i: &mut usize,
    ) -> Result<Option<usize>, PrintfParseError> {
        let start = *i;
        let n = self.parse_number(i);

        if let Some(n) = n {
            if *i < self.format.len() && self.format.as_bytes()[*i] == b'$' {
                *i += 1;
                if n == 0 {
                    return Err(PrintfParseError::new(
                        "argument positions are 1-based",
                        start,
                    ));
                }
                return Ok(Some(n - 1));
            }
        }

        *i = start;
        Ok(None)
    }

    fn parse_length(&self, i: &mut usize) -> Option<String> {
        let remaining = &self.format[*i..];
        for prefix in ["hh", "ll", "h", "l", "L", "j", "z", "t"] {
            if remaining.starts_with(prefix) {
                *i += prefix.len();
                return Some(prefix.to_string());
            }
        }
        None
    }

    fn parse_conversion(&self, i: &mut usize) -> Result<char, PrintfParseError> {
        if *i >= self.format.len() {
            return Err(PrintfParseError::new(
                "unterminated conversion directive",
                self.format.len(),
            ));
        }

        let c = self.format.as_bytes()[*i] as char;
        *i += 1;
        Ok(c)
    }

    fn parse_number(&self, i: &mut usize) -> Option<usize> {
        let start = *i;
        let bytes = self.format.as_bytes();
        let mut value: usize = 0;

        while *i < bytes.len() && bytes[*i].is_ascii_digit() {
            value = value
                .saturating_mul(10)
                .saturating_add((bytes[*i] - b'0') as usize);
            *i += 1;
        }

        if *i == start { None } else { Some(value) }
    }

    fn take_next_arg(&mut self) -> usize {
        let idx = self.next_arg_index;
        self.next_arg_index += 1;
        idx
    }

    fn register_argument(
        &mut self,
        index: usize,
        kind: PrintfArgKind,
    ) -> Result<(), PrintfParseError> {
        if self.arguments.len() <= index {
            self.arguments.resize(index + 1, None);
        }

        match &self.arguments[index] {
            None => {
                self.arguments[index] = Some(kind);
                Ok(())
            }
            Some(existing) if *existing == kind => Ok(()),
            Some(existing) => Err(PrintfParseError::new(
                format!(
                    "argument {} used with incompatible types: {:?} and {:?}",
                    index + 1,
                    existing,
                    kind
                ),
                0,
            )),
        }
    }

    fn arg_kind_for_conversion(c: char) -> Option<PrintfArgKind> {
        match c {
            'd' | 'i' => Some(PrintfArgKind::Int),
            'o' | 'u' | 'x' | 'X' => Some(PrintfArgKind::UInt),
            'f' | 'F' | 'e' | 'E' | 'g' | 'G' | 'a' | 'A' => Some(PrintfArgKind::Float),
            'c' => Some(PrintfArgKind::Char),
            's' => Some(PrintfArgKind::String),
            'p' => Some(PrintfArgKind::Pointer),
            'n' => Some(PrintfArgKind::Count),
            _ => None,
        }
    }
}
