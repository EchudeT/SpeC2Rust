use std::io::{self, Write};

use crate::output::{OutputDriver, RenderedSymbol};

pub struct Posix;

impl Posix {
    pub fn new() -> Self {
        Posix
    }

    pub fn print_symbol_type<W: Write + ?Sized>(writer: &mut W, symbol: &RenderedSymbol<'_>) -> io::Result<()> {
        if let (Some(declaration), Some(source)) = (
            symbol.symbol.decl.as_deref(),
            symbol.symbol.source.as_deref(),
        ) {
            write!(writer, "{declaration}, <{source} {}>", symbol.symbol.def_line)
        } else {
            write!(writer, "<>")
        }
    }
}

impl OutputDriver for Posix {
    fn symbol(
        &mut self,
        writer: &mut dyn Write,
        line: usize,
        symbol: &RenderedSymbol<'_>,
    ) -> io::Result<bool> {
        write!(writer, "{}: ", symbol.symbol.name)?;

        if symbol.symbol.active != 0 {
            write!(writer, "{}", line)?;
            return Ok(true);
        }

        Self::print_symbol_type(writer, symbol)?;
        Ok(false)
    }
}
