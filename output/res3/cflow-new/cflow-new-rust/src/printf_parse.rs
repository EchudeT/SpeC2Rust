use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrintfParse {
    items: Vec<FormatItem>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FormatItem {
    Text(String),
    Directive(Directive),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Directive {
    pub parameter: Option<usize>,
    pub flags: Vec<Flag>,
    pub width: Count,
    pub precision: Count,
    pub length: Option<LengthModifier>,
    pub conversion: char,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Flag {
    Alternate,
    ZeroPad,
    LeftAdjust,
    SpaceSign,
    PlusSign,
    Group,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Count {
    None,
    Number(usize),
    FromArgument(Option<usize>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LengthModifier {
    Char,
    Short,
    Long,
    LongLong,
    IntMax,
    Size,
    PtrDiff,
    LongDouble,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArgumentUse {
    pub index: usize,
    pub kind: ArgumentKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArgumentKind {
    Value(char),
    Width,
    Precision,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    message: String,
    position: usize,
}

impl ParseError {
    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn position(&self) -> usize {
        self.position
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at byte {}", self.message, self.position)
    }
}

impl std::error::Error for ParseError {}

impl PrintfParse {
    pub fn parse(format: &str) -> Result<Self, ParseError> {
        let mut parser = ParserState::new(format);
        parser.parse_all()?;
        Ok(Self { items: parser.items })
    }

    pub fn items(&self) -> &[FormatItem] {
        &self.items
    }

    pub fn directives(&self) -> impl Iterator<Item = &Directive> {
        self.items.iter().filter_map(|item| match item {
            FormatItem::Directive(d) => Some(d),
            FormatItem::Text(_) => None,
        })
    }

    pub fn argument_uses(&self) -> Vec<ArgumentUse> {
        let mut out = Vec::new();
        for directive in self.directives() {
            if let Count::FromArgument(index) = &directive.width {
                out.push(ArgumentUse {
                    index: index.unwrap_or(0),
                    kind: ArgumentKind::Width,
                });
            }
            if let Count::FromArgument(index) = &directive.precision {
                out.push(ArgumentUse {
                    index: index.unwrap_or(0),
                    kind: ArgumentKind::Precision,
                });
            }
            if directive.conversion != '%' {
                if let Some(index) = directive.parameter {
                    out.push(ArgumentUse {
                        index,
                        kind: ArgumentKind::Value(directive.conversion),
                    });
                }
            }
        }
        out
    }
}

struct ParserState<'a> {
    input: &'a str,
    pos: usize,
    next_sequential_arg: usize,
    items: Vec<FormatItem>,
}

impl<'a> ParserState<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            next_sequential_arg: 1,
            items: Vec::new(),
        }
    }

    fn parse_all(&mut self) -> Result<(), ParseError> {
        let mut text_start = self.pos;

        while let Some(ch) = self.peek_char() {
            if ch == '%' {
                if text_start < self.pos {
                    self.items
                        .push(FormatItem::Text(self.input[text_start..self.pos].to_string()));
                }
                let directive = self.parse_directive()?;
                self.items.push(FormatItem::Directive(directive));
                text_start = self.pos;
            } else {
                self.bump_char();
            }
        }

        if text_start < self.pos {
            self.items
                .push(FormatItem::Text(self.input[text_start..self.pos].to_string()));
        }

        Ok(())
    }

    fn parse_directive(&mut self) -> Result<Directive, ParseError> {
        let start = self.pos;
        self.expect_char('%')?;

        if self.peek_char() == Some('%') {
            self.bump_char();
            return Ok(Directive {
                parameter: None,
                flags: Vec::new(),
                width: Count::None,
                precision: Count::None,
                length: None,
                conversion: '%',
            });
        }

        let parameter = self.try_parse_positional_prefix()?;
        let flags = self.parse_flags();
        let width = self.parse_count()?;
        let precision = if self.peek_char() == Some('.') {
            self.bump_char();
            self.parse_precision_count()?
        } else {
            Count::None
        };
        let length = self.parse_length();
        let conversion = self
            .peek_char()
            .ok_or_else(|| self.error("unterminated format directive", start))?;

        if !is_conversion(conversion) {
            return Err(self.error("invalid conversion specifier", self.pos));
        }

        self.bump_char();

        let parameter = if conversion == '%' {
            None
        } else if let Some(index) = parameter {
            Some(index)
        } else {
            let index = self.next_sequential_arg;
            self.next_sequential_arg += 1;
            Some(index)
        };

        Ok(Directive {
            parameter,
            flags,
            width,
            precision,
            length,
            conversion,
        })
    }

    fn try_parse_positional_prefix(&mut self) -> Result<Option<usize>, ParseError> {
        let saved = self.pos;
        let number = self.parse_usize();

        match (number, self.peek_char()) {
            (Some(n), Some('$')) => {
                self.bump_char();
                if n == 0 {
                    Err(self.error("argument indexes are 1-based", saved))
                } else {
                    Ok(Some(n))
                }
            }
            _ => {
                self.pos = saved;
                Ok(None)
            }
        }
    }

    fn parse_flags(&mut self) -> Vec<Flag> {
        let mut flags = Vec::new();
        loop {
            let flag = match self.peek_char() {
                Some('#') => Flag::Alternate,
                Some('0') => Flag::ZeroPad,
                Some('-') => Flag::LeftAdjust,
                Some(' ') => Flag::SpaceSign,
                Some('+') => Flag::PlusSign,
                Some('\'') => Flag::Group,
                _ => break,
            };
            self.bump_char();
            if !flags.contains(&flag) {
                flags.push(flag);
            }
        }
        flags
    }

    fn parse_count(&mut self) -> Result<Count, ParseError> {
        match self.peek_char() {
            Some('*') => {
                self.bump_char();
                let positional = self.try_parse_star_position()?;
                Ok(Count::FromArgument(Some(
                    positional.unwrap_or_else(|| self.take_next_argument()),
                )))
            }
            Some(c) if c.is_ascii_digit() => Ok(Count::Number(self.parse_usize().unwrap_or(0))),
            _ => Ok(Count::None),
        }
    }

    fn parse_precision_count(&mut self) -> Result<Count, ParseError> {
        match self.peek_char() {
            Some('*') => {
                self.bump_char();
                let positional = self.try_parse_star_position()?;
                Ok(Count::FromArgument(Some(
                    positional.unwrap_or_else(|| self.take_next_argument()),
                )))
            }
            Some(c) if c.is_ascii_digit() => Ok(Count::Number(self.parse_usize().unwrap_or(0))),
            _ => Ok(Count::Number(0)),
        }
    }

    fn try_parse_star_position(&mut self) -> Result<Option<usize>, ParseError> {
        let saved = self.pos;
        let number = self.parse_usize();

        match (number, self.peek_char()) {
            (Some(n), Some('$')) => {
                self.bump_char();
                if n == 0 {
                    Err(self.error("argument indexes are 1-based", saved))
                } else {
                    Ok(Some(n))
                }
            }
            _ => {
                self.pos = saved;
                Ok(None)
            }
        }
    }

    fn parse_length(&mut self) -> Option<LengthModifier> {
        match (self.peek_char(), self.peek_next_char()) {
            (Some('h'), Some('h')) => {
                self.bump_char();
                self.bump_char();
                Some(LengthModifier::Char)
            }
            (Some('l'), Some('l')) => {
                self.bump_char();
                self.bump_char();
                Some(LengthModifier::LongLong)
            }
            _ => match self.peek_char() {
                Some('h') => {
                    self.bump_char();
                    Some(LengthModifier::Short)
                }
                Some('l') => {
                    self.bump_char();
                    Some(LengthModifier::Long)
                }
                Some('L') => {
                    self.bump_char();
                    Some(LengthModifier::LongDouble)
                }
                Some('j') => {
                    self.bump_char();
                    Some(LengthModifier::IntMax)
                }
                Some('z') => {
                    self.bump_char();
                    Some(LengthModifier::Size)
                }
                Some('t') => {
                    self.bump_char();
                    Some(LengthModifier::PtrDiff)
                }
                _ => None,
            },
        }
    }

    fn parse_usize(&mut self) -> Option<usize> {
        let start = self.pos;
        while matches!(self.peek_char(), Some(c) if c.is_ascii_digit()) {
            self.bump_char();
        }
        if self.pos == start {
            None
        } else {
            self.input[start..self.pos].parse().ok()
        }
    }

    fn take_next_argument(&mut self) -> usize {
        let index = self.next_sequential_arg;
        self.next_sequential_arg += 1;
        index
    }

    fn expect_char(&mut self, expected: char) -> Result<(), ParseError> {
        match self.peek_char() {
            Some(ch) if ch == expected => {
                self.bump_char();
                Ok(())
            }
            _ => Err(self.error("unexpected character", self.pos)),
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn peek_next_char(&self) -> Option<char> {
        let mut iter = self.input[self.pos..].chars();
        iter.next()?;
        iter.next()
    }

    fn bump_char(&mut self) {
        if let Some(ch) = self.peek_char() {
            self.pos += ch.len_utf8();
        }
    }

    fn error(&self, message: &str, position: usize) -> ParseError {
        ParseError {
            message: message.to_string(),
            position,
        }
    }
}

fn is_conversion(c: char) -> bool {
    matches!(
        c,
        'd' | 'i'
            | 'o'
            | 'u'
            | 'x'
            | 'X'
            | 'f'
            | 'F'
            | 'e'
            | 'E'
            | 'g'
            | 'G'
            | 'a'
            | 'A'
            | 'c'
            | 's'
            | 'p'
            | 'n'
            | 'm'
            | 'C'
            | 'S'
            | 'U'
    )
}
