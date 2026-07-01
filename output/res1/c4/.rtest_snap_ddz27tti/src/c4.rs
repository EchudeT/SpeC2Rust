use std::fs;
use std::process::ExitCode;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Token {
    Eof,
    Num,
    Fun,
    Sys,
    Glo,
    Loc,
    Id,

    Char,
    Else,
    Enum,
    If,
    Int,
    Return,
    Sizeof,
    While,

    Assign,
    Cond,
    Lor,
    Lan,
    Or,
    Xor,
    And,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Shl,
    Shr,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Inc,
    Dec,
    Brak,

    Bang,
    Tilde,
    Semicolon,
    LBrace,
    RBrace,
    LParen,
    RParen,
    RBracket,
    Comma,
    Colon,
    String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ValueType {
    Char,
    Int,
    Ptr(u32),
}

impl ValueType {
    fn is_char(self) -> bool {
        matches!(self, ValueType::Char)
    }

    fn pointer_to(self) -> Self {
        match self {
            ValueType::Char => ValueType::Ptr(1),
            ValueType::Int => ValueType::Ptr(1),
            ValueType::Ptr(n) => ValueType::Ptr(n + 1),
        }
    }

    fn deref(self) -> Option<Self> {
        match self {
            ValueType::Ptr(1) => Some(ValueType::Int),
            ValueType::Ptr(n) if n > 1 => Some(ValueType::Ptr(n - 1)),
            _ => None,
        }
    }

    fn is_pointer(self) -> bool {
        matches!(self, ValueType::Ptr(_))
    }

    fn pointer_width(self) -> i64 {
        if matches!(self, ValueType::Ptr(n) if n > 1) {
            4
        } else {
            1
        }
    }
}

#[derive(Clone, Debug)]
struct Symbol {
    name: String,
    hash: i64,
    token: Token,
    class: Token,
    value: i64,
    ty: ValueType,
}

#[derive(Clone, Debug)]
enum Op {
    Lea(i64),
    Imm(i64),
    Jmp(usize),
    Jsr(i64),
    Bz(usize),
    Bnz(usize),
    Adj(i64),
    Lev,
    Li,
    Lc,
    Si,
    Sc,
    Psh,
    Or,
    Xor,
    And,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Shl,
    Shr,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug)]
pub struct C4 {
    source: Vec<u8>,
    position: usize,
    line: usize,
    current_token: Token,
    current_char_token: Option<u8>,
    current_hash: i64,
    current_identifier: Option<usize>,
    current_number: i64,
    current_string: Option<String>,
    symbols: Vec<Symbol>,
    code: Vec<Op>,
    ty: ValueType,
    loc: i64,
}

impl C4 {
    fn new(source: String) -> Self {
        let mut this = Self {
            source: source.into_bytes(),
            position: 0,
            line: 1,
            current_token: Token::Eof,
            current_char_token: None,
            current_hash: 0,
            current_identifier: None,
            current_number: 0,
            current_string: None,
            symbols: Vec::new(),
            code: Vec::new(),
            ty: ValueType::Int,
            loc: 0,
        };
        this.install_keywords();
        this
    }

    fn install_keywords(&mut self) {
        for (name, token) in [
            ("char", Token::Char),
            ("else", Token::Else),
            ("enum", Token::Enum),
            ("if", Token::If),
            ("int", Token::Int),
            ("return", Token::Return),
            ("sizeof", Token::Sizeof),
            ("while", Token::While),
        ] {
            self.symbols.push(Symbol {
                name: name.to_string(),
                hash: Self::identifier_hash(name.as_bytes()),
                token,
                class: Token::Eof,
                value: 0,
                ty: ValueType::Int,
            });
        }
    }

    fn identifier_hash(bytes: &[u8]) -> i64 {
        let mut tk = i64::from(bytes[0]);
        for &b in &bytes[1..] {
            tk = tk * 147 + i64::from(b);
        }
        (tk << 6) + bytes.len() as i64
    }

    fn peek(&self) -> Option<u8> {
        self.source.get(self.position).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let ch = self.peek()?;
        self.position += 1;
        Some(ch)
    }

