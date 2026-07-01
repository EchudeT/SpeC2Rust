use crate::depmap::Depmap;
use crate::linked_list::LinkedList;
use crate::dot::{OutputCommand, OutputSymbol};
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::io::{self, Write};


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reference {
    pub source: String,
    pub line: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolKind {
    Identifier,
    Token,
    Undefined,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StorageKind {
    Extern,
    Static,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolFlag {
    None,
    Local,
    Alias,
    Parm,
    Start,
    Target,
}

#[derive(Clone, Debug)]
pub struct TreeNodeOutput {
    pub direct: bool,
    pub level: usize,
    pub last: bool,
    pub symbol_name: String,
    pub has_subtree: bool,
    pub recursive: bool,
    pub expand_line: Option<i32>,
    pub declaration: Option<String>,
}


type DriverCallback = dyn FnMut(
    OutputCommand,
    &mut dyn Write,
    i32,
    Option<&str>,
    Option<&TreeNodeOutput>,
) -> io::Result<bool>;

struct Driver {
    name: String,
    handler: Box<DriverCallback>,
}

pub struct Output {
    drivers: Vec<Driver>,
    driver_index: usize,
    symbols: HashMap<String, OutputSymbol>,
    starters: Vec<String>,
    out_line: i32,
    level_mark: Vec<bool>,
    level_mark_incr: usize,
    print_line_numbers: bool,
    print_levels: bool,
    level_begin: String,
    level_indent: [String; 2],
    level_end: [String; 2],
    brief_listing: bool,
    reverse_tree: bool,
    all_functions: i32,
    max_depth: usize,
    print_xref: bool,
    print_tree: bool,
    outname: String,
}

impl Output {
    pub fn new() -> Self {
        Self {
            drivers: Vec::new(),
            driver_index: 0,
            symbols: HashMap::new(),
            starters: Vec::new(),
            out_line: 0,
            level_mark: vec![false],
            level_mark_incr: 16,
            print_line_numbers: false,
            print_levels: false,
            level_begin: String::new(),
            level_indent: ["  ".to_string(), "| ".to_string()],
            level_end: ["+-".to_string(), "\\-".to_string()],
            brief_listing: false,
            reverse_tree: false,
            all_functions: 0,
            max_depth: 0,
            print_xref: false,
            print_tree: true,
            outname: "-".to_string(),
        }
    }

    pub fn register_output<F>(&mut self, name: impl Into<String>, handler: F) -> usize
    where
        F: FnMut(
                OutputCommand,
                &mut dyn Write,
                i32,
                Option<&str>,
                Option<&TreeNodeOutput>,
            ) -> io::Result<bool>
            + 'static,
    {
        let index = self.drivers.len();
        self.drivers.push(Driver {
            name: name.into(),
            handler: Box::new(handler),
        });
        index
    }

    pub fn driver(&mut self, name: &str) -> bool {
        if let Some(index) = self.drivers.iter().position(|driver| driver.name == name) {
            self.driver_index = index;
            true
        } else {
            false
        }
    }

    pub fn linked_list<T>(items: impl IntoIterator<Item = T>) -> LinkedList<T> {
        let mut list = LinkedList::new();
        for item in items {
            list.append(item);
        }
        list
    }

    pub fn set_level_mark(&mut self, lev: usize, mark: bool) {
        if lev >= self.level_mark.len() {
            let new_len = self.level_mark.len() + self.level_mark_incr.max(lev + 1 - self.level_mark.len());
            self.level_mark.resize(new_len, false);
        }
        self.level_mark[lev] = mark;
    }

    pub fn print_level<W: Write>(&self, writer: &mut W, lev: usize, last: bool) -> io::Result<()> {
        if self.print_line_numbers {
            write!(writer, "{:5} ", self.out_line)?;
        }
        if self.print_levels {
            write!(writer, "{{{:4}}} ", lev)?;
        }
        write!(writer, "{}", self.level_begin)?;
        for i in 0..lev {
            write!(writer, "{}", self.level_indent[self.level_mark[i] as usize])?;
        }
        write!(writer, "{}", self.level_end[last as usize])?;
        Ok(())
    }

    pub fn newline<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.invoke_driver(OutputCommand::Newline, writer, None, None)?;
        self.out_line += 1;
        Ok(())
    }

    pub fn begin<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        let _ = self.invoke_driver(OutputCommand::Begin, writer, None, None)?;
        Ok(())
    }

    pub fn end<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        let _ = self.invoke_driver(OutputCommand::End, writer, None, None)?;
        Ok(())
    }

    pub fn separator<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        let _ = self.invoke_driver(OutputCommand::Separator, writer, None, None)?;
        Ok(())
    }

    pub fn print_text<W: Write>(&mut self, writer: &mut W, buf: &str) -> io::Result<()> {
        let _ = self.invoke_driver(OutputCommand::Text, writer, Some(buf), None)?;
        Ok(())
    }

    pub fn print_symbol<W: Write>(
        &mut self,
        writer: &mut W,
        direct: bool,
        level: usize,
        last: bool,
        symbol_name: &str,
    ) -> io::Result<bool> {
        let Some(sym) = self.symbols.get(symbol_name) else {
            return Ok(false);
        };

        let node = TreeNodeOutput {
            direct,
            level,
            last,
            symbol_name: sym.name.clone(),
            has_subtree: !sym.related_symbols.is_empty(),
            recursive: false,
            expand_line: sym.expand_line,
            declaration: sym.declaration.clone(),
        };

        self.invoke_driver(OutputCommand::Symbol, writer, None, Some(&node))
    }

    pub fn compare_symbols(a: &OutputSymbol, b: &OutputSymbol) -> Ordering {
        a.name.cmp(&b.name)
    }

    pub fn is_var(symbol: &OutputSymbol) -> bool {
        symbol.source.is_some()
    }

    pub fn symbol_is_function(symbol: &OutputSymbol) -> bool {
        symbol.source.is_some()
    }

    pub fn clear_active(&mut self, symbol_name: &str) {
        if let Some(sym) = self.symbols.get_mut(symbol_name) {
            sym.active = false;
        }
    }

    pub fn print_refs<W: Write>(
        &self,
        writer: &mut W,
        name: &str,
        reflist: &[Reference],
    ) -> io::Result<()> {
        for reference in reflist {
            writeln!(writer, "{}   {}:{}", name, reference.source, reference.line)?;
        }
        Ok(())
    }

    pub fn print_function<W: Write>(&self, writer: &mut W, symbol: &OutputSymbol) -> io::Result<()> {
        if let Some(source) = &symbol.source {
            writeln!(
                writer,
                "{} * {}:{} {}",
                symbol.name,
                source,
                symbol.definition_line.unwrap_or_default(),
                symbol.declaration.clone().unwrap_or_default()
            )?;
        }
        self.print_refs(writer, &symbol.name, &[])
    }

    pub fn print_type<W: Write>(&self, writer: &mut W, symbol: &OutputSymbol) -> io::Result<()> {
        if let Some(source) = &symbol.source {
            writeln!(writer, "{} t {}:{}", symbol.name, source, symbol.definition_line.unwrap_or_default())?;
        }
        Ok(())
    }

    pub fn xref_output<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let mut symbols: Vec<&OutputSymbol> = self.symbols.values().filter(|s| Self::is_var(s)).collect();
        symbols.sort_by(|a, b| Self::compare_symbols(a, b));

        for symbol in symbols {
            self.print_function(writer, symbol)?;
        }
        Ok(())
    }

    pub fn set_active(&mut self, symbol_name: &str) {
        if let Some(sym) = self.symbols.get_mut(symbol_name) {
            sym.active = true;
        }
    }

    pub fn is_printable(&self, symbol_name: Option<&String>) -> bool {
        symbol_name
            .and_then(|name| self.symbols.get(name))
            .is_some_and(|sym| sym.source.is_some())
    }

    pub fn is_last(&self, items: &[String], index: usize) -> bool {
        for name in items.iter().skip(index + 1) {
            if self.is_printable(Some(name)) {
                return false;
            }
        }
        true
    }

    pub fn direct_tree<W: Write>(
        &mut self,
        writer: &mut W,
        lev: usize,
        last: bool,
        symbol_name: &str,
    ) -> io::Result<()> {
        let Some(symbol_state) = self.symbols.get(symbol_name).cloned() else {
            return Ok(());
        };

        if (self.max_depth != 0 && lev >= self.max_depth) || symbol_state.source.is_none() {
            return Ok(());
        }

        let rc = self.print_symbol(writer, true, lev, last, symbol_name)?;
        self.newline(writer)?;
        if rc || symbol_state.active {
            return Ok(());
        }

        self.set_active(symbol_name);
        let callees = symbol_state.related_symbols.clone();
        for (index, callee) in callees.iter().enumerate() {
            let child_is_last = self.is_last(&callees, index);
            self.set_level_mark(lev + 1, !child_is_last);
            self.direct_tree(writer, lev + 1, child_is_last, callee)?;
        }
        self.clear_active(symbol_name);
        Ok(())
    }

    pub fn inverted_tree<W: Write>(
        &mut self,
        writer: &mut W,
        lev: usize,
        last: bool,
        symbol_name: &str,
    ) -> io::Result<()> {
        let Some(symbol_state) = self.symbols.get(symbol_name).cloned() else {
            return Ok(());
        };

        if (self.max_depth != 0 && lev >= self.max_depth) || symbol_state.source.is_none() {
            return Ok(());
        }

        let rc = self.print_symbol(writer, false, lev, last, symbol_name)?;
        self.newline(writer)?;
        if rc || symbol_state.active {
            return Ok(());
        }

        self.set_active(symbol_name);
        let callers = symbol_state.related_symbols.clone();
        for (index, caller) in callers.iter().enumerate() {
            let child_is_last = self.is_last(&callers, index);
            self.set_level_mark(lev + 1, !child_is_last);
            self.inverted_tree(writer, lev + 1, child_is_last, caller)?;
        }
        self.clear_active(symbol_name);
        Ok(())
    }

    pub fn tree_output<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        let mut symbols: Vec<String> = self
            .symbols
            .values()
            .filter(|s| Self::is_var(s))
            .map(|s| s.name.clone())
            .collect();

        symbols.sort_by(|a, b| {
            let left = self.symbols.get(a).expect("symbol must exist");
            let right = self.symbols.get(b).expect("symbol must exist");
            Self::compare_symbols(left, right)
        });

        let mut symbols: Vec<String> = self
            .symbols
            .values()
            .filter(|s| Self::is_var(s))
            .map(|s| s.name.clone())
            .collect();

        symbols.sort_by(|a, b| {
            let left = self.symbols.get(a).expect("symbol must exist");
            let right = self.symbols.get(b).expect("symbol must exist");
            Self::compare_symbols(left, right)
        });

        self.begin(writer)?;

        if self.reverse_tree {
            for name in symbols {
                self.inverted_tree(writer, 0, false, &name)?;
                self.separator(writer)?;
            }
        } else {
            if !self.starters.is_empty() {
                let starters = self.starters.clone();
                for name in starters {
                    self.direct_tree(writer, 0, false, &name)?;
                    self.separator(writer)?;
                }
            } else if self.all_functions == 0 {
                self.all_functions = 1;
            }

            if self.all_functions != 0 {
                for name in symbols {
                    let Some(sym) = self.symbols.get(&name) else {
                        continue;
                    };
                    if sym.source.is_some()
                        && (self.all_functions > 1 || sym.related_symbols.is_empty())
                    {
                        self.direct_tree(writer, 0, false, &name)?;
                        self.separator(writer)?;
                    }
                }
            }
        }

        self.end(writer)
    }

    pub fn output<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        let _ = &self.outname;
        self.set_level_mark(0, false);
        if self.print_xref {
            self.xref_output(writer)?;
        }
        if self.print_tree {
            self.tree_output(writer)?;
        }
        Ok(())
    }

    fn invoke_driver<W: Write>(
        &mut self,
        command: OutputCommand,
        writer: &mut W,
        text: Option<&str>,
        symbol: Option<&TreeNodeOutput>,
    ) -> io::Result<bool> {
        if self.drivers.is_empty() {
            return Ok(false);
        }
        (self.drivers[self.driver_index].handler)(command, writer, self.out_line, text, symbol)
    }

    #[allow(dead_code)]
    pub fn insert_symbol(&mut self, symbol: OutputSymbol) {
        self.symbols.insert(symbol.name.clone(), symbol);
    }

    #[allow(dead_code)]
    pub fn set_starters(&mut self, starters: impl IntoIterator<Item = String>) {
        self.starters = starters.into_iter().collect();
    }

    #[allow(dead_code)]
    pub fn set_reverse_tree(&mut self, reverse_tree: bool) {
        self.reverse_tree = reverse_tree;
    }

    #[allow(dead_code)]
    pub fn set_all_functions(&mut self, all_functions: i32) {
        self.all_functions = all_functions;
    }

    #[allow(dead_code)]
    pub fn set_max_depth(&mut self, max_depth: usize) {
        self.max_depth = max_depth;
    }

    #[allow(dead_code)]
    pub fn set_print_xref(&mut self, print_xref: bool) {
        self.print_xref = print_xref;
    }

    #[allow(dead_code)]
    pub fn set_print_tree(&mut self, print_tree: bool) {
        self.print_tree = print_tree;
    }

    #[allow(dead_code)]
    pub fn set_print_line_numbers(&mut self, enabled: bool) {
        self.print_line_numbers = enabled;
    }

    #[allow(dead_code)]
    pub fn set_print_levels(&mut self, enabled: bool) {
        self.print_levels = enabled;
    }

    #[allow(dead_code)]
    pub fn set_level_format(
        &mut self,
        begin: impl Into<String>,
        indents: [String; 2],
        ends: [String; 2],
    ) {
        self.level_begin = begin.into();
        self.level_indent = indents;
        self.level_end = ends;
    }

    #[allow(dead_code)]
    pub fn set_brief_listing(&mut self, brief_listing: bool) {
        self.brief_listing = brief_listing;
    }

    #[allow(dead_code)]
    pub fn eliminate_non_targets(&mut self) {
        if self.starters.is_empty() {
            return;
        }

        let mut reachable = BTreeSet::new();
        let mut stack = self.starters.clone();
        while let Some(name) = stack.pop() {
            if !reachable.insert(name.clone()) {
                continue;
            }
            if let Some(symbol) = self.symbols.get(&name) {
                for callee in &symbol.related_symbols {
                    stack.push(callee.clone());
                }
            }
        }

        for (name, _symbol) in &mut self.symbols {
        }
    }
}

impl Default for Output {
    fn default() -> Self {
        Self::new()
    }
}
