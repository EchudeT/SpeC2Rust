use std::collections::HashSet;
use std::io::{self, Write};

use crate::output::{OutputDriver, RenderedSymbol};

pub struct Dot;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkedListEntry {
    pub name: String,
    pub included: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LinkedList {
    entries: Vec<LinkedListEntry>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OutputSymbol {
    pub name: String,
    pub declaration: Option<String>,
    pub source: Option<String>,
    pub definition_line: Option<i32>,
    pub active: bool,
    pub expand_line: Option<i32>,
    pub direct: bool,
    pub callee: LinkedList,
    pub caller: LinkedList,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputCommand {
    Begin,
    Init,
    End,
    Separator,
    Newline,
    Text,
    Symbol,
    Other,
}

impl Dot {
    pub fn new() -> Self {
        Dot
    }

    pub fn begin<W: Write + ?Sized>(writer: &mut W) -> io::Result<()> {
        writer.write_all(b"digraph cflow {\n")?;
        writer.write_all(b"    node [shape=\"box\"]\n")
    }

    pub fn declare_node<W: Write + ?Sized>(writer: &mut W, symbol: &RenderedSymbol<'_>) -> io::Result<()> {
        write!(writer, "    {} [label=\"", symbol.symbol.name)?;
        if let Some(decl) = &symbol.symbol.decl {
            let source = symbol.symbol.source.as_deref().unwrap_or("");
            let line = symbol.symbol.def_line;
            write!(writer, "{decl}\n{source}:{line}")?;
        } else {
            write!(writer, "{}()", symbol.symbol.name)?;
        }
        writer.write_all(b"\"]\n")
    }
}

impl OutputDriver for Dot {
    fn begin(&mut self, writer: &mut dyn Write, _line: usize) -> io::Result<()> {
        Self::begin(writer)
    }

    fn end(&mut self, writer: &mut dyn Write, _line: usize) -> io::Result<()> {
        writer.write_all(b"}\n")
    }

    fn symbol(
        &mut self,
        writer: &mut dyn Write,
        line: usize,
        symbol: &RenderedSymbol<'_>,
    ) -> io::Result<bool> {
        if symbol.symbol.active != 0 {
            return Ok(false);
        }

        Self::declare_node(writer, symbol)?;

        let links = if symbol.direct {
            &symbol.symbol.callee
        } else {
            &symbol.symbol.caller
        };

        for entry in links {
            writeln!(writer, "    {} -> {}", symbol.symbol.name, entry)?;
        }

        let _ = line;
        Ok(false)
    }
}

pub struct LinkIter<'a> {
    inner: std::slice::Iter<'a, LinkedListEntry>,
}

impl<'a> Iterator for LinkIter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        for entry in self.inner.by_ref() {
            if entry.included {
                return Some(&entry.name);
            }
        }
        None
    }
}

pub fn output_symbol(name: impl Into<String>) -> OutputSymbol {
    OutputSymbol {
        name: name.into(),
        ..OutputSymbol::default()
    }
}

pub fn linked_list() -> LinkedList {
    LinkedList::default()
}

pub fn linked_list_entry(name: impl Into<String>, included: bool) -> LinkedListEntry {
    LinkedListEntry {
        name: name.into(),
        included,
    }
}

impl LinkedList {
    pub fn push(&mut self, entry: LinkedListEntry) {
        self.entries.push(entry);
    }

    pub fn iter(&self) -> impl Iterator<Item = &LinkedListEntry> {
        self.entries.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl<'a> IntoIterator for &'a LinkedList {
    type Item = &'a String;
    type IntoIter = LinkIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LinkIter {
            inner: self.entries.iter(),
        }
    }
}

impl OutputSymbol {
    pub fn with_declaration(
        mut self,
        declaration: impl Into<String>,
        source: impl Into<String>,
        definition_line: i32,
    ) -> Self {
        self.declaration = Some(declaration.into());
        self.source = Some(source.into());
        self.definition_line = Some(definition_line);
        self
    }

    pub fn with_direction(mut self, direct: bool) -> Self {
        self.direct = direct;
        self
    }

    pub fn with_active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn with_callees<I, S>(mut self, names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.callee = names
            .into_iter()
            .map(|name| LinkedListEntry {
                name: name.into(),
                included: true,
            })
            .collect();
        self
    }

    pub fn with_callers<I, S>(mut self, names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.caller = names
            .into_iter()
            .map(|name| LinkedListEntry {
                name: name.into(),
                included: true,
            })
            .collect();
        self
    }

    pub fn reset_expand_line(&mut self) {
        self.expand_line = None;
    }

    pub fn unique_links(&self) -> Vec<&str> {
        let links = if self.direct {
            &self.callee
        } else {
            &self.caller
        };
        let mut seen = HashSet::new();
        let mut out = Vec::new();
        for entry in links.iter() {
            if entry.included && seen.insert(entry.name.as_str()) {
                out.push(entry.name.as_str());
            }
        }
        out
    }
}

impl FromIterator<LinkedListEntry> for LinkedList {
    fn from_iter<T: IntoIterator<Item = LinkedListEntry>>(iter: T) -> Self {
        Self {
            entries: iter.into_iter().collect(),
        }
    }
}
