use crate::printf_args::PrintfArgs;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrintfArgKind {
    Int,
    UInt,
    Float,
    Char,
    Str,
    Pointer,
    Count,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PrintfDirective {
    pub span: std::ops::Range<usize>,
    pub start: usize,
    pub end: usize,
    pub flags: String,
    pub width: Option<usize>,
    pub width_from_argument: Option<usize>,
    pub precision: Option<usize>,
    pub precision_from_argument: Option<usize>,
    pub length_modifier: Option<String>,
    pub conversion: char,
    pub argument_index: Option<usize>,
    pub argument_kind: Option<PrintfArgKind>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PrintfParseResult {
    pub directives: Vec<PrintfDirective>,
    pub argument_count: usize,
    pub max_width_length: usize,
    pub max_precision_length: usize,
}

pub struct PrintfParse;

impl PrintfParse {
    pub fn parse(format: &str) -> Result<PrintfParseResult, PrintfParseError> {
        let mut directives = Vec::new();
        let mut chars = format.char_indices().peekable();
        let mut next_regular_argument = 0usize;
        let mut max_width_length = 0usize;
        let mut max_precision_length = 0usize;

        while let Some((start, ch)) = chars.next() {
            if ch != '%' {
                if !ch.is_ascii() {
                    return Err(PrintfParseError::UnsupportedNonAscii { index: start, ch });
                }
                continue;
            }

            if let Some(&(_, '%')) = chars.peek() {
                chars.next();
                continue;
            }

            let mut directive = PrintfDirective {
                start,
                ..PrintfDirective::default()
            };

            let mut explicit_index = Self::parse_positional_index(&mut chars)?;
            if explicit_index.is_some() {
                directive.argument_index = explicit_index.map(|n| n - 1);
            }

            let mut flags = String::new();
            while let Some(&(_, c)) = chars.peek() {
                if matches!(c, '\'' | '-' | '+' | ' ' | '#' | '0' | 'I') {
                    flags.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            directive.flags = flags;

            if let Some(&(_, '*')) = chars.peek() {
                chars.next();
                let width_arg = if let Some(pos) = Self::parse_positional_index(&mut chars)? {
                    pos - 1
                } else {
                    let idx = next_regular_argument;
                    next_regular_argument += 1;
                    idx
                };
                directive.width_from_argument = Some(width_arg);
            } else {
                let width_digits = Self::parse_number(&mut chars);
                if let Some((value, len)) = width_digits {
                    max_width_length = max_width_length.max(len);
                    directive.width = Some(value);
                }
            }

            if let Some(&(_, '.')) = chars.peek() {
                chars.next();
                if let Some(&(_, '*')) = chars.peek() {
                    chars.next();
                    let prec_arg = if let Some(pos) = Self::parse_positional_index(&mut chars)? {
                        pos - 1
                    } else {
                        let idx = next_regular_argument;
                        next_regular_argument += 1;
                        idx
                    };
                    directive.precision_from_argument = Some(prec_arg);
                } else {
                    let precision_digits = Self::parse_number(&mut chars);
                    match precision_digits {
                        Some((value, len)) => {
                            max_precision_length = max_precision_length.max(len);
                            directive.precision = Some(value);
                        }
                        None => {
                            directive.precision = Some(0);
                        }
                    }
                }
            }

            directive.length_modifier = Self::parse_length_modifier(&mut chars);

            let (end, conversion) = chars
                .next()
                .ok_or(PrintfParseError::IncompleteDirective { index: start })?;

            if !conversion.is_ascii() {
                return Err(PrintfParseError::UnsupportedNonAscii {
                    index: end,
                    ch: conversion,
                });
            }

            directive.conversion = conversion;
            directive.argument_kind = Self::conversion_kind(conversion);

            if directive.argument_kind.is_none() {
                return Err(PrintfParseError::InvalidConversion {
                    index: end,
                    conversion,
                });
            }

            if conversion != '%' {
                if explicit_index.is_none() {
                    let idx = next_regular_argument;
                    next_regular_argument += 1;
                    directive.argument_index = Some(idx);
                }
            } else {
                directive.argument_index = None;
                directive.argument_kind = None;
            }

            directive.end = end + conversion.len_utf8();
            directive.span = directive.start..directive.end;
            directives.push(directive);
        }

        Ok(PrintfParseResult {
            directives,
            argument_count: next_regular_argument,
            max_width_length,
            max_precision_length,
        })
    }

    pub fn validate_arguments(
        format: &str,
        args: &PrintfArgs,
    ) -> Result<PrintfParseResult, PrintfParseError> {
        let parsed = Self::parse(format)?;
        if args.len() < parsed.argument_count {
            return Err(PrintfParseError::MissingArguments {
                expected: parsed.argument_count,
                actual: args.len(),
            });
        }
        Ok(parsed)
    }

    fn parse_positional_index<I>(
        chars: &mut std::iter::Peekable<I>,
    ) -> Result<Option<usize>, PrintfParseError>
    where
        I: Iterator<Item = (usize, char)>,
    {
        let mut probe = chars.clone();
        let mut digits = String::new();
        let mut first_index = None;

        while let Some((idx, c)) = probe.peek().copied() {
            if c.is_ascii_digit() {
                if first_index.is_none() {
                    first_index = Some(idx);
                }
                digits.push(c);
                probe.next();
            } else {
                break;
            }
        }

        if digits.is_empty() {
            return Ok(None);
        }

        if let Some((_, '$')) = probe.peek().copied() {
            for _ in 0..digits.len() {
                chars.next();
            }
            chars.next();

            let value = digits
                .parse::<usize>()
                .map_err(|_| PrintfParseError::InvalidPositionalIndex {
                    index: first_index.unwrap_or(0),
                })?;

            if value == 0 {
                return Err(PrintfParseError::InvalidPositionalIndex {
                    index: first_index.unwrap_or(0),
                });
            }

            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn parse_number<I>(chars: &mut std::iter::Peekable<I>) -> Option<(usize, usize)>
    where
        I: Iterator<Item = (usize, char)>,
    {
        let mut digits = String::new();

        while let Some(&(_, c)) = chars.peek() {
            if c.is_ascii_digit() {
                digits.push(c);
                chars.next();
            } else {
                break;
            }
        }

        if digits.is_empty() {
            None
        } else {
            let len = digits.len();
            let value = digits.parse::<usize>().ok()?;
            Some((value, len))
        }
    }

    fn parse_length_modifier<I>(chars: &mut std::iter::Peekable<I>) -> Option<String>
    where
        I: Iterator<Item = (usize, char)>,
    {
        let mut probe = chars.clone();
        let first = probe.next()?.1;
        let second = probe.peek().map(|(_, c)| *c);

        let token = match (first, second) {
            ('h', Some('h')) => Some("hh"),
            ('l', Some('l')) => Some("ll"),
            ('j', _) => Some("j"),
            ('z', _) => Some("z"),
            ('t', _) => Some("t"),
            ('L', _) => Some("L"),
            ('h', _) => Some("h"),
            ('l', _) => Some("l"),
            _ => None,
        }?;

        for _ in 0..token.len() {
            chars.next();
        }

        Some(token.to_string())
    }

    fn conversion_kind(conversion: char) -> Option<PrintfArgKind> {
        match conversion {
            'd' | 'i' => Some(PrintfArgKind::Int),
            'o' | 'u' | 'x' | 'X' => Some(PrintfArgKind::UInt),
            'f' | 'F' | 'e' | 'E' | 'g' | 'G' | 'a' | 'A' => Some(PrintfArgKind::Float),
            'c' => Some(PrintfArgKind::Char),
            's' => Some(PrintfArgKind::Str),
            'p' => Some(PrintfArgKind::Pointer),
            'n' => Some(PrintfArgKind::Count),
            '%' => Some(PrintfArgKind::Count),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrintfParseError {
    IncompleteDirective { index: usize },
    InvalidConversion { index: usize, conversion: char },
    InvalidPositionalIndex { index: usize },
    UnsupportedNonAscii { index: usize, ch: char },
    MissingArguments { expected: usize, actual: usize },
}
