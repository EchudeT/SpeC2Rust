use std::io::{self, Write};

use crate::output::{SimpleSymbolKind, SymbolRecord};

pub struct Posix {
    brief_listing: bool,
    print_line_numbers: bool,
    omit_symbol_names: bool,
}

pub enum PosixCommand<'a> {
    Init {
        emacs_option: bool,
    },
    Begin,
    End,
    Separator,
    Newline,
    Text(&'a str),
    Symbol {
        line: usize,
        level_prefix: &'a str,
        symbol: &'a mut PosixSymbol<'a>,
    },
}

pub struct PosixSymbol<'a> {
    pub name: &'a str,
    pub decl: Option<&'a str>,
    pub source: Option<&'a str>,
    pub def_line: usize,
    pub expand_line: Option<usize>,
    pub has_callee: bool,
}

impl Default for Posix {
    fn default() -> Self {
        Self {
            brief_listing: false,
            print_line_numbers: false,
            omit_symbol_names: false,
        }
    }
}

impl Posix {
    pub fn print_symbol_type<W: Write + ?Sized>(
        writer: &mut W,
        symbol: &PosixSymbol<'_>,
    ) -> io::Result<()> {
        if let Some(decl) = symbol.decl {
            let source = symbol.source.unwrap_or("");
            write!(writer, "{decl}, <{source} {}>", symbol.def_line)
        } else {
            write!(writer, "<>")
        }
    }

    pub fn print_symbol<W: Write + ?Sized>(
        &mut self,
        writer: &mut W,
        line: usize,
        level_prefix: &str,
        symbol: &mut PosixSymbol<'_>,
    ) -> io::Result<bool> {
        write!(writer, "{level_prefix}{}: ", symbol.name)?;

        if self.brief_listing {
            if let Some(expand_line) = symbol.expand_line {
                write!(writer, "{expand_line}")?;
                return Ok(true);
            } else if symbol.has_callee {
                symbol.expand_line = Some(line);
            }
        }

        Self::print_symbol_type(writer, symbol)?;
        Ok(false)
    }

    pub fn output_handler<W: Write + ?Sized>(
        &mut self,
        writer: &mut W,
        command: PosixCommand<'_>,
    ) -> io::Result<bool> {
        match command {
            PosixCommand::Init { emacs_option } => {
                if emacs_option {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "--format=posix is not compatible with --emacs",
                    ));
                }
                self.brief_listing = true;
                self.print_line_numbers = true;
                self.omit_symbol_names = true;
                Ok(false)
            }
            PosixCommand::Begin | PosixCommand::End | PosixCommand::Separator => Ok(false),
            PosixCommand::Newline => {
                writeln!(writer)?;
                Ok(false)
            }
            PosixCommand::Text(text) => {
                write!(writer, "{text}")?;
                Ok(false)
            }
            PosixCommand::Symbol {
                line,
                level_prefix,
                symbol,
            } => self.print_symbol(writer, line, level_prefix, symbol),
        }
    }
}

impl<'a> From<&'a mut SymbolRecord> for PosixSymbol<'a> {
    fn from(symbol: &'a mut SymbolRecord) -> Self {
        Self {
            name: &symbol.name,
            decl: symbol.decl.as_deref(),
            source: symbol.source.as_deref(),
            def_line: symbol.def_line,
            expand_line: symbol.expand_line,
            has_callee: !symbol.callees.is_empty(),
        }
    }
}
