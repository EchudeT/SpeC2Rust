use std::collections::{HashMap, HashSet};
use crate::symbol::SymbolType;
use std::fmt::Write as _;
use std::io::{self, Write};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleSrcSaveStack14 {
    pub mark: usize,
    pub save_end: isize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SaveStack14 {
    pub begin: usize,
    pub end: usize,
    pub rendered: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleSrcBalanceState08 {
    pub idx: usize,
    pub level: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BalanceState {
    pub idx: usize,
    pub level: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReferenceEntry {
    pub source: String,
    pub line: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolStorage {
    Any,
    Extern,
    ExplicitExtern,
    Static,
    Auto,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolEntry {
    pub name: String,
    pub storage: SymbolStorage,
    pub line: i32,
    pub declaration: Option<String>,
    pub arity: i32,
    pub references: Vec<ReferenceEntry>,
    pub callers: Vec<String>,
    pub callees: Vec<String>,
    pub is_type: bool,
    pub is_target: bool,
    pub is_local: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ident {
    pub name: Option<String>,
    pub type_end: isize,
    pub parmcnt: i32,
    pub line: i32,
    pub storage: SymbolStorage,
}

impl Default for Ident {
    fn default() -> Self {
        Self {
            name: None,
            type_end: -1,
            parmcnt: -1,
            line: -1,
            storage: SymbolStorage::Any,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenRecord {
    pub token_type: i32,
    pub line: i32,
    pub token: Option<String>,
}

impl TokenRecord {
    fn new(token_type: i32, line: i32, token: Option<String>) -> Self {
        Self {
            token_type,
            line,
            token,
        }
    }
}

pub struct Parser {
    debug: i32,
    verbose: bool,
    strict_ansi: bool,
    omit_symbol_names_option: bool,
    omit_arguments_option: bool,
    use_indentation: bool,
    globals_only_mode: bool,
    filename: String,
    line_num: i32,
    token_stack: Vec<TokenRecord>,
    curs: usize,
    tos: usize,
    tok: TokenRecord,
    pending_tokens: Vec<TokenRecord>,
    text_stk: String,
    need_space: bool,
    save_end: isize,
    start_pos: usize,
    level: i32,
    parm_level: i32,
    symbols: HashMap<String, SymbolEntry>,
    caller: Option<String>,
    parameter_symbols: HashSet<String>,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            debug: 0,
            verbose: false,
            strict_ansi: false,
            omit_symbol_names_option: false,
            omit_arguments_option: false,
            use_indentation: false,
            globals_only_mode: false,
            filename: "<input>".to_string(),
            line_num: 1,
            token_stack: Vec::new(),
            curs: 0,
            tos: 0,
            tok: TokenRecord::new(0, 1, None),
            pending_tokens: Vec::new(),
            text_stk: String::new(),
            need_space: false,
            save_end: -1,
            start_pos: 0,
            level: 0,
            parm_level: 0,
            symbols: HashMap::new(),
            caller: None,
            parameter_symbols: HashSet::new(),
        }
    }
}

impl Parser {
    pub fn balance_state(idx: usize, level: i32) -> BalanceState {
        BalanceState { idx, level }
    }

    pub fn module_src_save_stack_14(mark: usize, save_end: isize) -> ModuleSrcSaveStack14 {
        ModuleSrcSaveStack14 { mark, save_end }
    }

    pub fn save_stack_14(begin: usize, end: usize, rendered: impl Into<String>) -> SaveStack14 {
        SaveStack14 {
            begin,
            end,
            rendered: rendered.into(),
        }
    }

    pub fn module_src_balance_state_08(idx: usize, level: i32) -> ModuleSrcBalanceState08 {
        ModuleSrcBalanceState08 { idx, level }
    }

    pub fn print_token(&self, tokptr: &TokenRecord) -> String {
        match tokptr.token_type {
            256 | 257 | 258 | 259 | 260 | 261 | 262 | 263 => {
                format!("`{}'", tokptr.token.as_deref().unwrap_or(""))
            }
            264 | 265 => "`{'".to_string(),
            266 | 267 => "`}'".to_string(),
            268 => "`extern'".to_string(),
            269 => "`static'".to_string(),
            270 => "`typedef'".to_string(),
            271 => format!("\"{}\"", tokptr.token.as_deref().unwrap_or("")),
            t => {
                let ch = char::from_u32(t as u32).unwrap_or('?');
                format!("`{}'", ch)
            }
        }
    }

    pub fn token_type_str(&self, t: i32) -> String {
        match t {
            0 => "EOF".to_string(),
            258 => "WORD".to_string(),
            264 => "'{'".to_string(),
            266 => "'}'".to_string(),
            256 => "IDENTIFIER".to_string(),
            268 => "EXTERN".to_string(),
            269 => "STATIC".to_string(),
            270 => "TYPEDEF".to_string(),
            260 => "STRUCT".to_string(),
            259 => "MODIFIER".to_string(),
            263 => "OP".to_string(),
            272 => "UNION".to_string(),
            273 => "ENUM".to_string(),
            265 => "' {'".to_string(),
            267 => "' }'".to_string(),
            274 => "MEMBER_OF".to_string(),
            257 => "TYPE".to_string(),
            271 => "STRING".to_string(),
            261 => "PARM_WRAPPER".to_string(),
            262 => "QUALIFIER".to_string(),
            other if (0..=255).contains(&other) && (other as u8 as char).is_ascii_graphic() => {
                format!("'{}'({})", other as u8 as char, other)
            }
            other => other.to_string(),
        }
    }

    pub fn dbgtok(&self, t: &TokenRecord, delim: Option<char>) -> String {
        let mut out = String::new();
        if let Some(d) = delim {
            out.push(d);
        }
        let _ = write!(out, "{{ {} ", self.token_type_str(t.token_type));
        if t.token_type != 0 {
            let _ = write!(
                out,
                ", {}, {} ",
                t.token.as_deref().unwrap_or("NULL"),
                t.line
            );
        }
        out.push('}');
        out
    }

    pub fn debugtoken(&self, t: Option<&TokenRecord>, fmt: Option<&str>) -> String {
        if self.debug <= 1 {
            return String::new();
        }
        let mut out = String::new();
        if let Some(prefix) = fmt {
            let _ = write!(out, "{}: ", prefix);
        }
        if let Some(tok) = t {
            out.push_str(&self.dbgtok(tok, None));
            out.push_str("; ");
        }
        let _ = write!(out, "{}: {{", self.curs);
        for i in self.curs..self.tos {
            let delim = if i == self.curs { None } else { Some(',') };
            out.push_str(&self.dbgtok(&self.token_stack[i], delim));
        }
        out.push('}');
        out
    }

    pub fn file_error(&self, msg: &str, tokptr: Option<&TokenRecord>) -> String {
        let mut out = format!("{}:{}: {}", self.filename, self.tok.line, msg);
        if let Some(tok) = tokptr {
            out.push_str(" near ");
            out.push_str(&self.print_token(tok));
        }
        out.push('\n');
        let _ = io::stderr().write_all(out.as_bytes());
        out
    }

    pub fn mark(&self) -> usize {
        if self.debug > 1 {
            let _ = writeln!(io::stderr(), "marking stack at {}", self.curs);
        }
        self.curs
    }

    pub fn tokdel(&mut self, beg: usize, end: usize) {
        if end >= beg && beg < self.tos {
            let end_clamped = end.min(self.tos.saturating_sub(1));
            self.token_stack.drain(beg..=end_clamped);
            self.tos = self.token_stack.len();
            if self.curs > self.tos {
                self.curs = self.tos;
            }
        }
    }

    pub fn tokins(&mut self, pos: usize, token_type: i32, line: i32, token: impl Into<String>) {
        let rec = TokenRecord::new(token_type, line, Some(token.into()));
        let insert_at = pos.min(self.token_stack.len());
        self.token_stack.insert(insert_at, rec);
        self.tos = self.token_stack.len();
        let _ = self.debugtoken(
            self.token_stack.get(insert_at),
            Some(&format!("insert at {}", insert_at)),
        );
    }

    pub fn tokpush(&mut self, token_type: i32, line: i32, token: Option<String>) {
        self.token_stack.push(TokenRecord::new(token_type, line, token));
        self.tos = self.token_stack.len();
    }

    pub fn cleanup_stack(&mut self) {
        let delta = self.tos as isize - self.curs as isize;
        if delta > 0 {
            self.token_stack.drain(0..self.curs);
        } else if delta <= 0 {
            self.token_stack.clear();
        }
        self.tos = self.token_stack.len();
        self.curs = 0;
    }

    pub fn clearstack(&mut self) {
        self.tos = 0;
        self.curs = 0;
        self.token_stack.clear();
    }

    pub fn nexttoken(&mut self) -> i32 {
        if self.curs == self.tos {
            let next = if self.pending_tokens.is_empty() {
                TokenRecord::new(0, self.line_num, None)
            } else {
                self.pending_tokens.remove(0)
            };
            self.line_num = next.line;
            self.tokpush(next.token_type, next.line, next.token.clone());
        }
        self.tok = self.token_stack[self.curs].clone();
        self.curs += 1;
        let _ = self.debugtoken(Some(&self.tok), Some("next token"));
        self.tok.token_type
    }

    pub fn putback(&mut self) -> i32 {
        assert!(self.curs != 0, "INTERNAL ERROR: cannot return token to stream");
        self.curs -= 1;
        if self.curs > 0 {
            self.tok = self.token_stack[self.curs - 1].clone();
        } else {
            self.tok.token_type = 0;
            self.tok.token = None;
        }
        let _ = self.debugtoken(Some(&self.tok), Some("putback"));
        self.tok.token_type
    }

    pub fn init_parse(&mut self) {
        self.text_stk.clear();
        self.token_stack.clear();
        self.clearstack();
    }

    pub fn save_token(&mut self, tokptr: &TokenRecord) {
        match tokptr.token_type {
            256 | 257 | 260 | 261 | 258 | 262 => {
                if self.need_space {
                    self.text_stk.push(' ');
                }
                if let Some(token) = &tokptr.token {
                    self.text_stk.push_str(token);
                }
                self.need_space = true;
            }
            259 => {
                if self.need_space {
                    self.text_stk.push(' ');
                }
                if let Some(token) = &tokptr.token {
                    self.need_space = !token.starts_with('*');
                    self.text_stk.push_str(token);
                } else {
                    self.need_space = true;
                }
            }
            268 | 269 => {}
            x if x == ',' as i32 => {
                self.text_stk.push(',');
                self.need_space = true;
            }
            x if x == '(' as i32 => {
                if self.need_space {
                    self.text_stk.push(' ');
                }
                self.text_stk.push('(');
                self.need_space = false;
            }
            x if x == ')' as i32 => {
                self.text_stk.push(')');
                self.need_space = true;
            }
            x if x == '[' as i32 || x == ']' as i32 => {
                self.text_stk.push(char::from_u32(x as u32).unwrap_or('?'));
                self.need_space = false;
            }
            265 | 264 => {
                if self.need_space {
                    self.text_stk.push(' ');
                }
                self.text_stk.push('{');
                self.need_space = true;
            }
            267 | 266 => {
                if self.need_space {
                    self.text_stk.push(' ');
                }
                self.text_stk.push('}');
                self.need_space = true;
            }
            263 => {
                self.text_stk.push(' ');
                if let Some(token) = &tokptr.token {
                    self.text_stk.push_str(token);
                }
                self.need_space = true;
            }
            _ => {
                if self.verbose {
                    self.file_error("unrecognized definition", Some(tokptr));
                }
            }
        }
    }

    pub fn save_stack(&mut self) {
        self.start_pos = self.mark();
        self.save_end = self.curs as isize - 1;
    }

    pub fn undo_save_stack(&mut self) {
        self.save_end = -1;
    }

    pub fn save_stack_is_empty(&self) -> bool {
        self.save_end <= 0
    }

    pub fn finish_save_stack(&mut self, name: &str) -> String {
        let mut level = 0;
        let mut found_ident = !self.omit_symbol_names_option;
        self.need_space = false;
        self.text_stk.clear();

        let save_end = self.save_end.max(0) as usize;
        for i in 0..save_end.min(self.token_stack.len()) {
            match self.token_stack[i].token_type {
                x if x == '(' as i32 => {
                    if self.omit_arguments_option {
                        if level == 0 {
                            let tok = self.token_stack[i].clone();
                            self.save_token(&tok);
                        }
                        level += 1;
                    }
                }
                x if x == ')' as i32 => {
                    if self.omit_arguments_option {
                        level -= 1;
                    }
                }
                256 => {
                    if !found_ident && self.token_stack[i].token.as_deref() == Some(name) {
                        self.need_space = true;
                        found_ident = true;
                        continue;
                    }
                }
                _ => {}
            }
            if level == 0 {
                let tok = self.token_stack[i].clone();
                self.save_token(&tok);
            }
        }
        self.text_stk.clone()
    }

    pub fn skip_to(&mut self, c: i32) {
        while self.nexttoken() != 0 {
            if self.tok.token_type == c {
                break;
            }
        }
    }

    pub fn push_balance_state(&self, stack: &mut Vec<BalanceState>, idx: usize, level: i32) {
        stack.push(BalanceState { idx, level });
    }

    pub fn pop_balance_state(
        &self,
        stack: &mut Vec<BalanceState>,
        idx: &mut usize,
        level: &mut i32,
    ) {
        if let Some(state) = stack.pop() {
            *idx = state.idx;
            *level = state.level;
        }
    }

    pub fn free_balance_stack(&self, stack: &mut Vec<BalanceState>) {
        stack.clear();
    }

    pub fn find_closing_paren(&mut self, open_tok: i32, mut level: i32) -> i32 {
        let oparen = ['(', '[', '{'];
        let cparen = [')', ']', '}'];
        let mut tos = Vec::<BalanceState>::new();

        let mut idx = match oparen.iter().position(|c| *c as i32 == open_tok) {
            Some(v) => v,
            None => panic!("INTERNAL ERROR: malformed call to find_closing_paren"),
        };

        if level == 0 {
            if self.nexttoken() != open_tok {
                return 1;
            }
            level += 1;
        }

        while self.nexttoken() != 0 {
            if self.tok.token_type == 264 {
                self.tok.token_type = '{' as i32;
            } else if self.tok.token_type == 266 {
                self.tok.token_type = '}' as i32;
            }

            if self.tok.token_type == oparen[idx] as i32 {
                level += 1;
            } else if self.tok.token_type == cparen[idx] as i32 {
                level -= 1;
                if level == 0 {
                    if tos.is_empty() {
                        return 0;
                    }
                    self.pop_balance_state(&mut tos, &mut idx, &mut level);
                }
            } else if let Some(new_idx) = oparen
                .iter()
                .position(|c| *c as i32 == self.tok.token_type)
            {
                self.push_balance_state(&mut tos, idx, level);
                idx = new_idx;
                level = 1;
            }
        }

        self.free_balance_stack(&mut tos);
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
            identifier.storage = SymbolStorage::Extern;
            match self.tok.token_type {
                0 => return 0,
                262 => continue,
                270 => self.parse_typedef(),
                275 => self.skip_declaration(),
                268 => {
                    identifier.storage = SymbolStorage::ExplicitExtern;
                    self.nexttoken();
                    self.parse_declaration(&mut identifier, false);
                }
                269 => {
                    identifier.storage = SymbolStorage::Static;
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
                262 | 257 | 256 | 259 | 269 | 268 | 260 | 272 | 273 => {
                    self.nexttoken();
                    continue;
                }
                261 => {
                    if self.skip_balanced('(' as i32, 0) == -1 {
                        self.file_error("unexpected end of file in declaration", None);
                    }
                    continue;
                }
                x if x == '(' as i32 => {
                    res = self.nexttoken() != 259;
                    break;
                }
                _ => break,
            }
        }

        self.putback_to_mark(sp);
        res
    }

    pub fn parse_declaration(&mut self, ident: &mut Ident, parm: bool) {
        if self.is_function() {
            self.parse_function_declaration(ident, parm);
        } else {
            self.parse_variable_declaration(ident, parm);
        }
        self.parameter_symbols.clear();
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
        loop {
            match self.tok.token_type {
                0 => {
                    self.file_error("unexpected end of file in expression", None);
                    return;
                }
                x if x == ';' as i32 || x == ',' as i32 || x == ')' as i32 => {
                    self.putback();
                    return;
                }
                x if x == '(' as i32 => {
                    if self.skip_balanced('(' as i32, 1) == -1 {
                        self.file_error("unexpected end of file in expression", None);
                        return;
                    }
                }
                264 | 265 => {
                    if self.skip_balanced('{' as i32, 1) == -1 {
                        self.file_error("unexpected end of file in expression", None);
                        return;
                    }
                }
                256 => {
                    let name = self.tok.token.clone().unwrap_or_default();
                    let line = self.tok.line;
                    if self.nexttoken() == '(' as i32 {
                        self.call(&name, line);
                        if self.skip_balanced('(' as i32, 1) == -1 {
                            self.file_error("unexpected end of file in expression", None);
                            return;
                        }
                    } else {
                        self.putback();
                        self.reference(&name, line);
                    }
                }
                _ => {}
            }
            self.nexttoken();
        }
    }

    pub fn parse_function_declaration(&mut self, ident: &mut Ident, parm: bool) {
        let mut saw_error = false;
        ident.type_end = -1;
        self.parse_knr_dcl(ident);

        loop {
            match self.tok.token_type {
                x if x == ')' as i32 && parm => break,
                x if x == ';' as i32 || x == ',' as i32 => break,
                264 | 265 => {
                    if let Some(name) = &ident.name {
                        let caller_name = name.clone();
                        let allowed = self
                            .symbols
                            .get(&caller_name)
                            .map(|s| !(matches!(s.storage, SymbolStorage::Auto) || s.is_target))
                            .unwrap_or(true);
                        self.caller = if allowed { Some(caller_name) } else { None };
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
                    if !saw_error && self.verbose {
                        self.file_error("expected `;'", Some(&self.tok));
                    }
                    saw_error = true;
                    self.nexttoken();
                }
            }
        }
    }

    pub fn fake_struct(&mut self, ident: &mut Ident) -> bool {
        while self.tok.token_type == 262 {
            let is_cv = self
                .tok
                .token
                .as_deref()
                .map(|s| s == "const" || s == "volatile")
                .unwrap_or(false);
            if !is_cv {
                break;
            }
            self.nexttoken();
        }
        ident.type_end = -1;
        if self.tok.token_type == 260 {
            if self.nexttoken() == 256 {
                ident.type_end = self.curs as isize;
            }
            self.putback();
            self.skip_struct();
            if matches!(self.tok.token_type, 256 | 259 | 262) {
                self.putback();
            } else if self.tok.token_type == 257 {
                if self.curs > 0 {
                    self.token_stack[self.curs - 1].token_type = 256;
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

    pub fn parse_variable_declaration(&mut self, ident: &mut Ident, _parm: bool) {
        ident.type_end = -1;

        if self.tok.token_type == 260 {
            self.skip_struct();
            if self.tok.token_type == ';' as i32 {
                return;
            }
            self.putback();
        }

        self.parse_dcl(ident, false);

        if self.tok.token_type == '=' as i32 {
            self.nexttoken();
            if matches!(self.tok.token_type, 264 | 265) {
                self.initializer_list();
            } else {
                self.expression();
            }
        }

        while self.tok.token_type != ';' as i32 && self.tok.token_type != 0 {
            self.nexttoken();
        }

        if matches!(self.tok.token_type, 264 | 265) {
            self.func_body();
        }
    }

    pub fn initializer_list(&mut self) {
        let mut lev = 0;
        loop {
            match self.tok.token_type {
                265 | 264 => lev += 1,
                267 | 266 => {
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
                x if x == ',' as i32 => {}
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
        if self.nexttoken() == 256 {
            self.nexttoken();
        } else if self.tok.token_type == ';' as i32 {
            return;
        }

        if self.tok.token_type == 265 || self.tok.token_type == 264 {
            if self.skip_balanced('{' as i32, 1) == -1 {
                self.file_error("unexpected end of file in struct", None);
                return;
            }
        }

        while self.tok.token_type == 261 {
            if self.skip_balanced('(' as i32, 0) == -1 {
                self.file_error("unexpected end of file in struct", None);
                return;
            }
        }
    }

    pub fn parse_typedef(&mut self) {
        let mut ident = Ident::default();

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
        if self.curs > 0 {
            self.putback();
        }
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
            if self.tok.token_type == 259 {
                if idptr.type_end == -1 {
                    idptr.type_end = self.curs as isize - 1;
                }
            } else if self.tok.token_type == 261 {
                if self.skip_balanced('(' as i32, 0) == -1 {
                    self.file_error("unexpected end of file in function declaration", None);
                    return 1;
                }
                self.putback();
            } else if self.tok.token_type == 256 {
                let mut next_type;
                while self.tok.token_type == 256 {
                    next_type = self.nexttoken();
                    if next_type != 256 {
                        break;
                    }
                }
                next_type = self.tok.token_type;
                self.putback();
                if !(next_type == 257 || next_type == 259 || next_type == 262) {
                    break;
                }
            } else if self.tok.token_type == ')' as i32 || self.tok.token_type == ';' as i32 {
                return 1;
            }
        }
        if idptr.type_end == -1 {
            idptr.type_end = self.curs as isize - 1;
        }
        self.dirdcl(idptr)
    }

    pub fn getident(&mut self, idptr: Option<&mut Ident>) -> i32 {
        let pos = self.mark();
        let mut bc = 0;

        loop {
            bc += 1;
            if self.nexttoken() == 0 {
                if self.verbose {
                    self.file_error("unexpected end of file", None);
                }
                return -1;
            }
            if self.tok.token_type != '(' as i32 {
                break;
            }
        }

        if self.tok.token_type == 256 {
            let name = self.tok.token.clone();
            let line = self.tok.line;

            while bc > 0 {
                if self.nexttoken() == 0 {
                    if self.verbose {
                        self.file_error("unexpected end of file", None);
                    }
                    return -1;
                }
                if self.tok.token_type == ')' as i32 {
                    bc -= 1;
                } else {
                    break;
                }
            }

            if bc == 0 {
                if let Some(ident) = idptr {
                    ident.name = name;
                    ident.line = line;
                    ident.parmcnt = -1;
                }
                return 0;
            }
        }

        self.putback_to_mark(pos);
        1
    }

    pub fn dirdcl(&mut self, idptr: &mut Ident) -> i32 {
        if self.tok.token_type == '(' as i32 {
            match self.getident(Some(idptr)) {
                0 => {}
                -1 => return 1,
                _ => {
                    self.dcl(idptr);
                    if self.tok.token_type != ')' as i32 && self.verbose {
                        self.file_error("expected `)'", Some(&self.tok));
                        return 1;
                    }
                }
            }
        } else if self.tok.token_type == 256 {
            idptr.name = self.tok.token.clone();
            idptr.line = self.tok.line;
        }

        loop {
            match self.nexttoken() {
                x if x == '[' as i32 => {
                    if self.find_closing_paren('[' as i32, 1) == -1 {
                        self.file_error("unexpected end of file in declaration", None);
                        return -1;
                    }
                }
                x if x == '(' as i32 => {
                    self.maybe_parm_list(Some(&mut idptr.parmcnt));
                    if self.tok.token_type != ')' as i32 && self.verbose {
                        self.file_error("expected `)'", Some(&self.tok));
                        return 1;
                    }
                }
                _ => break,
            }
        }
        0
    }
    pub fn parmdcl(&mut self, idptr: Option<&mut Ident>) -> i32 {
        let mut idptr = idptr;
        while self.nexttoken() != 0 && self.tok.token_type != '(' as i32 {
            if self.tok.token_type == 259 {
                if let Some(id) = idptr.as_deref_mut() {
                    if id.type_end == -1 {
                        id.type_end = self.curs as isize - 1;
                    }
                }
            } else if self.tok.token_type == 256 {
                while self.tok.token_type == 256 {
                    self.nexttoken();
                }
                let token_type = self.tok.token_type;
                self.putback();
                if token_type != 259 {
                    break;
                }
            } else if self.tok.token_type == ')' as i32 || self.tok.token_type == ',' as i32 {
                return 0;
            }
        }
        if let Some(id) = idptr {
            if id.type_end == -1 {
                id.type_end = self.curs as isize - 1;
            }
            return self.dirdcl(id);
        }
        let mut tmp = Ident::default();
        self.dirdcl(&mut tmp)
    }
    pub fn maybe_parm_list(&mut self, parm_cnt_return: Option<&mut i32>) {
        let mut parmcnt = 0;
        let mut ident = Ident::default();

        self.parm_level += 1;
        let mut parm_cnt_return = parm_cnt_return;
        while self.nexttoken() != 0 {
            match self.tok.token_type {
                x if x == ')' as i32 => {
                    if let Some(slot) = parm_cnt_return.as_deref_mut() {
                        *slot = parmcnt;
                    }
                    self.parm_level -= 1;
                    return;
                }
                x if x == ',' as i32 => {}
                262 | 256 | 259 | 260 | 272 | 257 => {
                    parmcnt += 1;
                    ident.storage = SymbolStorage::Auto;
                    self.parse_declaration(&mut ident, true);
                    self.putback();
                }
                _ => {
                    if self.verbose {
                        self.file_error("unexpected token in parameter list", Some(&self.tok));
                    }
                    let mut level = 0;
                    loop {
                        if self.tok.token_type == '(' as i32 {
                            level += 1;
                        } else if self.tok.token_type == ')' as i32 {
                            if level == 0 {
                                break;
                            }
                            level -= 1;
                        }
                        if self.nexttoken() == 0 {
                            break;
                        }
                    }
                    self.putback();
                }
            }
        }
        self.parm_level -= 1;
        if self.verbose {
            self.file_error("unexpected end of file in parameter list", None);
        }
    }

    pub fn func_body(&mut self) {
        let mut ident = Ident::default();

        self.level += 1;
        while self.level > 0 {
            self.cleanup_stack();
            self.nexttoken();
            match self.tok.token_type {
                269 => {
                    ident.storage = SymbolStorage::Static;
                    self.nexttoken();
                    self.parse_variable_declaration(&mut ident, false);
                }
                257 | 260 => {
                    ident.storage = SymbolStorage::Auto;
                    self.parse_variable_declaration(&mut ident, false);
                }
                268 => {
                    ident.storage = SymbolStorage::ExplicitExtern;
                    self.parse_declaration(&mut ident, false);
                }
                264 | 265 => self.level += 1,
                266 => {
                    if self.use_indentation {
                        self.level = 0;
                    } else {
                        self.level -= 1;
                    }
                }
                267 => self.level -= 1,
                0 => {
                    if self.verbose {
                        self.file_error("unexpected end of file in function body", None);
                    }
                    break;
                }
                _ => self.expression(),
            }
        }
        self.caller = None;
    }

    pub fn get_knr_args(&mut self, ident: &mut Ident) -> i32 {
        match self.tok.token_type {
            262 | 256 | 257 | 260 => {
                let sp = self.mark();
                let mut parmcnt = 0;
                let mut stop = false;
                while !stop && parmcnt < ident.parmcnt {
                    let mut id = Ident::default();
                    id.type_end = -1;
                    match self.tok.token_type {
                        265 | 264 => {
                            self.putback();
                            stop = true;
                        }
                        257 | 262 | 256 | 260 => {
                            self.putback();
                            let new_sp = self.mark();
                            if self.dcl(&mut id) == 0 {
                                parmcnt += 1;
                                if self.tok.token_type == ',' as i32 {
                                    loop {
                                        self.tos = id.type_end.max(0) as usize;
                                        self.putback_to_mark(new_sp);
                                        self.dcl(&mut id);
                                        if self.tok.token_type != ',' as i32 {
                                            break;
                                        }
                                    }
                                } else if self.tok.token_type != ';' as i32 {
                                    self.putback();
                                }
                            } else {
                                self.putback_to_mark(sp);
                                return 1;
                            }
                        }
                        _ => {
                            self.putback_to_mark(sp);
                            return 1;
                        }
                    }
                    if !stop {
                        self.nexttoken();
                    }
                }
            }
            _ => {}
        }
        if self.tok.token_type != 264 && self.tok.token_type != 265 {
            return 1;
        }
        0
    }

    pub fn declare(&mut self, ident: &mut Ident, maybe_knr: bool) {
        let Some(name) = ident.name.clone() else {
            self.undo_save_stack();
            return;
        };

        let declaration = if self.save_stack_is_empty() {
            None
        } else {
            Some(self.finish_save_stack(&name))
        };

        let mut effective_arity = ident.parmcnt;
        if maybe_knr && effective_arity > 0 {
            let sp = self.mark();
            if self.get_knr_args(ident) != 0 {
                self.putback_to_mark(sp);
            }
            effective_arity = ident.parmcnt;
        }

        self.install(
            &name,
            SymbolType::Identifier,
            ident.storage.clone(),
            effective_arity,
            ident.line,
            declaration,
        );
    }
    pub fn putback_to_mark(&mut self, mark: usize) {
        assert!(mark <= self.tos, "INTERNAL ERROR: mark beyond top of stack");
        self.curs = mark;
        if self.curs > 0 {
            self.tok = self.token_stack[self.curs - 1].clone();
        } else {
            self.tok.token_type = 0;
            self.tok.token = None;
            self.tok.line = self.line_num;
        }
        let _ = self.debugtoken(Some(&self.tok), Some("putback to mark"));
    }

    fn ensure_symbol_entry(&mut self, name: &str) -> &mut SymbolEntry {
        self.symbols.entry(name.to_string()).or_insert_with(|| SymbolEntry {
            name: name.to_string(),
            storage: SymbolStorage::Extern,
            line: -1,
            declaration: None,
            arity: -1,
            references: Vec::new(),
            callers: Vec::new(),
            callees: Vec::new(),
            is_type: false,
            is_target: false,
            is_local: false,
        })
    }

    pub fn reference(&mut self, name: &str, line: i32) {
        if self.parameter_symbols.contains(name) {
            return;
        }

        let caller = self.caller.clone();
        let source = self.filename.clone();
        let entry = self.ensure_symbol_entry(name);
        entry.references.push(ReferenceEntry {
            source,
            line,
        });

        if let Some(caller_name) = caller {
            if !entry.callers.iter().any(|s| s == &caller_name) {
                entry.callers.push(caller_name);
            }
        }
    }

    pub fn call(&mut self, name: &str, line: i32) {
        self.reference(name, line);

        let callee_name = name.to_string();
        if let Some(caller_name) = self.caller.clone() {
            if caller_name != callee_name {
                {
                    let caller_entry = self.ensure_symbol_entry(&caller_name);
                    if !caller_entry.callees.iter().any(|s| s == &callee_name) {
                        caller_entry.callees.push(callee_name.clone());
                    }
                }
                let callee_entry = self.ensure_symbol_entry(&callee_name);
                if !callee_entry.callers.iter().any(|s| s == &caller_name) {
                    callee_entry.callers.push(caller_name);
                }
            }
        }
    }

    pub fn declare_type(&mut self, ident: &Ident) {
        let Some(name) = ident.name.as_deref() else {
            return;
        };

        let declaration = if self.save_stack_is_empty() {
            None
        } else {
            Some(self.finish_save_stack(name))
        };
        let entry = self.ensure_symbol_entry(name);
        entry.is_type = true;
        entry.line = ident.line;
        if entry.declaration.is_none() {
            entry.declaration = declaration;
        }
    }

    pub fn install(
        &mut self,
        name: &str,
        symbol_type: SymbolType,
        storage: SymbolStorage,
        arity: i32,
        line: i32,
        declaration: Option<String>,
    ) {
        let filename = self.filename.clone();
        let parm_level = self.parm_level;
        let entry = self.ensure_symbol_entry(name);
        entry.storage = storage.clone();
        entry.line = line;
        if declaration.is_some() {
            entry.declaration = declaration;
        }
        entry.arity = arity;
        entry.is_type = matches!(symbol_type, SymbolType::Token);
        entry.is_local = matches!(storage, SymbolStorage::Auto)
            || (parm_level > 0 && matches!(storage, SymbolStorage::Extern | SymbolStorage::ExplicitExtern));
        if !entry.is_local {
            entry.line = line;
        }
        if entry.references.is_empty() && line >= 0 {
            entry.references.push(ReferenceEntry {
                source: filename,
                line,
            });
            entry.references.clear();
        }
        if parm_level > 0 {
            self.parameter_symbols.insert(name.to_string());
        }
    }
}
