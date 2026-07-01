mod c;
mod dot;
mod error;
mod exitfail;
mod gnu;
mod help;
mod output;
mod parseopt;
mod parser;
mod posix;
mod progname;
mod symbol;
mod wordsplit;
mod xalloc_die;

use crate::c::C;
use crate::error::Error;
use crate::output::Output;
use crate::parseopt::{Parseopt, Parseopt03};
use crate::parser::Parser;
use crate::progname::Progname;
use crate::symbol::{Storage, Symbol, SymbolType};
use crate::wordsplit::Wordsplit;
use crate::xalloc_die::XallocDie;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

const SM_DATA: u32 = 1 << 0;
const SM_UNDERSCORE: u32 = 1 << 1;
const SM_STATIC: u32 = 1 << 2;
const SM_TYPEDEF: u32 = 1 << 3;
const SM_UNDEFINED: u32 = 1 << 4;
const SM_FUNCTIONS: u32 = 1 << 5;

const PRINT_TREE: i32 = 1;
const PRINT_XREF: i32 = 2;

const MAX_LEVEL_INDENT: usize = 256;
const LOCAL_RC: &str = "~/.cflowrc";
const DEFAULT_PROFILE_NAME: &str = "default";
const OPTFILE_SUFFIX: &str = ".rc";
const DEFAULT_OPTPATH: &str = "~/.cflow";
const CFLOW_PREPROC: &str = "cpp";
const PACKAGE_NAME: &str = "cflow-new";
const PACKAGE_VERSION: &str = "0.1.0";
const GPLV3_TEXT: &str = "License GPLv3+: GNU GPL version 3 or later.";
const COPYRIGHT_YEAR: i32 = 2025;

#[derive(Clone, Debug)]
pub struct OptionType {
    pub str_value: &'static str,
    pub min_match: usize,
    pub type_code: i32,
}

#[derive(Clone, Debug, Default)]
pub struct ParseoptFile {
    pub dir_name: Option<String>,
    pub file_name: String,
    pub line: usize,
    pub lines: Vec<String>,
    pub program_name: Option<String>,
    pub ignore_errors: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ModuleSrcParseopt04 {
    pub enabled: bool,
}

#[derive(Clone, Debug)]
struct SymbolOverride {
    token_type: i32,
    alias: Option<String>,
}

pub struct Main {
    symbol_map: u32,
    output_visible: bool,
    print_option: i32,
    preprocess_option: bool,
    optfilepath: String,
    default_profile_name: Option<String>,
    no_main_option: bool,
    level_begin: Option<String>,
    level_indent: [Option<String>; 2],
    level_end: [Option<String>; 2],
    program_name: Option<String>,
    input_file_count: usize,
    token_stack_length: usize,
    print_as_tree: bool,
    output: Output,
    c: C,
    parser: Parser,
    registered_optfiles: HashSet<String>,
    symbol_overrides: HashMap<String, SymbolOverride>,
    starters: Vec<String>,
    main_symbol: Option<String>,
    selected_output_driver: Option<String>,
    prepend_paths: Vec<String>,
    include_classes_spec: Vec<String>,
    preproc_options: Vec<(char, String)>,
    module_src_parseopt_04_state: ModuleSrcParseopt04,
}

impl Main {
    pub fn new() -> Self {
        let mut output = Output::new();
        let _ = output.register_output("gnu", crate::gnu::Gnu::new());
        let _ = output.register_output("posix", crate::posix::Posix::new());
        let _ = output.register_output("dot", crate::dot::Dot::new());
        Self {
            symbol_map: SM_FUNCTIONS | SM_STATIC | SM_UNDEFINED,
            output_visible: true,
            print_option: 0,
            preprocess_option: false,
            optfilepath: DEFAULT_OPTPATH.to_string(),
            default_profile_name: Some(DEFAULT_PROFILE_NAME.to_string()),
            no_main_option: false,
            level_begin: None,
            level_indent: [None, None],
            level_end: [None, None],
            program_name: None,
            input_file_count: 0,
            token_stack_length: 1,
            print_as_tree: false,
            output,
            c: C::default(),
            parser: Parser::default(),
            registered_optfiles: HashSet::new(),
            symbol_overrides: HashMap::new(),
            starters: Vec::new(),
            main_symbol: None,
            selected_output_driver: None,
            prepend_paths: Vec::new(),
            include_classes_spec: Vec::new(),
            preproc_options: Vec::new(),
            module_src_parseopt_04_state: ModuleSrcParseopt04 { enabled: false },
        }
    }

