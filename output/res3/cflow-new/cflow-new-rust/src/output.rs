use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

pub struct Output {
    drivers: Vec<DriverEntry>,
    driver_index: usize,
    out_line: usize,
    level_mark: Vec<bool>,
    level_mark_incr: usize,
    level_begin: String,
    level_indent: [String; 2],
    level_end: [String; 2],
    print_line_numbers: bool,
    print_levels: bool,
    brief_listing: bool,
    reverse_tree: bool,
    all_functions: usize,
    max_depth: Option<usize>,
    print_xref: bool,
    print_tree: bool,
}

struct DriverEntry {
    name: String,
    driver: Box<dyn OutputDriver>,
}

#[derive(Clone, Debug)]
struct OutputNode {
    name: String,
    kind: SymbolKind,
    storage: StorageClass,
    arity: i32,
    source: Option<String>,
    def_line: usize,
    decl: Option<String>,
    expand_line: Option<usize>,
    recursive: bool,
    active: usize,
    caller: Vec<String>,
    callee: Vec<String>,
    flag_start: bool,
    refs: Vec<Reference>,
}

#[derive(Clone, Debug)]
struct Reference {
    source: String,
    line: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SymbolKind {
    Identifier,
    Token,
    Undefined,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StorageClass {
    Extern,
    Static,
    Other,
}

#[derive(Clone, Debug)]
struct OutputSymbolView {
    direct: bool,
    level: usize,
    last: bool,
    name: String,
    kind: SymbolKind,
    storage: StorageClass,
    arity: i32,
    source: Option<String>,
    def_line: usize,
    decl: Option<String>,
    expand_line: Option<usize>,
    recursive: bool,
    has_caller: bool,
    has_callee: bool,
}

pub(crate) trait OutputDriver {
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

    fn newline(&mut self, writer: &mut dyn Write, _line: usize) -> io::Result<()> {
        writer.write_all(b"\n")
    }

    fn text(&mut self, writer: &mut dyn Write, _line: usize, text: &str) -> io::Result<()> {
        writer.write_all(text.as_bytes())
    }

    fn symbol(
        &mut self,
        writer: &mut dyn Write,
        line: usize,
        level_prefix: &str,
        symbol: &mut OutputSymbolView,
        brief_listing: bool,
    ) -> io::Result<bool>;
}

impl OutputSymbolView {
    fn has_subtree(&self) -> bool {
        if self.direct {
            self.has_callee
        } else {
            self.has_caller
        }
    }
}

impl OutputDriver for crate::posix::Posix {
    fn init(&mut self) -> io::Result<()> {
        self.output_handler(
            &mut io::sink(),
            crate::posix::PosixCommand::Init {
                emacs_option: false,
            },
        )?;
        Ok(())
    }

    fn begin(&mut self, writer: &mut dyn Write, _line: usize) -> io::Result<()> {
        let _ = self.output_handler(writer, crate::posix::PosixCommand::Begin)?;
        Ok(())
    }

    fn end(&mut self, writer: &mut dyn Write, _line: usize) -> io::Result<()> {
        let _ = self.output_handler(writer, crate::posix::PosixCommand::End)?;
        Ok(())
    }

    fn separator(&mut self, writer: &mut dyn Write, _line: usize) -> io::Result<()> {
        let _ = self.output_handler(writer, crate::posix::PosixCommand::Separator)?;
        Ok(())
    }

    fn newline(&mut self, writer: &mut dyn Write, _line: usize) -> io::Result<()> {
        let _ = self.output_handler(writer, crate::posix::PosixCommand::Newline)?;
        Ok(())
    }

    fn text(&mut self, writer: &mut dyn Write, _line: usize, text: &str) -> io::Result<()> {
        let _ = self.output_handler(writer, crate::posix::PosixCommand::Text(text))?;
        Ok(())
    }

    fn symbol(
        &mut self,
        writer: &mut dyn Write,
        line: usize,
        level_prefix: &str,
        symbol: &mut OutputSymbolView,
        _brief_listing: bool,
    ) -> io::Result<bool> {
        let mut psym = crate::posix::PosixSymbol {
            name: &symbol.name,
            decl: symbol.decl.as_deref(),
            source: symbol.source.as_deref(),
            def_line: symbol.def_line,
            expand_line: symbol.expand_line,
            has_callee: symbol.has_callee,
        };
        let rc = self.print_symbol(writer, line, level_prefix, &mut psym)?;
        symbol.expand_line = psym.expand_line;
        Ok(rc)
    }
}

struct PosixDriver;

impl OutputDriver for PosixDriver {
    fn symbol(
        &mut self,
        writer: &mut dyn Write,
        line: usize,
        level_prefix: &str,
        symbol: &mut OutputSymbolView,
        brief_listing: bool,
    ) -> io::Result<bool> {
        write!(writer, "{}{}: ", level_prefix, symbol.name)?;
        if brief_listing {
            if let Some(expand_line) = symbol.expand_line {
                write!(writer, "{expand_line}")?;
                return Ok(true);
            } else if symbol.has_callee {
                symbol.expand_line = Some(line);
            }
        }

        match symbol.kind {
            SymbolKind::Identifier => {
                if let Some(source) = &symbol.source {
                    if let Some(decl) = &symbol.decl {
                        write!(writer, "{}:{} {}", source, symbol.def_line, decl)?;
                    } else {
                        write!(writer, "{}:{}", source, symbol.def_line)?;
                    }
                } else if let Some(decl) = &symbol.decl {
                    write!(writer, "{decl}")?;
                }
            }
            SymbolKind::Token => {
                if let Some(source) = &symbol.source {
                    write!(writer, "type {}:{}", source, symbol.def_line)?;
                } else {
                    writer.write_all(b"type")?;
                }
            }
            SymbolKind::Undefined => {
                writer.write_all(b"undefined")?;
            }
        }

        Ok(false)
    }
}

impl OutputDriver for crate::gnu::Gnu {
    fn begin(&mut self, writer: &mut dyn Write, line: usize) -> io::Result<()> {
        let _ = self.output_handler(
            writer,
            line,
            crate::gnu::GnuCommand::Begin {
                emacs_mode: false,
                package_string: env!("CARGO_PKG_NAME"),
            },
            false,
            true,
        )?;
        Ok(())
    }

    fn end(&mut self, writer: &mut dyn Write, line: usize) -> io::Result<()> {
        let _ = self.output_handler(
            writer,
            line,
            crate::gnu::GnuCommand::End,
            false,
            true,
        )?;
        Ok(())
    }

    fn separator(&mut self, writer: &mut dyn Write, line: usize) -> io::Result<()> {
        let _ = self.output_handler(
            writer,
            line,
            crate::gnu::GnuCommand::Separator,
            false,
            true,
        )?;
        Ok(())
    }

    fn newline(&mut self, writer: &mut dyn Write, line: usize) -> io::Result<()> {
        let _ = self.output_handler(
            writer,
            line,
            crate::gnu::GnuCommand::Newline,
            false,
            true,
        )?;
        Ok(())
    }

    fn text(&mut self, writer: &mut dyn Write, line: usize, text: &str) -> io::Result<()> {
        let _ = self.output_handler(
            writer,
            line,
            crate::gnu::GnuCommand::Text(text),
            false,
            true,
        )?;
        Ok(())
    }

    fn symbol(
        &mut self,
        writer: &mut dyn Write,
        line: usize,
        _level_prefix: &str,
        symbol: &mut OutputSymbolView,
        brief_listing: bool,
    ) -> io::Result<bool> {
        let view = crate::gnu::SymbolView {
            name: &symbol.name,
            arity: symbol.arity,
            decl: symbol.decl.as_deref(),
            source: symbol.source.as_deref(),
            def_line: Some(symbol.def_line),
            active: None,
            recursive: symbol.recursive,
            has_callee: symbol.has_callee,
            has_caller: symbol.has_caller,
            expand_line: symbol.expand_line,
        };
        let mut osym = crate::gnu::OutputSymbol {
            direct: symbol.direct,
            level: symbol.level,
            last: symbol.last,
            sym: &view,
        };
        self.output_handler(
            writer,
            line,
            crate::gnu::GnuCommand::Symbol(&mut osym),
            brief_listing,
            true,
        )
    }
}

impl OutputDriver for crate::dot::Dot {
    fn begin(&mut self, writer: &mut dyn Write, _line: usize) -> io::Result<()> {
        self.output_handler(writer, crate::dot::DotCommand::Begin)
    }

    fn end(&mut self, writer: &mut dyn Write, _line: usize) -> io::Result<()> {
        self.output_handler(writer, crate::dot::DotCommand::End)
    }

    fn symbol(
        &mut self,
        writer: &mut dyn Write,
        line: usize,
        _level_prefix: &str,
        symbol: &mut OutputSymbolView,
        _brief_listing: bool,
    ) -> io::Result<bool> {
        let related = Vec::<String>::new();
        self.output_handler(
            writer,
            crate::dot::DotCommand::Symbol {
                line,
                name: &symbol.name,
                decl: symbol.decl.as_deref(),
                source: symbol.source.as_deref(),
                def_line: Some(symbol.def_line),
                active: false,
                expand_line: symbol.expand_line,
                direct: symbol.direct,
                related_symbols: related,
            },
        )?;
        Ok(false)
    }
}

struct GnuDriver;

impl OutputDriver for GnuDriver {
    fn symbol(
        &mut self,
        writer: &mut dyn Write,
        line: usize,
        level_prefix: &str,
        symbol: &mut OutputSymbolView,
        brief_listing: bool,
    ) -> io::Result<bool> {
        let has_subtree = if symbol.direct {
            symbol.has_callee
        } else {
            symbol.has_caller
        };

        write!(writer, "{level_prefix}")?;
        if has_subtree {
            write!(writer, "{}()", symbol.name)?;
        } else {
            write!(writer, "{}", symbol.name)?;
        }
        if symbol.recursive {
            writer.write_all(b" [recursive]")?;
        }

        if brief_listing {
            if let Some(expand_line) = symbol.expand_line {
                write!(writer, " [see {expand_line}]")?;
                return Ok(true);
            } else if symbol.has_callee {
                symbol.expand_line = Some(line);
            }
        }

        Ok(false)
    }
}

impl Output {
    pub fn new() -> Self {
        let mut output = Self {
            drivers: Vec::new(),
            driver_index: 0,
            out_line: 1,
            level_mark: Vec::new(),
            level_mark_incr: 16,
            level_begin: String::new(),
            level_indent: ["    ".to_string(), "|   ".to_string()],
            level_end: ["".to_string(), "`-- ".to_string()],
            print_line_numbers: false,
            print_levels: false,
            brief_listing: false,
            reverse_tree: false,
            all_functions: 0,
            max_depth: None,
            print_xref: false,
            print_tree: true,
        };
        let _ = output.register_output("gnu", Box::new(GnuDriver));
        let _ = output.register_output("posix", Box::new(PosixDriver));
        output
    }

    pub fn register_output(&mut self, name: &str, driver: Box<dyn OutputDriver>) -> usize {
        let index = self.drivers.len();
        self.drivers.push(DriverEntry {
            name: name.to_string(),
            driver,
        });
        index
    }

    pub fn driver(&mut self, name: &str) -> Option<usize> {
        for (index, entry) in self.drivers.iter().enumerate() {
            if entry.name == name {
                self.driver_index = index;
                return Some(index);
            }
        }
        None
    }

    pub fn set_level_mark(&mut self, level: usize, mark: bool) {
        if level >= self.level_mark.len() {
            let new_len = (level + 1).max(self.level_mark.len() + self.level_mark_incr);
            self.level_mark.resize(new_len, false);
        }
        self.level_mark[level] = mark;
    }

    pub fn print_level(&self, level: usize, last: bool) -> String {
        let mut buf = String::new();

        if self.print_line_numbers {
            buf.push_str(&format!("{:5} ", self.out_line));
        }
        if self.print_levels {
            buf.push_str(&format!("{{{:4}}} ", level));
        }
        buf.push_str(&self.level_begin);
        for i in 0..level {
            let mark = self.level_mark.get(i).copied().unwrap_or(false) as usize;
            buf.push_str(&self.level_indent[mark]);
        }
        buf.push_str(&self.level_end[usize::from(last)]);
        buf
    }

    pub fn newline<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        let line = self.out_line;
        self.drivers[self.driver_index]
            .driver
            .newline(writer, line)?;
        self.out_line += 1;
        Ok(())
    }

    pub fn begin<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        let line = self.out_line;
        self.drivers[self.driver_index].driver.begin(writer, line)
    }

