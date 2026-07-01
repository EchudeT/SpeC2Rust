mod c;
mod dot;
mod depmap;
mod error;
mod gnu;
mod help;
mod output;
mod parseopt;
mod parser;
mod posix;
mod linked_list;
mod progname;
mod symbol;
mod wordsplit;
mod wordwrap;
mod xalloc_die;

use crate::c::C;
use crate::dot::{Dot, OutputCommand, OutputSymbol};
use crate::error::Error;
use crate::gnu::Gnu;
use crate::help::{Context, Help};
use crate::output::{Output, SymbolFlag, SymbolKind};
use crate::parseopt::Parseopt;
use crate::parser::Parser;
use crate::posix::Posix;
use crate::progname::Progname;
use crate::symbol::Symbol;
use crate::wordsplit::Wordsplit;
use crate::wordwrap::Wordwrap;
use crate::xalloc_die::XallocDie;

use std::collections::{BTreeSet, HashSet};
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process;

const SM_DATA: u32 = 1 << 0;
const SM_UNDERSCORE: u32 = 1 << 1;
const SM_STATIC: u32 = 1 << 2;
const SM_TYPEDEF: u32 = 1 << 3;
const SM_UNDEFINED: u32 = 1 << 4;
const SM_FUNCTIONS: u32 = 1 << 5;

const LEVEL_BEGIN: i32 = 1;
const LEVEL_INDENT0: i32 = 2;
const LEVEL_INDENT1: i32 = 3;
const LEVEL_END0: i32 = 4;
const LEVEL_END1: i32 = 5;

const PRINT_TREE: i32 = 1;
const PRINT_XREF: i32 = 2;

const MAX_LEVEL_INDENT: usize = 64;
const PACKAGE_NAME: &str = "cflow-new";
const PACKAGE_VERSION: &str = "0.1.0";
const PACKAGE_STRING: &str = "cflow-new 0.1.0";
const DEFAULT_PREPROCESSOR: &str = "cpp";
const DEFAULT_OPTPATH: &str = ".:/etc/cflow";
const DEFAULT_PROFILE_NAME: &str = "default";
const OPTFILE_SUFFIX: &str = ".rc";
const LOCAL_RC: &str = "~/.cflowrc";

#[derive(Clone, Copy)]
pub struct OptionType {
    pub text: &'static str,
    pub min_match: usize,
    pub kind: i32,
}

#[derive(Clone)]
pub struct ParseoptFile {
    pub dir_name: Option<String>,
    pub file_name: String,
    pub line: usize,
    pub path: PathBuf,
}

#[derive(Clone)]
pub struct ModuleSrcParseopt03 {
    pub symbol_map: u32,
    pub print_option: i32,
    pub preprocess_option: bool,
}

#[derive(Clone)]
pub struct ModuleSrcParseopt04 {
    pub level_begin: Option<String>,
    pub level_indent: [Option<String>; 2],
    pub level_end: [Option<String>; 2],
}

#[derive(Clone)]
pub struct ModuleSrcParseoptFile07 {
    pub optfilepath: String,
    pub default_profile_name: Option<String>,
    pub seen_files: HashSet<PathBuf>,
}

pub struct Main {
    pub parser: Parser,
    pub lexer: C,
    pub output: Output,
    pub symbol_map: u32,
    pub print_option: i32,
    pub preprocess_option: bool,
    pub no_main_option: bool,
    pub level_begin: Option<String>,
    pub level_indent: [Option<String>; 2],
    pub level_end: [Option<String>; 2],
    pub optfilepath: String,
    pub default_profile_name: Option<String>,
    pub preprocessor: Option<String>,
    pub preproc_options: Vec<String>,
    pub output_driver_name: Option<String>,
    pub profile_name: Option<String>,
    pub install_targets: Vec<String>,
    pub starter_symbols: BTreeSet<String>,
    pub main_symbol: Option<String>,
    pub seen_optfiles: HashSet<PathBuf>,
    pub debug: i32,
    pub print_as_tree: bool,
    pub output_visible: bool,
    pub input_files: Vec<String>,
    pub include_overrides: BTreeSet<String>,
}

