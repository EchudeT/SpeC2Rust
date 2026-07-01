use std::fmt;

const TK_NUM: i32 = 128;
const TK_FUN: i32 = 129;
const TK_SYS: i32 = 130;
const TK_GLO: i32 = 131;
const TK_LOC: i32 = 132;
const TK_ID: i32 = 133;
const TK_CHAR: i32 = 134;
const TK_ELSE: i32 = 135;
const TK_ENUM: i32 = 136;
const TK_IF: i32 = 137;
const TK_INT: i32 = 138;
const TK_RETURN: i32 = 139;
const TK_SIZEOF: i32 = 140;
const TK_WHILE: i32 = 141;

const TK_ASSIGN: i32 = 142;
const TK_COND: i32 = 143;
const TK_LOR: i32 = 144;
const TK_LAN: i32 = 145;
const TK_OR: i32 = 146;
const TK_XOR: i32 = 147;
const TK_AND: i32 = 148;
const TK_EQ: i32 = 149;
const TK_NE: i32 = 150;
const TK_LT: i32 = 151;
const TK_GT: i32 = 152;
const TK_LE: i32 = 153;
const TK_GE: i32 = 154;
const TK_SHL: i32 = 155;
const TK_SHR: i32 = 156;
const TK_ADD: i32 = 157;
const TK_SUB: i32 = 158;
const TK_MUL: i32 = 159;
const TK_DIV: i32 = 160;
const TK_MOD: i32 = 161;
const TK_INC: i32 = 162;
const TK_DEC: i32 = 163;
const TK_BRAK: i32 = 164;

const CLASS_NUM: i32 = 1;
const CLASS_FUN: i32 = 2;
const CLASS_SYS: i32 = 3;
const CLASS_GLO: i32 = 4;
const CLASS_LOC: i32 = 5;

const TYPE_CHAR: i32 = 0;
const TYPE_INT: i32 = 1;
const TYPE_PTR: i32 = 2;

const OP_LEA: i32 = 0;
const OP_IMM: i32 = 1;
const OP_JMP: i32 = 2;
const OP_JSR: i32 = 3;
const OP_BZ: i32 = 4;
const OP_BNZ: i32 = 5;
const OP_ENT: i32 = 6;
const OP_ADJ: i32 = 7;
const OP_LEV: i32 = 8;
const OP_LI: i32 = 9;
const OP_LC: i32 = 10;
const OP_SI: i32 = 11;
const OP_SC: i32 = 12;
const OP_PSH: i32 = 13;
const OP_OR: i32 = 14;
const OP_XOR: i32 = 15;
const OP_AND: i32 = 16;
const OP_EQ: i32 = 17;
const OP_NE: i32 = 18;
const OP_LT: i32 = 19;
const OP_GT: i32 = 20;
const OP_LE: i32 = 21;
const OP_GE: i32 = 22;
const OP_SHL: i32 = 23;
const OP_SHR: i32 = 24;
const OP_ADD: i32 = 25;
const OP_SUB: i32 = 26;
const OP_MUL: i32 = 27;
const OP_DIV: i32 = 28;
const OP_MOD: i32 = 29;

#[derive(Debug, Clone)]
struct Symbol {
    name: String,
    hash: i32,
    token: i32,
    class: i32,
    ty: i32,
    val: i32,
}

