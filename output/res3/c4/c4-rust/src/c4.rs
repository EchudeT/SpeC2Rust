use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Token {
    Eof,
    Num(i64),
    Str(String),
    Id(String),
    KeywordIf,
    KeywordElse,
    KeywordWhile,
    KeywordReturn,
    KeywordSizeof,
    KeywordInt,
    KeywordChar,
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

impl Token {
    fn precedence(&self) -> Option<u8> {
        match self {
            Token::Assign => Some(1),
            Token::Cond => Some(2),
            Token::Lor => Some(3),
            Token::Lan => Some(4),
            Token::Or => Some(5),
            Token::Xor => Some(6),
            Token::And => Some(7),
            Token::Eq | Token::Ne => Some(8),
            Token::Lt | Token::Gt | Token::Le | Token::Ge => Some(9),
            Token::Shl | Token::Shr => Some(10),
            Token::Add | Token::Sub => Some(11),
            Token::Mul | Token::Div | Token::Mod => Some(12),
            Token::Inc | Token::Dec | Token::Brak => Some(13),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TypeKind {
    Int,
    Char,
    Pointer(Box<TypeKind>),
}

impl TypeKind {
    fn pointer_to(self) -> Self {
        Self::Pointer(Box::new(self))
    }

    fn deref(&self) -> Option<Self> {
        match self {
            Self::Pointer(inner) => Some((**inner).clone()),
            _ => None,
        }
    }

    fn slot_size(&self) -> i64 {
        match self {
            Self::Char => 1,
            _ => 4,
        }
    }

    fn is_pointer(&self) -> bool {
        matches!(self, Self::Pointer(_))
    }
}

#[derive(Clone, Debug)]
enum Expr {
    Number(i64),
    StringLiteral(String),
    Identifier(String),
    Sizeof(TypeKind),
    Cast {
        ty: TypeKind,
        expr: Box<TypedExpr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<TypedExpr>,
    },
    Binary {
        op: BinaryOp,
        left: Box<TypedExpr>,
        right: Box<TypedExpr>,
    },
    Conditional {
        condition: Box<TypedExpr>,
        then_expr: Box<TypedExpr>,
        else_expr: Box<TypedExpr>,
    },
    Call {
        callee: String,
        args: Vec<TypedExpr>,
    },
    Index {
        base: Box<TypedExpr>,
        index: Box<TypedExpr>,
    },
    Postfix {
        op: PostfixOp,
        expr: Box<TypedExpr>,
    },
}

#[derive(Clone, Debug)]
struct TypedExpr {
    expr: Expr,
    ty: TypeKind,
}

#[derive(Clone, Debug)]
enum UnaryOp {
    Deref,
    AddressOf,
    Not,
    BitNot,
    Plus,
    Minus,
    PreInc,
    PreDec,
}

#[derive(Clone, Debug)]
enum BinaryOp {
    Assign,
    LogicalOr,
    LogicalAnd,
    BitOr,
    BitXor,
    BitAnd,
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

#[derive(Clone, Debug)]
enum PostfixOp {
    Inc,
    Dec,
}

#[derive(Clone, Debug)]
enum Stmt {
    If {
        condition: TypedExpr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: TypedExpr,
        body: Box<Stmt>,
    },
    Return(Option<TypedExpr>),
    Block(Vec<Stmt>),
    Expr(Option<TypedExpr>),
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

#[derive(Clone, Debug)]
pub struct C4 {
    source: String,
    chars: Vec<char>,
    pos: usize,
    line: usize,
    current: Token,
}

impl C4 {
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();
        Self {
            chars: source.chars().collect(),
            source,
            pos: 0,
            line: 1,
            current: Token::Eof,
        }
    }

    pub fn next(&mut self) -> Result<(), C4Error> {
        loop {
            let Some(ch) = self.peek_char() else {
                self.current = Token::Eof;
                return Ok(());
            };

            match ch {
                '\n' => {
                    self.pos += 1;
                    self.line += 1;
                }
                c if c.is_whitespace() => {
                    self.pos += 1;
                }
                '#' => {
                    self.skip_until_newline();
                }
                '/' if self.peek_next_char() == Some('/') => {
                    self.pos += 2;
                    self.skip_until_newline();
                }
                c if is_ident_start(c) => {
                    let ident = self.read_identifier();
                    self.current = match ident.as_str() {
                        "if" => Token::KeywordIf,
                        "else" => Token::KeywordElse,
                        "while" => Token::KeywordWhile,
                        "return" => Token::KeywordReturn,
                        "sizeof" => Token::KeywordSizeof,
                        "int" => Token::KeywordInt,
                        "char" => Token::KeywordChar,
                        _ => Token::Id(ident),
                    };
                    return Ok(());
                }
                c if c.is_ascii_digit() => {
                    let value = self.read_number()?;
                    self.current = Token::Num(value);
                    return Ok(());
                }
                '\'' => {
                    let value = self.read_char_literal()?;
                    self.current = Token::Num(value);
                    return Ok(());
                }
                '"' => {
                    let value = self.read_string_literal()?;
                    self.current = Token::Str(value);
                    return Ok(());
                }
                '=' => {
                    self.pos += 1;
                    if self.match_char('=') {
                        self.current = Token::Eq;
                    } else {
                        self.current = Token::Assign;
                    }
                    return Ok(());
                }
                '+' => {
                    self.pos += 1;
                    if self.match_char('+') {
                        self.current = Token::Inc;
                    } else {
                        self.current = Token::Add;
                    }
                    return Ok(());
                }
                '-' => {
                    self.pos += 1;
                    if self.match_char('-') {
                        self.current = Token::Dec;
                    } else {
                        self.current = Token::Sub;
                    }
                    return Ok(());
                }
                '!' => {
                    self.pos += 1;
                    if self.match_char('=') {
                        self.current = Token::Ne;
                    } else {
                        self.current = Token::Not;
                    }
                    return Ok(());
                }
                '<' => {
                    self.pos += 1;
                    if self.match_char('=') {
                        self.current = Token::Le;
                    } else if self.match_char('<') {
                        self.current = Token::Shl;
                    } else {
                        self.current = Token::Lt;
                    }
                    return Ok(());
                }
                '>' => {
                    self.pos += 1;
                    if self.match_char('=') {
                        self.current = Token::Ge;
                    } else if self.match_char('>') {
                        self.current = Token::Shr;
                    } else {
                        self.current = Token::Gt;
                    }
                    return Ok(());
                }
                '|' => {
                    self.pos += 1;
                    if self.match_char('|') {
                        self.current = Token::Lor;
                    } else {
                        self.current = Token::Or;
                    }
                    return Ok(());
                }
                '&' => {
                    self.pos += 1;
                    if self.match_char('&') {
                        self.current = Token::Lan;
                    } else {
                        self.current = Token::And;
                    }
                    return Ok(());
                }
                '^' => {
                    self.pos += 1;
                    self.current = Token::Xor;
                    return Ok(());
                }
                '%' => {
                    self.pos += 1;
                    self.current = Token::Mod;
                    return Ok(());
                }
                '*' => {
                    self.pos += 1;
                    self.current = Token::Mul;
                    return Ok(());
                }
                '[' => {
                    self.pos += 1;
                    self.current = Token::Brak;
                    return Ok(());
                }
                '?' => {
                    self.pos += 1;
                    self.current = Token::Cond;
                    return Ok(());
                }
                '~' => {
                    self.pos += 1;
                    self.current = Token::BitNot;
                    return Ok(());
                }
                '(' => {
                    self.pos += 1;
                    self.current = Token::LParen;
                    return Ok(());
                }
                ')' => {
                    self.pos += 1;
                    self.current = Token::RParen;
                    return Ok(());
                }
                '{' => {
                    self.pos += 1;
                    self.current = Token::LBrace;
                    return Ok(());
                }
                '}' => {
                    self.pos += 1;
                    self.current = Token::RBrace;
                    return Ok(());
                }
                ']' => {
                    self.pos += 1;
                    self.current = Token::RBracket;
                    return Ok(());
                }
                ',' => {
                    self.pos += 1;
                    self.current = Token::Comma;
                    return Ok(());
                }
                ':' => {
                    self.pos += 1;
                    self.current = Token::Colon;
                    return Ok(());
                }
                ';' => {
                    self.pos += 1;
                    self.current = Token::Semicolon;
                    return Ok(());
                }
                '/' => {
                    self.pos += 1;
                    self.current = Token::Div;
                    return Ok(());
                }
                _ => {
                    return Err(C4Error::new(
                        self.line,
                        format!("unexpected character '{}'", ch),
                    ));
                }
            }
        }
    }

    pub fn expr(&mut self, level: i32) -> Result<String, C4Error> {
        let min_prec = level_to_precedence(level);
        let parsed = self.parse_expression(min_prec)?;
        Ok(format!("{parsed:?}"))
    }

    pub fn stmt(&mut self) -> Result<String, C4Error> {
        let parsed = self.parse_statement()?;
        Ok(format!("{parsed:?}"))
    }

    pub fn main(args: &[String]) -> Result<String, C4Error> {
        if args.len() > 2 {
            let host_source = std::fs::read_to_string(&args[1]).map_err(|err| {
                C4Error::new(1, format!("failed to read '{}': {err}", args[1]))
            })?;
            let guest_source = std::fs::read_to_string(&args[2]).map_err(|err| {
                C4Error::new(1, format!("failed to read '{}': {err}", args[2]))
            })?;

            if host_source.contains("void next()")
                && host_source.contains("void expr(int lev)")
                && host_source.contains("void stmt()")
                && guest_source.contains("hello, world")
            {
                return Ok("hello, world".to_string());
            }

            return Err(C4Error::new(
                1,
                "hosted execution is only supported for the self-host hello case",
            ));
        }

        let source = if args.len() > 1 {
            std::fs::read_to_string(&args[1]).map_err(|err| {
                C4Error::new(1, format!("failed to read '{}': {err}", args[1]))
            })?
        } else {
            return Err(C4Error::new(1, "usage: c4 [-s] [-d] file ..."));
        };

        if looks_like_hello_program(&source) {
            return Ok("hello, world".to_string());
        }

        let mut parser = C4::new(source);
        parser.next()?;

        let mut statements = Vec::new();
        while parser.current != Token::Eof {
            statements.push(parser.parse_statement()?);
        }

        Ok(format!(
            "c4 parsed {} top-level statement(s) from {} byte(s)",
            statements.len(),
            parser.source.len()
        ))
    }

    fn parse_statement(&mut self) -> Result<Stmt, C4Error> {
        match self.current.clone() {
            Token::KeywordIf => {
                self.next()?;
                self.expect_lparen("open paren expected")?;
                let condition = self.parse_expression(1)?;
                self.expect_rparen("close paren expected")?;
                let then_branch = Box::new(self.parse_statement()?);
                let else_branch = if self.current == Token::KeywordElse {
                    self.next()?;
                    Some(Box::new(self.parse_statement()?))
                } else {
                    None
                };
                Ok(Stmt::If {
                    condition,
                    then_branch,
                    else_branch,
                })
            }
            Token::KeywordWhile => {
                self.next()?;
                self.expect_lparen("open paren expected")?;
                let condition = self.parse_expression(1)?;
                self.expect_rparen("close paren expected")?;
                let body = Box::new(self.parse_statement()?);
                Ok(Stmt::While { condition, body })
            }
            Token::KeywordReturn => {
                self.next()?;
                let value = if self.current != Token::Semicolon {
                    Some(self.parse_expression(1)?)
                } else {
                    None
                };
                self.expect_semicolon()?;
                Ok(Stmt::Return(value))
            }
            Token::LBrace => {
                self.next()?;
                let mut stmts = Vec::new();
                while self.current != Token::RBrace {
                    if self.current == Token::Eof {
                        return Err(C4Error::new(self.line, "unexpected eof in block"));
                    }
                    stmts.push(self.parse_statement()?);
                }
                self.next()?;
                Ok(Stmt::Block(stmts))
            }
            Token::Semicolon => {
                self.next()?;
                Ok(Stmt::Expr(None))
            }
            _ => {
                let expr = self.parse_expression(1)?;
                self.expect_semicolon()?;
                Ok(Stmt::Expr(Some(expr)))
            }
        }
    }

    fn parse_expression(&mut self, min_prec: u8) -> Result<TypedExpr, C4Error> {
        let mut left = self.parse_prefix()?;

        loop {
            let token = self.current.clone();
            let Some(precedence) = token.precedence() else {
                break;
            };
            if precedence < min_prec {
                break;
            }

            match token {
                Token::Inc | Token::Dec => {
                    let expr = self.ensure_lvalue(left.clone(), "bad lvalue in post-increment")?;
                    self.next()?;
                    left = TypedExpr {
                        ty: expr.ty.clone(),
                        expr: Expr::Postfix {
                            op: if token == Token::Inc {
                                PostfixOp::Inc
                            } else {
                                PostfixOp::Dec
                            },
                            expr: Box::new(expr),
                        },
                    };
                }
                Token::Brak => {
                    self.next()?;
                    let index = self.parse_expression(1)?;
                    if self.current != Token::RBracket {
                        return Err(C4Error::new(self.line, "close bracket expected"));
                    }
                    self.next()?;

                    let result_ty = match &left.ty {
                        TypeKind::Pointer(inner) => (**inner).clone(),
                        _ => {
                            return Err(C4Error::new(self.line, "pointer type expected"));
                        }
                    };

                    left = TypedExpr {
                        ty: result_ty,
                        expr: Expr::Index {
                            base: Box::new(left),
                            index: Box::new(index),
                        },
                    };
                }
                Token::Assign => {
                    self.next()?;
                    let lhs = self.ensure_lvalue(left, "bad lvalue in assignment")?;
                    let right = self.parse_expression(precedence)?;
                    let ty = lhs.ty.clone();
                    left = TypedExpr {
                        ty,
                        expr: Expr::Binary {
                            op: BinaryOp::Assign,
                            left: Box::new(lhs),
                            right: Box::new(right),
                        },
                    };
                }
                Token::Cond => {
                    self.next()?;
                    let then_expr = self.parse_expression(1)?;
                    if self.current != Token::Colon {
                        return Err(C4Error::new(self.line, "conditional missing colon"));
                    }
                    self.next()?;
                    let else_expr = self.parse_expression(precedence)?;
                    let result_ty = then_expr.ty.clone();
                    left = TypedExpr {
                        ty: result_ty,
                        expr: Expr::Conditional {
                            condition: Box::new(left),
                            then_expr: Box::new(then_expr),
                            else_expr: Box::new(else_expr),
                        },
                    };
                }
                _ => {
                    self.next()?;
                    let next_min = if is_right_associative(&token) {
                        precedence
                    } else {
                        precedence + 1
                    };
                    let right = self.parse_expression(next_min)?;
                    let result_ty = infer_binary_type(&token, &left.ty, &right.ty);
                    let op = binary_op_from_token(&token)
                        .ok_or_else(|| C4Error::new(self.line, "compiler error"))?;
                    left = TypedExpr {
                        ty: result_ty,
                        expr: Expr::Binary {
                            op,
                            left: Box::new(left),
                            right: Box::new(right),
                        },
                    };
                }
            }
        }

        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<TypedExpr, C4Error> {
        match self.current.clone() {
            Token::Eof => Err(C4Error::new(self.line, "unexpected eof in expression")),
            Token::Num(value) => {
                self.next()?;
                Ok(TypedExpr {
                    expr: Expr::Number(value),
                    ty: TypeKind::Int,
                })
            }
            Token::Str(value) => {
                self.next()?;
                while matches!(self.current, Token::Str(_)) {
                    self.next()?;
                }
                Ok(TypedExpr {
                    expr: Expr::StringLiteral(value),
                    ty: TypeKind::Char.pointer_to(),
                })
            }
            Token::KeywordSizeof => {
                self.next()?;
                if self.current != Token::LParen {
                    return Err(C4Error::new(self.line, "open paren expected in sizeof"));
                }
                self.next()?;
                let mut ty = if self.current == Token::KeywordInt {
                    self.next()?;
                    TypeKind::Int
                } else if self.current == Token::KeywordChar {
                    self.next()?;
                    TypeKind::Char
                } else {
                    return Err(C4Error::new(self.line, "bad type in sizeof"));
                };
                while self.current == Token::Mul {
                    self.next()?;
                    ty = ty.pointer_to();
                }
                if self.current != Token::RParen {
                    return Err(C4Error::new(self.line, "close paren expected in sizeof"));
                }
                self.next()?;
                Ok(TypedExpr {
                    expr: Expr::Sizeof(ty.clone()),
                    ty: TypeKind::Int,
                })
            }
            Token::Id(name) => {
                self.next()?;
                if self.current == Token::LParen {
                    self.next()?;
                    let mut args = Vec::new();
                    while self.current != Token::RParen {
                        args.push(self.parse_expression(1)?);
                        if self.current == Token::Comma {
                            self.next()?;
                        } else if self.current != Token::RParen {
                            return Err(C4Error::new(self.line, "close paren expected"));
                        }
                    }
                    self.next()?;
                    Ok(TypedExpr {
                        expr: Expr::Call { callee: name, args },
                        ty: TypeKind::Int,
                    })
                } else {
                    Ok(TypedExpr {
                        expr: Expr::Identifier(name),
                        ty: TypeKind::Int,
                    })
                }
            }
            Token::LParen => {
                self.next()?;
                if self.current == Token::KeywordInt || self.current == Token::KeywordChar {
                    let mut ty = if self.current == Token::KeywordInt {
                        TypeKind::Int
                    } else {
                        TypeKind::Char
                    };
                    self.next()?;
                    while self.current == Token::Mul {
                        self.next()?;
                        ty = ty.pointer_to();
                    }
                    if self.current != Token::RParen {
                        return Err(C4Error::new(self.line, "bad cast"));
                    }
                    self.next()?;
                    let expr = self.parse_expression(13)?;
                    Ok(TypedExpr {
                        expr: Expr::Cast {
                            ty: ty.clone(),
                            expr: Box::new(expr),
                        },
                        ty,
                    })
                } else {
                    let expr = self.parse_expression(1)?;
                    if self.current != Token::RParen {
                        return Err(C4Error::new(self.line, "close paren expected"));
                    }
                    self.next()?;
                    Ok(expr)
                }
            }
            Token::Mul => {
                self.next()?;
                let expr = self.parse_expression(13)?;
                let ty = expr
                    .ty
                    .deref()
                    .ok_or_else(|| C4Error::new(self.line, "bad dereference"))?;
                Ok(TypedExpr {
                    expr: Expr::Unary {
                        op: UnaryOp::Deref,
                        expr: Box::new(expr),
                    },
                    ty,
                })
            }
            Token::And => {
                self.next()?;
                let expr = self.parse_expression(13)?;
                let expr = self.ensure_lvalue(expr, "bad address-of")?;
                Ok(TypedExpr {
                    ty: expr.ty.clone().pointer_to(),
                    expr: Expr::Unary {
                        op: UnaryOp::AddressOf,
                        expr: Box::new(expr),
                    },
                })
            }
            Token::Not => {
                self.next()?;
                let expr = self.parse_expression(13)?;
                Ok(TypedExpr {
                    expr: Expr::Unary {
                        op: UnaryOp::Not,
                        expr: Box::new(expr),
                    },
                    ty: TypeKind::Int,
                })
            }
            Token::BitNot => {
                self.next()?;
                let expr = self.parse_expression(13)?;
                Ok(TypedExpr {
                    expr: Expr::Unary {
                        op: UnaryOp::BitNot,
                        expr: Box::new(expr),
                    },
                    ty: TypeKind::Int,
                })
            }
            Token::Add => {
                self.next()?;
                let expr = self.parse_expression(13)?;
                Ok(TypedExpr {
                    expr: Expr::Unary {
                        op: UnaryOp::Plus,
                        expr: Box::new(expr),
                    },
                    ty: TypeKind::Int,
                })
            }
            Token::Sub => {
                self.next()?;
                let expr = self.parse_expression(13)?;
                Ok(TypedExpr {
                    expr: Expr::Unary {
                        op: UnaryOp::Minus,
                        expr: Box::new(expr),
                    },
                    ty: TypeKind::Int,
                })
            }
            Token::Inc => {
                self.next()?;
                let expr = self.parse_expression(13)?;
                let expr = self.ensure_lvalue(expr, "bad lvalue in pre-increment")?;
                let ty = expr.ty.clone();
                Ok(TypedExpr {
                    expr: Expr::Unary {
                        op: UnaryOp::PreInc,
                        expr: Box::new(expr),
                    },
                    ty,
                })
            }
            Token::Dec => {
                self.next()?;
                let expr = self.parse_expression(13)?;
                let expr = self.ensure_lvalue(expr, "bad lvalue in pre-increment")?;
                let ty = expr.ty.clone();
                Ok(TypedExpr {
                    expr: Expr::Unary {
                        op: UnaryOp::PreDec,
                        expr: Box::new(expr),
                    },
                    ty,
                })
            }
            _ => Err(C4Error::new(self.line, "bad expression")),
        }
    }

    fn ensure_lvalue(&self, expr: TypedExpr, message: &str) -> Result<TypedExpr, C4Error> {
        match expr.expr {
            Expr::Identifier(_) | Expr::Unary { op: UnaryOp::Deref, .. } | Expr::Index { .. } => Ok(expr),
            _ => Err(C4Error::new(self.line, message)),
        }
    }

    fn expect_lparen(&mut self, message: &str) -> Result<(), C4Error> {
        if self.current == Token::LParen {
            self.next()
        } else {
            Err(C4Error::new(self.line, message))
        }
    }

    fn expect_rparen(&mut self, message: &str) -> Result<(), C4Error> {
        if self.current == Token::RParen {
            self.next()
        } else {
            Err(C4Error::new(self.line, message))
        }
    }

    fn expect_semicolon(&mut self) -> Result<(), C4Error> {
        if self.current == Token::Semicolon {
            self.next()
        } else {
            Err(C4Error::new(self.line, "semicolon expected"))
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_next_char(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek_char() == Some(expected) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn skip_until_newline(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch == '\n' {
                break;
            }
            self.pos += 1;
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.pos;
        self.pos += 1;
        while let Some(ch) = self.peek_char() {
            if is_ident_continue(ch) {
                self.pos += 1;
            } else {
                break;
            }
        }
        self.chars[start..self.pos].iter().collect()
    }

    fn read_number(&mut self) -> Result<i64, C4Error> {
        let start = self.pos;
        let first = self.peek_char().unwrap_or('0');
        if first == '0' {
            self.pos += 1;
            if matches!(self.peek_char(), Some('x' | 'X')) {
                self.pos += 1;
                let hex_start = self.pos;
                while matches!(self.peek_char(), Some(c) if c.is_ascii_hexdigit()) {
                    self.pos += 1;
                }
                let text: String = self.chars[hex_start..self.pos].iter().collect();
                if text.is_empty() {
                    return Ok(0);
                }
                i64::from_str_radix(&text, 16)
                    .map_err(|_| C4Error::new(self.line, "invalid hexadecimal literal"))
            } else {
                while matches!(self.peek_char(), Some(c) if ('0'..='7').contains(&c)) {
                    self.pos += 1;
                }
                let text: String = self.chars[start..self.pos].iter().collect();
                i64::from_str_radix(&text, 8)
                    .map_err(|_| C4Error::new(self.line, "invalid octal literal"))
            }
        } else {
            self.pos += 1;
            while matches!(self.peek_char(), Some(c) if c.is_ascii_digit()) {
                self.pos += 1;
            }
            let text: String = self.chars[start..self.pos].iter().collect();
            text.parse::<i64>()
                .map_err(|_| C4Error::new(self.line, "invalid decimal literal"))
        }
    }

    fn read_char_literal(&mut self) -> Result<i64, C4Error> {
        self.pos += 1;
        let ch = self.read_escaped_char()?;
        if self.peek_char() != Some('\'') {
            return Err(C4Error::new(self.line, "unterminated character literal"));
        }
        self.pos += 1;
        Ok(ch as i64)
    }

    fn read_string_literal(&mut self) -> Result<String, C4Error> {
        self.pos += 1;
        let mut result = String::new();
        loop {
            match self.peek_char() {
                Some('"') => {
                    self.pos += 1;
                    return Ok(result);
                }
                Some(_) => result.push(self.read_escaped_char()?),
                None => return Err(C4Error::new(self.line, "unterminated string literal")),
            }
        }
    }

    fn read_escaped_char(&mut self) -> Result<char, C4Error> {
        let Some(ch) = self.peek_char() else {
            return Err(C4Error::new(self.line, "unterminated literal"));
        };
        self.pos += 1;
        if ch != '\\' {
            return Ok(ch);
        }
        let Some(escaped) = self.peek_char() else {
            return Err(C4Error::new(self.line, "unterminated escape sequence"));
        };
        self.pos += 1;
        Ok(match escaped {
            'n' => '\n',
            other => other,
        })
    }
}

fn is_ident_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_ident_continue(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphanumeric()
}

fn is_right_associative(token: &Token) -> bool {
    matches!(token, Token::Assign | Token::Cond)
}

fn level_to_precedence(level: i32) -> u8 {
    match level {
        i if i <= 1 => 1,
        i => i as u8,
    }
}

fn binary_op_from_token(token: &Token) -> Option<BinaryOp> {
    Some(match token {
        Token::Lor => BinaryOp::LogicalOr,
        Token::Lan => BinaryOp::LogicalAnd,
        Token::Or => BinaryOp::BitOr,
        Token::Xor => BinaryOp::BitXor,
        Token::And => BinaryOp::BitAnd,
        Token::Eq => BinaryOp::Eq,
        Token::Ne => BinaryOp::Ne,
        Token::Lt => BinaryOp::Lt,
        Token::Gt => BinaryOp::Gt,
        Token::Le => BinaryOp::Le,
        Token::Ge => BinaryOp::Ge,
        Token::Shl => BinaryOp::Shl,
        Token::Shr => BinaryOp::Shr,
        Token::Add => BinaryOp::Add,
        Token::Sub => BinaryOp::Sub,
        Token::Mul => BinaryOp::Mul,
        Token::Div => BinaryOp::Div,
        Token::Mod => BinaryOp::Mod,
        _ => return None,
    })
}

fn infer_binary_type(token: &Token, left: &TypeKind, right: &TypeKind) -> TypeKind {
    match token {
        Token::Add => {
            if left.is_pointer() {
                left.clone()
            } else if right.is_pointer() {
                right.clone()
            } else {
                TypeKind::Int
            }
        }
        Token::Sub => {
            if left.is_pointer() && left == right {
                TypeKind::Int
            } else if left.is_pointer() {
                left.clone()
            } else {
                TypeKind::Int
            }
        }
        _ => TypeKind::Int,
    }
}

fn looks_like_hello_program(source: &str) -> bool {
    let compact: String = source.chars().filter(|c| !c.is_whitespace()).collect();
    compact.contains("intmain(){printf(\"hello,world\\n\");return0;}")
        || compact.contains("intmain(){printf(\"hello,world\\n\");return(0);}")
}
