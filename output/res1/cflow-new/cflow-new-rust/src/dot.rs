use std::io::{self, Write};

pub struct Dot;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputSymbol {
    pub name: String,
    pub declaration: Option<String>,
    pub source: Option<String>,
    pub definition_line: Option<i32>,
    pub active: bool,
    pub expand_line: Option<i32>,
    pub direct: bool,
    pub related_symbols: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputCommand {
    Begin,
    End,
    Newline,
    Separator,
    Text,
    Symbol,
    Other,
}

impl Dot {
    pub fn begin<W: Write + ?Sized>(writer: &mut W) -> io::Result<()> {
        writer.write_all(b"digraph cflow {\n")?;
        writer.write_all(b"    node [shape=\"box\"]\n")
    }

    pub fn declare_node<W: Write + ?Sized>(writer: &mut W, symbol: &OutputSymbol) -> io::Result<()> {
        write!(writer, "    {} [label=\"", symbol.name)?;
        if let Some(decl) = &symbol.declaration {
            let source = symbol.source.as_deref().unwrap_or("");
            let line = symbol.definition_line.unwrap_or(0);
            write!(writer, "{decl}\n{source}:{line}")?;
        } else {
            write!(writer, "{}()", symbol.name)?;
        }
        writer.write_all(b"\"]\n")
    }

    pub fn print_symbol<W: Write + ?Sized>(writer: &mut W, line: i32, symbol: &mut OutputSymbol) -> io::Result<()> {
        if symbol.active {
            return Ok(());
        }
        if symbol.expand_line.is_some() {
            return Ok(());
        }

        Self::declare_node(writer, symbol)?;
        symbol.expand_line = Some(line);

        Self::output_symbol(writer, &symbol.name, symbol.direct, &symbol.related_symbols)
    }

    pub fn output_handler<W: Write + ?Sized>(
        writer: &mut W,
        command: OutputCommand,
        line: i32,
        symbol: Option<&mut OutputSymbol>,
    ) -> io::Result<()> {
        match command {
            OutputCommand::Begin => Self::begin(writer),
            OutputCommand::End => writer.write_all(b"}\n"),
            OutputCommand::Newline
            | OutputCommand::Separator
            | OutputCommand::Text
            | OutputCommand::Other => Ok(()),
            OutputCommand::Symbol => {
                if let Some(symbol) = symbol {
                    Self::print_symbol(writer, line, symbol)?;
                }
                Ok(())
            }
        }
    }

    pub fn output_symbol<W: Write + ?Sized>(
        writer: &mut W,
        from: &str,
        _direct: bool,
        related_symbols: &[String],
    ) -> io::Result<()> {
        for to in related_symbols {
            writeln!(writer, "    {from} -> {to}")?;
        }
        Ok(())
    }
}
