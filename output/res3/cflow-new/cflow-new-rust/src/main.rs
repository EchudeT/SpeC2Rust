mod c;
mod dot;
mod error;
mod getprogname;
mod gnu;
mod output;
mod parser;
mod posix;
mod symbol;
mod wordwrap;
mod wordsplit;
use crate::c::C;
use crate::error::Error;
use crate::getprogname::Getprogname;
use crate::output::Output;
use crate::parser::Parser;
use crate::symbol::Symbol;
use crate::wordwrap::Wordwrap;
use crate::wordsplit::Wordsplit;
use std::collections::HashSet;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process;

const SM_FUNCTIONS: i32 = 0x01;
const SM_DATA: i32 = 0x02;
const SM_UNDERSCORE: i32 = 0x04;
const SM_STATIC: i32 = 0x08;
const SM_TYPEDEF: i32 = 0x10;
const SM_UNDEFINED: i32 = 0x20;

const LEVEL_BEGIN: i32 = 1;
const LEVEL_INDENT0: i32 = 2;
const LEVEL_INDENT1: i32 = 3;
const LEVEL_END0: i32 = 4;
const LEVEL_END1: i32 = 5;

const PRINT_TREE: i32 = 1;
const PRINT_XREF: i32 = 2;

const MAX_LEVEL_INDENT: usize = 512;
const LOCAL_RC: &str = "~/.cflowrc";
const DEFAULT_PROFILE_NAME: &str = "default";
const DEFAULT_OPTPATH: &str = "~/.cflow";
const OPTFILE_SUFFIX: &str = ".rc";
const CFLOW_PREPROC: &str = "cpp";
const PACKAGE_NAME: &str = "cflow-new";
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
const COPYRIGHT_YEAR: i32 = 2026;
const GPLV3: &str = "License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n";

pub struct Main {
    symbol_map: i32,
    print_option: i32,
    print_as_tree: bool,
    no_main_option: bool,
    preprocess_option: bool,
    output_visible: bool,
    debug: i32,
    token_stack_length: usize,
    input_file_count: usize,
    level_begin: String,
    level_indent: [Option<String>; 2],
    level_end: [Option<String>; 2],
    optfilepath: String,
    default_profile_name: Option<String>,
    registered_optfiles: HashSet<String>,
    main_symbols: Vec<String>,
    target_symbols: Vec<String>,
    profile_name: Option<String>,
    output_driver: String,
    preprocessor: Option<String>,
    preproc_options: Vec<(char, String)>,
    symbol_overrides: Vec<String>,
    parser: Parser,
    lexer: C,
    output: Output,
}

#[derive(Clone, Copy)]
pub struct OptionType {
    pub name: &'static str,
    pub min_match: usize,
    pub kind: i32,
}

#[derive(Clone)]
pub struct ParseoptFile {
    pub dir_name: Option<String>,
    pub file_name: String,
    pub line: usize,
    pub file: Option<PathBuf>,
}

impl Main {
    pub fn new() -> Self {
        Self {
            symbol_map: SM_FUNCTIONS | SM_STATIC | SM_UNDEFINED,
            print_option: 0,
            print_as_tree: false,
            no_main_option: false,
            preprocess_option: false,
            output_visible: true,
            debug: 0,
            token_stack_length: 1,
            input_file_count: 0,
            level_begin: String::new(),
            level_indent: [None, None],
            level_end: [None, None],
            optfilepath: DEFAULT_OPTPATH.to_string(),
            default_profile_name: Some(DEFAULT_PROFILE_NAME.to_string()),
            registered_optfiles: HashSet::new(),
            main_symbols: Vec::new(),
            target_symbols: Vec::new(),
            profile_name: None,
            output_driver: "gnu".to_string(),
            preprocessor: None,
            preproc_options: Vec::new(),
            symbol_overrides: Vec::new(),
            parser: Parser::default(),
            lexer: C::default(),
            output: Output::new(),
        }
    }

    pub fn option_type(name: &'static str, min_match: usize, kind: i32) -> OptionType {
        OptionType {
            name,
            min_match,
            kind,
        }
    }

    pub fn module_src_parseopt_03() -> Vec<OptionType> {
        vec![
            Self::option_type("begin", 1, LEVEL_BEGIN),
            Self::option_type("0", 1, LEVEL_INDENT0),
            Self::option_type("1", 1, LEVEL_INDENT1),
            Self::option_type("end0", 4, LEVEL_END0),
            Self::option_type("end1", 4, LEVEL_END1),
        ]
    }

