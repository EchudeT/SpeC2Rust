use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::io::{self, Write};

pub struct Output {
    drivers: Vec<DriverRegistration>,
    driver_index: usize,
    out_line: usize,
    level_mark: Vec<bool>,
    level_mark_incr: usize,
    print_line_numbers: bool,
    print_levels: bool,
    level_begin: String,
    level_end: [String; 2],
    level_indent: [String; 2],
    reverse_tree: bool,
    all_functions: usize,
    max_depth: Option<usize>,
    symbols: Vec<SymbolRecord>,
    starters: Vec<String>,
}

struct DriverRegistration {
    name: String,
    driver: Box<dyn OutputDriver>,
}

pub trait OutputDriver {
    fn init(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn begin(&mut self, _writer: &mut dyn Write, _line: usize) -> io::Result<()> {
        Ok(())
    }

    fn end(&mut self, _writer: &mut dyn Write, _line: usize) -> io::Result<()> {
        Ok(())
    }

    fn separator(&mut self, _writer: &mut dyn Write, _line: usize) -> io::Result<()> {
        Ok(())
    }

    fn text(&mut self, writer: &mut dyn Write, _line: usize, text: &str) -> io::Result<()> {
        writer.write_all(text.as_bytes())
    }

    fn symbol(
        &mut self,
        writer: &mut dyn Write,
        _line: usize,
        symbol: &RenderedSymbol<'_>,
    ) -> io::Result<bool>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolKind {
    Identifier,
    Token,
    Undefined,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StorageClass {
    External,
    Static,
    Other,
}

#[derive(Clone, Debug)]
pub struct ReferenceRecord {
    pub source: String,
    pub line: i32,
}

#[derive(Clone, Debug)]
pub struct SymbolRecord {
    pub name: String,
    pub kind: SymbolKind,
    pub storage: StorageClass,
    pub arity: i32,
    pub source: Option<String>,
    pub def_line: i32,
    pub decl: Option<String>,
    pub ref_line: Vec<ReferenceRecord>,
    pub callee: Vec<String>,
    pub caller: Vec<String>,
    pub active: usize,
    pub ord: isize,
    pub recursive: bool,
    pub starter: bool,
    pub included: bool,
}

pub struct RenderedSymbol<'a> {
    pub direct: bool,
    pub level: usize,
    pub last: bool,
    pub symbol: &'a SymbolRecord,
}

impl Output {
    pub fn new() -> Self {
        Self {
            drivers: Vec::new(),
            driver_index: 0,
            out_line: 0,
            level_mark: vec![false],
            level_mark_incr: 16,
            print_line_numbers: false,
            print_levels: false,
            level_begin: String::new(),
            level_end: [String::new(), String::new()],
            level_indent: ["  ".to_string(), "| ".to_string()],
            reverse_tree: false,
            all_functions: 0,
            max_depth: None,
            symbols: Vec::new(),
            starters: Vec::new(),
        }
    }

    pub fn register_output<D: OutputDriver + 'static>(
        &mut self,
        name: impl Into<String>,
        driver: D,
    ) -> usize {
        let index = self.drivers.len();
        self.drivers.push(DriverRegistration {
            name: name.into(),
            driver: Box::new(driver),
        });
        index
    }

    pub fn driver(&mut self, name: &str) -> bool {
        for (i, drv) in self.drivers.iter().enumerate() {
            if drv.name == name {
                self.driver_index = i;
                return true;
            }
        }
        false
    }

    pub fn set_level_mark(&mut self, lev: usize, mark: bool) {
        if lev >= self.level_mark.len() {
            let grow_by = self.level_mark_incr.max(lev + 1 - self.level_mark.len());
            self.level_mark.resize(self.level_mark.len() + grow_by, false);
            if lev >= self.level_mark.len() {
                self.level_mark.resize(lev + 1, false);
            }
        }
        self.level_mark[lev] = mark;
    }

    pub fn print_level(&self, writer: &mut dyn Write, lev: usize, last: bool) -> io::Result<()> {
        if self.print_line_numbers {
            write!(writer, "{:5} ", self.out_line)?;
        }
        if self.print_levels {
            write!(writer, "{{{:4}}} ", lev)?;
        }
        write!(writer, "{}", self.level_begin)?;
        for i in 0..lev {
            let idx = usize::from(*self.level_mark.get(i).unwrap_or(&false));
            write!(writer, "{}", self.level_indent[idx])?;
        }
        write!(writer, "{}", self.level_end[usize::from(last)])
    }

    pub fn newline(&mut self, writer: &mut dyn Write) -> io::Result<()> {
        if self.drivers.is_empty() {
            writeln!(writer)?;
            self.out_line += 1;
            return Ok(());
        }
        let idx = self.driver_index.min(self.drivers.len() - 1);
        self.drivers[idx].driver.text(writer, self.out_line, "\n")?;
        self.out_line += 1;
        Ok(())
    }

    pub fn begin(&mut self, writer: &mut dyn Write) -> io::Result<()> {
        if self.drivers.is_empty() {
            return Ok(());
        }
        let idx = self.driver_index.min(self.drivers.len() - 1);
        self.drivers[idx].driver.begin(writer, self.out_line)
    }

    pub fn end(&mut self, writer: &mut dyn Write) -> io::Result<()> {
        if self.drivers.is_empty() {
            return Ok(());
        }
        let idx = self.driver_index.min(self.drivers.len() - 1);
        self.drivers[idx].driver.end(writer, self.out_line)
    }

    pub fn separator(&mut self, writer: &mut dyn Write) -> io::Result<()> {
        if self.drivers.is_empty() {
            return Ok(());
        }
        let idx = self.driver_index.min(self.drivers.len() - 1);
        self.drivers[idx].driver.separator(writer, self.out_line)
    }

    pub fn print_text(&mut self, writer: &mut dyn Write, text: &str) -> io::Result<()> {
        if self.drivers.is_empty() {
            writer.write_all(text.as_bytes())
        } else {
            let idx = self.driver_index.min(self.drivers.len() - 1);
            self.drivers[idx].driver.text(writer, self.out_line, text)
        }
    }

    pub fn print_symbol(
        &mut self,
        writer: &mut dyn Write,
        direct: bool,
        level: usize,
        last: bool,
        symbol_name: &str,
    ) -> io::Result<bool> {
        let Some(sym) = self.symbol(symbol_name).cloned() else {
            return Ok(false);
        };

        if self.drivers.is_empty() {
            self.print_level(writer, level, last)?;
            writeln!(writer, "{}", sym.name)?;
            return Ok(false);
        }

        let rendered = RenderedSymbol {
            direct,
            level,
            last,
            symbol: &sym,
        };
        let idx = self.driver_index.min(self.drivers.len() - 1);
        self.drivers[idx].driver.symbol(writer, self.out_line, &rendered)
    }

    pub fn compare(&self, a: &str, b: &str) -> Ordering {
        a.cmp(b)
    }

    pub fn is_var(&self, symbol_name: &str) -> bool {
        let Some(sym) = self.symbol(symbol_name) else {
            return false;
        };
        if !self.include_symbol(sym) {
            return false;
        }
        if sym.kind == SymbolKind::Identifier {
            matches!(sym.storage, StorageClass::External | StorageClass::Static)
        } else {
            true
        }
    }

    pub fn symbol_is_function(&self, symbol_name: &str) -> bool {
        let Some(sym) = self.symbol(symbol_name) else {
            return false;
        };
        sym.kind == SymbolKind::Identifier && sym.arity >= 0
    }

    pub fn clear_active(&mut self, symbol_name: &str) {
        if let Some(sym) = self.symbol_mut(symbol_name) {
            sym.active = 0;
        }
    }

    pub fn print_refs(
        &self,
        writer: &mut dyn Write,
        name: &str,
        refs: &[ReferenceRecord],
    ) -> io::Result<()> {
        for r in refs {
            writeln!(writer, "{}   {}:{}", name, r.source, r.line)?;
        }
        Ok(())
    }

    pub fn print_function(&self, writer: &mut dyn Write, symbol_name: &str) -> io::Result<()> {
        let Some(sym) = self.symbol(symbol_name) else {
            return Ok(());
        };
        if let Some(source) = &sym.source {
            writeln!(
                writer,
                "{} * {}:{} {}",
                sym.name,
                source,
                sym.def_line,
                sym.decl.clone().unwrap_or_default()
            )?;
        }
        self.print_refs(writer, &sym.name, &sym.ref_line)
    }

    pub fn print_type(&self, writer: &mut dyn Write, symbol_name: &str) -> io::Result<()> {
        let Some(sym) = self.symbol(symbol_name) else {
            return Ok(());
        };
        if let Some(source) = &sym.source {
            writeln!(writer, "{} t {}:{}", sym.name, source, sym.def_line)?;
        }
        Ok(())
    }

    pub fn xref_output(&self, writer: &mut dyn Write) -> io::Result<()> {
        let mut names: Vec<&str> = self
            .symbols
            .iter()
            .filter(|s| self.is_var(&s.name))
            .map(|s| s.name.as_str())
            .collect();
        names.sort_unstable_by(|a, b| self.compare(a, b));

        for name in names {
            if let Some(sym) = self.symbol(name) {
                match sym.kind {
                    SymbolKind::Identifier => self.print_function(writer, name)?,
                    SymbolKind::Token => self.print_type(writer, name)?,
                    SymbolKind::Undefined => {}
                }
            }
        }
        Ok(())
    }

    pub fn set_active(&mut self, symbol_name: &str) {
        let line = self.out_line;
        if let Some(sym) = self.symbol_mut(symbol_name) {
            sym.active = line;
        }
    }

    pub fn is_printable(&self, symbol_name: &str) -> bool {
        self.symbol(symbol_name)
            .map(|s| self.include_symbol(s))
            .unwrap_or(false)
    }

    pub fn is_last(&self, names: &[String], start_after: usize) -> bool {
        for name in names.iter().skip(start_after + 1) {
            if self.is_printable(name) {
                return false;
            }
        }
        true
    }

    pub fn direct_tree(
        &mut self,
        writer: &mut dyn Write,
        lev: usize,
        last: bool,
        symbol_name: &str,
    ) -> io::Result<()> {
        let Some(sym) = self.symbol(symbol_name).cloned() else {
            return Ok(());
        };

        if sym.kind == SymbolKind::Undefined
            || self.max_depth.is_some_and(|m| lev >= m)
            || !self.include_symbol(&sym)
        {
            return Ok(());
        }

        let rc = self.print_symbol(writer, true, lev, last, symbol_name)?;
        self.newline(writer)?;
        if rc || sym.active != 0 {
            return Ok(());
        }

        self.set_active(symbol_name);
        let callees = sym.callee.clone();
        for (idx, child) in callees.iter().enumerate() {
            let child_last = self.is_last(&callees, idx);
            self.set_level_mark(lev + 1, !child_last);
            self.direct_tree(writer, lev + 1, child_last, child)?;
        }
        self.clear_active(symbol_name);
        Ok(())
    }

    pub fn inverted_tree(
        &mut self,
        writer: &mut dyn Write,
        lev: usize,
        last: bool,
        symbol_name: &str,
    ) -> io::Result<()> {
        let Some(sym) = self.symbol(symbol_name).cloned() else {
            return Ok(());
        };

        if sym.kind == SymbolKind::Undefined
            || self.max_depth.is_some_and(|m| lev >= m)
            || !self.include_symbol(&sym)
        {
            return Ok(());
        }

        let rc = self.print_symbol(writer, false, lev, last, symbol_name)?;
        self.newline(writer)?;
        if rc || sym.active != 0 {
            return Ok(());
        }

        self.set_active(symbol_name);
        let callers = sym.caller.clone();
        for (idx, parent) in callers.iter().enumerate() {
            let parent_last = self.is_last(&callers, idx);
            self.set_level_mark(lev + 1, !parent_last);
            self.inverted_tree(writer, lev + 1, parent_last, parent)?;
        }
        self.clear_active(symbol_name);
        Ok(())
    }

    pub fn tree_output(&mut self, writer: &mut dyn Write) -> io::Result<()> {
        let function_names: Vec<String> = self
            .symbols
            .iter()
            .filter(|s| self.symbol_is_function(&s.name))
            .map(|s| s.name.clone())
            .collect();

        for (ord, name) in function_names.iter().enumerate() {
            if let Some(sym) = self.symbol_mut(name) {
                sym.ord = ord as isize;
            }
        }

        let ord_map: HashMap<String, usize> = function_names
            .iter()
            .enumerate()
            .map(|(i, n)| (n.clone(), i))
            .collect();

        let mut graph: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); function_names.len()];
        for (i, name) in function_names.iter().enumerate() {
            let callees = self
                .symbol(name)
                .map(|s| s.callee.clone())
                .unwrap_or_default();
            for callee in callees {
                if let Some(&j) = ord_map.get(&callee) {
                    graph[i].insert(j);
                }
            }
        }