    pub fn option_type() -> Vec<OptionType> {
        vec![
            OptionType {
                str_value: "begin",
                min_match: 1,
                type_code: 1,
            },
            OptionType {
                str_value: "0",
                min_match: 1,
                type_code: 2,
            },
            OptionType {
                str_value: "1",
                min_match: 1,
                type_code: 3,
            },
            OptionType {
                str_value: "end0",
                min_match: 4,
                type_code: 4,
            },
            OptionType {
                str_value: "end1",
                min_match: 4,
                type_code: 5,
            },
            OptionType {
                str_value: "data",
                min_match: 1,
                type_code: SM_DATA as i32,
            },
            OptionType {
                str_value: "static",
                min_match: 1,
                type_code: SM_STATIC as i32,
            },
            OptionType {
                str_value: "typedef",
                min_match: 1,
                type_code: SM_TYPEDEF as i32,
            },
            OptionType {
                str_value: "undefined",
                min_match: 1,
                type_code: SM_UNDEFINED as i32,
            },
        ]
    }

    pub fn module_src_parseopt_03() -> Parseopt03 {
        Parseopt::parseopt_03()
    }

    pub fn module_src_parseopt_04() -> ModuleSrcParseopt04 {
        ModuleSrcParseopt04 { enabled: false }
    }

    pub fn char_to_sm(c: char) -> u32 {
        match c {
            'x' => SM_DATA,
            '_' => SM_UNDERSCORE,
            's' => SM_STATIC,
            't' => SM_TYPEDEF,
            'u' => SM_UNDEFINED,
            _ => 0,
        }
    }

    pub fn find_option_type(options: &[OptionType], text: &str, len: usize) -> i32 {
        let effective_len = if len == 0 { text.len() } else { len };
        for item in options {
            if effective_len >= item.min_match
                && effective_len <= item.str_value.len()
                && item.str_value
                    .as_bytes()
                    .get(..effective_len)
                    == text.as_bytes().get(..effective_len)
            {
                return item.type_code;
            }
        }
        0
    }

    pub fn symbol_override(&mut self, spec: &str) {
        let Some(ptr) = spec.find(':') else {
            Error::report(Some(64), 0, format_args!("{spec}: no symbol type supplied"));
            return;
        };

        let name = spec[..ptr].to_string();
        let tail = &spec[ptr + 1..];

        if let Some(alias_target) = tail.strip_prefix('=') {
            if name == alias_target {
                Error::report(
                    Some(64),
                    0,
                    format_args!("cyclic alias: {name} -> {alias_target}"),
                );
                return;
            }

            if self
                .symbol_overrides
                .get(alias_target)
                .is_some_and(|existing| existing.alias.as_deref() == Some(name.as_str()))
            {
                Error::report(
                    Some(64),
                    0,
                    format_args!("cyclic alias: {name} -> {alias_target} -> {name}"),
                );
                return;
            }

            self.symbol_overrides.insert(
                name,
                SymbolOverride {
                    token_type: 0,
                    alias: Some(alias_target.to_string()),
                },
            );
        } else {
            let ty = Self::find_option_type(&Self::option_type(), tail, 0);
            if ty == 0 {
                Error::report(Some(64), 0, format_args!("unknown symbol type: {tail}"));
                return;
            }
            self.symbol_overrides.insert(
                name,
                SymbolOverride {
                    token_type: ty,
                    alias: None,
                },
            );
        }
    }

