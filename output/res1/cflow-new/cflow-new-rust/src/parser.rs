use crate::c::C;
use crate::output::SymbolFlag;
use std::fmt;
use std::io::{self, Write};

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

#[derive(Clone, Debug, Default)]
struct TokenEntry {
    token_type: i32,
    line: i32,
    token: Option<String>,
}

#[derive(Clone, Debug, Default)]
struct Ident {
    name: Option<String>,
    type_end: isize,
    parmcnt: i32,
    line: i32,
    storage: StorageClass,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum StorageClass {
    #[default]
    Any,
    Auto,
    Static,
    Extern,
    ExplicitExtern,
}


#[derive(Clone, Debug, Default, PartialEq, Eq)]
enum SymbolType {
    #[default]
    Undefined,
    Identifier,
    Token,
}

#[derive(Clone, Debug)]
struct SymbolRecord {
    name: String,
    storage: StorageClass,
    flag: SymbolFlag,
    symbol_type: SymbolType,
    token_type: Option<i32>,
    source: Option<String>,
    def_line: Option<i32>,
    decl: Option<String>,
    arity: i32,
    level: usize,
    references: Vec<(String, i32)>,
    callers: Vec<String>,
    callees: Vec<String>,
}

impl Default for SymbolRecord {
    fn default() -> Self {
        Self {
            name: String::new(),
            storage: StorageClass::default(),
            flag: SymbolFlag::None,
            symbol_type: SymbolType::default(),
            token_type: None,
            source: None,
            def_line: None,
            decl: None,
            arity: 0,
            level: 0,
            references: Vec::new(),
            callers: Vec::new(),
            callees: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct BalanceState {
    pub idx: i32,
    pub level: i32,
}

pub struct Parser {
    lexer: C,
    token_stack: Vec<TokenEntry>,
    tos: usize,
    curs: usize,
    tok: TokenEntry,
    token_stack_increase: usize,
    start_pos: usize,
    save_end: isize,
    need_space: bool,
    text_stk: String,
    debug: i32,
    verbose: bool,
    omit_symbol_names_option: bool,
    omit_arguments_option: bool,
    strict_ansi: bool,
    use_indentation: bool,
    level: usize,
    parm_level: usize,
    filename: String,
    caller: Option<String>,
    symbols: Vec<SymbolRecord>,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            lexer: C::default(),
            token_stack: vec![TokenEntry::default(); 128],
            tos: 0,
            curs: 0,
            tok: TokenEntry::default(),
            token_stack_increase: 128,
            start_pos: 0,
            save_end: -1,
            need_space: false,
            text_stk: String::new(),
            debug: 0,
            verbose: true,
            omit_symbol_names_option: false,
            omit_arguments_option: false,
            strict_ansi: false,
            use_indentation: false,
            level: 0,
            parm_level: 0,
            filename: String::new(),
            caller: None,
            symbols: Vec::new(),
        }
    }
}

impl Parser {
    pub fn print_token(&self, tokptr: &TokenEntry) {
        let mut stderr = io::stderr().lock();
        match tokptr.token_type {
            IDENTIFIER | TYPE | WORD | MODIFIER | STRUCT | PARM_WRAPPER | QUALIFIER | OP => {
                let _ = write!(stderr, "`{}'", tokptr.token.as_deref().unwrap_or(""));
            }
            LBRACE0 | LBRACE => {
                let _ = write!(stderr, "`{{'");
            }
            RBRACE0 | RBRACE => {
                let _ = write!(stderr, "`}}'");
            }
            EXTERN => {
                let _ = write!(stderr, "`extern'");
            }
            STATIC => {
                let _ = write!(stderr, "`static'");
            }
            TYPEDEF => {
                let _ = write!(stderr, "`typedef'");
            }
            STRING => {
                let _ = write!(stderr, "\"{}\"", tokptr.token.as_deref().unwrap_or(""));
            }
            other => {
                let ch = char::from_u32(other as u32).unwrap_or('?');
                let _ = write!(stderr, "`{}'", ch);
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
            other if (other as u32) < 128 && (other as u8 as char).is_ascii_graphic() => {
                format!("'{}'({})", other as u8 as char, other)
            }
            other => other.to_string(),
        }
    }

    pub fn dbgtok(&self, t: &TokenEntry, delim: i32) {
        let mut stderr = io::stderr().lock();
        if delim != 0 {
            let _ = write!(stderr, "{}", char::from_u32(delim as u32).unwrap_or(','));
        }
        let _ = write!(stderr, "{{ {} ", self.token_type_str(t.token_type));
        if t.token_type != 0 {
            let _ = write!(
                stderr,
                ", {}, {} ",
                t.token.as_deref().unwrap_or("NULL"),
                t.line
            );
        }
        let _ = write!(stderr, "}}");
    }

    pub fn debugtoken(&self, t: Option<&TokenEntry>, args: fmt::Arguments<'_>) {
        if self.debug > 1 {
            let prefix = format!("{}", args);
            {
                let mut stderr = io::stderr().lock();
                if !prefix.is_empty() {
                    let _ = write!(stderr, "{}: ", prefix);
                }
            }
            if let Some(tok) = t {
                self.dbgtok(tok, 0);
                let mut stderr = io::stderr().lock();
                let _ = write!(stderr, "; ");
            }
            {
                let mut stderr = io::stderr().lock();
                let _ = write!(stderr, "{}: {{", self.curs);
            }
            for i in self.curs..self.tos {
                self.dbgtok(&self.token_stack[i], if i == self.curs { 0 } else { ',' as i32 });
            }
            let mut stderr = io::stderr().lock();
            let _ = writeln!(stderr, "}}");
        }
    }

    pub fn file_error(&self, msg: &str, tokptr: Option<&TokenEntry>) {
        let mut stderr = io::stderr().lock();
        let _ = write!(stderr, "{}:{}: {}", self.filename, self.tok.line, msg);
        if let Some(tok) = tokptr {
            let _ = write!(stderr, " near ");
            drop(stderr);
            self.print_token(tok);
            stderr = io::stderr().lock();
        }
        let _ = writeln!(stderr);
    }

    pub fn mark(&self, pos: &mut usize) {
        *pos = self.curs;
        if self.debug > 1 {
            let _ = writeln!(io::stderr().lock(), "marking stack at {}", self.curs);
        }
    }


    pub fn tokdel(&mut self, beg: usize, end: usize) {
        if end >= beg && beg < self.tos {
            let end_clamped = end.min(self.tos.saturating_sub(1));
            let count = end_clamped - beg + 1;
            self.token_stack.drain(beg..=end_clamped);
            self.tos = self.tos.saturating_sub(count);
            if self.curs > self.tos {
                self.curs = self.tos;
            }
            while self.token_stack.len() < self.tos + 1 {
                self.token_stack.push(TokenEntry::default());
            }
        }
    }

    pub fn tokins(&mut self, pos: usize, token_type: i32, line: i32, token: impl Into<String>) {
        if self.tos + 1 >= self.token_stack.len() {
            self.token_stack
                .resize(self.token_stack.len() + self.token_stack_increase, TokenEntry::default());
        }
        self.token_stack.insert(
            pos,
            TokenEntry {
                token_type,
                line,
                token: Some(token.into()),
            },
        );
        self.tos += 1;
        self.debugtoken(
            self.token_stack.get(pos),
            format_args!("insert at {}", pos),
        );
    }

    pub fn tokpush(&mut self, token_type: i32, line: i32, token: Option<String>) {
        if self.tos >= self.token_stack.len() {
            self.token_stack
                .resize(self.token_stack.len() + self.token_stack_increase, TokenEntry::default());
        }
        if self.tos < self.token_stack.len() {
            self.token_stack[self.tos] = TokenEntry {
                token_type,
                line,
                token,
            };
        } else {
            self.token_stack.push(TokenEntry {
                token_type,
                line,
                token,
            });
        }
        self.tos += 1;
        if self.tos == self.token_stack.len() {
            self.token_stack
                .resize(self.token_stack.len() + self.token_stack_increase, TokenEntry::default());
        }
    }

    pub fn cleanup_stack(&mut self) {
        let delta = self.tos as isize - self.curs as isize;
        let kept = if delta > 0 { delta as usize } else { 0 };
        if self.curs < self.tos {
            let remaining: Vec<_> = self.token_stack[self.curs..self.tos].to_vec();
            for (i, tok) in remaining.into_iter().enumerate() {
                self.token_stack[i] = tok;
            }
        }
        self.tos = kept;
        self.curs = 0;
    }

    pub fn clearstack(&mut self) {
        self.tos = 0;
        self.curs = 0;
    }

    pub fn nexttoken(&mut self) -> i32 {
        if self.curs == self.tos {
            let token_type = self.lexer.get_token();
            let token = if self.lexer.yytext().is_empty() {
                None
            } else {
                Some(self.lexer.yytext().to_string())
            };
            self.tokpush(token_type, self.lexer.line_num() as i32, token);
        }
        self.tok = self.token_stack[self.curs].clone();
        self.curs += 1;
        self.debugtoken(Some(&self.tok), format_args!("next token"));
        self.tok.token_type
    }

    pub fn putback(&mut self) -> i32 {
        assert!(self.curs != 0, "INTERNAL ERROR: cannot return token to stream");
        self.curs -= 1;
        if self.curs > 0 {
            self.tok = self.token_stack[self.curs - 1].clone();
        } else {
            self.tok.token_type = 0;
        }
        self.debugtoken(Some(&self.tok), format_args!("putback"));
        self.tok.token_type
    }

    pub fn init_parse(&mut self) {
        self.text_stk.clear();
        self.token_stack = vec![TokenEntry::default(); self.token_stack.len().max(128)];
        self.clearstack();
    }

    pub fn save_token(&mut self, tokptr: &TokenEntry) {
        match tokptr.token_type {
            IDENTIFIER | TYPE | STRUCT | PARM_WRAPPER | WORD | QUALIFIER => {
                if self.need_space {
                    self.text_stk.push(' ');
                }
                if let Some(token) = &tokptr.token {
                    self.text_stk.push_str(token);
                }
                self.need_space = true;
            }
            MODIFIER => {
                if self.need_space {
                    self.text_stk.push(' ');
                }
                let token = tokptr.token.as_deref().unwrap_or("");
                self.need_space = !token.starts_with('*');
                self.text_stk.push_str(token);
            }
            EXTERN | STATIC => {}
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
            x if x == '[' as i32 => {
                self.text_stk.push('[');
                self.need_space = false;
            }
            x if x == ']' as i32 => {
                self.text_stk.push(']');
                self.need_space = false;
            }
            LBRACE | LBRACE0 => {
                if self.need_space {
                    self.text_stk.push(' ');
                }
                self.text_stk.push('{');
                self.need_space = true;
            }
            RBRACE | RBRACE0 => {
                if self.need_space {
                    self.text_stk.push(' ');
                }
                self.text_stk.push('}');
                self.need_space = true;
            }
            OP => {
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
        let mut pos = 0;
        self.mark(&mut pos);
        self.start_pos = pos;
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

        for i in 0..self.save_end.max(0) as usize {
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
                IDENTIFIER => {
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

    pub fn push_balance_state(&self, stack: &mut Vec<BalanceState>, idx: i32, level: i32) {
        stack.push(BalanceState { idx, level });
    }

    pub fn pop_balance_state(&self, stack: &mut Vec<BalanceState>) -> Option<(i32, i32)> {
        stack.pop().map(|s| (s.idx, s.level))
    }

    pub fn free_balance_stack(&self, stack: &mut Vec<BalanceState>) {
        stack.clear();
    }

    pub fn find_closing_paren(&mut self, open_tok: i32, mut level: i32) -> i32 {
        let opens = ['(', '[', '{'];
        let closes = [')', ']', '}'];
        let idx0 = opens.iter().position(|&c| c as i32 == open_tok);
        let Some(mut idx) = idx0.map(|v| v as i32) else {
            panic!("INTERNAL ERROR: malformed call to find_closing_paren");
        };
        let mut stack: Vec<BalanceState> = Vec::new();

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

            if self.tok.token_type == opens[idx as usize] as i32 {
                level += 1;
            } else if self.tok.token_type == closes[idx as usize] as i32 {
                level -= 1;
                if level == 0 {
                    if let Some((prev_idx, prev_level)) = self.pop_balance_state(&mut stack) {
                        idx = prev_idx;
                        level = prev_level;
                    } else {
                        return 0;
                    }
                }
            } else if let Some(new_idx) = opens
                .iter()
                .position(|&c| c as i32 == self.tok.token_type)
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
            identifier.storage = StorageClass::Extern;
            match self.tok.token_type {
                0 => return 0,
                QUALIFIER => continue,
                TYPEDEF => self.parse_typedef(),
                DECLARATION => self.skip_declaration(),
                EXTERN => {
                    identifier.storage = StorageClass::ExplicitExtern;
                    self.nexttoken();
                    self.parse_declaration(&mut identifier, false);
                }
                STATIC => {
                    identifier.storage = StorageClass::Static;
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
        let mut sp = 0;
        let mut res = false;

        self.mark(&mut sp);
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
                x if x == '(' as i32 => {
                    res = self.nexttoken() != MODIFIER;
                }
                _ => {}
            }
            break;
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
                x if x == ';' as i32 => return,
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
                x if x == ',' as i32 => {
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
                x if x == '(' as i32 => {
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
                x if x == ')' as i32 => parens_lev -= 1,
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
                x if x == ')' as i32 && parm => break,
                x if x == ';' as i32 || x == ',' as i32 => break,
                LBRACE0 | LBRACE => {
                    if let Some(name) = ident.name.clone() {
                        self.caller = Some(name);
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
            let tok = self.tok.token.as_deref().unwrap_or("");
            if tok == "const" || tok == "volatile" {
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
            if matches!(self.tok.token_type, IDENTIFIER | MODIFIER | QUALIFIER) {
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
        let mut sp = 0;
        self.mark(&mut sp);
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
                    let line = self.tok.line;
                    self.curs = sp;
                    self.tokdel(self.curs, pos.saturating_sub(1));
                    self.tokins(self.curs, IDENTIFIER, line, "{ ... }");
                    self.debugtoken(Some(&self.tok), format_args!("modified stack"));
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
                x if x == ')' as i32 && parm => break,
                x if x == ';' as i32 => break,
                x if x == ',' as i32 => {
                    if parm {
                        break;
                    }
                    self.tos = ident.type_end.max(0) as usize;
                    self.curs = sp;
                    continue;
                }
                x if x == '=' as i32 => {
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
                        }
                        _ => {
                            self.expression();
                            continue;
                        }
                    }
                    break;
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
            storage: StorageClass::Any,
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
                let mut type_tok;
                loop {
                    type_tok = self.tok.token_type;
                    if self.tok.token_type != IDENTIFIER {
                        break;
                    }
                    self.nexttoken();
                    if self.tok.token_type != IDENTIFIER {
                        type_tok = self.tok.token_type;
                        break;
                    }
                }
                self.putback();
                if !(type_tok == TYPE || type_tok == MODIFIER || type_tok == QUALIFIER) {
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

    pub fn getident(&mut self, idptr: &mut Ident, parm_ptr: &mut Option<bool>) -> i32 {
        let mut pos = 0;
        let mut bc = 0;
        self.mark(&mut pos);

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

        if self.tok.token_type == IDENTIFIER {
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
                idptr.name = name;
                idptr.line = line;
                *parm_ptr = Some(true);
                return 0;
            }
        }
        self.curs = pos;
        1
    }

    pub fn dirdcl(&mut self, idptr: &mut Ident) -> i32 {
        let mut wrapper = false;
        let mut parm_ptr = None;

        if self.tok.token_type == '(' as i32 {
            match self.getident(idptr, &mut parm_ptr) {
                0 => {}
                -1 => return 1,
                1 => {
                    self.dcl(idptr);
                    if self.tok.token_type != ')' as i32 && self.verbose {
                        self.file_error("expected `)'", Some(&self.tok));
                        return 1;
                    }
                }
                _ => {}
            }
        } else if self.tok.token_type == IDENTIFIER {
            idptr.name = self.tok.token.clone();
            idptr.line = self.tok.line;
            parm_ptr = Some(true);
        }

        if self.nexttoken() == PARM_WRAPPER {
            wrapper = true;
            self.nexttoken();
        } else {
            self.putback();
        }

        while self.nexttoken() == '[' as i32 || self.tok.token_type == '(' as i32 {
            if self.tok.token_type == '[' as i32 {
                if self.find_closing_paren('[' as i32, 1) == -1 {
                    self.file_error("unexpected end of file in declaration", None);
                    return -1;
                }
            } else {
                self.maybe_parm_list(if parm_ptr.is_some() {
                    Some(&mut idptr.parmcnt)
                } else {
                    None
                });
                if self.tok.token_type != ')' as i32 && self.verbose {
                    self.file_error("expected `)'", Some(&self.tok));
                    return 1;
                }
            }
        }
        if wrapper {
            self.nexttoken();
        }

        while self.tok.token_type == PARM_WRAPPER {
            if self.skip_balanced('(' as i32, 0) == -1 {
                self.file_error("unexpected end of file in function declaration", None);
            }
        }

        0
    }

    pub fn parmdcl(&mut self, idptr: &mut Ident) -> i32 {
        while self.nexttoken() != 0 && self.tok.token_type != '(' as i32 {
            if self.tok.token_type == MODIFIER {
                if idptr.type_end == -1 {
                    idptr.type_end = self.curs as isize - 1;
                }
            } else if self.tok.token_type == IDENTIFIER {
                while self.tok.token_type == IDENTIFIER {
                    self.nexttoken();
                }
                let type_tok = self.tok.token_type;
                self.putback();
                if type_tok != MODIFIER {
                    break;
                }
            } else if self.tok.token_type == ')' as i32 || self.tok.token_type == ',' as i32 {
                return 0;
            }
        }
        if idptr.type_end == -1 {
            idptr.type_end = self.curs as isize - 1;
        }
        self.dirdcl(idptr)
    }

    pub fn maybe_parm_list(&mut self, parm_cnt_return: Option<&mut i32>) {
        let mut parmcnt = 0;
        let mut
first = true;

        if self.nexttoken() == ')' as i32 {
            if let Some(out) = parm_cnt_return {
                *out = 0;
            }
            return;
        }

        loop {
            if !first && self.tok.token_type != ',' as i32 {
                break;
            }
            first = false;

            let mut ident = Ident {
                name: None,
                type_end: -1,
                parmcnt: -1,
                line: -1,
                storage: StorageClass::Auto,
            };

            self.nexttoken();
            if self.parmdcl(&mut ident) != 0 {
                break;
            }
            parmcnt += 1;

            if self.tok.token_type == ')' as i32 {
                break;
            }
            if self.tok.token_type != ',' as i32 {
                break;
            }
        }

        if let Some(out) = parm_cnt_return {
            *out = parmcnt;
        }
    }

    pub fn func_body(&mut self) {
        let mut level = 1;
        self.level += 1;

        loop {
            let tok = self.nexttoken();
            match tok {
                0 => {
                    if self.verbose {
                        self.file_error("unexpected end of file in function body", None);
                    }
                    break;
                }
                LBRACE | LBRACE0 => {
                    level += 1;
                }
                RBRACE | RBRACE0 => {
                    level -= 1;
                    if level == 0 {
                        break;
                    }
                }
                TYPEDEF => self.parse_typedef(),
                EXTERN | STATIC | TYPE | STRUCT | UNION | ENUM | IDENTIFIER | MODIFIER | QUALIFIER => {
                    let mut ident = Ident {
                        name: None,
                        type_end: -1,
                        parmcnt: -1,
                        line: -1,
                        storage: if tok == STATIC {
                            StorageClass::Static
                        } else {
                            StorageClass::Auto
                        },
                    };
                    if tok == EXTERN {
                        ident.storage = StorageClass::ExplicitExtern;
                    }
                    self.parse_variable_declaration(&mut ident, false);
                    self.cleanup_stack();
                }
                _ => self.expression(),
            }
        }

        self.cleanup_stack();
        self.level = self.level.saturating_sub(1);
    }

    pub fn get_knr_args(&mut self, ident: &mut Ident) -> i32 {
        if ident.parmcnt <= 0 {
            return 0;
        }
        ident.parmcnt
    }

    pub fn declare(&mut self, ident: &mut Ident, maybe_knr: bool) {
        if ident.name.is_none() {
            self.undo_save_stack();
            return;
        }

        let name = ident.name.clone().unwrap_or_default();
        let declaration = if self.save_stack_is_empty() {
            None
        } else {
            Some(self.finish_save_stack(&name))
        };

        let knr_args = if maybe_knr { self.get_knr_args(ident) } else { 0 };
        let source = self.filename.clone();
        let level = self.level;
        let symbol = self.get_symbol(&name);

        symbol.storage = ident.storage.clone();
        symbol.def_line = Some(ident.line);
        symbol.source = Some(source);
        symbol.decl = declaration;
        symbol.arity = if ident.parmcnt >= 0 {
            ident.parmcnt
        } else {
            knr_args
        };
        symbol.level = level;
        symbol.symbol_type = SymbolType::Identifier;
        if matches!(ident.storage, StorageClass::Auto) {
            symbol.flag = SymbolFlag::Local;
        }
    }

    pub fn declare_type(&mut self, ident: &Ident) {
        if let Some(name) = &ident.name {
            let source = self.filename.clone();
            let symbol = self.get_symbol(name);
            symbol.symbol_type = SymbolType::Token;
            symbol.storage = StorageClass::Extern;
            symbol.token_type = Some(TYPE);
            symbol.def_line = Some(ident.line);
            symbol.source = Some(source);
        }
    }

    pub fn get_symbol(&mut self, name: &str) -> &mut SymbolRecord {
        if let Some(idx) = self.symbols.iter().position(|s| s.name == name) {
            return &mut self.symbols[idx];
        }
        self.symbols.push(SymbolRecord {
            name: name.to_string(),
            ..Default::default()
        });
        self.symbols.last_mut().expect("symbol just pushed")
    }

    pub fn add_reference(&mut self, name: &str, line: i32) -> &mut SymbolRecord {
        let source = self.filename.clone();
        let symbol = self.get_symbol(name);
        symbol.references.push((source, line));
        symbol
    }

    pub fn call(&mut self, name: &str, line: i32) {
        let callee_name = {
            let callee = self.add_reference(name, line);
            callee.name.clone()
        };

        if let Some(caller_name) = self.caller.clone() {
            {
                let caller = self.get_symbol(&caller_name);
                if !caller.callees.iter().any(|n| n == &callee_name) {
                    caller.callees.push(callee_name.clone());
                }
            }
            let callee = self.get_symbol(&callee_name);
            if !callee.callers.iter().any(|n| n == &caller_name) {
                callee.callers.push(caller_name);
            }
        }
    }

    pub fn reference(&mut self, name: &str, line: i32) {
        let referenced_name = {
            let sym = self.add_reference(name, line);
            sym.name.clone()
        };

        if let Some(caller_name) = self.caller.clone() {
            let caller = self.get_symbol(&caller_name);
            if !caller.callees.iter().any(|n| n == &referenced_name) {
                caller.callees.push(referenced_name);
            }
        }
    }

    pub fn reset_static_caller(&mut self) {
        if let Some(caller_name) = self.caller.clone() {
            let should_reset = self
                .symbols
                .iter()
                .find(|s| s.name == caller_name)
                .map(|s| s.storage == StorageClass::Static || s.flag == SymbolFlag::Local)
                .unwrap_or(false);
            if should_reset {
                self.caller = None;
            }
        }
    }

    pub fn balance_state(idx: i32, level: i32) -> BalanceState {
        BalanceState { idx, level }
    }

    pub fn module_src_save_stack_14(&self) -> (isize, usize, bool) {
        (self.save_end, self.start_pos, self.need_space)
    }

    pub fn module_src_balance_state_08(&self) -> BalanceState {
        BalanceState::default()
    }
}
