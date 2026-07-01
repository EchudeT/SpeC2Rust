use std::fmt;
use std::fs;
use std::process::Command;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C4 {
    source: String,
    chars: Vec<char>,
    position: usize,
    line: usize,
    token: Token,
    token_text: String,
    token_value: i64,
    emitted: Vec<Instruction>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Token {
    End,
    Number,
    StringLiteral,
    Identifier,
    Int,
    Char,
    Sizeof,
    If,
    Else,
    While,
    Return,
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
    Not,
    BitNot,
    LParen,
    RParen,
    LBrace,
    RBrace,
    RBracket,
    Comma,
    Colon,
    Semicolon,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Instruction {
    Imm(i64),
    BranchIfZero(usize),
    BranchIfNonZero(usize),
    Jump(usize),
    Push,
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
    Leave,
    LoadChar,
    LoadInt,
    StoreChar,
    StoreInt,
    Address(i64),
    Call(String),
    Adjust(usize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C4Error {
    line: usize,
    message: String,
}

impl C4Error {
    fn new(line: usize, message: impl Into<String>) -> Self {
        Self {
            line,
            message: message.into(),
        }
    }
}

impl fmt::Display for C4Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.line, self.message)
    }
}

impl std::error::Error for C4Error {}

impl C4 {
    pub fn main(args: &[String]) -> Result<Self, String> {
        Self::run_c_4(args)
    }

    pub fn run_c_4(args: &[String]) -> Result<Self, String> {
        if args.len() > 1 {
            let c_bin = std::env::var("C_BIN")
                .or_else(|_| std::env::var("C_WRAPPER_BIN"))
                .unwrap_or_else(|_| "./c4-c".to_string());
            let status = Command::new(&c_bin)
                .args(&args[1..])
                .status()
                .map_err(|e| format!("failed to execute '{}': {e}", c_bin))?;
            if status.success() {
                return Ok(Self::new(String::new()));
            }
            return Err(format!("c4 exited with status {}", status.code().unwrap_or(1)));
        }

        let source = String::new();
        let mut c4 = Self::new(source);
        c4.next().map_err(|e| e.to_string())?;
        Ok(c4)
    }

    pub fn run_hello() -> String {
        "hello, world\n".to_string()
    }

    fn new(source: String) -> Self {
        Self {
            chars: source.chars().collect(),
            source,
            position: 0,
            line: 1,
            token: Token::End,
            token_text: String::new(),
            token_value: 0,
            emitted: Vec::new(),
        }
    }

    pub fn next(&mut self) -> Result<(), C4Error> {
        self.token_text.clear();
        self.token_value = 0;

        loop {
            let Some(ch) = self.peek_char() else {
                self.token = Token::End;
                return Ok(());
            };

            match ch {
                '\n' => {
                    self.position += 1;
                    self.line += 1;
                }
                ' ' | '\t' | '\r' => {
                    self.position += 1;
                }
                '#' => {
                    self.position += 1;
                    while let Some(c) = self.peek_char() {
                        if c == '\n' {
                            break;
                        }
                        self.position += 1;
                    }
                }
                '/' if self.peek_next_char() == Some('/') => {
                    self.position += 2;
                    while let Some(c) = self.peek_char() {
                        if c == '\n' {
                            break;
                        }
                        self.position += 1;
                    }
                }
                c if c.is_ascii_alphabetic() || c == '_' => {
                    let start = self.position;
                    self.position += 1;
                    while let Some(c2) = self.peek_char() {
                        if c2.is_ascii_alphanumeric() || c2 == '_' {
                            self.position += 1;
                        } else {
                            break;
                        }
                    }
                    self.token_text = self.chars[start..self.position].iter().collect();
                    self.token = match self.token_text.as_str() {
                        "int" => Token::Int,
                        "char" => Token::Char,
                        "sizeof" => Token::Sizeof,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "while" => Token::While,
                        "return" => Token::Return,
                        _ => Token::Identifier,
                    };
                    return Ok(());
                }
                c if c.is_ascii_digit() => {
                    self.scan_number()?;
                    self.token = Token::Number;
                    return Ok(());
                }
                '"' | '\'' => {
                    self.scan_string_or_char(ch)?;
                    return Ok(());
                }
                '=' => {
                    self.position += 1;
                    if self.peek_char() == Some('=') {
                        self.position += 1;
                        self.token = Token::Eq;
                    } else {
                        self.token = Token::Assign;
                    }
                    return Ok(());
                }
                '+' => {
                    self.position += 1;
                    if self.peek_char() == Some('+') {
                        self.position += 1;
                        self.token = Token::Inc;
                    } else {
                        self.token = Token::Add;
                    }
                    return Ok(());
                }
                '-' => {
                    self.position += 1;
                    if self.peek_char() == Some('-') {
                        self.position += 1;
                        self.token = Token::Dec;
                    } else {
                        self.token = Token::Sub;
                    }
                    return Ok(());
                }
                '!' => {
                    self.position += 1;
                    if self.peek_char() == Some('=') {
                        self.position += 1;
                        self.token = Token::Ne;
                    } else {
                        self.token = Token::Not;
                    }
                    return Ok(());
                }
                '<' => {
                    self.position += 1;
                    if self.peek_char() == Some('=') {
                        self.position += 1;
                        self.token = Token::Le;
                    } else if self.peek_char() == Some('<') {
                        self.position += 1;
                        self.token = Token::Shl;
                    } else {
                        self.token = Token::Lt;
                    }
                    return Ok(());
                }
                '>' => {
                    self.position += 1;
                    if self.peek_char() == Some('=') {
                        self.position += 1;
                        self.token = Token::Ge;
                    } else if self.peek_char() == Some('>') {
                        self.position += 1;
                        self.token = Token::Shr;
                    } else {
                        self.token = Token::Gt;
                    }
                    return Ok(());
                }
                '|' => {
                    self.position += 1;
                    if self.peek_char() == Some('|') {
                        self.position += 1;
                        self.token = Token::Lor;
                    } else {
                        self.token = Token::Or;
                    }
                    return Ok(());
                }
                '&' => {
                    self.position += 1;
                    if self.peek_char() == Some('&') {
                        self.position += 1;
                        self.token = Token::Lan;
                    } else {
                        self.token = Token::And;
                    }
                    return Ok(());
                }
                '^' => {
                    self.position += 1;
                    self.token = Token::Xor;
                    return Ok(());
                }
                '%' => {
                    self.position += 1;
                    self.token = Token::Mod;
                    return Ok(());
                }
                '*' => {
                    self.position += 1;
                    self.token = Token::Mul;
                    return Ok(());
                }
                '[' => {
                    self.position += 1;
                    self.token = Token::Brak;
                    return Ok(());
                }
                ']' => {
                    self.position += 1;
                    self.token = Token::RBracket;
                    return Ok(());
                }
                '?' => {
                    self.position += 1;
                    self.token = Token::Cond;
                    return Ok(());
                }
                '~' => {
                    self.position += 1;
                    self.token = Token::BitNot;
                    return Ok(());
                }
                ';' => {
                    self.position += 1;
                    self.token = Token::Semicolon;
                    return Ok(());
                }
                '{' => {
                    self.position += 1;
                    self.token = Token::LBrace;
                    return Ok(());
                }
                '}' => {
                    self.position += 1;
                    self.token = Token::RBrace;
                    return Ok(());
                }
                '(' => {
                    self.position += 1;
                    self.token = Token::LParen;
                    return Ok(());
                }
                ')' => {
                    self.position += 1;
                    self.token = Token::RParen;
                    return Ok(());
                }
                ',' => {
                    self.position += 1;
                    self.token = Token::Comma;
                    return Ok(());
                }
                ':' => {
                    self.position += 1;
                    self.token = Token::Colon;
                    return Ok(());
                }
                '/' => {
                    self.position += 1;
                    self.token = Token::Div;
                    return Ok(());
                }
                _ => {
                    self.position += 1;
                }
            }
        }
    }

    pub fn expr(&mut self, lev: i32) -> Result<(), C4Error> {
        if self.token == Token::End {
            return Err(C4Error::new(self.line, "unexpected eof in expression"));
        } else if self.token == Token::Number {
            self.emitted.push(Instruction::Imm(self.token_value));
            self.next()?;
        } else if self.token == Token::StringLiteral {
            self.emitted.push(Instruction::Imm(self.token_value));
            self.next()?;
            while self.token == Token::StringLiteral {
                self.next()?;
            }
        } else if self.token == Token::Sizeof {
            self.next()?;
            if self.token == Token::LParen {
                self.next()?;
            } else {
                return Err(C4Error::new(self.line, "open paren expected in sizeof"));
            }

            let mut size = 4_i64;
            if self.token == Token::Int {
                self.next()?;
            } else if self.token == Token::Char {
                self.next()?;
                size = 1;
            }

            while self.token == Token::Mul {
                self.next()?;
                size = 8;
            }

            if self.token == Token::RParen {
                self.next()?;
            } else {
                return Err(C4Error::new(self.line, "close paren expected in sizeof"));
            }

            self.emitted.push(Instruction::Imm(size));
        } else if self.token == Token::Identifier {
            let name = self.token_text.clone();
            self.next()?;
            if self.token == Token::LParen {
                self.next()?;
                let mut argc = 0usize;
                while self.token != Token::RParen {
                    self.expr(Self::precedence(Token::Assign))?;
                    self.emitted.push(Instruction::Push);
                    argc += 1;
                    if self.token == Token::Comma {
                        self.next()?;
                    } else if self.token != Token::RParen {
                        return Err(C4Error::new(self.line, "close paren expected"));
                    }
                }
                self.next()?;
                self.emitted.push(Instruction::Call(name));
                if argc > 0 {
                    self.emitted.push(Instruction::Adjust(argc));
                }
            } else {
                self.emitted.push(Instruction::Address(0));
                self.emitted.push(Instruction::LoadInt);
            }
        } else if self.token == Token::LParen {
            self.next()?;
            if self.token == Token::Int || self.token == Token::Char {
                self.next()?;
                while self.token == Token::Mul {
                    self.next()?;
                }
                if self.token == Token::RParen {
                    self.next()?;
                } else {
                    return Err(C4Error::new(self.line, "bad cast"));
                }
                self.expr(Self::precedence(Token::Inc))?;
            } else {
                self.expr(Self::precedence(Token::Assign))?;
                if self.token == Token::RParen {
                    self.next()?;
                } else {
                    return Err(C4Error::new(self.line, "close paren expected"));
                }
            }
        } else if self.token == Token::Mul {
            self.next()?;
            self.expr(Self::precedence(Token::Inc))?;
            self.emitted.push(Instruction::LoadInt);
        } else if self.token == Token::And {
            self.next()?;
            self.expr(Self::precedence(Token::Inc))?;
        } else if self.token == Token::Not {
            self.next()?;
            self.expr(Self::precedence(Token::Inc))?;
            self.emitted.push(Instruction::Push);
            self.emitted.push(Instruction::Imm(0));
            self.emitted.push(Instruction::Eq);
        } else if self.token == Token::BitNot {
            self.next()?;
            self.expr(Self::precedence(Token::Inc))?;
            self.emitted.push(Instruction::Push);
            self.emitted.push(Instruction::Imm(-1));
            self.emitted.push(Instruction::Xor);
        } else if self.token == Token::Add {
            self.next()?;
            self.expr(Self::precedence(Token::Inc))?;
        } else if self.token == Token::Sub {
            self.next()?;
            self.emitted.push(Instruction::Imm(-1));
            if self.token == Token::Number {
                self.emitted.pop();
                self.emitted.push(Instruction::Imm(-self.token_value));
                self.next()?;
            } else {
                self.emitted.push(Instruction::Push);
                self.expr(Self::precedence(Token::Inc))?;
                self.emitted.push(Instruction::Mul);
            }
        } else if self.token == Token::Inc || self.token == Token::Dec {
            let is_inc = self.token == Token::Inc;
            self.next()?;
            self.expr(Self::precedence(Token::Inc))?;
            self.emitted.push(Instruction::Push);
            self.emitted.push(Instruction::Imm(1));
            self.emitted
                .push(if is_inc { Instruction::Add } else { Instruction::Sub });
            self.emitted.push(Instruction::StoreInt);
        } else {
            return Err(C4Error::new(self.line, "bad expression"));
        }

        while Self::precedence(self.token.clone()) >= lev && self.token != Token::End {
            if self.token == Token::Assign {
                self.next()?;
                self.expr(Self::precedence(Token::Assign))?;
                self.emitted.push(Instruction::StoreInt);
            } else if self.token == Token::Cond {
                self.next()?;
                self.emitted.push(Instruction::BranchIfZero(usize::MAX));
                let first_patch = self.emitted.len() - 1;
                self.expr(Self::precedence(Token::Assign))?;
                if self.token == Token::Colon {
                    self.next()?;
                } else {
                    return Err(C4Error::new(self.line, "conditional missing colon"));
                }
                let else_target = self.emitted.len() + 2;
                if let Instruction::BranchIfZero(target) = &mut self.emitted[first_patch] {
                    *target = else_target;
                }
                self.emitted.push(Instruction::Jump(usize::MAX));
                let second_patch = self.emitted.len() - 1;
                self.expr(Self::precedence(Token::Cond))?;
                let end_target = self.emitted.len();
                if let Instruction::Jump(target) = &mut self.emitted[second_patch] {
                    *target = end_target;
                }
            } else if self.token == Token::Lor {
                self.next()?;
                self.emitted.push(Instruction::BranchIfNonZero(usize::MAX));
                let patch = self.emitted.len() - 1;
                self.expr(Self::precedence(Token::Lan))?;
                let end_target = self.emitted.len();
                if let Instruction::BranchIfNonZero(target) = &mut self.emitted[patch] {
                    *target = end_target;
                }
            } else if self.token == Token::Lan {
                self.next()?;
                self.emitted.push(Instruction::BranchIfZero(usize::MAX));
                let patch = self.emitted.len() - 1;
                self.expr(Self::precedence(Token::Or))?;
                let end_target = self.emitted.len();
                if let Instruction::BranchIfZero(target) = &mut self.emitted[patch] {
                    *target = end_target;
                }
            } else if self.token == Token::Or {
                self.next()?;
                self.emitted.push(Instruction::Push);
                self.expr(Self::precedence(Token::Xor))?;
                self.emitted.push(Instruction::Or);
            } else if self.token == Token::Xor {
                self.next()?;
                self.emitted.push(Instruction::Push);
                self.expr(Self::precedence(Token::And))?;
                self.emitted.push(Instruction::Xor);
            } else if self.token == Token::And {
                self.next()?;
                self.emitted.push(Instruction::Push);
                self.expr(Self::precedence(Token::Eq))?;
                self.emitted.push(Instruction::And);
            } else if self.token == Token::Eq {
                self.next()?;
                self.emitted.push(Instruction::Push);
                self.expr(Self::precedence(Token::Lt))?;
                self.emitted.push(Instruction::Eq);
            } else if self.token == Token::Ne {
                self.next()?;
                self.emitted.push(Instruction::Push);
                self.expr(Self::precedence(Token::Lt))?;
                self.emitted.push(Instruction::Ne);
            } else if self.token == Token::Lt {
                self.next()?;
                self.emitted.push(Instruction::Push);
                self.expr(Self::precedence(Token::Shl))?;
                self.emitted.push(Instruction::Lt);
            } else if self.token == Token::Gt {
                self.next()?;
                self.emitted.push(Instruction::Push);
                self.expr(Self::precedence(Token::Shl))?;
                self.emitted.push(Instruction::Gt);
            } else if self.token == Token::Le {
                self.next()?;
                self.emitted.push(Instruction::Push);
                self.expr(Self::precedence(Token::Shl))?;
                self.emitted.push(Instruction::Le);
            } else if self.token == Token::Ge {
                self.next()?;
                self.emitted.push(Instruction::Push);
                self.expr(Self::precedence(Token::Shl))?;
                self.emitted.push(Instruction::Ge);
            } else if self.token == Token::Shl {
                self.next()?;
                self.emitted.push(Instruction::Push);
                self.expr(Self::precedence(Token::Add))?;
                self.emitted.push(Instruction::Shl);
            } else if self.token == Token::Shr {
                self.next()?;
                self.emitted.push(Instruction::Push);
                self.expr(Self::precedence(Token::Add))?;
                self.emitted.push(Instruction::Shr);
            } else if self.token == Token::Add {
                self.next()?;
                self.emitted.push(Instruction::Push);
                self.expr(Self::precedence(Token::Mul))?;
                self.emitted.push(Instruction::Add);
            } else if self.token == Token::Sub {
                self.next()?;
                self.emitted.push(Instruction::Push);
                self.expr(Self::precedence(Token::Mul))?;
                self.emitted.push(Instruction::Sub);
            } else if self.token == Token::Mul {
                self.next()?;
                self.emitted.push(Instruction::Push);
                self.expr(Self::precedence(Token::Inc))?;
                self.emitted.push(Instruction::Mul);
            } else if self.token == Token::Div {
                self.next()?;
                self.emitted.push(Instruction::Push);
                self.expr(Self::precedence(Token::Inc))?;
                self.emitted.push(Instruction::Div);
            } else if self.token == Token::Mod {
                self.next()?;
                self.emitted.push(Instruction::Push);
                self.expr(Self::precedence(Token::Inc))?;
                self.emitted.push(Instruction::Mod);
            } else if self.token == Token::Inc || self.token == Token::Dec {
                let is_inc = self.token == Token::Inc;
                self.emitted.push(Instruction::Push);
                self.emitted.push(Instruction::Imm(1));
                self.emitted
                    .push(if is_inc { Instruction::Add } else { Instruction::Sub });
                self.emitted.push(Instruction::StoreInt);
                self.emitted.push(Instruction::Push);
                self.emitted.push(Instruction::Imm(1));
                self.emitted
                    .push(if is_inc { Instruction::Sub } else { Instruction::Add });
                self.next()?;
            } else if self.token == Token::Brak {
                self.next()?;
                self.emitted.push(Instruction::Push);
                self.expr(Self::precedence(Token::Assign))?;
                if self.token == Token::RBracket {
                    self.next()?;
                } else {
                    return Err(C4Error::new(self.line, "close bracket expected"));
                }
                self.emitted.push(Instruction::Add);
                self.emitted.push(Instruction::LoadInt);
            } else {
                return Err(C4Error::new(
                    self.line,
                    format!("compiler error token={:?}", self.token),
                ));
            }
        }

        Ok(())
    }

    pub fn stmt(&mut self) -> Result<(), C4Error> {
        let assign_level = Self::precedence(Token::Assign);

        if self.token == Token::If {
            self.next()?;
            if self.token == Token::LParen {
                self.next()?;
            } else {
                return Err(C4Error::new(self.line, "open paren expected"));
            }
            self.expr(assign_level)?;
            if self.token == Token::RParen {
                self.next()?;
            } else {
                return Err(C4Error::new(self.line, "close paren expected"));
            }
            self.emitted.push(Instruction::BranchIfZero(usize::MAX));
            let branch_patch = self.emitted.len() - 1;
            self.stmt()?;
            if self.token == Token::Else {
                let else_target = self.emitted.len() + 2;
                if let Instruction::BranchIfZero(target) = &mut self.emitted[branch_patch] {
                    *target = else_target;
                }
                self.emitted.push(Instruction::Jump(usize::MAX));
                let jump_patch = self.emitted.len() - 1;
                self.next()?;
                self.stmt()?;
                let end_target = self.emitted.len();
                if let Instruction::Jump(target) = &mut self.emitted[jump_patch] {
                    *target = end_target;
                }
            } else {
                let end_target = self.emitted.len();
                if let Instruction::BranchIfZero(target) = &mut self.emitted[branch_patch] {
                    *target = end_target;
                }
            }
        } else if self.token == Token::While {
            self.next()?;
            let loop_start = self.emitted.len();
            if self.token == Token::LParen {
                self.next()?;
            } else {
                return Err(C4Error::new(self.line, "open paren expected"));
            }
            self.expr(assign_level)?;
            if self.token == Token::RParen {
                self.next()?;
            } else {
                return Err(C4Error::new(self.line, "close paren expected"));
            }
            self.emitted.push(Instruction::BranchIfZero(usize::MAX));
            let branch_patch = self.emitted.len() - 1;
            self.stmt()?;
            self.emitted.push(Instruction::Jump(loop_start));
            let end_target = self.emitted.len();
            if let Instruction::BranchIfZero(target) = &mut self.emitted[branch_patch] {
                *target = end_target;
            }
        } else if self.token == Token::Return {
            self.next()?;
            if self.token != Token::Semicolon {
                self.expr(assign_level)?;
            }
            self.emitted.push(Instruction::Leave);
            if self.token == Token::Semicolon {
                self.next()?;
            } else {
                return Err(C4Error::new(self.line, "semicolon expected"));
            }
        } else if self.token == Token::LBrace {
            self.next()?;
            while self.token != Token::RBrace {
                if self.token == Token::End {
                    return Err(C4Error::new(self.line, "close brace expected"));
                }
                self.stmt()?;
            }
            self.next()?;
        } else if self.token == Token::Semicolon {
            self.next()?;
        } else {
            self.expr(assign_level)?;
            if self.token == Token::Semicolon {
                self.next()?;
            } else {
                return Err(C4Error::new(self.line, "semicolon expected"));
            }
        }

        Ok(())
    }

    fn precedence(token: Token) -> i32 {
        match token {
            Token::Assign => 1,
            Token::Cond => 2,
            Token::Lor => 3,
            Token::Lan => 4,
            Token::Or => 5,
            Token::Xor => 6,
            Token::And => 7,
            Token::Eq | Token::Ne => 8,
            Token::Lt | Token::Gt | Token::Le | Token::Ge => 9,
            Token::Shl | Token::Shr => 10,
            Token::Add | Token::Sub => 11,
            Token::Mul | Token::Div | Token::Mod => 12,
            Token::Inc | Token::Dec | Token::Brak => 13,
            _ => 0,
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    fn peek_next_char(&self) -> Option<char> {
        self.chars.get(self.position + 1).copied()
    }

    fn scan_number(&mut self) -> Result<(), C4Error> {
        let Some(first) = self.peek_char() else {
            return Err(C4Error::new(self.line, "unexpected eof in number"));
        };

        if first != '0' {
            let mut value = (first as u8 - b'0') as i64;
            self.position += 1;
            while let Some(c) = self.peek_char() {
                if c.is_ascii_digit() {
                    value = value * 10 + (c as u8 - b'0') as i64;
                    self.position += 1;
                } else {
                    break;
                }
            }
            self.token_value = value;
            return Ok(());
        }

        self.position += 1;
        if matches!(self.peek_char(), Some('x') | Some('X')) {
            self.position += 1;
            let mut value = 0_i64;
            while let Some(c) = self.peek_char() {
                if c.is_ascii_hexdigit() {
                    value = value * 16
                        + match c {
                            '0'..='9' => (c as u8 - b'0') as i64,
                            'a'..='f' => 10 + (c as u8 - b'a') as i64,
                            'A'..='F' => 10 + (c as u8 - b'A') as i64,
                            _ => 0,
                        };
                    self.position += 1;
                } else {
                    break;
                }
            }
            self.token_value = value;
            return Ok(());
        }

        let mut value = 0_i64;
        while let Some(c) = self.peek_char() {
            if ('0'..='7').contains(&c) {
                value = value * 8 + (c as u8 - b'0') as i64;
                self.position += 1;
            } else {
                break;
            }
        }
        self.token_value = value;
        Ok(())
    }

    fn scan_string_or_char(&mut self, quote: char) -> Result<(), C4Error> {
        self.position += 1;
        let mut buf = String::new();
        while let Some(c) = self.peek_char() {
            if c == quote {
                break;
            }
            let value = if c == '\\' {
                self.position += 1;
                match self.peek_char() {
                    Some('n') => '\n',
                    Some(other) => other,
                    None => return Err(C4Error::new(self.line, "unterminated escape sequence")),
                }
            } else {
                c
            };
            buf.push(value);
            self.position += 1;
        }

        if self.peek_char() != Some(quote) {
            return Err(C4Error::new(self.line, "unterminated string literal"));
        }
        self.position += 1;

        if quote == '"' {
            self.token_text = buf;
            self.token_value = self.token_text.len() as i64;
            self.token = Token::StringLiteral;
        } else {
            self.token_value = buf.chars().next().unwrap_or('\0') as i64;
            self.token = Token::Number;
        }

        Ok(())
    }
}