    pub fn module_src_parseopt_04() -> Vec<OptionType> {
        vec![
            Self::option_type("x", 1, SM_DATA),
            Self::option_type("_", 1, SM_UNDERSCORE),
            Self::option_type("s", 1, SM_STATIC),
            Self::option_type("t", 1, SM_TYPEDEF),
            Self::option_type("u", 1, SM_UNDEFINED),
        ]
    }

    pub fn char_to_sm(c: char) -> i32 {
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
        let len = if len == 0 { text.len() } else { len };
        for opt in options {
            if len >= opt.min_match
                && len <= opt.name.len()
                && text.get(..len).is_some()
                && &opt.name.as_bytes()[..len] == &text.as_bytes()[..len]
            {
                return opt.kind;
            }
        }
        0
    }

    pub fn parse_number(input: &str, base: u32, count: usize) -> (i32, usize) {
        let mut n = 0i32;
        let mut consumed = 0usize;

        for ch in input.chars().take(count) {
            let value = if ch.is_ascii_digit() {
                ch as u32 - '0' as u32
            } else {
                ch.to_ascii_uppercase() as u32 - 'A' as u32 + 10
            };
            if value > base {
                break;
            }
            n = n.saturating_mul(base as i32).saturating_add(value as i32);
            consumed += ch.len_utf8();
        }

        let ret_index = consumed.saturating_sub(1);
        (n, ret_index)
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
                    match chars[i] {
                        'a' => out.push('\u{0007}'),
                        'b' => out.push('\u{0008}'),
                        'e' => out.push('\u{001b}'),
                        'f' => out.push('\u{000c}'),
                        'n' => out.push('\n'),
                        'r' => out.push('\r'),
                        't' => out.push('\t'),
                        'x' | 'X' => {
                            i += 1;
                            let rest: String = chars[i..].iter().collect();
                            let (n, adv) = Self::parse_number(&rest, 16, 2);
                            out.push(char::from_u32(n as u32).unwrap_or('\0'));
                            i += adv;
                        }
                        '0' => {
                            i += 1;
                            let rest: String = chars[i..].iter().collect();
                            let (n, adv) = Self::parse_number(&rest, 8, 3);
                            out.push(char::from_u32(n as u32).unwrap_or('\0'));
                            i += adv;
                        }
                        c => out.push(c),
                    }
                    i += 1;
                }
                'x' => {
                    if out.is_empty() {
                        out.push('x');
                        i += 1;
                        continue;
                    }
                    let start = i + 1;
                    let mut end = start;
                    while end < chars.len() && chars[end].is_ascii_digit() {
                        end += 1;
                    }
                    let num = chars[start..end]
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap_or(0);
                    let prev = out.chars().last().unwrap_or(' ');
                    for _ in 1..num {
                        if out.len() >= MAX_LEVEL_INDENT - 1 {
                            Error::report(Some(64), Some(0), "level indent string is too long");
                        }
                        out.push(prev);
                    }
                    i = end;
                }
                c => {
                    if out.len() >= MAX_LEVEL_INDENT - 1 {
                        Error::report(Some(64), Some(0), "level indent string is too long");
                    }
                    out.push(c);
                    i += 1;
                }
            }
        }

        out
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
            Error::report(Some(64), Some(0), "level-indent syntax");
            return;
        };

        let key = &text[..eq];
        let value = &text[eq + 1..];
        match Self::find_option_type(&Self::module_src_parseopt_03(), key, key.len()) {
            LEVEL_BEGIN => self.level_begin = Self::parse_level_string(value),
            LEVEL_INDENT0 => self.level_indent[0] = Some(Self::parse_level_string(value)),
            LEVEL_INDENT1 => self.level_indent[1] = Some(Self::parse_level_string(value)),
            LEVEL_END0 => self.level_end[0] = Some(Self::parse_level_string(value)),
            LEVEL_END1 => self.level_end[1] = Some(Self::parse_level_string(value)),
            _ => Error::report(
                Some(64),
                Some(0),
                format!("unknown level indent option: {text}"),
            ),
        }
    }

    pub fn symbol_override(&mut self, text: &str) {
        let Some(ptr) = text.find(':') else {
            Error::report(Some(64), Some(0), format!("{text}: no symbol type supplied"));
            return;
        };

        let name = text[..ptr].to_string();
        let rhs = &text[ptr + 1..];

        if let Some(alias) = rhs.strip_prefix('=') {
            if name == alias {
                Error::report(Some(64), Some(0), format!("cyclic alias: {name} -> {alias}"));
            }
            self.symbol_overrides.push(format!("{name}:={alias}"));
        } else {
            let ty = Self::find_option_type(&Self::module_src_parseopt_04(), rhs, 0);
            if ty == 0 {
                Error::report(Some(64), Some(0), format!("unknown symbol type: {rhs}"));
            }
            self.symbol_overrides.push(format!("{name}:{rhs}"));
        }
    }

    pub fn optset_include_classes(&mut self, arg: &str) {
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
                    process::exit(64);
                }
            }
        }
    }

    pub fn optset_output_driver(&mut self, arg: &str) {
        match arg {
            "gnu" | "posix" | "dot" => {
                self.output_driver = arg.to_string();
            }
            _ => {
                eprintln!("{arg}: No such output driver");
                process::exit(64);
            }
        }
    }

    pub fn optset_xref(&mut self) {
        self.print_option = PRINT_XREF;
        self.symbol_map &= !SM_STATIC;
    }

    pub fn optset_symbol(&mut self, arg: &str) {
        self.symbol_override(arg);
    }

    pub fn optset_preproc_option(&mut self, opt_name: &str, arg: &str) {
        let key = opt_name.chars().next().unwrap_or_default();
        self.preproc_options.push((key, arg.to_string()));
        self.preprocess_option = true;
    }

    pub fn optset_preprocess(&mut self, arg: Option<&str>) {
        self.preprocess_option = true;
        self.preprocessor = Some(arg.unwrap_or(CFLOW_PREPROC).to_string());
    }

    pub fn optset_level_indent(&mut self, arg: &str) {
        self.set_level_indent(arg);
    }

    pub fn symbol(&mut self, arg: Option<&str>, clear: bool) {
        if clear {
            self.main_symbols.clear();
            self.no_main_option = true;
        } else if let Some(name) = arg {
            self.main_symbols.push(name.to_string());
        }
    }

    pub fn optset_install_target(&mut self, arg: &str) {
        self.target_symbols.push(arg.to_string());
    }

    pub fn optset_int_1(target: &mut i32, arg: Option<&str>) {
        if let Some(value) = arg {
            if let Ok(parsed) = value.parse::<i32>() {
                *target = parsed;
            }
        } else {
            *target = 1;
        }
    }

    pub fn optset_prepend_path(&mut self, arg: &str) {
        if self.optfilepath.is_empty() {
            self.optfilepath = arg.to_string();
        } else {
            self.optfilepath = format!("{arg}:{}", self.optfilepath);
        }
    }

    pub fn version_hook<W: Write>(&self, writer: &mut W, program_name: &str) -> io::Result<()> {
        writeln!(writer, "{program_name} ({PACKAGE_NAME}) {PACKAGE_VERSION}")?;
        writeln!(
            writer,
            "Copyright (C) 2005-{COPYRIGHT_YEAR} Sergey Poznyakoff"
        )?;
        write!(writer, "{GPLV3}")?;
        write!(writer, "Written by Sergey Poznyakoff.")?;
        Ok(())
    }

    pub fn help_hook<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writeln!(writer, "References:")?;
        writeln!(
            writer,
            " [1]   https://www.gnu.org/software/cflow/manual/html_section/ASCII-Tree.html"
        )?;
        let mut ww = Wordwrap::open(Vec::<u8>::new());
        ww.para()?;
        let _ = ww.close();
        writeln!(writer, "Profile search path: {}", DEFAULT_OPTPATH)?;
        Ok(())
    }

    pub fn po_env_error(&self, pri_is_error: bool, message: &str) {
        if pri_is_error {
            if let Some(program_name) = Getprogname::program_name() {
                eprint!("{program_name}: ");
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

        let mut ws = Wordsplit::default();
        if ws.wordsplit(&env_text, 0) != 0 {
            Error::report(
                Some(1),
                Some(0),
                format!("failed to parse CFLOW_OPTIONS: {}", ws.strerror()),
            );
            return;
        }
        let words = ws.get_words();
        if self.parseopt_file(&words).is_err() {
            Error::report(Some(70), Some(0), "parseopt_getopt failed");
        }
    }

    pub fn fromfile_error(&self, pf: &ParseoptFile, message: &str) {
        if let Some(program_name) = Getprogname::program_name() {
            eprint!("{program_name}: ");
        }
        if let Some(dir) = &pf.dir_name {
            eprint!("{}/{}:{}: ", dir, pf.file_name, pf.line);
        } else {
            eprint!("{}:{}: ", pf.file_name, pf.line);
        }
        eprintln!("{message}");
    }

    pub fn fromfile(&mut self, pf: &mut ParseoptFile) -> i32 {
        let Some(path) = pf.file.clone() else {
            return 1;
        };
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                self.fromfile_error(pf, &format!("file read error: {e}"));
                return 1;
            }
        };
        let mut res = 0;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            pf.line += 1;
            let Ok(line) = line else {
                self.fromfile_error(pf, "file read error");
                return 1;
            };
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let mut ws = Wordsplit::default();
            if ws.wordsplit(trimmed, 0) != 0 {
                self.fromfile_error(pf, &format!("error splitting line: {}", ws.strerror()));
                res = 1;
                break;
            }
            let words = ws.get_words();
            if self.parseopt_file(&words).is_err() {
                res = 1;
                break;
            }
        }

        res
    }

    pub fn optfile_register(&mut self, key: &str) -> i32 {
        if self.registered_optfiles.contains(key) {
            1
        } else {
            self.registered_optfiles.insert(key.to_string());
            0
        }
    }

    pub fn expand_tilde(path: &str) -> Option<String> {
        if !path.starts_with('~') {
            return Some(path.to_string());
        }

        let slash = path.find('/').unwrap_or(path.len());
        let head = &path[..slash];
        let tail = &path[slash..];

        if head == "~" {
            let home = env::var("HOME").ok()?;
            return Some(format!("{home}{tail}"));
        }

        None
    }

    pub fn optfile_lookup(&mut self, name: &str) -> Result<ParseoptFile, i32> {
        let direct = Path::new(name);
        if direct.exists() {
            let key = direct.to_string_lossy().to_string();
            if self.optfile_register(&key) != 0 {
                return Err(1);
            }
            return Ok(ParseoptFile {
                dir_name: None,
                file_name: name.to_string(),
                line: 0,
                file: Some(direct.to_path_buf()),
            });
        }

        let file_name = format!("{name}{OPTFILE_SUFFIX}");
        for dir in self.optfilepath.split(':') {
            let Some(dirname) = Self::expand_tilde(dir) else {
                continue;
            };
            let candidate = Path::new(&dirname).join(&file_name);
            if candidate.exists() {
                let key = candidate.to_string_lossy().to_string();
                if self.optfile_register(&key) != 0 {
                    return Err(1);
                }
                return Ok(ParseoptFile {
                    dir_name: Some(dirname),
                    file_name,
                    line: 0,
                    file: Some(candidate),
                });
            }
        }

        Err(0)
    }

    pub fn optset_profile(&mut self, arg: &str) {
        let mut pf = match self.optfile_lookup(arg) {
            Ok(pf) => pf,
            Err(1) => return,
            Err(_) => {
                eprintln!("no such profile: {arg}");
                process::exit(64);
            }
        };

        let rc = self.fromfile(&mut pf);
        if rc != 0 {
            process::exit(64);
        }
        self.profile_name = Some(arg.to_string());
    }

    pub fn parseopt_from_rc(&mut self, file_name: &str) -> i32 {
        let path = Path::new(file_name);
        if !path.exists() {
            return 0;
        }
        let mut pf = ParseoptFile {
            dir_name: None,
            file_name: file_name.to_string(),
            line: 0,
            file: Some(path.to_path_buf()),
        };
        self.fromfile(&mut pf)
    }

    pub fn parse_rc(&mut self) -> i32 {
        if let Ok(name) = env::var("CFLOWRC") {
            if name.is_empty() {
                return 0;
            }
            return self.parseopt_from_rc(&name);
        }

        if let Some(name) = Self::expand_tilde(LOCAL_RC) {
            if Path::new(&name).exists() {
                eprintln!("warning: processing obsolete {LOCAL_RC} file");
                let n = self.optfilepath.find(':').unwrap_or(self.optfilepath.len());
                if n > 0 {
                    let mut m = n;
                    if self.optfilepath[..n].ends_with('/') {
                        m -= 1;
                    }
                    eprintln!(
                        "warning: consider renaming it to {}/{}{}",
                        &self.optfilepath[..m],
                        self.default_profile_name
                            .as_deref()
                            .unwrap_or(DEFAULT_PROFILE_NAME),
                        OPTFILE_SUFFIX
                    );
                }
                return self.parseopt_from_rc(&name);
            }
        }
        0
    }

    pub fn init_hook(&mut self) {
        let Some(default_profile_name) = self.default_profile_name.clone() else {
            return;
        };
        let Ok(mut pf) = self.optfile_lookup(&default_profile_name) else {
            return;
        };
        pf.line = 0;
        let _ = self.fromfile(&mut pf);
    }

    pub fn globals_only(&self) -> bool {
        (self.symbol_map & SM_STATIC) == 0
    }

    pub fn include_symbol(&self, sym: Option<&Symbol>) -> bool {
        let Some(sym) = sym else {
            return false;
        };
        if sym.name.is_empty() {
            return false;
        }
        if self.globals_only() && sym.name.starts_with('_') {
            return false;
        }

        let mut ty = 0;
        if sym.name.starts_with('_') {
            if (self.symbol_map & SM_UNDERSCORE) == 0 {
                return false;
            }
        }

        ty |= SM_FUNCTIONS;
        (self.symbol_map & ty) == ty
    }

    pub fn xalloc_die() -> ! {
        Error::report(Some(1), Some(12), "Exiting");
        panic!("allocation failure");
    }

    fn init_runtime(&mut self) {
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
    }

    pub fn parseopt_file(&mut self, args: &[String]) -> Result<(), ()> {
        let mut i = 0usize;
        while i < args.len() {
            let arg = &args[i];
            match arg.as_str() {
                "--xref" => self.optset_xref(),
                "--tree" => self.print_as_tree = true,
                "--profile" => {
                    i += 1;
                    let Some(val) = args.get(i) else {
                        return Err(());
                    };
                    self.optset_profile(val);
                }
                "--output" => {
                    i += 1;
                    let Some(val) = args.get(i) else {
                        return Err(());
                    };
                    self.optset_output_driver(val);
                }
                "--symbol" => {
                    i += 1;
                    let Some(val) = args.get(i) else {
                        return Err(());
                    };
                    self.optset_symbol(val);
                }
                "--include" => {
                    i += 1;
                    let Some(val) = args.get(i) else {
                        return Err(());
                    };
                    self.optset_include_classes(val);
                }
                "--level-indent" => {
                    i += 1;
                    let Some(val) = args.get(i) else {
                        return Err(());
                    };
                    self.optset_level_indent(val);
                }
                "--preprocess" => {
                    let next = args.get(i + 1).map(|s| s.as_str());
                    if let Some(v) = next {
                        if !v.starts_with('-') {
                            i += 1;
                            self.optset_preprocess(Some(v));
                        } else {
                            self.optset_preprocess(None);
                        }
                    } else {
                        self.optset_preprocess(None);
                    }
                }
                "--target" => {
                    i += 1;
                    let Some(val) = args.get(i) else {
                        return Err(());
                    };
                    self.optset_install_target(val);
                }
                "--main" => {
                    i += 1;
                    let Some(val) = args.get(i) else {
                        return Err(());
                    };
                    self.symbol(Some(val), false);
                }
                "--no-main" => self.symbol(None, true),
                "--prepend-path" => {
                    i += 1;
                    let Some(val) = args.get(i) else {
                        return Err(());
                    };
                    self.optset_prepend_path(val);
                }
                "--help" => {
                    let mut stdout = io::stdout();
                    let _ = self.help_hook(&mut stdout);
                    process::exit(0);
                }
                "--version" => {
                    let program_name =
                        Getprogname::program_name().unwrap_or_else(|| PACKAGE_NAME.to_string());
                    let mut stdout = io::stdout();
                    let _ = self.version_hook(&mut stdout, &program_name);
                    process::exit(0);
                }
                _ => {}
            }
            i += 1;
        }
        Ok(())
    }

    pub fn main(&mut self, argv: &[String]) -> i32 {
        if let Some(arg0) = argv.first() {
            Getprogname::set_program_name(arg0);
        }


        if env::var_os("POSIXLY_CORRECT").is_some() {
            self.optset_output_driver("posix");
        }

        self.optfilepath = DEFAULT_OPTPATH.to_string();
        self.parseopt_from_env();
        let _ = self.parse_rc();
        if self.parseopt_file(argv).is_err() {
            Error::report(Some(70), Some(0), "parseopt_getopt failed");
        }

        if self.token_stack_length == 0 {
            Error::report(Some(64), Some(0), "argument to -p cannot be 0");
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

        if !self.no_main_option && self.main_symbols.is_empty() {
            self.main_symbols.push("main".to_string());
        }

        if self.print_option == 0 {
            self.print_option = PRINT_TREE;
        }

        self.init_runtime();

        let mut status = 0;
        for arg in argv.iter().skip(1) {
            if arg.starts_with('-') {
                continue;
            }
            self.input_file_count += 1;
            if fs::read_to_string(arg).is_err() {
                status = 1;
            }
        }

        if self.input_file_count == 0 {
            Error::report(Some(64), Some(0), "no input files");
        }

        let _ = self.output.begin(&mut io::stdout());
        let _ = self.output.end(&mut io::stdout());
        status
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut app = Main::new();
    process::exit(app.main(&args));
}