impl Main {
    pub fn new() -> Self {
        let mut output = Output::new();
        output.register_output("gnu", |command, writer, line, text, symbol| {
            let mut os = symbol.map(|s| OutputSymbol {
                name: s.symbol_name.clone(),
                declaration: s.declaration.clone(),
                source: None,
                definition_line: s.expand_line,
                active: false,
                expand_line: s.expand_line,
                direct: s.direct,
                related_symbols: Vec::new(),
            });
            Gnu::output_handler(
                writer,
                command,
                line,
                text,
                os.as_mut(),
                false,
                PACKAGE_STRING,
                false,
                true,
            )
        });
        output.register_output("posix", |command, writer, line, text, symbol| {
            let mut os = symbol.map(|s| OutputSymbol {
                name: s.symbol_name.clone(),
                declaration: s.declaration.clone(),
                source: None,
                definition_line: s.expand_line,
                active: false,
                expand_line: s.expand_line,
                direct: s.direct,
                related_symbols: Vec::new(),
            });
            Posix::output_handler(writer, command, line, text, os.as_mut(), false)
        });
        output.register_output("dot", |command, writer, line, _text, symbol| match command {
            OutputCommand::Begin => {
                Dot::begin(writer)?;
                Ok(false)
            }
            OutputCommand::End => {
                writeln!(writer, "}}")?;
                Ok(false)
            }
            OutputCommand::Symbol => {
                if let Some(s) = symbol {
                    let mut os = OutputSymbol {
                        name: s.symbol_name.clone(),
                        declaration: s.declaration.clone(),
                        source: None,
                        definition_line: s.expand_line,
                        active: false,
                        expand_line: s.expand_line,
                        direct: s.direct,
                        related_symbols: Vec::new(),
                    };
                    Dot::print_symbol(writer, line, &mut os)?;
                }
                Ok(false)
            }
            _ => Ok(false),
        });

        Self {
            parser: Parser::default(),
            lexer: C::default(),
            output,
            symbol_map: SM_FUNCTIONS | SM_STATIC | SM_UNDEFINED,
            print_option: 0,
            preprocess_option: false,
            no_main_option: false,
            level_begin: None,
            level_indent: [None, None],
            level_end: [None, None],
            optfilepath: DEFAULT_OPTPATH.to_string(),
            default_profile_name: Some(DEFAULT_PROFILE_NAME.to_string()),
            preprocessor: None,
            preproc_options: Vec::new(),
            output_driver_name: None,
            profile_name: None,
            install_targets: Vec::new(),
            starter_symbols: BTreeSet::new(),
            main_symbol: None,
            seen_optfiles: HashSet::new(),
            debug: 0,
            print_as_tree: true,
            output_visible: true,
            input_files: Vec::new(),
            include_overrides: BTreeSet::new(),
        }
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

    pub fn option_type() -> Vec<OptionType> {
        vec![
            OptionType {
                text: "begin",
                min_match: 1,
                kind: LEVEL_BEGIN,
            },
            OptionType {
                text: "0",
                min_match: 1,
                kind: LEVEL_INDENT0,
            },
            OptionType {
                text: "1",
                min_match: 1,
                kind: LEVEL_INDENT1,
            },
            OptionType {
                text: "end0",
                min_match: 4,
                kind: LEVEL_END0,
            },
            OptionType {
                text: "end1",
                min_match: 4,
                kind: LEVEL_END1,
            },
            OptionType {
                text: "data",
                min_match: 1,
                kind: SM_DATA as i32,
            },
            OptionType {
                text: "underscore",
                min_match: 1,
                kind: SM_UNDERSCORE as i32,
            },
            OptionType {
                text: "static",
                min_match: 1,
                kind: SM_STATIC as i32,
            },
            OptionType {
                text: "typedef",
                min_match: 1,
                kind: SM_TYPEDEF as i32,
            },
            OptionType {
                text: "undefined",
                min_match: 1,
                kind: SM_UNDEFINED as i32,
            },
        ]
    }

    pub fn find_option_type(optype: &[OptionType], text: &str, len: usize) -> i32 {
        let effective_len = if len == 0 { text.len() } else { len };
        for opt in optype {
            if effective_len >= opt.min_match
                && effective_len <= opt.text.len()
                && opt.text.as_bytes().get(..effective_len) == text.as_bytes().get(..effective_len)
            {
                return opt.kind;
            }
        }
        0
    }

    pub fn symbol_override(&mut self, text: &str) {
        let Some(pos) = text.find(':') else {
            Error::report(Some(64), Some(0), format_args!("{text}: no symbol type supplied"));
            return;
        };

        let name = text[..pos].to_string();
        let rest = &text[pos + 1..];

        if let Some(alias) = rest.strip_prefix('=') {
            if name == alias {
                Error::report(
                    Some(64),
                    Some(0),
                    format_args!("cyclic alias: {name} -> {alias}"),
                );
                return;
            }
            self.include_overrides.insert(name);
            self.include_overrides.insert(alias.to_string());
        } else {
            let symbol_types = [
                OptionType {
                    text: "x",
                    min_match: 1,
                    kind: SM_DATA as i32,
                },
                OptionType {
                    text: "_",
                    min_match: 1,
                    kind: SM_UNDERSCORE as i32,
                },
                OptionType {
                    text: "s",
                    min_match: 1,
                    kind: SM_STATIC as i32,
                },
                OptionType {
                    text: "t",
                    min_match: 1,
                    kind: SM_TYPEDEF as i32,
                },
                OptionType {
                    text: "u",
                    min_match: 1,
                    kind: SM_UNDEFINED as i32,
                },
            ];
            let kind = Self::find_option_type(&symbol_types, rest, 0);
            if kind == 0 {
                Error::report(
                    Some(64),
                    Some(0),
                    format_args!("unknown symbol type: {rest}"),
                );
                return;
            }
            self.include_overrides.insert(name);
            self.symbol_map |= kind as u32;
        }
    }

    pub fn parse_number(str_ptr: &mut &str, base: u32, count: usize) -> i32 {
        let bytes = str_ptr.as_bytes();
        let mut idx = 0usize;
        let mut remaining = count;
        let mut n = 0i32;

        while idx < bytes.len() && remaining > 0 {
            let c = bytes[idx] as char;
            let i = if c.is_ascii_digit() {
                (c as u8 - b'0') as u32
            } else {
                c.to_ascii_uppercase() as u32 - 'A' as u32 + 10
            };
            if i > base {
                break;
            }
            n = n * base as i32 + i as i32;
            idx += 1;
            remaining -= 1;
        }

        let adjust = idx.saturating_sub(1);
        *str_ptr = &str_ptr[adjust..];
        n
    }

    pub fn parse_level_string(text: &str) -> String {
        let mut out = vec![b' '; MAX_LEVEL_INDENT];
        out[MAX_LEVEL_INDENT - 1] = 0;
        let mut p = 0usize;
        let bytes = text.as_bytes();
        let mut i = 0usize;

        while i < bytes.len() {
            match bytes[i] as char {
                '\\' => {
                    i += 1;
                    if i >= bytes.len() {
                        break;
                    }
                    let ch = match bytes[i] as char {
                        'a' => 0x07,
                        'b' => 0x08,
                        'e' => 0x1b,
                        'f' => 0x0c,
                        'n' => b'\n',
                        'r' => b'\r',
                        't' => b'\t',
                        'x' | 'X' => {
                            i += 1;
                            let mut s = &text[i..];
                            let v = Self::parse_number(&mut s, 16, 2) as u8;
                            i = text.len() - s.len();
                            v
                        }
                        '0' => {
                            i += 1;
                            let mut s = &text[i..];
                            let v = Self::parse_number(&mut s, 8, 3) as u8;
                            i = text.len() - s.len();
                            v
                        }
                        other => other as u8,
                    };
                    if p + 1 >= out.len() {
                        Error::report(
                            Some(64),
                            Some(0),
                            format_args!("level indent string is too long"),
                        );
                        break;
                    }
                    out[p] = ch;
                    p += 1;
                    i += 1;
                }
                'x' if p > 0 => {
                    let mut j = i + 1;
                    while j < bytes.len() && (bytes[j] as char).is_ascii_digit() {
                        j += 1;
                    }
                    let num = text[i + 1..j].parse::<usize>().unwrap_or(0);
                    let c = out[p - 1];
                    for _ in 1..num {
                        if p + 1 >= out.len() {
                            Error::report(
                                Some(64),
                                Some(0),
                                format_args!("level indent string is too long"),
                            );
                            break;
                        }
                        out[p] = c;
                        p += 1;
                    }
                    i = j;
                }
                other => {
                    if p + 1 >= out.len() {
                        Error::report(
                            Some(64),
                            Some(0),
                            format_args!("level indent string is too long"),
                        );
                        break;
                    }
                    out[p] = other as u8;
                    p += 1;
                    i += 1;
                }
            }
        }

        let end = p.min(out.len() - 1);
        out[end] = 0;
        String::from_utf8_lossy(&out[..p]).to_string()
    }

    pub fn set_level_indent(&mut self, text: &str) {
        if let Ok(n) = text.parse::<usize>() {
            if n > 0 {
                let s = " ".repeat(n.saturating_sub(1));
                self.level_indent[0] = Some(s.clone());
                self.level_indent[1] = Some(s);
                return;
            }
        }

        let Some(eq) = text.find('=') else {
            Error::report(Some(64), Some(0), format_args!("level-indent syntax"));
            return;
        };
        let key = &text[..eq];
        let val = &text[eq + 1..];

        let level_options = [
            OptionType {
                text: "begin",
                min_match: 1,
                kind: LEVEL_BEGIN,
            },
            OptionType {
                text: "0",
                min_match: 1,
                kind: LEVEL_INDENT0,
            },
            OptionType {
                text: "1",
                min_match: 1,
                kind: LEVEL_INDENT1,
            },
            OptionType {
                text: "end0",
                min_match: 4,
                kind: LEVEL_END0,
            },
            OptionType {
                text: "end1",
                min_match: 4,
                kind: LEVEL_END1,
            },
        ];

        match Self::find_option_type(&level_options, key, key.len()) {
            LEVEL_BEGIN => self.level_begin = Some(Self::parse_level_string(val)),
            LEVEL_INDENT0 => self.level_indent[0] = Some(Self::parse_level_string(val)),
            LEVEL_INDENT1 => self.level_indent[1] = Some(Self::parse_level_string(val)),
            LEVEL_END0 => self.level_end[0] = Some(Self::parse_level_string(val)),
            LEVEL_END1 => self.level_end[1] = Some(Self::parse_level_string(val)),
            _ => Error::report(
                Some(64),
                Some(0),
                format_args!("unknown level indent option: {text}"),
            ),
        }
    }

    pub fn optset_include_classes(&mut self, arg: &str) -> Result<(), String> {
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
                _ => return Err(format!("Unknown symbol class: {ch}")),
            }
        }
        Ok(())
    }

    pub fn output_driver(&self) -> Option<&str> {
        self.output_driver_name.as_deref()
    }

    pub fn optset_output_driver(&mut self, arg: &str) -> Result<(), String> {
        if !self.output.driver(arg) {
            return Err(format!("{arg}: No such output driver"));
        }
        self.output_driver_name = Some(arg.to_string());
        Ok(())
    }

    pub fn optset_xref(&mut self) -> Result<(), String> {
        self.print_option = PRINT_XREF;
        self.symbol_map &= !SM_STATIC;
        Ok(())
    }

    pub fn optset_symbol(&mut self, arg: &str) -> Result<(), String> {
        self.symbol_override(arg);
        Ok(())
    }

    pub fn optset_preproc_option(&mut self, opt_name: &str, arg: &str) -> Result<(), String> {
        let ch = opt_name
            .chars()
            .next()
            .ok_or_else(|| "empty preprocessor option".to_string())?;
        self.preproc_options.push(format!(" -{ch}{arg}"));
        self.preprocess_option = true;
        self.lexer.pp_option(ch, arg);
        Ok(())
    }

    pub fn optset_preprocess(&mut self, arg: Option<&str>) -> Result<(), String> {
        self.preprocess_option = true;
        let pp = arg.unwrap_or(DEFAULT_PREPROCESSOR).to_string();
        self.preprocessor = Some(pp.clone());
        self.lexer.set_preprocessor(Some(&pp));
        Ok(())
    }

    pub fn optset_level_indent(&mut self, arg: &str) -> Result<(), String> {
        self.set_level_indent(arg);
        Ok(())
    }

    pub fn symbol(&mut self, arg: Option<&str>) -> Result<(), String> {
        if let Some(name) = arg {
            self.starter_symbols.insert(name.to_string());
            self.main_symbol = Some(name.to_string());
        } else {
            self.starter_symbols.clear();
            self.no_main_option = true;
            self.main_symbol = None;
        }
        Ok(())
    }

    pub fn optset_install_target(&mut self, arg: &str) -> Result<(), String> {
        self.install_targets.push(arg.to_string());
        Symbol::install_target(arg.to_string());
        Ok(())
    }

    pub fn optset_int_1(&mut self, target: &mut i32, arg: Option<&str>) -> Result<(), String> {
        if let Some(s) = arg {
            *target = s.parse::<i32>().map_err(|e| e.to_string())?;
        } else {
            *target = 1;
        }
        Ok(())
    }

    pub fn optset_prepend_path(&mut self, arg: &str) -> Result<(), String> {
        self.optfilepath = format!("{arg}:{}", self.optfilepath);
        Ok(())
    }

    pub fn version_hook(&self, program_name: &str, writer: &mut dyn Write) -> io::Result<()> {
        writeln!(writer, "{program_name} ({PACKAGE_NAME}) {PACKAGE_VERSION}")?;
        writeln!(writer, "Copyright (C) 2005-2026 Sergey Poznyakoff")?;
        writeln!(writer, "License GPLv3+: GNU GPL version 3 or later.")?;
        write!(writer, "Written by Sergey Poznyakoff.")?;
        Ok(())
    }

    pub fn help_hook(&self, writer: &mut Wordwrap) -> io::Result<()> {
        writer.printf(format_args!("References:\n"))?;
        writer.puts(" [1]   https://www.gnu.org/software/cflow/manual/html_section/ASCII-Tree.html\n")?;
        writer.para()?;
        writer.printf(format_args!("Profile search path: {}\n", DEFAULT_OPTPATH))?;
        Ok(())
    }

    pub fn po_env_error(&self, program_name: Option<&str>, message: &str) {
        if let Some(name) = program_name {
            eprint!("{name}: ");
        }
        eprintln!("CFLOW_OPTIONS: {message}");
    }

    pub fn parseopt_from_env(&mut self) {
        let Some(env_text) = env::var_os("CFLOW_OPTIONS") else {
            return;
        };
        let env_text = env_text.to_string_lossy().to_string();
        if env_text.is_empty() {
            return;
        }

        let ws = match Wordsplit::new(&env_text, 0) {
            Ok(ws) => ws,
            Err(_) => {
                Error::report(
                    Some(1),
                    Some(0),
                    format_args!("failed to parse CFLOW_OPTIONS"),
                );
                return;
            }
        };

        let mut parseopt = Parseopt::new();
        let words = ws.get_words().to_vec();
        let _ = parseopt.getopt(words.len(), &words);
    }

    pub fn fromfile_error(&self, pf: &ParseoptFile, message: &str) {
        if let Some(dir) = &pf.dir_name {
            eprintln!("{dir}/{}:{}: {message}", pf.file_name, pf.line);
        } else {
            eprintln!("{}:{}: {message}", pf.file_name, pf.line);
        }
    }

    pub fn fromfile(&mut self, pf: &mut ParseoptFile) -> Result<(), String> {
        let file = File::open(&pf.path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            pf.line += 1;
            let line = line.map_err(|e| e.to_string())?;
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let ws = Wordsplit::new(trimmed, 0).map_err(|_| "error splitting line".to_string())?;
            let words = ws.get_words().to_vec();
            let mut parseopt = Parseopt::new();
            let _ = parseopt.getopt(words.len(), &words);
        }
        Ok(())
    }

    pub fn optfile_register(&mut self, path: &Path) -> bool {
        let canonical = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        !self.seen_optfiles.insert(canonical)
    }


    pub fn optfile_lookup(&mut self, name: &str) -> Result<Option<ParseoptFile>, String> {
        let direct = PathBuf::from(name);
        if direct.exists() {
            if self.optfile_register(&direct) {
                return Ok(None);
            }
            return Ok(Some(ParseoptFile {
                dir_name: None,
                file_name: name.to_string(),
                line: 0,
                path: direct,
            }));
        }

        let file_name = format!("{name}{OPTFILE_SUFFIX}");
        for dir in self.optfilepath.split(':') {
            let Some(expanded) = Wordsplit::tilde_expand(dir) else {
                continue;
            };
            let candidate = Path::new(&expanded).join(&file_name);
            if candidate.exists() {
                if self.optfile_register(&candidate) {
                    return Ok(None);
                }
                return Ok(Some(ParseoptFile {
                    dir_name: Some(expanded),
                    file_name: file_name.clone(),
                    line: 0,
                    path: candidate,
                }));
            }
        }

        Ok(None)
    }

    pub fn optset_profile(&mut self, arg: &str) -> Result<(), String> {
        let Some(mut pf) = self.optfile_lookup(arg)? else {
            return Ok(());
        };
        self.fromfile(&mut pf)?;
        self.profile_name = Some(arg.to_string());
        Ok(())
    }

    pub fn parseopt_from_rc(&mut self, file_name: &str) -> i32 {
        let path = PathBuf::from(file_name);
        if !path.exists() {
            return 0;
        }

        let mut pf = ParseoptFile {
            dir_name: None,
            file_name: file_name.to_string(),
            line: 0,
            path,
        };

        match self.fromfile(&mut pf) {
            Ok(()) => 0,
            Err(_) => 1,
        }
    }

    pub fn parse_rc(&mut self) -> i32 {
        if let Ok(name) = env::var("CFLOWRC") {
            if name.is_empty() {
                return 0;
            }
            return self.parseopt_from_rc(&name);
        }

        let name = Wordsplit::tilde_expand(LOCAL_RC).unwrap_or_else(|| LOCAL_RC.to_string());
        let path = PathBuf::from(&name);
        if path.exists() {
            Error::report(
                Some(0),
                Some(0),
                format_args!("warning: processing obsolete {LOCAL_RC} file"),
            );
            return self.parseopt_from_rc(&name);
        }
        0
    }

    pub fn init_hook(&mut self) {
        let Some(profile) = self.default_profile_name.clone() else {
            return;
        };
        if let Ok(Some(mut pf)) = self.optfile_lookup(&profile) {
            let _ = self.fromfile(&mut pf);
        }
    }

    pub fn globals_only(&self) -> bool {
        (self.symbol_map & SM_STATIC) == 0
    }

    pub fn include_symbol(&self, sym: &Symbol) -> bool {
        if sym.visible != self.output_visible {
            return false;
        }

        let mut kind_mask = 0u32;

        match sym.kind {
            SymbolKind::Identifier => {
                if sym.name.starts_with('_') && (self.symbol_map & SM_UNDERSCORE) == 0 {
                    return false;
                }

                if format!("{:?}", sym.storage).contains("Static") {
                    kind_mask |= SM_STATIC;
                }

                if sym.arity == -1 && !format!("{:?}", sym.storage).contains("Auto") {
                    kind_mask |= SM_DATA;
                } else if sym.arity >= 0 {
                    kind_mask |= SM_FUNCTIONS;
                }

                if sym.source.is_none() {
                    kind_mask |= SM_UNDEFINED;
                }
            }
            SymbolKind::Token => {
                if sym.source.is_some() {
                    kind_mask |= SM_TYPEDEF;
                } else {
                    return false;
                }
            }
            _ => return false,
        }

        (self.symbol_map & kind_mask) == kind_mask
    }

    pub fn xalloc_die(&self) -> ! {
        let _ = &self;
        XallocDie::from_main_context()
    }

    pub fn module_src_parseopt_03(&self) -> ModuleSrcParseopt03 {
        ModuleSrcParseopt03 {
            symbol_map: self.symbol_map,
            print_option: self.print_option,
            preprocess_option: self.preprocess_option,
        }
    }

    pub fn module_src_parseopt_04(&self) -> ModuleSrcParseopt04 {
        ModuleSrcParseopt04 {
            level_begin: self.level_begin.clone(),
            level_indent: self.level_indent.clone(),
            level_end: self.level_end.clone(),
        }
    }

    pub fn module_src_parseopt_file_07(&self) -> ModuleSrcParseoptFile07 {
        ModuleSrcParseoptFile07 {
            optfilepath: self.optfilepath.clone(),
            default_profile_name: self.default_profile_name.clone(),
            seen_files: self.seen_optfiles.clone(),
        }
    }

    pub fn parseopt_file(&self, path: impl Into<PathBuf>) -> ParseoptFile {
        let path = path.into();
        let file_name = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        ParseoptFile {
            dir_name: path.parent().map(|p| p.to_string_lossy().to_string()),
            file_name,
            line: 0,
            path,
        }
    }

    pub fn main(&mut self, args: &[String]) -> i32 {
        let mut status = 0;

        Progname::set_program_name(args.first().map(String::as_str));

        if env::var_os("POSIXLY_CORRECT").is_some() {
            if !self.output.driver("posix") {
                Error::report(
                    Some(0),
                    Some(0),
                    format_args!("INTERNAL ERROR: posix: No such output driver"),
                );
                process::abort();
            }
            self.output_driver_name = Some("posix".to_string());
        }

        self.optfilepath = DEFAULT_OPTPATH.to_string();
        self.parseopt_from_env();
        self.parse_rc();

        self.input_files = args.iter().skip(1).cloned().collect();

        if self.input_files.is_empty() {
            Error::report(Some(64), Some(0), format_args!("no input files"));
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

        if !self.no_main_option && self.starter_symbols.is_empty() {
            Symbol::set_default_starter();
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

        self.lexer.init_lex((self.debug > 2) as i32);
        self.parser.init_parse();

        for file in &self.input_files {
            if self.lexer.source(file) == 0 {
                let _ = self.parser.yyparse();
            } else {
                status = 70;
            }
        }

        if self.lexer.input_file_count() == 0 {
            Error::report(Some(64), Some(0), format_args!("no input files"));
            return 64;
        }

        let mut sink = io::stdout();
        let _ = self.output.output(&mut sink);
        status
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut app = Main::new();
    process::exit(app.main(&args));
}
