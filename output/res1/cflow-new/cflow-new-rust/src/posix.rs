use std::io::{self, Write};

use crate::dot::{OutputCommand, OutputSymbol};
use crate::error::Error;

pub struct Posix;

impl Posix {
    pub fn print_symbol_type<W: Write + ?Sized>(writer: &mut W, symbol: &OutputSymbol) -> io::Result<()> {
        if let Some(declaration) = &symbol.declaration {
            let source = symbol.source.as_deref().unwrap_or("");
            let def_line = symbol.definition_line.unwrap_or(0);
            write!(writer, "{declaration}, <{source} {def_line}>")
        } else {
            write!(writer, "<>")
        }
    }

    pub fn print_symbol<W: Write + ?Sized>(
        writer: &mut W,
        line: i32,
        symbol: &mut OutputSymbol,
    ) -> io::Result<bool> {
        write!(writer, "{}: ", symbol.name)?;

        if let Some(expand_line) = symbol.expand_line {
            write!(writer, "{expand_line}")?;
            return Ok(true);
        } else if !symbol.related_symbols.is_empty() {
            symbol.expand_line = Some(line);
        }

        Self::print_symbol_type(writer, symbol)?;
        Ok(false)
    }

    pub fn output_handler<W: Write + ?Sized>(
        writer: &mut W,
        command: OutputCommand,
        line: i32,
        text: Option<&str>,
        symbol: Option<&mut OutputSymbol>,
        emacs_option: bool,
    ) -> io::Result<bool> {
        match command {
            OutputCommand::Begin => {
                if emacs_option {
                    Error::report(
                        Some(64),
                        Some(0),
                        format_args!("--format=posix is not compatible with --emacs"),
                    );
                }
                Ok(false)
            }
            OutputCommand::End => Ok(false),
            OutputCommand::Newline => {
                writeln!(writer)?;
                Ok(false)
            }
            OutputCommand::Separator => {
                write!(writer, " ")?;
                Ok(false)
            }
            OutputCommand::Text | OutputCommand::Other => {
                if let Some(text) = text {
                    if text == "\n" {
                        writeln!(writer)?;
                    } else {
                        write!(writer, "{text}")?;
                    }
                }
                Ok(false)
            }
            OutputCommand::Symbol => {
                if let Some(symbol) = symbol {
                    Self::print_symbol(writer, line, symbol)
                } else {
                    Ok(false)
                }
            }
        }
    }
}