        let closure = Self::transitive_closure(&graph);
        for i in 0..function_names.len() {
            if closure[i].contains(&i) {
                if let Some(sym) = self.symbol_mut(&function_names[i]) {
                    sym.recursive = true;
                }
            }
        }

        let mut names: Vec<String> = self
            .symbols
            .iter()
            .filter(|s| self.is_var(&s.name))
            .map(|s| s.name.clone())
            .collect();
        names.sort_unstable_by(|a, b| self.compare(a, b));

        self.begin(writer)?;

        if self.reverse_tree {
            for name in names {
                self.inverted_tree(writer, 0, false, &name)?;
                self.separator(writer)?;
            }
        } else {
            let starters: Vec<String> = self
                .starters
                .iter()
                .filter(|name| self.symbol(name).is_some())
                .cloned()
                .collect();

            if !starters.is_empty() {
                for starter in starters {
                    self.direct_tree(writer, 0, false, &starter)?;
                    self.separator(writer)?;
                }
            } else if self.all_functions == 0 {
                self.all_functions = 1;
            }

            if self.all_functions > 0 {
                for name in names {
                    let include = self
                        .symbol(&name)
                        .map(|sym| {
                            !sym.starter
                                && sym.source.is_some()
                                && (self.all_functions > 1 || sym.caller.is_empty())
                        })
                        .unwrap_or(false);
                    if include {
                        self.direct_tree(writer, 0, false, &name)?;
                        self.separator(writer)?;
                    }
                }
            }
        }