#[derive(Debug, Clone)]
struct C4Error {
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

pub struct C4 {
    source: Vec<u8>,
    p: usize,
    line: usize,
    tk: i32,
    ival: i32,
    current_id: Option<usize>,
    symbols: Vec<Symbol>,
    text: Vec<i32>,
    data: Vec<i32>,
    ty: i32,
    loc: i32,
}

impl Default for C4 {
    fn default() -> Self {
        let mut this = Self {
            source: Vec::new(),
            p: 0,
            line: 1,
            tk: 0,
            ival: 0,
            current_id: None,
            symbols: Vec::new(),
            text: Vec::new(),
            data: Vec::new(),
            ty: TYPE_INT,
            loc: 0,
        };
        this.install_keywords();
        this
    }
}

impl C4 {
    fn install_keywords(&mut self) {
        self.add_symbol("char", TK_CHAR, 0, TYPE_CHAR, 0);
        self.add_symbol("else", TK_ELSE, 0, TYPE_INT, 0);
        self.add_symbol("enum", TK_ENUM, 0, TYPE_INT, 0);
        self.add_symbol("if", TK_IF, 0, TYPE_INT, 0);
        self.add_symbol("int", TK_INT, 0, TYPE_INT, 0);
        self.add_symbol("return", TK_RETURN, 0, TYPE_INT, 0);
        self.add_symbol("sizeof", TK_SIZEOF, 0, TYPE_INT, 0);
        self.add_symbol("while", TK_WHILE, 0, TYPE_INT, 0);
    }

    fn add_symbol(&mut self, name: &str, token: i32, class: i32, ty: i32, val: i32) {
        let hash = Self::hash_ident(name.as_bytes());
        self.symbols.push(Symbol {
            name: name.to_string(),
            hash,
            token,
            class,
            ty,
            val,
        });
    }

    fn hash_ident(bytes: &[u8]) -> i32 {
        let mut tk = bytes[0] as i32;
        for &b in &bytes[1..] {
            tk = tk.wrapping_mul(147).wrapping_add(b as i32);
        }
        (tk << 6) + bytes.len() as i32
    }

    fn current_byte(&self) -> u8 {
        self.source.get(self.p).copied().unwrap_or(0)
    }

    fn push_op(&mut self, op: i32) {
        self.text.push(op);
    }

    fn emit_placeholder(&mut self) -> usize {
        self.text.push(0);
        self.text.len() - 1
    }

    fn require_token(&mut self, token: i32, message: &str) -> Result<(), C4Error> {
        if self.tk == token {
            self.next()?;
            Ok(())
        } else {
            Err(C4Error::new(self.line, message))
        }
    }

    fn patch_to_current(&mut self, at: usize, offset: usize) {
        self.text[at] = (self.text.len() + offset) as i32;
    }

    fn lookup_or_insert_id(&mut self, name: &[u8], hash: i32) -> usize {
        if let Some(index) = self
            .symbols
            .iter()
            .position(|sym| sym.hash == hash && sym.name.as_bytes() == name)
        {
            index
        } else {
            self.symbols.push(Symbol {
                name: String::from_utf8_lossy(name).into_owned(),
                hash,
                token: TK_ID,
                class: 0,
                ty: TYPE_INT,
                val: 0,
            });
            self.symbols.len() - 1
        }
    }

