use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct YyTransInfo {
    pub verify: i32,
    pub advance: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleSrcYyFlex16 {
    pub debug_level: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleSrcYyInit18 {
    pub initialized: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleSrcYyBufferState11 {
    pub name: Option<String>,
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BufferStatus {
    New,
    Normal,
    EofPending,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct YyBufferState {
    pub name: Option<String>,
    pub content: String,
    pub position: usize,
    pub is_our_buffer: bool,
    pub fill_buffer: bool,
    pub line: usize,
    pub column: usize,
    status: BufferStatus,
}

impl YyBufferState {
    fn new_named(name: Option<String>, content: String, is_our_buffer: bool) -> Self {
        Self {
            name,
            content,
            position: 0,
            is_our_buffer,
            fill_buffer: true,
            line: 1,
            column: 0,
            status: BufferStatus::New,
        }
    }

    fn remaining(&self) -> &str {
        &self.content[self.position..]
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputHandle {
    Path(PathBuf),
    Inline(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputHandle {
    Sink,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    EndOfInput,
    Identifier(String),
    Number(i32),
    StringLiteral(String),
    CharLiteral(char),
    Symbol(char),
}

pub struct C {
    current_buffer: Option<YyBufferState>,
    buffer_stack: Vec<YyBufferState>,
    debug: bool,
    current_text: String,
    current_length: usize,
    current_line: usize,
    input: Option<InputHandle>,
    output: Option<OutputHandle>,
    preprocessor: Option<String>,
    preprocessor_options: Vec<String>,
    token_table: Vec<String>,
    source_name: Option<String>,
    source_content: String,
    source_pos: usize,
}

impl Default for C {
    fn default() -> Self {
        Self {
            current_buffer: None,
            buffer_stack: Vec::new(),
            debug: false,
            current_text: String::new(),
            current_length: 0,
            current_line: 1,
            input: None,
            output: None,
            preprocessor: None,
            preprocessor_options: Vec::new(),
            token_table: Vec::new(),
            source_name: None,
            source_content: String::new(),
            source_pos: 0,
        }
    }
}

impl C {
    pub fn r#if() -> bool {
        true
    }

    pub fn yy_get_next_buffer(&mut self) -> i32 {
        let Some(buf) = self.current_buffer.as_mut() else {
            return 0;
        };

        if buf.position >= buf.content.len() {
            if !buf.fill_buffer {
                return 0;
            }
            buf.status = BufferStatus::EofPending;
            0
        } else {
            1
        }
    }

    pub fn yy_get_previous_state(&self) -> usize {
        self.current_buffer.as_ref().map(|b| b.position).unwrap_or(0)
    }

    pub fn yy_try_nul_trans(&self, current_state: usize) -> usize {
        if self
            .current_buffer
            .as_ref()
            .is_some_and(|b| b.position >= b.content.len())
        {
            0
        } else {
            current_state
        }
    }

    pub fn yyunput(&mut self, ch: char) {
        if let Some(buf) = self.current_buffer.as_mut() {
            let step = ch.len_utf8();
            if buf.position >= step {
                buf.position -= step;
            }
            self.current_text.clear();
            self.current_length = 0;
            buf.status = BufferStatus::Normal;
        }
    }

    pub fn yyrestart(&mut self) {
        if self.current_buffer.is_none() {
            self.yyensure_buffer_stack();
            self.current_buffer = Some(self.yy_create_buffer(String::new(), None));
        }
        if let Some(mut buf) = self.current_buffer.take() {
            self.yy_init_buffer(&mut buf, self.input.clone());
            self.current_buffer = Some(buf);
            self.yy_load_buffer_state();
        }
    }

    pub fn yy_switch_to_buffer(&mut self, new_buffer: YyBufferState) {
        self.yyensure_buffer_stack();
        self.current_buffer = Some(new_buffer);
        self.yy_load_buffer_state();
    }

    pub fn yy_load_buffer_state(&mut self) {
        if let Some(buf) = self.current_buffer.as_mut() {
            self.current_line = buf.line;
            self.source_name = buf.name.clone();
            self.source_content = buf.content.clone();
            self.source_pos = buf.position;
            self.input = buf.name.clone().map(|n| InputHandle::Path(PathBuf::from(n)));
            if buf.status == BufferStatus::New {
                buf.status = BufferStatus::Normal;
            }
        }
    }

    pub fn yy_create_buffer(&self, content: String, name: Option<String>) -> YyBufferState {
        YyBufferState::new_named(name, content, true)
    }

    pub fn yy_delete_buffer(&mut self) {
        self.current_buffer = None;
    }

    pub fn yy_init_buffer(&mut self, buffer: &mut YyBufferState, input: Option<InputHandle>) {
        self.yy_flush_buffer_inner(buffer);
        buffer.fill_buffer = true;
        if buffer != self.current_buffer.as_ref().unwrap_or(buffer) {
            buffer.line = 1;
            buffer.column = 0;
        }
        if let Some(InputHandle::Path(path)) = input {
            buffer.name = Some(path.to_string_lossy().into_owned());
        }
    }

    pub fn yy_flush_buffer(&mut self) {
        if let Some(mut buf) = self.current_buffer.take() {
            self.yy_flush_buffer_inner(&mut buf);
            self.current_buffer = Some(buf);
            self.yy_load_buffer_state();
        }
    }

    fn yy_flush_buffer_inner(&mut self, buffer: &mut YyBufferState) {
        buffer.position = 0;
        buffer.status = BufferStatus::New;
    }

    pub fn yypush_buffer_state(&mut self, new_buffer: YyBufferState) {
        self.yyensure_buffer_stack();
        if let Some(current) = self.current_buffer.take() {
            self.buffer_stack.push(current);
        }
        self.current_buffer = Some(new_buffer);
        self.yy_load_buffer_state();
    }

    pub fn yypop_buffer_state(&mut self) {
        self.current_buffer = self.buffer_stack.pop();
        if self.current_buffer.is_some() {
            self.yy_load_buffer_state();
        }
    }

    pub fn yyensure_buffer_stack(&mut self) {
        if self.buffer_stack.capacity() == 0 {
            self.buffer_stack.reserve(8);
        }
    }

    pub fn yy_scan_buffer(&self, buffer: String) -> YyBufferState {
        YyBufferState::new_named(None, buffer, false)
    }

    pub fn yy_scan_string(&self, text: &str) -> YyBufferState {
        self.yy_scan_bytes(text.as_bytes())
    }

    pub fn yy_scan_bytes(&self, bytes: &[u8]) -> YyBufferState {
        let content = String::from_utf8_lossy(bytes).into_owned();
        self.yy_scan_buffer(content)
    }

    pub fn yy_fatal_error(message: &str) -> ! {
        panic!("{message}")
    }

    pub fn yyget_lineno(&self) -> usize {
        self.current_line
    }

    pub fn yyget_in(&self) -> Option<&InputHandle> {
        self.input.as_ref()
    }

    pub fn yyget_out(&self) -> Option<&OutputHandle> {
        self.output.as_ref()
    }

    pub fn yyget_leng(&self) -> usize {
        self.current_length
    }

    pub fn yyget_text(&self) -> &str {
        &self.current_text
    }

    pub fn yyset_lineno(&mut self, line_number: usize) {
        self.current_line = line_number;
        if let Some(buf) = self.current_buffer.as_mut() {
            buf.line = line_number;
        }
    }

    pub fn yyset_in(&mut self, input: InputHandle) {
        self.input = Some(input);
    }

    pub fn yyset_out(&mut self, output: OutputHandle) {
        self.output = Some(output);
    }

    pub fn yyget_debug(&self) -> bool {
        self.debug
    }

    pub fn yyset_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn yy_init_globals(&mut self) -> bool {
        *self = Self::default();
        true
    }

    pub fn reset(&mut self) -> bool {
        self.buffer_stack.clear();
        self.current_buffer = None;
        self.yy_init_globals()
    }
    pub fn yy_flex_strncpy(dst: &mut String, src: &str, n: usize) {
        dst.clear();
        dst.extend(src.chars().take(n));
    }

    pub fn yy_flex_strlen(src: &str) -> usize {
        src.len()
    }

    pub fn alloc_buffer(size: usize) -> Vec<u8> {
        vec![0; size]
    }

    pub fn resize_buffer(mut buffer: Vec<u8>, size: usize) -> Vec<u8> {
        buffer.resize(size, 0);
        buffer
    }

    pub fn yyfree<T>(_value: T) {}

    pub fn init_tokens(&mut self) {
        self.token_table = vec![
            "identifier".to_string(),
            "number".to_string(),
            "string".to_string(),
            "char".to_string(),
        ];
    }

    pub fn init_lex(&mut self, debug_level: i32) {
        self.yy_init_globals();
        self.init_tokens();
        self.debug = debug_level > 0;
    }

    pub fn ident(&self, text: &str) -> Token {
        Token::Identifier(text.to_string())
    }

    pub fn set_preprocessor(&mut self, arg: &str) {
        self.preprocessor = Some(arg.to_string());
    }

    pub fn pp_option(&mut self, opt: i32, arg: &str) {
        self.preprocessor_options.push(format!("{opt}:{arg}"));
    }

    pub fn pp_finalize(&mut self) {
        self.preprocessor = None;
        self.preprocessor_options.clear();
        self.source_name = None;
        self.source_content.clear();
        self.source_pos = 0;
    }

    pub fn pp_open<P: AsRef<Path>>(&mut self, name: P) -> Option<PathBuf> {
        self.pp_finalize();
        let path = name.as_ref().to_path_buf();
        let content = fs::read_to_string(&path).ok()?;
        let display_name = path.to_string_lossy().into_owned();

        self.source_name = Some(display_name.clone());
        self.source_content = content.clone();
        self.source_pos = 0;
        self.input = Some(InputHandle::Path(path.clone()));
        self.current_buffer = Some(self.yy_create_buffer(content, Some(display_name)));
        self.yy_load_buffer_state();

        Some(path)
    }

    pub fn pp_close(&mut self) {
        self.input = None;
        self.current_buffer = None;
        self.source_content.clear();
        self.source_pos = 0;
    }

    pub fn yywrap(&mut self) -> bool {
        self.pp_close();
        true
    }

    pub fn get_token(&mut self) -> Token {
        self.skip_whitespace();

        let Some(ch) = self.source() else {
            return Token::EndOfInput;
        };

        if ch == '_' || ch.is_ascii_alphabetic() {
            let mut text = String::from(ch);
            while let Some(next) = self.peek_char() {
                if next == '_' || next.is_ascii_alphanumeric() {
                    self.consume_char();
                    text.push(next);
                } else {
                    break;
                }
            }
            self.current_length = text.len();
            self.current_text = text.clone();
            return self.ident(&text);
        }

        if ch.is_ascii_digit() {
            self.unconsume_char(ch);
            let value = self.read_decimal_number();
            let text = value.to_string();
            self.current_length = text.len();
            self.current_text = text;
            return Token::Number(value);
        }

        if ch == '"' {
            let mut text = String::new();
            while let Some(next) = self.source() {
                if next == '"' {
                    break;
                }
                if next == '\\' {
                    text.push(self.backslash());
                } else {
                    text.push(next);
                }
            }
            self.current_length = text.len();
            self.current_text = text.clone();
            return Token::StringLiteral(text);
        }

        if ch == '\'' {
            let value = match self.source() {
                Some('\\') => self.backslash(),
                Some(c) => c,
                None => '\0',
            };
            let _ = self.source();
            self.current_text = value.to_string();
            self.current_length = self.current_text.len();
            return Token::CharLiteral(value);
        }

        self.current_text = ch.to_string();
        self.current_length = self.current_text.len();
        Token::Symbol(ch)
    }

    pub fn source(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.consume_char();
        Some(ch)
    }

    pub fn getnum(&mut self, base: u32, count: usize) -> i32 {
        let mut value = 0i32;
        let mut used = 0usize;

        while used < count {
            let Some(ch) = self.peek_char() else {
                break;
            };
            let Some(digit) = ch.to_digit(base) else {
                break;
            };
            self.consume_char();
            value = value.saturating_mul(base as i32).saturating_add(digit as i32);
            used += 1;
        }

        value
    }

    pub fn backslash(&mut self) -> char {
        match self.source() {
            Some('a') => '\u{0007}',
            Some('b') => '\u{0008}',
            Some('f') => '\u{000c}',
            Some('n') => '\n',
            Some('r') => '\r',
            Some('t') => '\t',
            Some('v') => '\u{000b}',
            Some('\\') => '\\',
            Some('\'') => '\'',
            Some('"') => '"',
            Some('?') => '?',
            Some('x') => char::from_u32(self.getnum(16, usize::MAX) as u32).unwrap_or('\0'),
            Some(c @ '0'..='7') => {
                self.unconsume_char(c);
                char::from_u32(self.getnum(8, 3) as u32).unwrap_or('\0')
            }
            Some('\n') => '\n',
            Some(other) => other,
            None => '\\',
        }
    }

    pub fn update_loc(&mut self, ch: char) {
        if ch == '\n' {
            self.current_line += 1;
            if let Some(buf) = self.current_buffer.as_mut() {
                buf.line += 1;
                buf.column = 0;
            }
        } else if let Some(buf) = self.current_buffer.as_mut() {
            buf.column += 1;
        }
    }

    pub fn module_src_yy_scan_19(&self) -> ModuleSrcYyFlex16 {
        ModuleSrcYyFlex16 {
            debug_level: i32::from(self.debug),
        }
    }

    fn peek_char(&self) -> Option<char> {
        if let Some(buf) = self.current_buffer.as_ref() {
            return buf.remaining().chars().next();
        }
        self.source_content[self.source_pos..].chars().next()
    }

    fn consume_char(&mut self) {
        if let Some(ch) = self.peek_char() {
            let len = ch.len_utf8();
            if let Some(buf) = self.current_buffer.as_mut() {
                buf.position += len;
            } else {
                self.source_pos += len;
            }
            self.update_loc(ch);
        }
    }

    fn unconsume_char(&mut self, ch: char) {
        let len = ch.len_utf8();
        if let Some(buf) = self.current_buffer.as_mut() {
            buf.position = buf.position.saturating_sub(len);
        } else {
            self.source_pos = self.source_pos.saturating_sub(len);
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek_char() {
                Some(c) if c.is_whitespace() => {
                    self.consume_char();
                }
                _ => break,
            }
        }
    }

    fn read_decimal_number(&mut self) -> i32 {
        self.getnum(10, usize::MAX)
    }
}

pub type YyBufferStateAlias = YyBufferState;
pub type ModuleSrcYyScan19 = ModuleSrcYyFlex16;
pub type YyBufferStateCompat = ModuleSrcYyBufferState11;

pub use ModuleSrcYyBufferState11 as module_src_yy_buffer_state_11;
pub use ModuleSrcYyFlex16 as module_src_yy_flex_16;
pub use ModuleSrcYyInit18 as module_src_yy_init_18;
pub use YyBufferState as yy_buffer_state;
pub use YyTransInfo as yy_trans_info;
