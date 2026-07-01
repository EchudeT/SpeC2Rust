use std::collections::HashSet;
use std::io::{self, Write};

pub struct Dot {
    declared_nodes: HashSet<String>,
    begun: bool,
    finished: bool,
}

impl Default for Dot {
    fn default() -> Self {
        Self {
            declared_nodes: HashSet::new(),
            begun: false,
            finished: false,
        }
    }
}

impl Dot {
    pub fn begin<W: Write + ?Sized>(&mut self, writer: &mut W) -> io::Result<()> {
        if self.begun {
            return Ok(());
        }
        self.begun = true;
        writeln!(writer, "digraph cflow {{")?;
        writeln!(writer, "    node [shape=\"box\"]")
    }

    pub fn declare_node<W: Write + ?Sized>(
        &mut self,
        writer: &mut W,
        name: &str,
        decl: Option<&str>,
        source: Option<&str>,
        def_line: Option<usize>,
    ) -> io::Result<()> {
        if !self.declared_nodes.insert(name.to_owned()) {
            return Ok(());
        }

        write!(writer, "    {} [label=\"", name)?;
        if let Some(decl_text) = decl {
            let source_text = source.unwrap_or("");
            let line = def_line.unwrap_or(0);
            write!(writer, "{}\n{}:{}", decl_text, source_text, line)?;
        } else {
            write!(writer, "{}()", name)?;
        }
        writeln!(writer, "\"]")
    }

    pub fn print_symbol<W>(
        &mut self,
        writer: &mut W,
        line: usize,
        name: &str,
        decl: Option<&str>,
        source: Option<&str>,
        def_line: Option<usize>,
        active: bool,
        expand_line: Option<usize>,
        direct: bool,
        related_symbols: Vec<String>,
    ) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        if active || expand_line.is_some() {
            return Ok(());
        }

        let _ = line;
        self.declare_node(writer, name, decl, source, def_line)?;

        for related in related_symbols {
            if direct {
                writeln!(writer, "    {} -> {}", name, related)?;
            } else {
                writeln!(writer, "    {} -> {}", name, related)?;
            }
        }

        Ok(())
    }

    pub fn output_handler<W>(
        &mut self,
        writer: &mut W,
        command: DotCommand<'_>,
    ) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        match command {
            DotCommand::Begin => self.begin(writer),
            DotCommand::End => {
                if !self.finished {
                    self.finished = true;
                    writeln!(writer, "}}")?;
                }
                Ok(())
            }
            DotCommand::Symbol {
                line,
                name,
                decl,
                source,
                def_line,
                active,
                expand_line,
                direct,
                related_symbols,
            } => self.print_symbol(
                writer,
                line,
                name,
                decl,
                source,
                def_line,
                active,
                expand_line,
                direct,
                related_symbols,
            ),
        }
    }
}

pub enum DotCommand<'a> {
    Begin,
    End,
    Symbol {
        line: usize,
        name: &'a str,
        decl: Option<&'a str>,
        source: Option<&'a str>,
        def_line: Option<usize>,
        active: bool,
        expand_line: Option<usize>,
        direct: bool,
        related_symbols: Vec<String>,
    },
}