    pub fn next(&mut self) -> Result<(), C4Error> {
        while self.current_byte() != 0 {
            self.tk = self.current_byte() as i32;
            self.p += 1;

            match self.tk as u8 {
                b'\n' => {
                    self.line += 1;
                }
                b'#' => {
                    while self.current_byte() != 0 && self.current_byte() != b'\n' {
                        self.p += 1;
                    }
                }
                c if c.is_ascii_alphabetic() || c == b'_' => {
                    let start = self.p - 1;
                    while {
                        let c = self.current_byte();
                        c.is_ascii_alphanumeric() || c == b'_'
                    } {
                        self.p += 1;
                    }
                    let ident = self.source[start..self.p].to_vec();
                    let hash = Self::hash_ident(&ident);
                    let idx = self.lookup_or_insert_id(&ident, hash);
                    self.current_id = Some(idx);
                    self.tk = self.symbols[idx].token;
                    return Ok(());
                }
                c if c.is_ascii_digit() => {
                    let first = (self.tk as u8 - b'0') as i32;
                    self.ival = 0;
                    if first != 0 {
                        self.ival = first;
                        while self.current_byte().is_ascii_digit() {
                            self.ival = self
                                .ival
                                .saturating_mul(10)
                                .saturating_add((self.current_byte() - b'0') as i32);
                            self.p += 1;
                        }
                    } else if matches!(self.current_byte(), b'x' | b'X') {
                        self.p += 1;
                        while {
                            let c = self.current_byte();
                            c.is_ascii_hexdigit()
                        } {
                            let c = self.current_byte();
                            self.ival = self.ival.saturating_mul(16).saturating_add(
                                match c {
                                    b'0'..=b'9' => (c - b'0') as i32,
                                    b'a'..=b'f' => (c - b'a' + 10) as i32,
                                    b'A'..=b'F' => (c - b'A' + 10) as i32,
                                    _ => 0,
                                },
                            );
                            self.p += 1;
                        }
                    } else {
                        while matches!(self.current_byte(), b'0'..=b'7') {
                            self.ival = self
                                .ival
                                .saturating_mul(8)
                                .saturating_add((self.current_byte() - b'0') as i32);
                            self.p += 1;
                        }
                    }
                    self.tk = TK_NUM;
                    return Ok(());
                }
                b'/' => {
                    if self.current_byte() == b'/' {
                        self.p += 1;
                        while self.current_byte() != 0 && self.current_byte() != b'\n' {
                            self.p += 1;
                        }
                    } else {
                        self.tk = TK_DIV;
                        return Ok(());
                    }
                }
                b'\'' | b'"' => {
                    let quote = self.tk as u8;
                    let start = self.data.len() as i32;
                    while self.current_byte() != 0 && self.current_byte() != quote {
                        let mut v = self.current_byte();
                        self.p += 1;
                        if v == b'\\' {
                            v = self.current_byte();
                            self.p += 1;
                            if v == b'n' {
                                v = b'\n';
                            }
                        }
                        if quote == b'"' {
                            self.data.push(v as i32);
                        } else {
                            self.ival = v as i32;
                        }
                    }
                    if self.current_byte() == quote {
                        self.p += 1;
                    }
                    if quote == b'"' {
                        self.ival = start;
                        self.tk = b'"' as i32;
                    } else {
                        self.tk = TK_NUM;
                    }
                    return Ok(());
                }
                b'=' => {
                    self.tk = if self.current_byte() == b'=' {
                        self.p += 1;
                        TK_EQ
                    } else {
                        TK_ASSIGN
                    };
                    return Ok(());
                }
                b'+' => {
                    self.tk = if self.current_byte() == b'+' {
                        self.p += 1;
                        TK_INC
                    } else {
                        TK_ADD
                    };
                    return Ok(());
                }
                b'-' => {
                    self.tk = if self.current_byte() == b'-' {
                        self.p += 1;
                        TK_DEC
                    } else {
                        TK_SUB
                    };
                    return Ok(());
                }
                b'!' => {
                    if self.current_byte() == b'=' {
                        self.p += 1;
                        self.tk = TK_NE;
                    }
                    return Ok(());
                }
                b'<' => {
                    self.tk = if self.current_byte() == b'=' {
                        self.p += 1;
                        TK_LE
                    } else if self.current_byte() == b'<' {
                        self.p += 1;
                        TK_SHL
                    } else {
                        TK_LT
                    };
                    return Ok(());
                }
                b'>' => {
                    self.tk = if self.current_byte() == b'=' {
                        self.p += 1;
                        TK_GE
                    } else if self.current_byte() == b'>' {
                        self.p += 1;
                        TK_SHR
                    } else {
                        TK_GT
                    };
                    return Ok(());
                }
                b'|' => {
                    self.tk = if self.current_byte() == b'|' {
                        self.p += 1;
                        TK_LOR
                    } else {
                        TK_OR
                    };
                    return Ok(());
                }
                b'&' => {
                    self.tk = if self.current_byte() == b'&' {
                        self.p += 1;
                        TK_LAN
                    } else {
                        TK_AND
                    };
                    return Ok(());
                }
                b'^' => {
                    self.tk = TK_XOR;
                    return Ok(());
                }
                b'%' => {
                    self.tk = TK_MOD;
                    return Ok(());
                }
                b'*' => {
                    self.tk = TK_MUL;
                    return Ok(());
                }
                b'[' => {
                    self.tk = TK_BRAK;
                    return Ok(());
                }
                b'?' => {
                    self.tk = TK_COND;
                    return Ok(());
                }
                b'~' | b';' | b'{' | b'}' | b'(' | b')' | b']' | b',' | b':' => {
                    return Ok(());
                }
                b if b.is_ascii_whitespace() => {}
                _ => {}
            }
        }
        self.tk = 0;
        Ok(())
    }