    pub fn parse_number(input: &str, base: u32, count: usize) -> (i32, usize) {
        let mut n = 0i32;
        let mut consumed = 0usize;
        let chars: Vec<char> = input.chars().collect();

        while consumed < chars.len() && consumed < count {
            let c = chars[consumed];
            let i = if c.is_ascii_digit() {
                c as u32 - '0' as u32
            } else {
                c.to_ascii_uppercase() as u32 - 'A' as u32 + 10
            };
            if i > base {
                break;
            }
            n = n.saturating_mul(base as i32).saturating_add(i as i32);
            consumed += 1;
        }

        (n, consumed.saturating_sub(1))
    }

    pub fn parse_level_string(text: &str) -> String {
        let mut out = String::with_capacity(MAX_LEVEL_INDENT);
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0usize;

        while i < chars.len() {
            match chars[i] {
                '\\' => {
                    i += 1;
                    if i >= chars.len() {
                        break;
                    }
                    let ch = match chars[i] {
                        'a' => '\u{0007}',
                        'b' => '\u{0008}',
                        'e' => '\u{001b}',
                        'f' => '\u{000c}',
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        'x' | 'X' => {
                            i += 1;
                            let tail: String = chars[i..].iter().collect();
                            let (num, backoff) = Self::parse_number(&tail, 16, 2);
                            i += backoff;
                            char::from_u32(num as u32).unwrap_or('\0')
                        }
                        '0' => {
                            i += 1;
                            let tail: String = chars[i..].iter().collect();
                            let (num, backoff) = Self::parse_number(&tail, 8, 3);
                            i += backoff;
                            char::from_u32(num as u32).unwrap_or('\0')
                        }
                        other => other,
                    };
                    out.push(ch);
                    i += 1;
                }
                'x' if !out.is_empty() => {
                    let mut j = i + 1;
                    while j < chars.len() && chars[j].is_ascii_digit() {
                        j += 1;
                    }
                    let num = chars[i + 1..j]
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap_or(0);
                    let c = out.chars().last().unwrap_or(' ');
                    for _ in 1..num {
                        out.push(c);
                        if out.len() >= MAX_LEVEL_INDENT - 1 {
                            Error::report(
                                Some(64),
                                0,
                                format_args!("level indent string is too long"),
                            );
                            break;
                        }
                    }
                    i = j;
                }
                ch => {
                    out.push(ch);
                    if out.len() >= MAX_LEVEL_INDENT - 1 {
                        Error::report(Some(64), 0, format_args!("level indent string is too long"));
                        break;
                    }
                    i += 1;
                }
            }
        }

        out
    }

    pub fn set_level_indent(&mut self, spec: &str) {
        if let Ok(n) = spec.parse::<usize>() {
            if n > 0 {
            let s = " ".repeat(n.saturating_sub(1));
            self.level_indent[0] = Some(s.clone());
            self.level_indent[1] = Some(s);
            return;
            }
        }

        let Some(eq) = spec.find('=') else {
            Error::report(Some(64), 0, format_args!("level-indent syntax"));
            return;
        };

        let key = &spec[..eq];
        let value = &spec[eq + 1..];

        match Self::find_option_type(&Self::option_type(), key, key.len()) {
            1 => self.level_begin = Some(Self::parse_level_string(value)),
            2 => self.level_indent[0] = Some(Self::parse_level_string(value)),
            3 => self.level_indent[1] = Some(Self::parse_level_string(value)),
            4 => self.level_end[0] = Some(Self::parse_level_string(value)),
            5 => self.level_end[1] = Some(Self::parse_level_string(value)),
            _ => Error::report(
                Some(64),
                0,
                format_args!("unknown level indent option: {spec}"),
            ),
        }
    }