    pub fn end<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        let line = self.out_line;
        self.drivers[self.driver_index].driver.end(writer, line)
    }

    pub fn separator<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        let line = self.out_line;
        self.drivers[self.driver_index]
            .driver
            .separator(writer, line)
    }

    pub fn print_text<W: Write>(&mut self, writer: &mut W, text: &str) -> io::Result<()> {
        let line = self.out_line;
        self.drivers[self.driver_index]
            .driver
            .text(writer, line, text)
    }

    pub fn print_symbol<W: Write>(
        &mut self,
        writer: &mut W,
        direct: bool,
        level: usize,
        last: bool,
        symbols: &mut HashMap<String, OutputNode>,
        name: &str,
    ) -> io::Result<bool> {
        let Some(node) = symbols.get(name).cloned() else {
            return Ok(false);
        };

        let prefix = self.print_level(level, last);
        let mut view = OutputSymbolView {
            direct,
            level,
            last,
            name: node.name.clone(),
            kind: node.kind,
            storage: node.storage,
            arity: node.arity,
            source: node.source.clone(),
            def_line: node.def_line,
            decl: node.decl.clone(),
            expand_line: node.expand_line,
            recursive: node.recursive,
            has_caller: !node.caller.is_empty(),
            has_callee: !node.callee.is_empty(),
        };

        let line = self.out_line;
        let result = self.drivers[self.driver_index]
            .driver
            .symbol(writer, line, &prefix, &mut view, self.brief_listing)?;

        if let Some(current) = symbols.get_mut(name) {
            current.expand_line = view.expand_line;
        }

        Ok(result)
    }

    pub fn compare(&self, left: &str, right: &str) -> Ordering {
        left.cmp(right)
    }

    pub fn is_var(&self, symbol: &SymbolRecord) -> bool {
        if self.include_symbol(symbol) {
            if symbol.kind == SimpleSymbolKind::Identifier {
                symbol.storage == SimpleStorageClass::Extern
                    || symbol.storage == SimpleStorageClass::Static
            } else {
                true
            }
        } else {
            false
        }
    }

    pub fn symbol_is_function(&self, symbol: &SymbolRecord) -> bool {
        symbol.kind == SimpleSymbolKind::Identifier && symbol.arity >= 0
    }

    pub fn clear_active(&self, symbol: &mut SymbolRecord) {
        symbol.active = 0;
    }

    pub fn print_refs<W: Write>(
        &self,
        writer: &mut W,
        name: &str,
        refs: &[SimpleReference],
    ) -> io::Result<()> {
        for reference in refs {
            writeln!(writer, "{}   {}:{}", name, reference.source, reference.line)?;
        }
        Ok(())
    }

    pub fn print_function<W: Write>(
        &self,
        writer: &mut W,
        symbol: &SymbolRecord,
    ) -> io::Result<()> {
        if let Some(source) = &symbol.source {
            writeln!(
                writer,
                "{} * {}:{} {}",
                symbol.name,
                source,
                symbol.def_line,
                symbol.decl.as_deref().unwrap_or("")
            )?;
        }
        self.print_refs(writer, &symbol.name, &symbol.refs)
    }

    pub fn print_type<W: Write>(&self, writer: &mut W, symbol: &SymbolRecord) -> io::Result<()> {
        if let Some(source) = &symbol.source {
            writeln!(writer, "{} t {}:{}", symbol.name, source, symbol.def_line)?;
        }
        Ok(())
    }

    pub fn xref_output<W: Write>(
        &mut self,
        writer: &mut W,
        symbols: &[SymbolRecord],
    ) -> io::Result<()> {
        let mut vars: Vec<&SymbolRecord> = symbols.iter().filter(|s| self.is_var(s)).collect();
        vars.sort_by(|a, b| self.compare(&a.name, &b.name));

        for symbol in vars {
            match symbol.kind {
                SimpleSymbolKind::Identifier => self.print_function(writer, symbol)?,
                SimpleSymbolKind::Token => self.print_type(writer, symbol)?,
                SimpleSymbolKind::Undefined => {}
            }
        }
        Ok(())
    }

    pub fn set_active(&self, symbol: &mut SymbolRecord) {
        symbol.active = self.out_line;
    }

    pub fn is_printable(&self, symbol: Option<&OutputNode>) -> bool {
        match symbol {
            Some(symbol) => self.include_output_symbol(symbol),
            None => false,
        }
    }

    pub fn is_last(&self, entries: &[String], start_after: usize, symbols: &HashMap<String, OutputNode>) -> bool {
        for name in entries.iter().skip(start_after + 1) {
            if let Some(symbol) = symbols.get(name) {
                if self.is_printable(Some(symbol)) {
                    return false;
                }
            }
        }
        true
    }

    pub fn direct_tree<W: Write>(
        &mut self,
        writer: &mut W,
        level: usize,
        last: bool,
        name: &str,
        symbols: &mut HashMap<String, OutputNode>,
    ) -> io::Result<()> {
        let Some(node) = symbols.get(name).cloned() else {
            return Ok(());
        };

        if node.kind == SymbolKind::Undefined
            || self.max_depth.is_some_and(|max| level >= max)
            || !self.include_output_symbol(&node)
        {
            return Ok(());
        }

        let rc = self.print_symbol(writer, true, level, last, symbols, name)?;
        self.newline(writer)?;
        if rc || node.active != 0 {
            return Ok(());
        }

        if let Some(symbol) = symbols.get_mut(name) {
            symbol.active = self.out_line;
        }

        for (index, callee) in node.callee.iter().enumerate() {
            let mark = !self.is_last(&node.callee, index, symbols);
            self.set_level_mark(level + 1, mark);
            let child_last = self.is_last(&node.callee, index, symbols);
            self.direct_tree(writer, level + 1, child_last, callee, symbols)?;
        }

        if let Some(symbol) = symbols.get_mut(name) {
            symbol.active = 0;
        }

        Ok(())
    }

    pub fn inverted_tree<W: Write>(
        &mut self,
        writer: &mut W,
        level: usize,
        last: bool,
        name: &str,
        symbols: &mut HashMap<String, OutputNode>,
    ) -> io::Result<()> {
        let Some(node) = symbols.get(name).cloned() else {
            return Ok(());
        };

        if node.kind == SymbolKind::Undefined
            || self.max_depth.is_some_and(|max| level >= max)
            || !self.include_output_symbol(&node)
        {
            return Ok(());
        }

        let rc = self.print_symbol(writer, false, level, last, symbols, name)?;
        self.newline(writer)?;
        if rc || node.active != 0 {
            return Ok(());
        }

        if let Some(symbol) = symbols.get_mut(name) {
            symbol.active = self.out_line;
        }

        for (index, caller) in node.caller.iter().enumerate() {
            let mark = !self.is_last(&node.caller, index, symbols);
            self.set_level_mark(level + 1, mark);
            let child_last = self.is_last(&node.caller, index, symbols);
            self.inverted_tree(writer, level + 1, child_last, caller, symbols)?;
        }

        if let Some(symbol) = symbols.get_mut(name) {
            symbol.active = 0;
        }

        Ok(())
    }

    pub fn tree_output<W: Write>(
        &mut self,
        writer: &mut W,
        symbols: &[SymbolRecord],
        starters: &[String],
    ) -> io::Result<()> {
        let mut map = self.build_output_nodes(symbols);
        self.mark_recursive(&mut map);

        let mut names: Vec<String> = map
            .values()
            .filter(|s| self.output_is_var(s))
            .map(|s| s.name.clone())
            .collect();
        names.sort_by(|a, b| self.compare(a, b));

        self.begin(writer)?;

        if self.reverse_tree {
            for name in &names {
                self.inverted_tree(writer, 0, false, name, &mut map)?;
                self.separator(writer)?;
            }
        } else {
            if !starters.is_empty() {
                for starter in starters {
                    self.direct_tree(writer, 0, false, starter, &mut map)?;
                    self.separator(writer)?;
                }
            } else if self.all_functions == 0 {
                self.all_functions = 1;
            }

            if self.all_functions > 0 {
                for name in &names {
                    let Some(symbol) = map.get(name) else {
                        continue;
                    };
                    if !symbol.flag_start
                        && symbol.source.is_some()
                        && (self.all_functions > 1 || symbol.caller.is_empty())
                    {
                        self.direct_tree(writer, 0, false, name, &mut map)?;
                        self.separator(writer)?;
                    }
                }
            }
        }

        self.end(writer)
    }

    pub fn output<W: Write>(
        &mut self,
        writer: &mut W,
        symbols: &[SymbolRecord],
        starters: &[String],
    ) -> io::Result<()> {
        if self.drivers.is_empty() {
            return Ok(());
        }

        self.drivers[self.driver_index].driver.init()?;
        self.set_level_mark(0, false);

        if self.print_xref {
            self.xref_output(writer, symbols)?;
        }
        if self.print_tree {
            self.tree_output(writer, symbols, starters)?;
        }

        Ok(())
    }

    fn include_symbol(&self, symbol: &SymbolRecord) -> bool {
        symbol.kind != SimpleSymbolKind::Undefined
    }

    fn include_output_symbol(&self, symbol: &OutputNode) -> bool {
        symbol.kind != SymbolKind::Undefined
    }

    fn output_is_var(&self, symbol: &OutputNode) -> bool {
        if self.include_output_symbol(symbol) {
            if symbol.kind == SymbolKind::Identifier {
                symbol.storage == StorageClass::Extern || symbol.storage == StorageClass::Static
            } else {
                true
            }
        } else {
            false
        }
    }

    fn build_output_nodes(&self, symbols: &[SymbolRecord]) -> HashMap<String, OutputNode> {
        let mut map = HashMap::new();
        for symbol in symbols {
            map.insert(
                symbol.name.clone(),
                OutputNode {
                    name: symbol.name.clone(),
                    kind: match symbol.kind {
                        SimpleSymbolKind::Identifier => SymbolKind::Identifier,
                        SimpleSymbolKind::Token => SymbolKind::Token,
                        SimpleSymbolKind::Undefined => SymbolKind::Undefined,
                    },
                    storage: match symbol.storage {
                        SimpleStorageClass::Extern => StorageClass::Extern,
                        SimpleStorageClass::Static => StorageClass::Static,
                        SimpleStorageClass::Other => StorageClass::Other,
                    },
                    arity: symbol.arity,
                    source: symbol.source.clone(),
                    def_line: symbol.def_line,
                    decl: symbol.decl.clone(),
                    expand_line: symbol.expand_line,
                    recursive: symbol.recursive,
                    active: symbol.active,
                    caller: symbol.callers.clone(),
                    callee: symbol.callees.clone(),
                    flag_start: symbol.flag_start,
                    refs: symbol
                        .refs
                        .iter()
                        .map(|r| Reference {
                            source: r.source.clone(),
                            line: r.line,
                        })
                        .collect(),
                },
            );
        }
        map
    }

    fn mark_recursive(&self, symbols: &mut HashMap<String, OutputNode>) {
        let names: Vec<String> = symbols
            .values()
            .filter(|s| s.kind == SymbolKind::Identifier && s.arity >= 0)
            .map(|s| s.name.clone())
            .collect();

        let index_by_name: HashMap<String, usize> =
            names.iter().enumerate().map(|(i, n)| (n.clone(), i)).collect();

        let mut reach = vec![vec![false; names.len()]; names.len()];
        for (i, name) in names.iter().enumerate() {
            if let Some(symbol) = symbols.get(name) {
                for callee in &symbol.callee {
                    if let Some(&j) = index_by_name.get(callee) {
                        reach[i][j] = true;
                    }
                }
            }
        }

        for k in 0..names.len() {
            for i in 0..names.len() {
                if reach[i][k] {
                    for j in 0..names.len() {
                        if reach[k][j] {
                            reach[i][j] = true;
                        }
                    }
                }
            }
        }

        let recursive: HashSet<String> = names
            .iter()
            .enumerate()
            .filter_map(|(i, name)| reach[i][i].then_some(name.clone()))
            .collect();

        for name in recursive {
            if let Some(symbol) = symbols.get_mut(&name) {
                symbol.recursive = true;
            }
        }
    }

}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SimpleSymbolKind {
    Identifier,
    Token,
    Undefined,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SimpleStorageClass {
    Extern,
    Static,
    Other,
}

#[derive(Clone, Debug)]
pub struct SimpleReference {
    pub source: String,
    pub line: usize,
}

#[derive(Clone, Debug)]
pub struct SymbolRecord {
    pub name: String,
    pub kind: SimpleSymbolKind,
    pub storage: SimpleStorageClass,
    pub arity: i32,
    pub source: Option<String>,
    pub def_line: usize,
    pub decl: Option<String>,
    pub expand_line: Option<usize>,
    pub recursive: bool,
    pub active: usize,
    pub callers: Vec<String>,
    pub callees: Vec<String>,
    pub flag_start: bool,
    pub refs: Vec<SimpleReference>,
}