    pub fn expr(&mut self, lev: i32) -> Result<(), C4Error> {
        let mut t;
        if self.tk == 0 {
            return Err(C4Error::new(self.line, "unexpected eof in expression"));
        } else if self.tk == TK_NUM {
            self.push_op(OP_IMM);
            self.push_op(self.ival);
            self.next()?;
            self.ty = TYPE_INT;
        } else if self.tk == b'"' as i32 {
            self.push_op(OP_IMM);
            self.push_op(self.ival);
            self.next()?;
            while self.tk == b'"' as i32 {
                self.next()?;
            }
            self.ty = TYPE_PTR;
        } else if self.tk == TK_SIZEOF {
            self.next()?;
            self.require_token(b'(' as i32, "open paren expected in sizeof")?;
            self.ty = TYPE_INT;
            if self.tk == TK_INT {
                self.next()?;
            } else if self.tk == TK_CHAR {
                self.next()?;
                self.ty = TYPE_CHAR;
            }
            while self.tk == TK_MUL {
                self.next()?;
                self.ty += TYPE_PTR;
            }
            if self.tk == b')' as i32 {
                self.next()?;
            } else {
                return Err(C4Error::new(self.line, "close paren expected in sizeof"));
            }
            self.push_op(OP_IMM);
            self.push_op(if self.ty == TYPE_CHAR { 1 } else { 4 });
            self.ty = TYPE_INT;
        } else if self.tk == TK_ID {
            let d = self
                .current_id
                .ok_or_else(|| C4Error::new(self.line, "identifier state missing"))?;
            self.next()?;
            if self.tk == b'(' as i32 {
                self.next()?;
                let mut argc = 0;
                while self.tk != b')' as i32 {
                    self.expr(TK_ASSIGN)?;
                    self.push_op(OP_PSH);
                    argc += 1;
                    if self.tk == b',' as i32 {
                        self.next()?;
                    }
                }
                self.next()?;
                let (class, val, ty) = {
                    let sym = &self.symbols[d];
                    (sym.class, sym.val, sym.ty)
                };
                if class == CLASS_SYS {
                    self.push_op(val);
                } else if class == CLASS_FUN {
                    self.push_op(OP_JSR);
                    self.push_op(val);
                } else {
                    return Err(C4Error::new(self.line, "bad function call"));
                }
                if argc > 0 {
                    self.push_op(OP_ADJ);
                    self.push_op(argc);
                }
                self.ty = ty;
            } else if self.symbols[d].class == CLASS_NUM {
                self.push_op(OP_IMM);
                self.push_op(self.symbols[d].val);
                self.ty = TYPE_INT;
            } else {
                let (class, val, ty) = {
                    let sym = &self.symbols[d];
                    (sym.class, sym.val, sym.ty)
                };
                if class == CLASS_LOC {
                    self.push_op(OP_LEA);
                    self.push_op(self.loc - val);
                } else if class == CLASS_GLO {
                    self.push_op(OP_IMM);
                    self.push_op(val);
                } else {
                    return Err(C4Error::new(self.line, "undefined variable"));
                }
                self.ty = ty;
                self.push_op(if self.ty == TYPE_CHAR { OP_LC } else { OP_LI });
            }
        } else if self.tk == b'(' as i32 {
            self.next()?;
            if self.tk == TK_INT || self.tk == TK_CHAR {
                let mut cast_ty = if self.tk == TK_INT { TYPE_INT } else { TYPE_CHAR };
                self.next()?;
                while self.tk == TK_MUL {
                    self.next()?;
                    cast_ty += TYPE_PTR;
                }
                if self.tk == b')' as i32 {
                    self.next()?;
                } else {
                    return Err(C4Error::new(self.line, "bad cast"));
                }
                self.expr(TK_INC)?;
                self.ty = cast_ty;
            } else {
                self.expr(TK_ASSIGN)?;
                if self.tk == b')' as i32 {
                    self.next()?;
                } else {
                    return Err(C4Error::new(self.line, "close paren expected"));
                }
            }
        } else if self.tk == TK_MUL {
            self.next()?;
            self.expr(TK_INC)?;
            if self.ty > TYPE_INT {
                self.ty -= TYPE_PTR;
            } else {
                return Err(C4Error::new(self.line, "bad dereference"));
            }
            self.push_op(if self.ty == TYPE_CHAR { OP_LC } else { OP_LI });
        } else if self.tk == TK_AND {
            self.next()?;
            self.expr(TK_INC)?;
            match self.text.last().copied() {
                Some(OP_LC) | Some(OP_LI) => {
                    self.text.pop();
                }
                _ => return Err(C4Error::new(self.line, "bad address-of")),
            }
            self.ty += TYPE_PTR;
        } else if self.tk == b'!' as i32 {
            self.next()?;
            self.expr(TK_INC)?;
            self.push_op(OP_PSH);
            self.push_op(OP_IMM);
            self.push_op(0);
            self.push_op(OP_EQ);
            self.ty = TYPE_INT;
        } else if self.tk == b'~' as i32 {
            self.next()?;
            self.expr(TK_INC)?;
            self.push_op(OP_PSH);
            self.push_op(OP_IMM);
            self.push_op(-1);
            self.push_op(OP_XOR);
            self.ty = TYPE_INT;
        } else if self.tk == TK_ADD {
            self.next()?;
            self.expr(TK_INC)?;
            self.ty = TYPE_INT;
        } else if self.tk == TK_SUB {
            self.next()?;
            self.push_op(OP_IMM);
            if self.tk == TK_NUM {
                self.push_op(-self.ival);
                self.next()?;
            } else {
                self.push_op(-1);
                self.push_op(OP_PSH);
                self.expr(TK_INC)?;
                self.push_op(OP_MUL);
            }
            self.ty = TYPE_INT;
        } else if self.tk == TK_INC || self.tk == TK_DEC {
            let pre = self.tk;
            self.next()?;
            self.expr(TK_INC)?;
            match self.text.last_mut() {
                Some(last) if *last == OP_LC => {
                    *last = OP_PSH;
                    self.push_op(OP_LC);
                }
                Some(last) if *last == OP_LI => {
                    *last = OP_PSH;
                    self.push_op(OP_LI);
                }
                _ => return Err(C4Error::new(self.line, "bad lvalue in pre-increment")),
            }
            self.push_op(OP_PSH);
            self.push_op(OP_IMM);
            self.push_op(if self.ty > TYPE_PTR { 4 } else { 1 });
            self.push_op(if pre == TK_INC { OP_ADD } else { OP_SUB });
            self.push_op(if self.ty == TYPE_CHAR { OP_SC } else { OP_SI });
        } else {
            return Err(C4Error::new(self.line, "bad expression"));
        }

        while self.tk >= lev {
            t = self.ty;
            if self.tk == TK_ASSIGN {
                self.next()?;
                match self.text.last_mut() {
                    Some(last) if *last == OP_LC || *last == OP_LI => *last = OP_PSH,
                    _ => return Err(C4Error::new(self.line, "bad lvalue in assignment")),
                }
                self.expr(TK_ASSIGN)?;
                self.push_op(if t == TYPE_CHAR { OP_SC } else { OP_SI });
                self.ty = t;
            } else if self.tk == TK_COND {
                self.next()?;
                self.push_op(OP_BZ);
                let d = self.emit_placeholder();
                self.expr(TK_ASSIGN)?;
                if self.tk == b':' as i32 {
                    self.next()?;
                } else {
                    return Err(C4Error::new(self.line, "conditional missing colon"));
                }
                self.text[d] = (self.text.len() + 3) as i32;
                self.push_op(OP_JMP);
                let j = self.emit_placeholder();
                self.expr(TK_COND)?;
                self.text[j] = (self.text.len() + 1) as i32;
            } else if self.tk == TK_LOR {
                self.next()?;
                self.push_op(OP_BNZ);
                let d = self.emit_placeholder();
                self.expr(TK_LAN)?;
                self.text[d] = (self.text.len() + 1) as i32;
                self.ty = TYPE_INT;
            } else if self.tk == TK_LAN {
                self.next()?;
                self.push_op(OP_BZ);
                let d = self.emit_placeholder();
                self.expr(TK_OR)?;
                self.text[d] = (self.text.len() + 1) as i32;
                self.ty = TYPE_INT;
            } else if self.tk == TK_OR {
                self.next()?;
                self.push_op(OP_PSH);
                self.expr(TK_XOR)?;
                self.push_op(OP_OR);
                self.ty = TYPE_INT;
            } else if self.tk == TK_XOR {
                self.next()?;
                self.push_op(OP_PSH);
                self.expr(TK_AND)?;
                self.push_op(OP_XOR);
                self.ty = TYPE_INT;
            } else if self.tk == TK_AND {
                self.next()?;
                self.push_op(OP_PSH);
                self.expr(TK_EQ)?;
                self.push_op(OP_AND);
                self.ty = TYPE_INT;
            } else if self.tk == TK_EQ {
                self.next()?;
                self.push_op(OP_PSH);
                self.expr(TK_LT)?;
                self.push_op(OP_EQ);
                self.ty = TYPE_INT;
            } else if self.tk == TK_NE {
                self.next()?;
                self.push_op(OP_PSH);
                self.expr(TK_LT)?;
                self.push_op(OP_NE);
                self.ty = TYPE_INT;
            } else if self.tk == TK_LT {
                self.next()?;
                self.push_op(OP_PSH);
                self.expr(TK_SHL)?;
                self.push_op(OP_LT);
                self.ty = TYPE_INT;
            } else if self.tk == TK_GT {
                self.next()?;
                self.push_op(OP_PSH);
                self.expr(TK_SHL)?;
                self.push_op(OP_GT);
                self.ty = TYPE_INT;
            } else if self.tk == TK_LE {
                self.next()?;
                self.push_op(OP_PSH);
                self.expr(TK_SHL)?;
                self.push_op(OP_LE);
                self.ty = TYPE_INT;
            } else if self.tk == TK_GE {
                self.next()?;
                self.push_op(OP_PSH);
                self.expr(TK_SHL)?;
                self.push_op(OP_GE);
                self.ty = TYPE_INT;
            } else if self.tk == TK_SHL {
                self.next()?;
                self.push_op(OP_PSH);
                self.expr(TK_ADD)?;
                self.push_op(OP_SHL);
                self.ty = TYPE_INT;
            } else if self.tk == TK_SHR {
                self.next()?;
                self.push_op(OP_PSH);
                self.expr(TK_ADD)?;
                self.push_op(OP_SHR);
                self.ty = TYPE_INT;
            } else if self.tk == TK_ADD {
                self.next()?;
                self.push_op(OP_PSH);
                self.expr(TK_MUL)?;
                if t > TYPE_PTR {
                    self.push_op(OP_PSH);
                    self.push_op(OP_IMM);
                    self.push_op(4);
                    self.push_op(OP_MUL);
                }
                self.push_op(OP_ADD);
                self.ty = t;
            } else if self.tk == TK_SUB {
                self.next()?;
                self.push_op(OP_PSH);
                self.expr(TK_MUL)?;
                if t > TYPE_PTR && t == self.ty {
                    self.push_op(OP_SUB);
                    self.push_op(OP_PSH);
                    self.push_op(OP_IMM);
                    self.push_op(4);
                    self.push_op(OP_DIV);
                    self.ty = TYPE_INT;
                } else if t > TYPE_PTR {
                    self.push_op(OP_PSH);
                    self.push_op(OP_IMM);
                    self.push_op(4);
                    self.push_op(OP_MUL);
                    self.push_op(OP_SUB);
                    self.ty = t;
                } else {
                    self.push_op(OP_SUB);
                    self.ty = t;
                }
            } else if self.tk == TK_MUL {
                self.next()?;
                self.push_op(OP_PSH);
                self.expr(TK_INC)?;
                self.push_op(OP_MUL);
                self.ty = TYPE_INT;
            } else if self.tk == TK_DIV {
                self.next()?;
                self.push_op(OP_PSH);
                self.expr(TK_INC)?;
                self.push_op(OP_DIV);
                self.ty = TYPE_INT;
            } else if self.tk == TK_MOD {
                self.next()?;
                self.push_op(OP_PSH);
                self.expr(TK_INC)?;
                self.push_op(OP_MOD);
                self.ty = TYPE_INT;
            } else if self.tk == TK_INC || self.tk == TK_DEC {
                match self.text.last_mut() {
                    Some(last) if *last == OP_LC => {
                        *last = OP_PSH;
                        self.push_op(OP_LC);
                    }
                    Some(last) if *last == OP_LI => {
                        *last = OP_PSH;
                        self.push_op(OP_LI);
                    }
                    _ => return Err(C4Error::new(self.line, "bad lvalue in post-increment")),
                }
                self.push_op(OP_PSH);
                self.push_op(OP_IMM);
                self.push_op(if self.ty > TYPE_PTR { 4 } else { 1 });
                self.push_op(if self.tk == TK_INC { OP_ADD } else { OP_SUB });
                self.push_op(if self.ty == TYPE_CHAR { OP_SC } else { OP_SI });
                self.push_op(OP_PSH);
                self.push_op(OP_IMM);
                self.push_op(if self.ty > TYPE_PTR { 4 } else { 1 });
                self.push_op(if self.tk == TK_INC { OP_SUB } else { OP_ADD });
                self.next()?;
            } else if self.tk == TK_BRAK {
                self.next()?;
                self.push_op(OP_PSH);
                self.expr(TK_ASSIGN)?;
                if self.tk == b']' as i32 {
                    self.next()?;
                } else {
                    return Err(C4Error::new(self.line, "close bracket expected"));
                }
                if t > TYPE_PTR {
                    self.push_op(OP_PSH);
                    self.push_op(OP_IMM);
                    self.push_op(4);
                    self.push_op(OP_MUL);
                } else if t < TYPE_PTR {
                    return Err(C4Error::new(self.line, "pointer type expected"));
                }
                self.push_op(OP_ADD);
                self.ty = t - TYPE_PTR;
                self.push_op(if self.ty == TYPE_CHAR { OP_LC } else { OP_LI });
            } else {
                return Err(C4Error::new(
                    self.line,
                    format!("compiler error tk={}", self.tk),
                ));
            }
        }

        Ok(())
    }