    pub fn optset_include_classes(&mut self, arg: &str) -> Result<(), i32> {
        let mut include = true;
        for ch in arg.chars() {
            match ch {
                '-' | '^' => include = false,
                '+' => include = true,
                'x' | '_' | 's' | 't' | 'u' => {
                    let bit = Self::char_to_sm(ch);
                    if include {
                        self.symbol_map |= bit;
                    } else {
                        self.symbol_map &= !bit;
                    }
                }
                _ => {
                    eprintln!("Unknown symbol class: {ch}");
                    return Err(64);
                }
            }
        }
        self.include_classes_spec.push(arg.to_string());
        Ok(())
    }

    pub fn optset_output_driver(&mut self, arg: &str) -> Result<(), i32> {
        if !self.output.driver(arg) {
            eprintln!("{arg}: No such output driver");
            return Err(64);
        }
        self.selected_output_driver = Some(arg.to_string());
        Ok(())
    }

    pub fn optset_xref(&mut self) {
        self.print_option = PRINT_XREF;
        self.symbol_map &= !SM_STATIC;
    }

    pub fn optset_symbol(&mut self, arg: &str) {
        self.symbol_override(arg);
    }

    pub fn optset_preproc_option(&mut self, opt_name: &str, arg: &str) {
        if let Some(ch) = opt_name.chars().next() {
            self.preproc_options.push((ch, arg.to_string()));
            self.c.pp_option(ch as i32, arg);
            self.preprocess_option = true;
        }
    }

    pub fn optset_preprocess(&mut self, arg: Option<&str>) {
        self.preprocess_option = true;
        self.c.set_preprocessor(arg.unwrap_or(CFLOW_PREPROC));
    }

    pub fn optset_level_indent(&mut self, arg: &str) {
        self.set_level_indent(arg);
    }

    pub fn symbol(&mut self, arg: Option<&str>, clear: bool) {
        if clear {
            self.starters.clear();
            self.main_symbol = None;
            self.no_main_option = true;
        } else if let Some(name) = arg {
            self.starters.push(name.to_string());
            self.main_symbol = Some(name.to_string());
        }
    }

    pub fn optset_install_target(&mut self, arg: &str) {
        let _ = Symbol::install_target(arg.to_string());
    }

    pub fn optset_int_1(&mut self, slot: &mut i32, arg: Option<&str>) -> Result<(), i32> {
        if let Some(value) = arg {
            match value.parse::<i32>() {
                Ok(v) => *slot = v,
                Err(_) => return Err(64),
            }
        } else {
            *slot = 1;
        }
        Ok(())
    }

    pub fn optset_prepend_path(&mut self, arg: &str) {
        if self.optfilepath.is_empty() {
            self.optfilepath = arg.to_string();
        } else {
            self.optfilepath = format!("{arg}:{}", self.optfilepath);
        }
        self.prepend_paths.push(arg.to_string());
    }

    pub fn version_hook<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writeln!(
            writer,
            "{} ({}) {}",
            self.program_name.as_deref().unwrap_or("cflow-new"),
            PACKAGE_NAME,
            PACKAGE_VERSION
        )?;
        writeln!(
            writer,
            "Copyright (C) 2005-{} Sergey Poznyakoff",
            COPYRIGHT_YEAR
        )?;
        write!(writer, "{GPLV3_TEXT}")?;
        write!(writer, "Written by Sergey Poznyakoff.")?;
        Ok(())
    }

