use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

use crate::error::Error;
use crate::output::{SimpleReference, SimpleStorageClass, SymbolRecord};

const WORD: i32 = 256;
const LBRACE0: i32 = 257;
const RBRACE0: i32 = 258;
const IDENTIFIER: i32 = 259;
const EXTERN: i32 = 260;
const STATIC: i32 = 261;
const TYPEDEF: i32 = 262;
const STRUCT: i32 = 263;
const MODIFIER: i32 = 264;
const OP: i32 = 265;
const UNION: i32 = 266;
const ENUM: i32 = 267;
const LBRACE: i32 = 268;
const RBRACE: i32 = 269;
const MEMBER_OF: i32 = 270;
const TYPE: i32 = 271;
const STRING: i32 = 272;
const PARM_WRAPPER: i32 = 273;
const QUALIFIER: i32 = 274;
const DECLARATION: i32 = 275;

pub fn find_closing_paren(s: &str, start: usize, len: usize, parens: &str) -> Option<usize> {
    let p = parens.as_bytes();
    if p.len() != 2 || start >= len || len > s.len() {
        return None;
    }
    let open = p[0];
    let close = p[1];
    let bytes = s.as_bytes();
    let mut depth = 1i32;
    let mut i = start + 1;
    while i < len {
        match bytes[i] {
            b'\\' => {
                i += 1;
            }
            b'\'' | b'"' => {
                let quote = bytes[i];
                i += 1;
                while i < len {
                    if bytes[i] == b'\\' {
                        i += 2;
                        continue;
                    }
                    if bytes[i] == quote {
                        break;
                    }
                    i += 1;
                }
            }
            c if c == open => depth += 1,
            c if c == close => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenEntry {
    pub token_type: i32,
    pub line: usize,
    pub token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct module_src_save_stack_14 {
    pub start_pos: usize,
    pub save_end: isize,
    pub need_space: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct module_src_balance_state_08 {
    pub idx: i32,
    pub level: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct balance_state {
    pub idx: i32,
    pub level: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ident {
    pub name: Option<String>,
    pub type_end: isize,
    pub parmcnt: i32,
    pub line: isize,
    pub storage: Storage,
}

impl Default for Ident {
    fn default() -> Self {
        Self {
            name: None,
            type_end: -1,
            parmcnt: -1,
            line: -1,
            storage: Storage::Any,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Storage {
    Any,
    Auto,
    Static,
    Extern,
    ExplicitExtern,
}

impl Default for Storage {
    fn default() -> Self {
        Self::Any
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SymbolFlag {
    None,
    Start,
    Target,
    Parm,
    Local,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SymbolType {
    Undefined,
    Identifier,
    Token,
}

#[derive(Clone, Debug)]
struct SymbolData {
    name: String,
    storage: Storage,
    flag: SymbolFlag,
    symbol_type: SymbolType,
    token_type: Option<i32>,
    source: Option<String>,
    def_line: usize,
    decl: Option<String>,
    level: usize,
    arity: i32,
    references: Vec<SimpleReference>,
    callers: HashSet<String>,
    callees: HashSet<String>,
}

impl SymbolData {
    fn new(name: &str, storage: Storage) -> Self {
        Self {
            name: name.to_string(),
            storage,
            flag: SymbolFlag::None,
            symbol_type: SymbolType::Identifier,
            token_type: None,
            source: None,
            def_line: 0,
            decl: None,
            level: 0,
            arity: -1,
            references: Vec::new(),
            callers: HashSet::new(),
            callees: HashSet::new(),
        }
    }
}

pub struct Parser {
    pub debug: i32,
    pub verbose: bool,
    pub strict_ansi: bool,
    pub omit_arguments_option: bool,
    pub omit_symbol_names_option: bool,
    pub use_indentation: bool,
    pub level: usize,
    pub parm_level: usize,
    pub line_num: usize,
    pub filename: String,
    pub token_stack_length: usize,
    pub token_stack_increase: usize,
    pub token_stack: Vec<TokenEntry>,
    pub curs: usize,
    pub tos: usize,
    pub tok: TokenEntry,
    pub save: module_src_save_stack_14,
    pub symbols: HashMap<String, SymbolData>,
    pub typedef_names: HashSet<String>,
    pub input_tokens: Vec<TokenEntry>,
    pub input_index: usize,
    pub caller: Option<String>,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            debug: 0,
            verbose: false,
            strict_ansi: false,
            omit_arguments_option: false,
            omit_symbol_names_option: false,
            use_indentation: false,
            level: 0,
            parm_level: 0,
            line_num: 1,
            filename: "<input>".to_string(),
            token_stack_length: 128,
            token_stack_increase: 64,
            token_stack: Vec::with_capacity(128),
            curs: 0,
            tos: 0,
            tok: TokenEntry {
                token_type: 0,
                line: 0,
                token: None,
            },
            save: module_src_save_stack_14::default(),
            symbols: HashMap::new(),
            typedef_names: HashSet::new(),
            input_tokens: Vec::new(),
            input_index: 0,
            caller: None,
        }
    }
}

impl Parser {
    fn current_stack_slice(&self) -> &[TokenEntry] {
        let end = self.tos.min(self.token_stack.len());
        let start = self.curs.min(end);
        &self.token_stack[start..end]
    }

    fn ensure_stack_room(&mut self) {
        if self.token_stack.len() < self.tos {
            self.token_stack.resize(
                self.tos,
                TokenEntry {
                    token_type: 0,
                    line: 0,
                    token: None,
                },
            );
        }
    }

    fn get_token(&mut self) -> i32 {
        if let Some(tok) = self.input_tokens.get(self.input_index).cloned() {
            self.input_index += 1;
            self.line_num = tok.line;
            self.tok = tok.clone();
            tok.token_type
        } else {
            self.tok = TokenEntry {
                token_type: 0,
                line: self.line_num,
                token: None,
            };
            0
        }
    }

    fn install_ident(&mut self, name: &str, storage: Storage) -> &mut SymbolData {
        self.symbols
            .entry(name.to_string())
            .or_insert_with(|| SymbolData::new(name, storage))
    }

    fn lookup_name(&self, name: &str) -> Option<&SymbolData> {
        self.symbols.get(name)
    }

    fn globals_only(&self) -> bool {
        false
    }

    fn delete_parms(&mut self, _level: usize) {}

    fn move_parms(&mut self, _level: usize) {}

    fn delete_autos(&mut self, _level: usize) {}

    fn ident_change_storage(sp: &mut SymbolData, storage: Storage) {
        sp.storage = storage;
    }

    pub fn print_token(&self, tokptr: &TokenEntry) -> String {
        match tokptr.token_type {
            IDENTIFIER | TYPE | WORD | MODIFIER | STRUCT | PARM_WRAPPER | QUALIFIER | OP => {
                format!("`{}'", tokptr.token.as_deref().unwrap_or_default())
            }
            LBRACE0 | LBRACE => "`{'".to_string(),
            RBRACE0 | RBRACE => "`}'".to_string(),
            EXTERN => "`extern'".to_string(),
            STATIC => "`static'".to_string(),
            TYPEDEF => "`typedef'".to_string(),
            STRING => format!("\"{}\"", tokptr.token.as_deref().unwrap_or_default()),
            other => {
                let ch = char::from_u32(other as u32).unwrap_or('?');
                format!("`{}'", ch)
            }
        }
    }

    pub fn token_type_str(&self, t: i32) -> String {
        match t {
            0 => "EOF".to_string(),
            WORD => "WORD".to_string(),
            LBRACE0 => "'{'".to_string(),
            RBRACE0 => "'}'".to_string(),
            IDENTIFIER => "IDENTIFIER".to_string(),
            EXTERN => "EXTERN".to_string(),
            STATIC => "STATIC".to_string(),
            TYPEDEF => "TYPEDEF".to_string(),
            STRUCT => "STRUCT".to_string(),
            MODIFIER => "MODIFIER".to_string(),
            OP => "OP".to_string(),
            UNION => "UNION".to_string(),
            ENUM => "ENUM".to_string(),
            LBRACE => "' {'".to_string(),
            RBRACE => "' }'".to_string(),
            MEMBER_OF => "MEMBER_OF".to_string(),
            TYPE => "TYPE".to_string(),
            STRING => "STRING".to_string(),
            PARM_WRAPPER => "PARM_WRAPPER".to_string(),
            QUALIFIER => "QUALIFIER".to_string(),
            _ => {
                if (t as u32) < 128 && (t as u8).is_ascii_graphic() {
                    format!("'{}'({})", t as u8 as char, t)
                } else {
                    t.to_string()
                }
            }
        }
    }

    pub fn dbgtok(&self, t: &TokenEntry, delim: Option<char>) -> String {
        let mut out = String::new();
        if let Some(c) = delim {
            out.push(c);
        }
        out.push_str("{ ");
        out.push_str(&self.token_type_str(t.token_type));
        out.push(' ');
        if t.token_type != 0 {
            out.push_str(&format!(
                ", {}, {} ",
                t.token.as_deref().unwrap_or("NULL"),
                t.line
            ));
        }
        out.push('}');
        out
    }

    pub fn debugtoken(&self, t: Option<&TokenEntry>, fmt: Option<&str>) {
        if self.debug > 1 {
            let mut stderr = io::stderr().lock();
            if let Some(msg) = fmt {
                let _ = write!(stderr, "{}: ", msg);
            }
            if let Some(tok) = t {
                let _ = write!(stderr, "{}; ", self.dbgtok(tok, None));
            }
            let _ = write!(stderr, "{}: {{", self.curs);
            for (i, tok) in self.current_stack_slice().iter().enumerate() {
                let delim = if i == 0 { None } else { Some(',') };
                let _ = write!(stderr, "{}", self.dbgtok(tok, delim));
            }
            let _ = writeln!(stderr, "}}");
        }
    }

    pub fn file_error(&self, msg: &str, tokptr: Option<&TokenEntry>) {
        let mut stderr = io::stderr().lock();
        let _ = write!(stderr, "{}:{}: {}", self.filename, self.tok.line, msg);
        if let Some(tok) = tokptr {
            let _ = write!(stderr, " near {}", self.print_token(tok));
        }
        let _ = writeln!(stderr);
    }

    pub fn mark(&self) -> usize {
        if self.debug > 1 {
            let _ = writeln!(io::stderr().lock(), "marking stack at {}", self.curs);
        }
        self.curs
    }


    pub fn tokdel(&mut self, beg: usize, end: usize) {
        if end >= beg && beg < self.tos {
            let real_end = end.min(self.tos.saturating_sub(1));
            self.token_stack.drain(beg..=real_end);
            self.tos -= real_end - beg + 1;
            if self.curs > self.tos {
                self.curs = self.tos;
            }
        }
    }

    pub fn tokins(&mut self, pos: usize, token_type: i32, line: usize, token: impl Into<String>) {
        let pos = pos.min(self.tos);
        let entry = TokenEntry {
            token_type,
            line,
            token: Some(token.into()),
        };
        if self.tos == self.token_stack.len() {
            self.token_stack.reserve(self.token_stack_increase);
        }
        self.token_stack.insert(pos, entry);
        self.tos += 1;
        let tok = self.token_stack[pos].clone();
        self.debugtoken(Some(&tok), Some(&format!("insert at {}", pos)));
    }

    pub fn tokpush(&mut self, token_type: i32, line: usize, token: Option<String>) {
        let entry = TokenEntry {
            token_type,
            line,
            token,
        };
        if self.tos < self.token_stack.len() {
            self.token_stack[self.tos] = entry;
        } else {
            self.token_stack.push(entry);
        }
        self.tos += 1;
        if self.tos == self.token_stack_length {
            self.token_stack_length += self.token_stack_increase;
            self.token_stack.reserve(self.token_stack_increase);
        }
    }

    pub fn cleanup_stack(&mut self) {
        let delta = self.tos as isize - self.curs as isize;
        let keep = if delta > 0 { delta as usize } else { 0 };
        if self.curs < self.tos {
            let remaining: Vec<_> = self.token_stack[self.curs..self.tos].to_vec();
            for (i, tok) in remaining.into_iter().enumerate() {
                if i < self.token_stack.len() {
                    self.token_stack[i] = tok;
                } else {
                    self.token_stack.push(tok);
                }
            }
        }
        self.tos = keep;
        self.curs = 0;
    }

    pub fn clearstack(&mut self) {
        self.tos = 0;
        self.curs = 0;
    }

    pub fn nexttoken(&mut self) -> i32 {
        if self.curs == self.tos {
            let token_type = self.get_token();
            let token = self.tok.token.clone();
            self.tokpush(token_type, self.line_num, token);
        }
        self.tok = self.token_stack[self.curs].clone();
        self.curs += 1;
        let tok = self.tok.clone();
        self.debugtoken(Some(&tok), Some("next token"));
        self.tok.token_type
    }

    pub fn putback(&mut self) -> i32 {
        if self.curs == 0 {
            Error::report(None, None, "INTERNAL ERROR: cannot return token to stream");
            panic!("cannot return token to stream");
        }
        self.curs -= 1;
        if self.curs > 0 {
            self.tok = self.token_stack[self.curs - 1].clone();
        } else {
            self.tok.token_type = 0;
            self.tok.token = None;
        }
        let tok = self.tok.clone();
        self.debugtoken(Some(&tok), Some("putback"));
        self.tok.token_type
    }

    pub fn init_parse(&mut self) {
        self.token_stack = Vec::with_capacity(self.token_stack_length);
        self.clearstack();
    }

    pub fn save_token(&mut self, tokptr: &TokenEntry) {
        let piece = match tokptr.token_type {
            IDENTIFIER | TYPE | STRUCT | PARM_WRAPPER | WORD | QUALIFIER => {
                let mut s = String::new();
                if self.save.need_space {
                    s.push(' ');
                }
                s.push_str(tokptr.token.as_deref().unwrap_or_default());
                self.save.need_space = true;
                s
            }
            MODIFIER => {
                let mut s = String::new();
                if self.save.need_space {
                    s.push(' ');
                }
                let text = tokptr.token.as_deref().unwrap_or_default();
                self.save.need_space = !text.starts_with('*');
                s.push_str(text);
                s
            }
            EXTERN | STATIC => String::new(),
            44 => {
                self.save.need_space = true;
                ",".to_string()
            }
            40 => {
                let mut s = String::new();
                if self.save.need_space {
                    s.push(' ');
                }
                s.push('(');
                self.save.need_space = false;
                s
            }
            41 => {
                self.save.need_space = true;
                ")".to_string()
            }
            91 | 93 => {
                self.save.need_space = false;
                char::from_u32(tokptr.token_type as u32)
                    .unwrap_or_default()
                    .to_string()
            }
            LBRACE | LBRACE0 => {
                let mut s = String::new();
                if self.save.need_space {
                    s.push(' ');
                }
                s.push('{');
                self.save.need_space = true;
                s
            }
            RBRACE | RBRACE0 => {
                let mut s = String::new();
                if self.save.need_space {
                    s.push(' ');
                }
                s.push('}');
                self.save.need_space = true;
                s
            }
            OP => {
                self.save.need_space = true;
                format!(" {}", tokptr.token.as_deref().unwrap_or_default())
            }
            _ => {
                if self.verbose {
                    self.file_error("unrecognized definition", Some(tokptr));
                }
                String::new()
            }
        };
        if !piece.is_empty() {
            self.module_text_push(&piece);
        }
    }

    fn module_text_push(&mut self, _s: &str) {}

    pub fn save_stack(&mut self) {
        self.save.start_pos = self.mark();
        self.save.save_end = self.curs.saturating_sub(1) as isize;
    }

    pub fn undo_save_stack(&mut self) {
        self.save.save_end = -1;
    }

    pub fn save_stack_is_empty(&self) -> bool {
        self.save.save_end <= 0
    }

    pub fn finish_save_stack(&mut self, name: &str) -> String {
        let mut out = String::new();
        let mut level = 0;
        let mut found_ident = !self.omit_symbol_names_option;
        self.save.need_space = false;
        let end = self.save.save_end.max(0) as usize;
        for i in 0..end.min(self.token_stack.len()) {
            let tok = self.token_stack[i].clone();
            match tok.token_type {
                40 => {
                    if self.omit_arguments_option {
                        if level == 0 {
                            out.push_str(&self.token_to_saved_text(&tok));
                        }
                        level += 1;
                    }
                    if !self.omit_arguments_option && level == 0 {
                        out.push_str(&self.token_to_saved_text(&tok));
                    }
                    continue;
                }
                41 => {
                    if self.omit_arguments_option {
                        level -= 1;
                    } else if level == 0 {
                        out.push_str(&self.token_to_saved_text(&tok));
                    }
                    continue;
                }
                IDENTIFIER => {
                    if !found_ident && tok.token.as_deref() == Some(name) {
                        self.save.need_space = true;
                        found_ident = true;
                        continue;
                    }
                }
                _ => {}
            }
            if level == 0 {
                out.push_str(&self.token_to_saved_text(&tok));
            }
        }
        out
    }

    fn token_to_saved_text(&mut self, tokptr: &TokenEntry) -> String {
        let old = self.save.need_space;
        let before = self.save.need_space;
        let mut acc = String::new();
        match tokptr.token_type {
            IDENTIFIER | TYPE | STRUCT | PARM_WRAPPER | WORD | QUALIFIER => {
                if before {
                    acc.push(' ');
                }
                acc.push_str(tokptr.token.as_deref().unwrap_or_default());
                self.save.need_space = true;
            }
            MODIFIER => {
                if before {
                    acc.push(' ');
                }
                let text = tokptr.token.as_deref().unwrap_or_default();
                self.save.need_space = !text.starts_with('*');
                acc.push_str(text);
            }
            EXTERN | STATIC => {}
            44 => {
                acc.push(',');
                self.save.need_space = true;
            }
            40 => {
                if before {
                    acc.push(' ');
                }
                acc.push('(');
                self.save.need_space = false;
            }
            41 => {
                acc.push(')');
                self.save.need_space = true;
            }
            91 => {
                acc.push('[');
                self.save.need_space = false;
            }
            93 => {
                acc.push(']');
                self.save.need_space = false;
            }
            LBRACE | LBRACE0 => {
                if before {
                    acc.push(' ');
                }
                acc.push('{');
                self.save.need_space = true;
            }
            RBRACE | RBRACE0 => {
                if before {
                    acc.push(' ');
                }
                acc.push('}');
                self.save.need_space = true;
            }
            OP => {
                acc.push(' ');
                acc.push_str(tokptr.token.as_deref().unwrap_or_default());
                self.save.need_space = true;
            }
            _ => {}
        }
        if acc.is_empty() {
            self.save.need_space = old;
        }
        acc
    }

    pub fn skip_to(&mut self, c: i32) {
        while self.nexttoken() != 0 {
            if self.tok.token_type == c {
                break;
            }
        }
    }

    pub fn push_balance_state(
        &self,
        stack: &mut Vec<balance_state>,
        idx: i32,
        level: i32,
    ) {
        stack.push(balance_state { idx, level });
    }

    pub fn pop_balance_state(
        &self,
        stack: &mut Vec<balance_state>,
        idx: &mut i32,
        level: &mut i32,
    ) {
        if let Some(state) = stack.pop() {
            *idx = state.idx;
            *level = state.level;
        }
    }

    pub fn free_balance_stack(&self, stack: &mut Vec<balance_state>) {
        stack.clear();
    }

    pub fn find_closing_paren(&mut self, open_tok: i32, mut level: i32) -> i32 {
        let oparen = ['(', '[', '{'];
        let cparen = [')', ']', '}'];
        let mut stack = Vec::new();
        let mut idx = if let Some(pos) = oparen.iter().position(|c| *c as i32 == open_tok) {
            pos as i32
        } else {
            Error::report(
                Some(70),
                Some(0),
                "INTERNAL ERROR: malformed call to find_closing_paren",
            );
            panic!("malformed call to find_closing_paren");
        };

        if level == 0 {
            if self.nexttoken() != open_tok {
                return 1;
            }
            level += 1;
        }
        while self.nexttoken() != 0 {
            if self.tok.token_type == LBRACE0 {
                self.tok.token_type = '{' as i32;
            } else if self.tok.token_type == RBRACE0 {
                self.tok.token_type = '}' as i32;
            }

            if self.tok.token_type == oparen[idx as usize] as i32 {
                level += 1;
            } else if self.tok.token_type == cparen[idx as usize] as i32 {
                level -= 1;
                if level == 0 {
                    if stack.is_empty() {
                        return 0;
                    }
                    self.pop_balance_state(&mut stack, &mut idx, &mut level);
                }
            } else if let Some(new_idx) = oparen
                .iter()
                .position(|c| *c as i32 == self.tok.token_type)
            {
                self.push_balance_state(&mut stack, idx, level);
                idx = new_idx as i32;
                level = 1;
            }
        }
        self.free_balance_stack(&mut stack);
        -1
    }

    pub fn skip_balanced(&mut self, open_tok: i32, level: i32) -> i32 {
        let rc = self.find_closing_paren(open_tok, level);
        if rc == 0 {
            self.nexttoken();
        }
        rc
    }

    pub fn yyparse(&mut self) -> i32 {
        let mut identifier = Ident::default();
        self.level = 0;
        self.caller = None;
        self.clearstack();
        while self.nexttoken() != 0 {
            identifier.storage = Storage::Extern;
            match self.tok.token_type {
                0 => return 0,
                QUALIFIER => continue,
                TYPEDEF => self.parse_typedef(),
                DECLARATION => self.skip_declaration(),
                EXTERN => {
                    identifier.storage = Storage::ExplicitExtern;
                    self.nexttoken();
                    self.parse_declaration(&mut identifier, false);
                }
                STATIC => {
                    identifier.storage = Storage::Static;
                    self.nexttoken();
                    self.parse_declaration(&mut identifier, false);
                }
                _ => self.parse_declaration(&mut identifier, false),
            }
            self.cleanup_stack();
        }
        0
    }

    pub fn is_function(&mut self) -> bool {
        let sp = self.mark();
        let mut res = false;
        loop {
            match self.tok.token_type {
                QUALIFIER | TYPE | IDENTIFIER | MODIFIER | STATIC | EXTERN | STRUCT | UNION
                | ENUM => {
                    self.nexttoken();
                    continue;
                }
                PARM_WRAPPER => {
                    if self.skip_balanced('(' as i32, 0) == -1 {
                        self.file_error("unexpected end of file in declaration", None);
                    }
                    continue;
                }
                40 => {
                    res = self.nexttoken() != MODIFIER;
                    break;
                }
                _ => break,
            }
        }
        self.curs = sp;
        res
    }

    pub fn parse_declaration(&mut self, ident: &mut Ident, parm: bool) {
        if self.is_function() {
            self.parse_function_declaration(ident, parm);
        } else {
            self.parse_variable_declaration(ident, parm);
        }
        self.delete_parms(self.parm_level);
    }

    pub fn skip_declaration(&mut self) {
        if self.skip_balanced('(' as i32, 0) == -1 {
            self.file_error("unexpected end of file in declaration", None);
        }
        if self.tok.token_type != ';' as i32 {
            self.putback();
        }
    }

    pub fn expression(&mut self) {
        let mut parens_lev = 0;
        loop {
            match self.tok.token_type {
                59 => return,
                PARM_WRAPPER => {
                    if self.skip_balanced('(' as i32, 0) == -1 {
                        self.file_error("unexpected end of file in expression", None);
                        return;
                    }
                    self.putback();
                }
                LBRACE | LBRACE0 | RBRACE | RBRACE0 => {
                    self.putback();
                    return;
                }
                44 => {
                    if parens_lev == 0 {
                        return;
                    }
                }
                0 => {
                    if self.verbose {
                        self.file_error("unexpected end of file in expression", None);
                    }
                    return;
                }
                IDENTIFIER => {
                    let name = self.tok.token.clone().unwrap_or_default();
                    let line = self.tok.line;
                    self.nexttoken();
                    if self.tok.token_type == '(' as i32 {
                        self.call(&name, line);
                        parens_lev += 1;
                    } else {
                        self.reference(&name, line);
                        while parens_lev > 0 && self.tok.token_type == ')' as i32 {
                            parens_lev -= 1;
                            self.nexttoken();
                        }
                        if self.tok.token_type == MEMBER_OF {
                            self.nexttoken();
                        } else {
                            self.putback();
                        }
                    }
                }
                40 => {
                    if {
                        let t = self.nexttoken();
                        t == TYPE || self.tok.token_type == STRUCT
                    } {
                        if self.skip_balanced('(' as i32, 1) == -1 {
                            self.file_error("unexpected end of file in expression", None);
                            return;
                        }
                        if self.tok.token_type == LBRACE || self.tok.token_type == LBRACE0 {
                            if self.skip_balanced('{' as i32, 1) == -1 {
                                self.file_error("unexpected end of file in expression", None);
                                return;
                            }
                        }
                    } else {
                        parens_lev += 1;
                    }
                    self.putback();
                }
                41 => parens_lev -= 1,
                MEMBER_OF => {
                    self.nexttoken();
                }
                _ => {}
            }
            self.nexttoken();
        }
    }

    pub fn parse_function_declaration(&mut self, ident: &mut Ident, parm: bool) {
        let mut error_recovery = false;
        ident.type_end = -1;
        self.parse_knr_dcl(ident);

        loop {
            match self.tok.token_type {
                41 if parm => break,
                59 | 44 => break,
                LBRACE0 | LBRACE => {
                    if let Some(name) = ident.name.clone() {
                        self.caller = Some(name.clone());
                        if let Some(caller) = self.lookup_name(&name) {
                            if caller.storage == Storage::Auto || caller.flag == SymbolFlag::Target
                            {
                                self.caller = None;
                            }
                        }
                        self.func_body();
                    }
                    break;
                }
                0 => {
                    if self.verbose {
                        self.file_error("unexpected end of file in declaration", None);
                    }
                    break;
                }
                _ => {
                    if error_recovery {
                        self.nexttoken();
                    } else {
                        if self.verbose {
                            self.file_error("expected `;'", Some(&self.tok));
                        }
                        error_recovery = true;
                    }
                }
            }
        }
    }

    pub fn fake_struct(&mut self, ident: &mut Ident) -> bool {
        while self.tok.token_type == QUALIFIER {
            let q = self.tok.token.clone().unwrap_or_default();
            if q == "const" || q == "volatile" {
                self.nexttoken();
            } else {
                break;
            }
        }
        ident.type_end = -1;
        if self.tok.token_type == STRUCT {
            if self.nexttoken() == IDENTIFIER {
                ident.type_end = self.curs as isize;
            }
            self.putback();
            self.skip_struct();
            if self.tok.token_type == IDENTIFIER
                || self.tok.token_type == MODIFIER
                || self.tok.token_type == QUALIFIER
            {
                self.putback();
            } else if self.tok.token_type == TYPE {
                if self.curs > 0 {
                    self.token_stack[self.curs - 1].token_type = IDENTIFIER;
                }
                self.putback();
            } else if self.tok.token_type == '(' as i32 {
                return false;
            } else if self.tok.token_type != ';' as i32 {
                self.file_error("missing `;' after struct declaration", Some(&self.tok));
            }
            return true;
        }
        false
    }

    pub fn parse_variable_declaration(&mut self, ident: &mut Ident, parm: bool) {
        let sp = self.mark();
        ident.type_end = -1;

        while self.tok.token_type == QUALIFIER {
            self.nexttoken();
        }

        if self.tok.token_type == STRUCT || self.tok.token_type == UNION {
            if self.nexttoken() == IDENTIFIER {
                ident.type_end = self.curs as isize;
            }
            self.putback();
            self.skip_struct();
            while self.tok.token_type == MODIFIER || self.tok.token_type == QUALIFIER {
                self.nexttoken();
            }
            if self.tok.token_type == IDENTIFIER {
                if ident.type_end == -1 {
                    let pos = self.curs - 1;
                    self.curs = sp;
                    self.tokdel(self.curs, pos.saturating_sub(1));
                    self.tokins(self.curs, IDENTIFIER, self.tok.line, "{ ... }");
                    let tok = self.tok.clone();
                    self.debugtoken(Some(&tok), Some("modified stack"));
                }
            } else {
                if self.tok.token_type == ';' as i32 {
                    return;
                }
                self.curs = sp;
            }
        }

        loop {
            self.parse_dcl(ident, false);
            match self.tok.token_type {
                41 if parm => break,
                59 => break,
                44 if parm => break,
                44 => {
                    if ident.type_end >= 0 {
                        self.tos = ident.type_end as usize;
                    }
                    self.curs = sp;
                    continue;
                }
                61 => {
                    self.nexttoken();
                    match self.tok.token_type {
                        LBRACE | LBRACE0 => {
                            self.initializer_list();
                            continue;
                        }
                        RBRACE | RBRACE0 => {
                            if self.verbose {
                                self.file_error("expected expression", Some(&self.tok));
                            }
                            break;
                        }
                        _ => {
                            self.expression();
                            continue;
                        }
                    }
                }
                LBRACE0 | LBRACE => {
                    self.func_body();
                    break;
                }
                0 => {
                    if self.verbose {
                        self.file_error("unexpected end of file in declaration", None);
                    }
                    break;
                }
                _ => {
                    if self.verbose {
                        self.file_error("expected `;'", Some(&self.tok));
                    }
                    break;
                }
            }
        }
    }

    pub fn initializer_list(&mut self) {
        let mut lev = 0;
        loop {
            match self.tok.token_type {
                LBRACE | LBRACE0 => lev += 1,
                RBRACE | RBRACE0 => {
                    lev -= 1;
                    if lev <= 0 {
                        self.nexttoken();
                        return;
                    }
                }
                0 => {
                    self.file_error("unexpected end of file in initializer list", None);
                    return;
                }
                44 => {}
                _ => self.expression(),
            }
            self.nexttoken();
        }
    }

    pub fn parse_knr_dcl(&mut self, ident: &mut Ident) {
        ident.type_end = -1;
        self.parse_dcl(ident, !self.strict_ansi);
    }

    pub fn skip_struct(&mut self) {
        if self.nexttoken() == IDENTIFIER {
            self.nexttoken();
        } else if self.tok.token_type == ';' as i32 {
            return;
        }

        if self.tok.token_type == LBRACE || self.tok.token_type == LBRACE0 {
            if self.skip_balanced('{' as i32, 1) == -1 {
                self.file_error("unexpected end of file in struct", None);
                return;
            }
        }

        while self.tok.token_type == PARM_WRAPPER {
            if self.skip_balanced('(' as i32, 0) == -1 {
                self.file_error("unexpected end of file in struct", None);
                return;
            }
        }
    }

    pub fn parse_typedef(&mut self) {
        let mut ident = Ident {
            name: None,
            type_end: -1,
            parmcnt: -1,
            line: -1,
            storage: Storage::Any,
        };

        self.nexttoken();
        if !self.fake_struct(&mut ident) {
            self.putback();
        }

        self.dcl(&mut ident);
        if ident.name.is_some() {
            self.declare_type(&ident);
        }
    }

    pub fn parse_dcl(&mut self, ident: &mut Ident, maybe_knr: bool) {
        ident.parmcnt = -1;
        ident.name = None;
        self.putback();
        self.dcl(ident);
        self.save_stack();
        if ident.name.is_some() {
            self.declare(ident, maybe_knr);
        } else {
            self.undo_save_stack();
        }
    }

    pub fn dcl(&mut self, idptr: &mut Ident) -> i32 {
        while self.nexttoken() != 0 && self.tok.token_type != '(' as i32 {
            if self.tok.token_type == MODIFIER {
                if idptr.type_end == -1 {
                    idptr.type_end = self.curs as isize - 1;
                }
            } else if self.tok.token_type == PARM_WRAPPER {
                if self.skip_balanced('(' as i32, 0) == -1 {
                    self.file_error("unexpected end of file in function declaration", None);
                    return 1;
                }
                self.putback();
            } else if self.tok.token_type == IDENTIFIER {
                let mut next_type;
                loop {
                    if self.tok.token_type != IDENTIFIER {
                        break;
                    }
                    self.nexttoken();
                }
                next_type = self.tok.token_type;
                self.putback();
                if next_type != MODIFIER {
                    idptr.type_end = self.curs as isize - 1;
                    break;
                }
            }
        }

        if self.tok.token_type == '(' as i32 {
            self.putback();
        }
        self.dirdcl(idptr)
    }

    pub fn getident(&mut self, idptr: &mut Ident) -> Option<Vec<i32>> {
        let mut parm_types = Vec::new();
        loop {
            match self.nexttoken() {
                IDENTIFIER => {
                    if idptr.name.is_none() {
                        idptr.name = self.tok.token.clone();
                        idptr.line = self.tok.line as isize;
                        return Some(parm_types);
                    }
                    parm_types.push(IDENTIFIER);
                }
                TYPE | STRUCT | UNION | ENUM | MODIFIER | QUALIFIER => {
                    parm_types.push(self.tok.token_type);
                }
                _ => {
                    self.putback();
                    break;
                }
            }
        }
        None
    }

    pub fn dirdcl(&mut self, idptr: &mut Ident) -> i32 {
        let mut rc = 0;
        if self.nexttoken() == '(' as i32 {
            rc = self.dcl(idptr);
            if self.nexttoken() != ')' as i32 {
                self.file_error("missing `)'", Some(&self.tok));
                return 1;
            }
        } else {
            self.putback();
            let _ = self.getident(idptr);
        }

        loop {
            match self.nexttoken() {
                40 => {
                    self.putback();
                    if self.parmdcl(idptr) != 0 {
                        return 1;
                    }
                }
                91 => {
                    if self.skip_balanced('[' as i32, 1) == -1 {
                        self.file_error("unexpected end of file in array declarator", None);
                        return 1;
                    }
                }
                _ => {
                    self.putback();
                    break;
                }
            }
        }
        rc
    }

    pub fn parmdcl(&mut self, idptr: &mut Ident) -> i32 {
        let mut parm_cnt = 0;
        self.nexttoken();
        if self.nexttoken() == ')' as i32 {
            idptr.parmcnt = 0;
            return 0;
        }
        self.putback();
        self.putback();
        self.maybe_parm_list(&mut parm_cnt);
        idptr.parmcnt = parm_cnt;
        0
    }

    pub fn maybe_parm_list(&mut self, parm_cnt_return: &mut i32) {
        let saved_level = self.parm_level;
        self.parm_level = self.level + 1;

        if self.nexttoken() != '(' as i32 {
            self.putback();
            *parm_cnt_return = -1;
            self.parm_level = saved_level;
            return;
        }

        if self.nexttoken() == ')' as i32 {
            *parm_cnt_return = 0;
            self.parm_level = saved_level;
            return;
        }
        self.putback();

        let mut cnt = 0;
        loop {
            let mut ident = Ident {
                storage: Storage::Auto,
                ..Default::default()
            };
            self.parse_declaration(&mut ident, true);
            cnt += 1;
            match self.tok.token_type {
                41 => break,
                44 => continue,
                _ => {
                    if self.verbose {
                        self.file_error("expected `,' or `)' in parameter list", Some(&self.tok));
                    }
                    break;
                }
            }
        }
        *parm_cnt_return = cnt;
        self.parm_level = saved_level;
    }

    pub fn func_body(&mut self) {
        let mut level = 1;
        self.level += 1;
        loop {
            match self.nexttoken() {
                0 => {
                    if self.verbose {
                        self.file_error("unexpected end of file in function body", None);
                    }
                    break;
                }
                LBRACE | LBRACE0 => level += 1,
                RBRACE | RBRACE0 => {
                    level -= 1;
                    if level == 0 {
                        break;
                    }
                }
                TYPE | STRUCT | UNION | ENUM | QUALIFIER | STATIC | EXTERN => {
                    let mut ident = Ident::default();
                    self.parse_variable_declaration(&mut ident, false);
                }
                IDENTIFIER => {
                    self.expression();
                }
                _ => {}
            }
        }
        self.cleanup_stack();
        self.delete_autos(self.level);
        self.level = self.level.saturating_sub(1);
    }

    pub fn get_knr_args(&mut self, ident: &mut Ident) -> bool {
        if ident.parmcnt <= 0 {
            return false;
        }
        let mut saw_decl = false;
        let sp = self.mark();
        loop {
            match self.nexttoken() {
                TYPE | STRUCT | UNION | ENUM | QUALIFIER | STATIC | EXTERN => {
                    saw_decl = true;
                    let mut parm = Ident {
                        storage: Storage::Auto,
                        ..Default::default()
                    };
                    self.parse_declaration(&mut parm, false);
                    if self.tok.token_type == LBRACE || self.tok.token_type == LBRACE0 {
                        self.putback();
                        break;
                    }
                }
                _ => {
                    self.curs = sp;
                    break;
                }
            }
        }
        saw_decl
    }

    pub fn declare(&mut self, ident: &mut Ident, maybe_knr: bool) {
        let Some(name) = ident.name.clone() else {
            self.undo_save_stack();
            return;
        };

        let mut is_knr = false;
        if maybe_knr && ident.parmcnt > 0 {
            is_knr = self.get_knr_args(ident);
        }

        let decl = if self.save_stack_is_empty() {
            None
        } else {
            Some(self.finish_save_stack(&name))
        };
        self.undo_save_stack();

        let storage = if self.level == 0 {
            ident.storage
        } else {
            Storage::Auto
        };

        let storage_class = match storage {
            Storage::Static => SimpleStorageClass::Static,
            Storage::Extern | Storage::ExplicitExtern => SimpleStorageClass::Extern,
            Storage::Auto | Storage::Any => SimpleStorageClass::Other,
        };

        let sym = self
            .symbols
            .entry(name.clone())
            .or_insert_with(|| SymbolData::new(&name, Storage::Extern));
        sym.storage = storage;
        sym.level = self.level;
        sym.arity = ident.parmcnt;
        sym.def_line = ident.line.max(0) as usize;
        if decl.is_some() {
            sym.decl = decl;
        }

        if self.level == 0 && is_knr {
            sym.flag = SymbolFlag::Parm;
        }
    }

    pub fn declare_type(&mut self, ident: &Ident) {
        if let Some(name) = ident.name.as_deref() {
            self.typedef_names.insert(name.to_string());
            let sym = self
                .symbols
                .entry(name.to_string())
                .or_insert_with(|| SymbolData::new(name, Storage::Extern));
            sym.symbol_type = SymbolType::Token;
            sym.token_type = Some(TYPE);
            sym.storage = Storage::Extern;
            sym.arity = -1;
        }
    }

    pub fn get_symbol(&mut self, name: &str) -> &mut crate::output::SymbolRecord {
        let data = self
            .symbols
            .entry(name.to_string())
            .or_insert_with(|| SymbolData::new(name, Storage::Extern));

        let kind = match data.symbol_type {
            SymbolType::Token => crate::output::SimpleSymbolKind::Token,
            SymbolType::Undefined => crate::output::SimpleSymbolKind::Undefined,
            SymbolType::Identifier => crate::output::SimpleSymbolKind::Identifier,
        };

        let storage = match data.storage {
            Storage::Static => SimpleStorageClass::Static,
            Storage::Extern | Storage::ExplicitExtern => SimpleStorageClass::Extern,
            Storage::Auto | Storage::Any => SimpleStorageClass::Other,
        };

        let record = SymbolRecord {
            name: data.name.clone(),
            kind,
            storage,
            arity: data.arity,
            source: data.source.clone(),
            def_line: data.def_line,
            decl: data.decl.clone(),
            expand_line: None,
            recursive: false,
            active: 0,
            callers: data.callers.iter().cloned().collect(),
            callees: data.callees.iter().cloned().collect(),
            flag_start: matches!(data.flag, SymbolFlag::Start),
            refs: data.references.clone(),
        };

        Box::leak(Box::new(record))
    }

    pub fn add_reference(
        &mut self,
        name: &str,
        line: usize,
    ) -> &mut crate::output::SymbolRecord {
        let source_name = self.filename.clone();
        let data = self
            .symbols
            .entry(name.to_string())
            .or_insert_with(|| SymbolData::new(name, Storage::Extern));

        data.references.push(SimpleReference {
            source: source_name,
            line,
        });

        self.get_symbol(name)
    }

    pub fn call(&mut self, name: &str, line: usize) {
        let _ = self.add_reference(name, line);
        let callee_name = name.to_string();
        if let Some(caller_name) = self.caller.clone() {
            if caller_name != callee_name {
                if let Some(caller) = self.symbols.get_mut(&caller_name) {
                    caller.callees.insert(callee_name.clone());
                }
                if let Some(callee) = self.symbols.get_mut(&callee_name) {
                    callee.callers.insert(caller_name);
                }
            }
        }
    }

    pub fn reference(&mut self, name: &str, line: usize) {
        let _ = self.add_reference(name, line);
    }

    pub fn reset_static_caller(&mut self) {
        if let Some(name) = self.caller.clone() {
            if let Some(caller) = self.symbols.get(&name) {
                if caller.storage == Storage::Static || caller.flag == SymbolFlag::Local {
                    self.caller = None;
                }
            }
        }
    }

    pub fn balance_state(&self, idx: i32, level: i32) -> balance_state {
        balance_state { idx, level }
    }

    pub fn maybe_knr(&self, ident: &Ident) -> bool {
        !self.strict_ansi && ident.parmcnt > 0
    }
}