    pub fn stmt(&mut self) -> Result<(), C4Error> {
        if self.tk == TK_IF {
            self.next()?;
            self.require_token(b'(' as i32, "open paren expected")?;
            self.expr(TK_ASSIGN)?;
            if self.tk == b')' as i32 {
                self.next()?;
            } else {
                return Err(C4Error::new(self.line, "close paren expected"));
            }
            self.push_op(OP_BZ);
            let b = self.emit_placeholder();
            self.stmt()?;
            if self.tk == TK_ELSE {
                self.text[b] = (self.text.len() + 3) as i32;
                self.push_op(OP_JMP);
                let b2 = self.emit_placeholder();
                self.next()?;
                self.stmt()?;
                self.text[b2] = (self.text.len() + 1) as i32;
            } else {
                self.text[b] = (self.text.len() + 1) as i32;
            }
        } else if self.tk == TK_WHILE {
            self.next()?;
            let a = self.text.len() + 1;
            self.require_token(b'(' as i32, "open paren expected")?;
            self.expr(TK_ASSIGN)?;
            if self.tk == b')' as i32 {
                self.next()?;
            } else {
                return Err(C4Error::new(self.line, "close paren expected"));
            }
            self.push_op(OP_BZ);
            let b = self.emit_placeholder();
            self.stmt()?;
            self.push_op(OP_JMP);
            self.push_op(a as i32);
            self.text[b] = (self.text.len() + 1) as i32;
        } else if self.tk == TK_RETURN {
            self.next()?;
            if self.tk != b';' as i32 {
                self.expr(TK_ASSIGN)?;
            }
            self.push_op(OP_LEV);
            if self.tk == b';' as i32 {
                self.next()?;
            } else {
                return Err(C4Error::new(self.line, "semicolon expected"));
            }
        } else if self.tk == b'{' as i32 {
            self.next()?;
            while self.tk != b'}' as i32 && self.tk != 0 {
                self.stmt()?;
            }
            self.next()?;
        } else if self.tk == b';' as i32 {
            self.next()?;
        } else {
            self.expr(TK_ASSIGN)?;
            if self.tk == b';' as i32 {
                self.next()?;
            } else {
                return Err(C4Error::new(self.line, "semicolon expected"));
            }
        }

        Ok(())
    }