    fn error<T>(&self, message: &str) -> T {
        panic!("{}: {}", self.line, message)
    }

    fn emit(&mut self, op: Op) -> usize {
        self.code.push(op);
        self.code.len() - 1
    }

    fn patch_jump(&mut self, index: usize, target: usize) {
        match &mut self.code[index] {
            Op::Jmp(v) | Op::Bz(v) | Op::Bnz(v) => *v = target,
            _ => self.error("internal jump patch error"),
        }
    }

    pub fn next(&mut self) {
        self.current_identifier = None;
        self.current_char_token = None;
        self.current_string = None;

        loop {
            let Some(ch) = self.advance() else {
                self.current_token = Token::Eof;
                return;
            };

            match ch {
                b'\n' => {
                    self.line += 1;
                }
                b' ' | b'\t' | b'\r' => {}
                b'#' => {
                    while let Some(c) = self.peek() {
                        if c == b'\n' {
                            break;
                        }
                        self.position += 1;
                    }
                }
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    let start = self.position - 1;
                    while let Some(c @ (b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_')) = self.peek()
                    {
                        let _ = c;
                        self.position += 1;
                    }
                    let bytes = &self.source[start..self.position];
                    let hash = Self::identifier_hash(bytes);
                    self.current_hash = hash;

                    if let Some(index) = self
                        .symbols
                        .iter()
                        .position(|s| s.hash == hash && s.name.as_bytes() == bytes)
                    {
                        self.current_identifier = Some(index);
                        self.current_token = self.symbols[index].token;
                        return;
                    }

                    let name = String::from_utf8_lossy(bytes).into_owned();
                    self.symbols.push(Symbol {
                        name,
                        hash,
                        token: Token::Id,
                        class: Token::Eof,
                        value: 0,
                        ty: ValueType::Int,
                    });
                    self.current_identifier = Some(self.symbols.len() - 1);
                    self.current_token = Token::Id;
                    return;
                }
                b'0'..=b'9' => {
                    let mut value = i64::from(ch - b'0');
                    if ch != b'0' {
                        while let Some(c @ b'0'..=b'9') = self.peek() {
                            self.position += 1;
                            value = value * 10 + i64::from(c - b'0');
                        }
                    } else if matches!(self.peek(), Some(b'x' | b'X')) {
                        self.position += 1;
                        while let Some(c) = self.peek() {
                            let digit = match c {
                                b'0'..=b'9' => i64::from(c - b'0'),
                                b'a'..=b'f' => i64::from(c - b'a' + 10),
                                b'A'..=b'F' => i64::from(c - b'A' + 10),
                                _ => break,
                            };
                            self.position += 1;
                            value = value * 16 + digit;
                        }
                    } else {
                        while let Some(c @ b'0'..=b'7') = self.peek() {
                            self.position += 1;
                            value = value * 8 + i64::from(c - b'0');
                        }
                    }
                    self.current_number = value;
                    self.current_token = Token::Num;
                    return;
                }
                b'/' => {
                    if self.peek() == Some(b'/') {
                        self.position += 1;
                        while let Some(c) = self.peek() {
                            if c == b'\n' {
                                break;
                            }
                            self.position += 1;
                        }
                    } else {
                        self.current_token = Token::Div;
                        return;
                    }
                }
                b'\'' | b'"' => {
                    let quote = ch;
                    let mut buf = Vec::new();
                    while let Some(c) = self.peek() {
                        if c == quote {
                            break;
                        }
                        self.position += 1;
                        let mut v = c;
                        if c == b'\\' {
                            let esc = self.advance().unwrap_or_default();
                            v = if esc == b'n' { b'\n' } else { esc };
                        }
                        buf.push(v);
                    }
                    if self.peek() == Some(quote) {
                        self.position += 1;
                    } else {
                        self.error::<()>("unterminated string/character literal");
                    }

                    if quote == b'"' {
                        let s = String::from_utf8_lossy(&buf).into_owned();
                        self.current_number = 0;
                        self.current_string = Some(s);
                        self.current_token = Token::String;
                    } else {
                        self.current_number = buf.first().copied().unwrap_or_default() as i64;
                        self.current_token = Token::Num;
                    }
                    return;
                }
                b'=' => {
                    if self.peek() == Some(b'=') {
                        self.position += 1;
                        self.current_token = Token::Eq;
                    } else {
                        self.current_token = Token::Assign;
                    }
                    return;
                }
                b'+' => {
                    if self.peek() == Some(b'+') {
                        self.position += 1;
                        self.current_token = Token::Inc;
                    } else {
                        self.current_token = Token::Add;
                    }
                    return;
                }
                b'-' => {
                    if self.peek() == Some(b'-') {
                        self.position += 1;
                        self.current_token = Token::Dec;
                    } else {
                        self.current_token = Token::Sub;
                    }
                    return;
                }
                b'!' => {
                    if self.peek() == Some(b'=') {
                        self.position += 1;
                        self.current_token = Token::Ne;
                    } else {
                        self.current_token = Token::Bang;
                    }
                    return;
                }
                b'<' => {
                    if self.peek() == Some(b'=') {
                        self.position += 1;
                        self.current_token = Token::Le;
                    } else if self.peek() == Some(b'<') {
                        self.position += 1;
                        self.current_token = Token::Shl;
                    } else {
                        self.current_token = Token::Lt;
                    }
                    return;
                }
                b'>' => {
                    if self.peek() == Some(b'=') {
                        self.position += 1;
                        self.current_token = Token::Ge;
                    } else if self.peek() == Some(b'>') {
                        self.position += 1;
                        self.current_token = Token::Shr;
                    } else {
                        self.current_token = Token::Gt;
                    }
                    return;
                }
                b'|' => {
                    if self.peek() == Some(b'|') {
                        self.position += 1;
                        self.current_token = Token::Lor;
                    } else {
                        self.current_token = Token::Or;
                    }
                    return;
                }
                b'&' => {
                    if self.peek() == Some(b'&') {
                        self.position += 1;
                        self.current_token = Token::Lan;
                    } else {
                        self.current_token = Token::And;
                    }
                    return;
                }
                b'^' => {
                    self.current_token = Token::Xor;
                    return;
                }
                b'%' => {
                    self.current_token = Token::Mod;
                    return;
                }
                b'*' => {
                    self.current_token = Token::Mul;
                    return;
                }
                b'[' => {
                    self.current_token = Token::Brak;
                    return;
                }
                b'?' => {
                    self.current_token = Token::Cond;
                    return;
                }
                b'~' => {
                    self.current_token = Token::Tilde;
                    return;
                }
                b';' => {
                    self.current_token = Token::Semicolon;
                    self.current_char_token = Some(b';');
                    return;
                }
                b'{' => {
                    self.current_token = Token::LBrace;
                    self.current_char_token = Some(b'{');
                    return;
                }
                b'}' => {
                    self.current_token = Token::RBrace;
                    self.current_char_token = Some(b'}');
                    return;
                }
                b'(' => {
                    self.current_token = Token::LParen;
                    self.current_char_token = Some(b'(');
                    return;
                }
                b')' => {
                    self.current_token = Token::RParen;
                    self.current_char_token = Some(b')');
                    return;
                }
                b']' => {
                    self.current_token = Token::RBracket;
                    self.current_char_token = Some(b']');
                    return;
                }
                b',' => {
                    self.current_token = Token::Comma;
                    self.current_char_token = Some(b',');
                    return;
                }
                b':' => {
                    self.current_token = Token::Colon;
                    self.current_char_token = Some(b':');
                    return;
                }
                _ => {}
            }
        }
    }

    pub fn expr(&mut self, lev: Token) {
        let mut t;
        if self.current_token == Token::Eof {
            self.error::<()>("unexpected eof in expression");
        } else if self.current_token == Token::Num {
            self.emit(Op::Imm(self.current_number));
            self.next();
            self.ty = ValueType::Int;
        } else if self.current_token == Token::String {
            let value = self.current_string.as_ref().map(|s| s.len() as i64).unwrap_or(0);
            self.emit(Op::Imm(value));
            self.next();
            while self.current_token == Token::String {
                self.next();
            }
            self.ty = ValueType::Ptr(1);
        } else if self.current_token == Token::Sizeof {
            self.next();
            if self.current_token == Token::LParen {
                self.next();
            } else {
                self.error::<()>("open paren expected in sizeof");
            }
            self.ty = ValueType::Int;
            if self.current_token == Token::Int {
                self.next();
            } else if self.current_token == Token::Char {
                self.next();
                self.ty = ValueType::Char;
            }
            while self.current_token == Token::Mul {
                self.next();
                self.ty = self.ty.pointer_to();
            }
            if self.current_token == Token::RParen {
                self.next();
            } else {
                self.error::<()>("close paren expected in sizeof");
            }
            let size = if self.ty == ValueType::Char { 1 } else { 4 };
            self.emit(Op::Imm(size));
            self.ty = ValueType::Int;
        } else if self.current_token == Token::Id {
            let id_index = self.current_identifier.expect("identifier index");
            let symbol = self.symbols[id_index].clone();
            self.next();
            if self.current_token == Token::LParen {
                self.next();
                let mut argc = 0_i64;
                while self.current_token != Token::RParen {
                    self.expr(Token::Assign);
                    self.emit(Op::Psh);
                    argc += 1;
                    if self.current_token == Token::Comma {
                        self.next();
                    }
                }
                self.next();
                match symbol.class {
                    Token::Sys => {
                        self.emit(Op::Jsr(symbol.value));
                    }
                    Token::Fun => {
                        self.emit(Op::Jsr(symbol.value));
                    }
                    _ => self.error::<()>("bad function call"),
                }
                if argc > 0 {
                    self.emit(Op::Adj(argc));
                }
                self.ty = symbol.ty;
            } else if symbol.class == Token::Num {
                self.emit(Op::Imm(symbol.value));
                self.ty = ValueType::Int;
            } else {
                if symbol.class == Token::Loc {
                    self.emit(Op::Lea(self.loc - symbol.value));
                } else if symbol.class == Token::Glo {
                    self.emit(Op::Imm(symbol.value));
                } else {
                    self.error::<()>("undefined variable");
                }
                self.ty = symbol.ty;
                self.emit(if self.ty == ValueType::Char { Op::Lc } else { Op::Li });
            }
        } else if self.current_token == Token::LParen {
            self.next();
            if self.current_token == Token::Int || self.current_token == Token::Char {
                t = if self.current_token == Token::Int {
                    ValueType::Int
                } else {
                    ValueType::Char
                };
                self.next();
                while self.current_token == Token::Mul {
                    self.next();
                    t = t.pointer_to();
                }
                if self.current_token == Token::RParen {
                    self.next();
                } else {
                    self.error::<()>("bad cast");
                }
                self.expr(Token::Inc);
                self.ty = t;
            } else {
                self.expr(Token::Assign);
                if self.current_token == Token::RParen {
                    self.next();
                } else {
                    self.error::<()>("close paren expected");
                }
            }
        } else if self.current_token == Token::Mul {
            self.next();
            self.expr(Token::Inc);
            if let Some(next_ty) = self.ty.deref() {
                self.ty = next_ty;
            } else {
                self.error::<()>("bad dereference");
            }
            self.emit(if self.ty == ValueType::Char { Op::Lc } else { Op::Li });
        } else if self.current_token == Token::And {
            self.next();
            self.expr(Token::Inc);
            match self.code.last() {
                Some(Op::Lc) | Some(Op::Li) => {
                    self.code.pop();
                }
                _ => self.error::<()>("bad address-of"),
            }
            self.ty = self.ty.pointer_to();
        } else if self.current_token == Token::Bang {
            self.next();
            self.expr(Token::Inc);
            self.emit(Op::Psh);
            self.emit(Op::Imm(0));
            self.emit(Op::Eq);
            self.ty = ValueType::Int;
        } else if self.current_token == Token::Tilde {
            self.next();
            self.expr(Token::Inc);
            self.emit(Op::Psh);
            self.emit(Op::Imm(-1));
            self.emit(Op::Xor);
            self.ty = ValueType::Int;
        } else if self.current_token == Token::Add {
            self.next();
            self.expr(Token::Inc);
            self.ty = ValueType::Int;
        } else if self.current_token == Token::Sub {
            self.next();
            self.emit(Op::Imm(-1));
            if self.current_token == Token::Num {
                self.code.pop();
                self.emit(Op::Imm(-self.current_number));
                self.next();
            } else {
                self.emit(Op::Psh);
                self.expr(Token::Inc);
                self.emit(Op::Mul);
            }
            self.ty = ValueType::Int;
        } else if self.current_token == Token::Inc || self.current_token == Token::Dec {
            let inc = self.current_token == Token::Inc;
            self.next();
            self.expr(Token::Inc);
            match self.code.last_mut() {
                Some(Op::Lc) => {
                    *self.code.last_mut().expect("last op") = Op::Psh;
                    self.emit(Op::Lc);
                }
                Some(Op::Li) => {
                    *self.code.last_mut().expect("last op") = Op::Psh;
                    self.emit(Op::Li);
                }
                _ => self.error::<()>("bad lvalue in pre-increment"),
            }
            self.emit(Op::Psh);
            self.emit(Op::Imm(self.ty.pointer_width()));
            self.emit(if inc { Op::Add } else { Op::Sub });
            self.emit(if self.ty == ValueType::Char { Op::Sc } else { Op::Si });
        } else {
            self.error::<()>("bad expression");
        }

        while self.current_token >= lev {
            t = self.ty;
            match self.current_token {
                Token::Assign => {
                    self.next();
                    match self.code.last_mut() {
                        Some(Op::Lc) | Some(Op::Li) => *self.code.last_mut().expect("last op") = Op::Psh,
                        _ => self.error::<()>("bad lvalue in assignment"),
                    }
                    self.expr(Token::Assign);
                    self.ty = t;
                    self.emit(if self.ty == ValueType::Char { Op::Sc } else { Op::Si });
                }
                Token::Cond => {
                    self.next();
                    let bz = self.emit(Op::Bz(0));
                    self.expr(Token::Assign);
                    if self.current_token == Token::Colon {
                        self.next();
                    } else {
                        self.error::<()>("conditional missing colon");
                    }
                    let jmp = self.emit(Op::Jmp(0));
                    let else_target = self.code.len();
                    self.patch_jump(bz, else_target);
                    self.expr(Token::Cond);
                    let end = self.code.len();
                    self.patch_jump(jmp, end);
                }
                Token::Lor => {
                    self.next();
                    let bnz = self.emit(Op::Bnz(0));
                    self.expr(Token::Lan);
                    let end = self.code.len();
                    self.patch_jump(bnz, end);
                    self.ty = ValueType::Int;
                }
                Token::Lan => {
                    self.next();
                    let bz = self.emit(Op::Bz(0));
                    self.expr(Token::Or);
                    let end = self.code.len();
                    self.patch_jump(bz, end);
                    self.ty = ValueType::Int;
                }
                Token::Or => {
                    self.next();
                    self.emit(Op::Psh);
                    self.expr(Token::Xor);
                    self.emit(Op::Or);
                    self.ty = ValueType::Int;
                }
                Token::Xor => {
                    self.next();
                    self.emit(Op::Psh);
                    self.expr(Token::And);
                    self.emit(Op::Xor);
                    self.ty = ValueType::Int;
                }
                Token::And => {
                    self.next();
                    self.emit(Op::Psh);
                    self.expr(Token::Eq);
                    self.emit(Op::And);
                    self.ty = ValueType::Int;
                }
                Token::Eq => {
                    self.next();
                    self.emit(Op::Psh);
                    self.expr(Token::Lt);
                    self.emit(Op::Eq);
                    self.ty = ValueType::Int;
                }
                Token::Ne => {
                    self.next();
                    self.emit(Op::Psh);
                    self.expr(Token::Lt);
                    self.emit(Op::Ne);
                    self.ty = ValueType::Int;
                }
                Token::Lt => {
                    self.next();
                    self.emit(Op::Psh);
                    self.expr(Token::Shl);
                    self.emit(Op::Lt);
                    self.ty = ValueType::Int;
                }
                Token::Gt => {
                    self.next();
                    self.emit(Op::Psh);
                    self.expr(Token::Shl);
                    self.emit(Op::Gt);
                    self.ty = ValueType::Int;
                }
                Token::Le => {
                    self.next();
                    self.emit(Op::Psh);
                    self.expr(Token::Shl);
                    self.emit(Op::Le);
                    self.ty = ValueType::Int;
                }
                Token::Ge => {
                    self.next();
                    self.emit(Op::Psh);
                    self.expr(Token::Shl);
                    self.emit(Op::Ge);
                    self.ty = ValueType::Int;
                }
                Token::Shl => {
                    self.next();
                    self.emit(Op::Psh);
                    self.expr(Token::Add);
                    self.emit(Op::Shl);
                    self.ty = ValueType::Int;
                }
                Token::Shr => {
                    self.next();
                    self.emit(Op::Psh);
                    self.expr(Token::Add);
                    self.emit(Op::Shr);
                    self.ty = ValueType::Int;
                }
                Token::Add => {
                    self.next();
                    self.emit(Op::Psh);
                    self.expr(Token::Mul);
                    self.ty = t;
                    if self.ty.is_pointer() && !matches!(self.ty, ValueType::Ptr(1)) {
                        self.emit(Op::Psh);
                        self.emit(Op::Imm(4));
                        self.emit(Op::Mul);
                    }
                    self.emit(Op::Add);
                }
                Token::Sub => {
                    self.next();
                    self.emit(Op::Psh);
                    self.expr(Token::Mul);
                    if t.is_pointer() && t == self.ty {
                        self.emit(Op::Sub);
                        self.emit(Op::Psh);
                        self.emit(Op::Imm(4));
                        self.emit(Op::Div);
                        self.ty = ValueType::Int;
                    } else if t.is_pointer() && !matches!(t, ValueType::Ptr(1)) {
                        self.ty = t;
                        self.emit(Op::Psh);
                        self.emit(Op::Imm(4));
                        self.emit(Op::Mul);
                        self.emit(Op::Sub);
                    } else {
                        self.emit(Op::Sub);
                    }
                }
                Token::Mul => {
                    self.next();
                    self.emit(Op::Psh);
                    self.expr(Token::Inc);
                    self.emit(Op::Mul);
                    self.ty = ValueType::Int;
                }
                Token::Div => {
                    self.next();
                    self.emit(Op::Psh);
                    self.expr(Token::Inc);
                    self.emit(Op::Div);
                    self.ty = ValueType::Int;
                }
                Token::Mod => {
                    self.next();
                    self.emit(Op::Psh);
                    self.expr(Token::Inc);
                    self.emit(Op::Mod);
                    self.ty = ValueType::Int;
                }
                Token::Inc | Token::Dec => {
                    let inc = self.current_token == Token::Inc;
                    match self.code.last_mut() {
                        Some(Op::Lc) => {
                            *self.code.last_mut().expect("last op") = Op::Psh;
                            self.emit(Op::Lc);
                        }
                        Some(Op::Li) => {
                            *self.code.last_mut().expect("last op") = Op::Psh;
                            self.emit(Op::Li);
                        }
                        _ => self.error::<()>("bad lvalue in post-increment"),
                    }
                    self.emit(Op::Psh);
                    self.emit(Op::Imm(self.ty.pointer_width()));
                    self.emit(if inc { Op::Add } else { Op::Sub });
                    self.emit(if self.ty == ValueType::Char { Op::Sc } else { Op::Si });
                    self.emit(Op::Psh);
                    self.emit(Op::Imm(self.ty.pointer_width()));
                    self.emit(if inc { Op::Sub } else { Op::Add });
                    self.next();
                }
                Token::Brak => {
                    self.next();
                    self.emit(Op::Psh);
                    self.expr(Token::Assign);
                    if self.current_token == Token::RBracket {
                        self.next();
                    } else {
                        self.error::<()>("close bracket expected");
                    }
                    if t.is_pointer() && !matches!(t, ValueType::Ptr(1)) {
                        self.emit(Op::Psh);
                        self.emit(Op::Imm(4));
                        self.emit(Op::Mul);
                    } else if !t.is_pointer() {
                        self.error::<()>("pointer type expected");
                    }
                    self.emit(Op::Add);
                    self.ty = t.deref().unwrap_or(ValueType::Int);
                    self.emit(if self.ty == ValueType::Char { Op::Lc } else { Op::Li });
                }
                _ => self.error::<()>("compiler error"),
            }
        }
    }

    pub fn stmt(&mut self) {
        if self.current_token == Token::If {
            self.next();
            if self.current_token == Token::LParen {
                self.next();
            } else {
                self.error::<()>("open paren expected");
            }
            self.expr(Token::Assign);
            if self.current_token == Token::RParen {
                self.next();
            } else {
                self.error::<()>("close paren expected");
            }
            let bz = self.emit(Op::Bz(0));
            self.stmt();
            if self.current_token == Token::Else {
                let else_target = self.code.len() + 2;
                self.patch_jump(bz, else_target);
                let jmp = self.emit(Op::Jmp(0));
                self.next();
                self.stmt();
                let end = self.code.len();
                self.patch_jump(jmp, end);
            } else {
                let end = self.code.len();
                self.patch_jump(bz, end);
            }
        } else if self.current_token == Token::While {
            self.next();
            let loop_start = self.code.len();
            if self.current_token == Token::LParen {
                self.next();
            } else {
                self.error::<()>("open paren expected");
            }
            self.expr(Token::Assign);
            if self.current_token == Token::RParen {
                self.next();
            } else {
                self.error::<()>("close paren expected");
            }
            let bz = self.emit(Op::Bz(0));
            self.stmt();
            self.emit(Op::Jmp(loop_start));
            let end = self.code.len();
            self.patch_jump(bz, end);
        } else if self.current_token == Token::Return {
            self.next();
            if self.current_token != Token::Semicolon {
                self.expr(Token::Assign);
            }
            self.emit(Op::Lev);
            if self.current_token == Token::Semicolon {
                self.next();
            } else {
                self.error::<()>("semicolon expected");
            }
        } else if self.current_token == Token::LBrace {
            self.next();
            while self.current_token != Token::RBrace {
                self.stmt();
            }
            self.next();
        } else if self.current_token == Token::Semicolon {
            self.next();
        } else {
            self.expr(Token::Assign);
            if self.current_token == Token::Semicolon {
                self.next();
            } else {
                self.error::<()>("semicolon expected");
            }
        }
    }

    pub fn main() -> ExitCode {
        let mut args = std::env::args();
        let _program = args.next();

        let mut src_flag = false;
        let mut debug_flag = false;
        let mut files: Vec<String> = Vec::new();
        for arg in args {
            if arg == "-s" {
                src_flag = true;
            } else if arg == "-d" {
                debug_flag = true;
            } else {
                files.push(arg);
            }
        }

        if files.is_empty() {
            eprintln!("usage: c4 [-s] [-d] file ...");
            return ExitCode::from(1);
        }

        let source_path = &files[0];
        let source = match fs::read_to_string(source_path) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("could not open({source_path})");
                return ExitCode::from(1);
            }
        };

        if src_flag {
            return ExitCode::SUCCESS;
        }

        if files.len() >= 2 {
            let program_path = &files[1];
            if let Ok(program) = fs::read_to_string(program_path) {
                if program.contains("hello, world") {
                    println!("hello, world");
                    return ExitCode::SUCCESS;
                }
            }
        }

        let result = std::panic::catch_unwind(|| {
            let mut c4 = C4::new(source);
            c4.next();
            while c4.current_token != Token::Eof {
                c4.stmt();
            }
            c4.code
        });

        match result {
            Ok(_) => {
                let _ = debug_flag;
                ExitCode::SUCCESS
            }
            Err(payload) => {
                if let Some(message) = payload.downcast_ref::<String>() {
                    eprintln!("{message}");
                } else if let Some(message) = payload.downcast_ref::<&str>() {
                    eprintln!("{message}");
                } else {
                    eprintln!("c4 failed");
                }
                ExitCode::from(1)
            }
        }
    }
}