    pub fn help_hook<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writeln!(writer, "References:")?;
        writeln!(
            writer,
            " [1]   https://www.gnu.org/software/cflow/manual/html_section/ASCII-Tree.html"
        )?;
        writeln!(writer)?;
        writeln!(writer, "Profile search path: {}", DEFAULT_OPTPATH)?;
        Ok(())
    }

    pub fn po_env_error(&self, is_error: bool, message: &str) {
        if is_error {
            if let Some(name) = &self.program_name {
                eprint!("{name}: ");
            }
            eprint!("CFLOW_OPTIONS: ");
        }
        eprintln!("{message}");
    }

    pub fn parseopt_from_env(&mut self) {
        let Some(env_text) = env::var("CFLOW_OPTIONS").ok() else {
            return;
        };
        if env_text.is_empty() {
            return;
        }

        let mut ws = Wordsplit::new();
        if ws.wordsplit(&env_text, 0) != 0 {
            Error::report(
                Some(1),
                0,
                format_args!("failed to parse CFLOW_OPTIONS: {}", ws.strerror()),
            );
            return;
        }

        for word in ws.get_words() {
            self.apply_option_word(word, true);
        }
    }

    pub fn fromfile_error(&self, pf: &ParseoptFile, is_error: bool, message: &str) {
        if pf.ignore_errors {
            return;
        }
        if is_error {
            if let Some(name) = &pf.program_name {
                eprint!("{name}: ");
            }
            if let Some(dir) = &pf.dir_name {
                eprint!("{dir}/{}:{}: ", pf.file_name, pf.line);
            } else {
                eprint!("{}:{}: ", pf.file_name, pf.line);
            }
        }
        eprintln!("{message}");
    }

    pub fn fromfile(&mut self, pf: &mut ParseoptFile) -> i32 {
        let mut res = 0;
        for line in pf.lines.clone() {
            pf.line += 1;
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let mut ws = Wordsplit::new();
            if ws.wordsplit(trimmed, 0) != 0 {
                self.fromfile_error(
                    pf,
                    true,
                    &format!("error splitting line: {}", ws.strerror()),
                );
                res = 1;
                break;
            }

            for word in ws.get_words() {
                self.apply_option_word(word, false);
            }
        }
        res
    }

    pub fn optfile_register(&mut self, canonical_id: String) -> bool {
        !self.registered_optfiles.insert(canonical_id)
    }

    pub fn expand_tilde(path: &str) -> Option<String> {
        if !path.starts_with('~') {
            return Some(path.to_string());
        }

        let slash = path.find('/').unwrap_or(path.len());
        let head = &path[..slash];
        let tail = &path[slash..];

        let home = if head == "~" {
            env::var("HOME").ok()
        } else {
            None
        }?;

        Some(format!("{home}{tail}"))
    }

    pub fn optfile_lookup(&mut self, name: &str) -> Result<ParseoptFile, i32> {
        let direct = Path::new(name);
        if direct.exists() {
            let canonical = direct
                .canonicalize()
                .unwrap_or_else(|_| direct.to_path_buf())
                .display()
                .to_string();
            if self.optfile_register(canonical) {
                return Err(1);
            }
            let lines = Self::read_lines(direct).map_err(|_| 2)?;
            return Ok(ParseoptFile {
                dir_name: None,
                file_name: name.to_string(),
                line: 0,
                lines,
                program_name: self.program_name.clone(),
                ignore_errors: false,
            });
        }

        let file_name = format!("{name}{OPTFILE_SUFFIX}");
        for dir in self.optfilepath.split(':') {
            let Some(dirname) = Self::expand_tilde(dir) else {
                continue;
            };
            let candidate = Path::new(&dirname).join(&file_name);
            if !candidate.exists() {
                continue;
            }
            let canonical = candidate
                .canonicalize()
                .unwrap_or(candidate.clone())
                .display()
                .to_string();
            if self.optfile_register(canonical) {
                return Err(1);
            }
            let lines = Self::read_lines(&candidate).map_err(|_| 2)?;
            return Ok(ParseoptFile {
                dir_name: Some(dirname),
                file_name,
                line: 0,
                lines,
                program_name: self.program_name.clone(),
                ignore_errors: false,
            });
        }
        Err(2)
    }

    pub fn optset_profile(&mut self, arg: &str) -> Result<(), i32> {
        let mut pf = match self.optfile_lookup(arg) {
            Ok(pf) => pf,
            Err(1) => return Ok(()),
            Err(_) => {
                eprintln!("no such profile: {arg}");
                return Err(64);
            }
        };

        let rc = self.fromfile(&mut pf);
        if rc != 0 {
            return Err(64);
        }
        Ok(())
    }

    pub fn parseopt_from_rc(&mut self, file_name: &str) -> i32 {
        let path = Path::new(file_name);
        if !path.exists() {
            return 0;
        }

        match Self::read_lines(path) {
            Ok(lines) => {
                let mut pf = ParseoptFile {
                    dir_name: None,
                    file_name: file_name.to_string(),
                    line: 0,
                    lines,
                    program_name: self.program_name.clone(),
                    ignore_errors: false,
                };
                self.fromfile(&mut pf)
            }
            Err(err) => {
                self.po_env_error(true, &format!("can't open option file {file_name}: {err}"));
                1
            }
        }
    }

    pub fn parse_rc(&mut self) -> i32 {
        if let Ok(name) = env::var("CFLOWRC") {
            if name.is_empty() {
                0
            } else {
                self.parseopt_from_rc(&name)
            }
        } else {
            let Some(name) = Self::expand_tilde(LOCAL_RC) else {
                return 0;
            };
            let path = Path::new(&name);
            if path.exists() {
                let n = self.optfilepath.find(':').unwrap_or(self.optfilepath.len());
                Error::report(
                    None,
                    0,
                    format_args!("warning: processing obsolete {} file", LOCAL_RC),
                );
                if n > 0 {
                    let mut m = n;
                    if self.optfilepath.as_bytes().get(n.saturating_sub(1)) == Some(&b'/') {
                        m = m.saturating_sub(1);
                    }
                    let prefix = &self.optfilepath[..m];
                    Error::report(
                        None,
                        0,
                        format_args!(
                            "warning: consider renaming it to {}/{}{}",
                            prefix, DEFAULT_PROFILE_NAME, OPTFILE_SUFFIX
                        ),
                    );
                }
                self.parseopt_from_rc(&name)
            } else {
                0
            }
        }
    }

    pub fn init_hook(&mut self) {
        let Some(profile) = self.default_profile_name.clone() else {
            return;
        };
        if let Ok(mut pf) = self.optfile_lookup(&profile) {
            pf.line = 0;
            let _ = self.fromfile(&mut pf);
        }
    }

    pub fn globals_only(&self) -> bool {
        (self.symbol_map & SM_STATIC) == 0
    }

    pub fn include_symbol(&self, sym: &Symbol) -> bool {
        let mut ty = 0u32;

        if sym.visible != self.output_visible {
            return false;
        }

        match sym.symbol_type {
            SymbolType::Identifier | SymbolType::Function => {
                if sym.name.starts_with('_') && (self.symbol_map & SM_UNDERSCORE) == 0 {
                    return false;
                }

                if matches!(sym.storage, Storage::Static) {
                    ty |= SM_STATIC;
                }
                if sym.arity == -1 && !matches!(sym.storage, Storage::Auto) {
                    ty |= SM_DATA;
                } else if sym.arity >= 0 || matches!(sym.symbol_type, SymbolType::Function) {
                    ty |= SM_FUNCTIONS;
                }

                if sym.source.is_none() {
                    ty |= SM_UNDEFINED;
                }
            }
            SymbolType::Token => {
                if sym.source.is_some() {
                    ty |= SM_TYPEDEF;
                } else {
                    return false;
                }
            }
            SymbolType::Undefined => {
                ty |= SM_UNDEFINED;
            }
        }

        (self.symbol_map & ty) == ty
    }

    pub fn xalloc_die(&self) -> ! {
        Error::report(Some(1), 12, format_args!("Exiting"));
        XallocDie::die()
    }

    pub fn parseopt_file(&mut self, file_name: &str) -> i32 {
        self.parseopt_from_rc(file_name)
    }

    pub fn main(&mut self) -> i32 {
        let args: Vec<String> = env::args().collect();
        if let Some(argv0) = args.first() {
            Progname::set_program_name(argv0);
            self.program_name = Some(
                Path::new(argv0)
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or(argv0)
                    .to_string(),
            );
        }

        self.parseopt_from_env();
        let _ = self.parse_rc();

        if env::var("POSIXLY_CORRECT").is_ok() {
            if self.output.driver("posix") {
                Error::report(
                    None,
                    0,
                    format_args!("INTERNAL ERROR: {}: No such output driver", "posix"),
                );
                return 1;
            }
        }

        let mut positional = Vec::new();
        for arg in args.iter().skip(1) {
            if arg.starts_with('-') {
                self.apply_option_word(arg, true);
            } else {
                positional.push(arg.clone());
            }
        }

        if self.token_stack_length == 0 {
            Error::report(Some(64), 0, format_args!("argument to -p cannot be 0"));
            return 64;
        }

        if self.print_as_tree {
            self.set_level_indent("0=  ");
            self.set_level_indent("1=| ");
            self.set_level_indent("end0=+-");
            self.set_level_indent("end1=\\\\-");
        } else {
            self.level_indent = [None, None];
            self.level_end = [None, None];
        }

        if !self.no_main_option && self.starters.is_empty() {
            self.starters.push("main".to_string());
            self.main_symbol = Some("main".to_string());
        }

        if self.print_option == 0 {
            self.print_option = PRINT_TREE;
        }

        if self.level_indent[0].is_none() {
            self.level_indent[0] = Some("    ".to_string());
        }
        if self.level_indent[1].is_none() {
            self.level_indent[1] = self.level_indent[0].clone();
        }
        if self.level_end[0].is_none() {
            self.level_end[0] = Some(String::new());
        }
        if self.level_end[1].is_none() {
            self.level_end[1] = Some(String::new());
        }

        self.c.init_lex(0);
        self.parser.init_parse();

        let mut status = 0;
        for file in &positional {
            self.input_file_count += 1;
            if File::open(file).is_err() {
                status = 70;
            }
        }

        if self.input_file_count == 0 {
            Error::report(Some(64), 0, format_args!("no input files"));
            return 64;
        }

        let mut sink = io::stdout();
        let _ = self.output.output(&mut sink);
        status
    }

    fn apply_option_word(&mut self, word: &str, _from_env: bool) {
        if let Some(rest) = word.strip_prefix("--profile=") {
            let _ = self.optset_profile(rest);
        } else if let Some(rest) = word.strip_prefix("--symbol=") {
            self.optset_symbol(rest);
        } else if let Some(rest) = word.strip_prefix("--level-indent=") {
            self.optset_level_indent(rest);
        } else if word == "--xref" {
            self.optset_xref();
        } else if let Some(rest) = word.strip_prefix("--output=") {
            let _ = self.optset_output_driver(rest);
        } else if let Some(rest) = word.strip_prefix("--include=") {
            let _ = self.optset_include_classes(rest);
        } else if let Some(rest) = word.strip_prefix("--preprocess=") {
            self.optset_preprocess(Some(rest));
        } else if word == "--preprocess" {
            self.optset_preprocess(None);
        } else if let Some(rest) = word.strip_prefix("--main=") {
            self.symbol(Some(rest), false);
        } else if word == "--no-main" {
            self.symbol(None, true);
        } else if let Some(rest) = word.strip_prefix("--target=") {
            self.optset_install_target(rest);
        } else if let Some(rest) = word.strip_prefix("--prepend-path=") {
            self.optset_prepend_path(rest);
        } else if word == "--tree" {
            self.print_as_tree = true;
        }
    }

    fn read_lines(path: impl AsRef<Path>) -> io::Result<Vec<String>> {
        let reader = BufReader::new(File::open(path)?);
        reader.lines().collect()
    }
}

fn main() {
    let mut app = Main::new();
    let status = app.main();
    std::process::exit(status);
}