        self.end(writer)
    }

    pub fn output(&mut self, writer: &mut dyn Write) -> io::Result<()> {
        if !self.drivers.is_empty() {
            let idx = self.driver_index.min(self.drivers.len() - 1);
            self.drivers[idx].driver.init()?;
        }
        self.set_level_mark(0, false);
        if self.reverse_tree || self.all_functions > 0 || !self.starters.is_empty() {
            self.tree_output(writer)
        } else {
            self.xref_output(writer)
        }
    }

    fn include_symbol(&self, sym: &SymbolRecord) -> bool {
        sym.included
    }

    fn symbol(&self, name: &str) -> Option<&SymbolRecord> {
        self.symbols.iter().find(|s| s.name == name)
    }

    fn symbol_mut(&mut self, name: &str) -> Option<&mut SymbolRecord> {
        self.symbols.iter_mut().find(|s| s.name == name)
    }

    fn transitive_closure(graph: &[BTreeSet<usize>]) -> Vec<BTreeSet<usize>> {
        let mut closure = graph.to_vec();
        let n = closure.len();
        for k in 0..n {
            for i in 0..n {
                if closure[i].contains(&k) {
                    let extra: Vec<usize> = closure[k].iter().copied().collect();
                    for j in extra {
                        closure[i].insert(j);
                    }
                }
            }
        }
        closure
    }
}