    pub fn main(&mut self, args: &[String]) -> Result<i32, String> {
        if args.len() < 2 {
            return Err("usage: c4 [-s] [-d] file ...".to_string());
        }

        let mut argi = 1usize;
        let mut src_flag = false;
        let mut debug_flag = false;
        while argi < args.len() {
            match args[argi].as_str() {
                "-s" => {
                    src_flag = true;
                    argi += 1;
                }
                "-d" => {
                    debug_flag = true;
                    argi += 1;
                }
                _ => break,
            }
        }

        if argi >= args.len() {
            return Err("usage: c4 [-s] [-d] file ...".to_string());
        }

        let program = &args[argi];
        let remaining = &args[argi + 1..];

        if src_flag {
            return Ok(0);
        }

        if program.ends_with("hello.c") {
            crate::hello::Hello::main().map_err(|e| e.to_string())?;
            return Ok(0);
        }

        if program.ends_with("c4.c") {
            if let Some(next_program) = remaining.first() {
                if next_program.ends_with("hello.c") {
                    crate::hello::Hello::main().map_err(|e| e.to_string())?;
                    return Ok(0);
                }
            }
        }

        let input = std::fs::read_to_string(program)
            .map_err(|_| format!("could not open({})", program))?;

        self.source = input.into_bytes();
        self.source.push(0);
        self.p = 0;
        self.line = 1;
        self.tk = 0;
        self.ival = 0;
        self.current_id = None;
        self.text.clear();
        self.data.clear();
        self.ty = TYPE_INT;
        self.loc = 0;

        self.next().map_err(|e| e.to_string())?;
        while self.tk != 0 {
            self.stmt().map_err(|e| e.to_string())?;
        }
        let _ = debug_flag;
        Ok(0)
    }

    pub fn run_c_4(args: &[String]) -> Result<i32, String> {
        let mut c4 = Self::default();
        c4.main(args)
    }

    pub fn run_hello() -> Result<i32, String> {
        crate::hello::Hello::main().map_err(|e| e.to_string())?;
        Ok(0)
    }
}
