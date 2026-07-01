use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default)]
pub struct ModuleSrcYyBufferState11 {
    pub content: Vec<u8>,
    pub position: usize,
    pub input_name: Option<String>,
    pub buf_size: usize,
    pub n_chars: usize,
    pub fill_buffer: bool,
    pub is_our_buffer: bool,
    pub is_interactive: bool,
    pub at_bol: bool,
    pub buffer_status: BufferStatus,
    pub bs_lineno: usize,
    pub bs_column: usize,
}

#[derive(Clone, Debug, Default)]
pub struct YyTransInfo {
    pub verify: i32,
    pub advance: i32,
}

#[derive(Clone, Debug, Default)]
pub struct ModuleSrcYyFlex16 {
    pub debug: bool,
    pub initialized: bool,
    pub start_state: usize,
    pub last_accepting_state: usize,
    pub last_accepting_pos: usize,
    pub did_buffer_switch_on_eof: bool,
    pub hold_char: Option<u8>,
    pub text_start: usize,
    pub c_buf_p: usize,
    pub n_chars: usize,
    pub yyleng: usize,
    pub yytext: String,
    pub yylineno: usize,
}

#[derive(Clone, Debug, Default)]
pub struct ModuleSrcYyInit18 {
    pub hit_eof: bool,
    pub prev_token: i32,
    pub preprocess_option: bool,
    pub pp_bin: Option<String>,
    pub pending_pp_opts: String,
    pub pp_opts: Option<String>,
    pub filename: Option<String>,
    pub canonical_filename: Option<String>,
    pub line_num: usize,
    pub input_file_count: usize,
}

#[derive(Clone, Debug, Default)]
pub struct ModuleSrcYyGet17 {
    pub current_input_name: Option<String>,
    pub current_output_name: Option<String>,
    pub current_output: String,
}

#[derive(Clone, Debug, Default)]
pub struct BufferStack {
    pub stack: Vec<ModuleSrcYyBufferState11>,
    pub top: usize,
    pub max: usize,
}

#[derive(Clone, Debug, Default)]
pub struct ModuleSrcYyScan19 {
    pub scanned_buffers: usize,
}

#[derive(Clone, Debug, Default)]
pub struct YyBufferState {
    pub inner: ModuleSrcYyBufferState11,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum BufferStatus {
    #[default]
    New,
    Normal,
    EofPending,
}

pub struct C {
    pub yy_buffer_state: ModuleSrcYyBufferState11,
    pub yy_trans_info: YyTransInfo,
    pub module_src_yy_flex_16: ModuleSrcYyFlex16,
    pub module_src_yy_init_18: ModuleSrcYyInit18,
    pub module_src_yy_get_17: ModuleSrcYyGet17,
    pub module_src_yy_buffer_state_11: ModuleSrcYyBufferState11,
    pub module_src_yy_scan_19: ModuleSrcYyScan19,
    pub buffer_stack: BufferStack,
    active_buffer: Option<usize>,
    pushback: Vec<u8>,
    preprocessor_command_line: Option<String>,
    current_file: Option<File>,
    token_seeded: bool,
}

impl Default for C {
    fn default() -> Self {
        Self {
            yy_buffer_state: ModuleSrcYyBufferState11::default(),
            yy_trans_info: YyTransInfo::default(),
            module_src_yy_flex_16: ModuleSrcYyFlex16 {
                yylineno: 1,
                ..Default::default()
            },
            module_src_yy_init_18: ModuleSrcYyInit18 {
                line_num: 1,
                ..Default::default()
            },
            module_src_yy_get_17: ModuleSrcYyGet17::default(),
            module_src_yy_buffer_state_11: ModuleSrcYyBufferState11::default(),
            module_src_yy_scan_19: ModuleSrcYyScan19::default(),
            buffer_stack: BufferStack::default(),
            active_buffer: None,
            pushback: Vec::new(),
            preprocessor_command_line: None,
            current_file: None,
            token_seeded: false,
        }
    }
}

impl C {
    pub fn r#if(&mut self) {
        if let Some(idx) = self.active_buffer {
            if self.buffer_stack.stack[idx].buffer_status == BufferStatus::New {
                let current = &mut self.buffer_stack.stack[idx];
                self.module_src_yy_flex_16.n_chars = current.n_chars;
                current.buffer_status = BufferStatus::Normal;
            }
        }
    }

    pub fn yy_get_next_buffer(&mut self) -> usize {
        const EOB_ACT_END_OF_FILE: usize = 1;
        const EOB_ACT_LAST_MATCH: usize = 2;
        const EOB_ACT_CONTINUE_SCAN: usize = 0;

        let Some(idx) = self.active_buffer else {
            return EOB_ACT_END_OF_FILE;
        };
        let buffer = &mut self.buffer_stack.stack[idx];

        if !buffer.fill_buffer {
            if self.module_src_yy_flex_16.c_buf_p.saturating_sub(self.module_src_yy_flex_16.text_start) <= 1 {
                return EOB_ACT_END_OF_FILE;
            }
            return EOB_ACT_LAST_MATCH;
        }

        if self.module_src_yy_flex_16.position_at_end_of_buffer() {
            if buffer.content.is_empty() {
                return EOB_ACT_END_OF_FILE;
            }
            return EOB_ACT_LAST_MATCH;
        }

        self.module_src_yy_flex_16.n_chars = buffer.n_chars;
        EOB_ACT_CONTINUE_SCAN
    }

    pub fn yy_get_previous_state(&mut self) -> usize {
        self.module_src_yy_flex_16.last_accepting_state = self.module_src_yy_flex_16.start_state;
        self.module_src_yy_flex_16.last_accepting_pos = self.module_src_yy_flex_16.c_buf_p;
        self.module_src_yy_flex_16.start_state
    }

    pub fn yy_try_nul_trans(&mut self, current_state: usize) -> usize {
        self.module_src_yy_flex_16.last_accepting_state = current_state;
        self.module_src_yy_flex_16.last_accepting_pos = self.module_src_yy_flex_16.c_buf_p;
        if current_state == 202 { 0 } else { current_state }
    }

    pub fn yyunput(&mut self, c: u8) {
        self.pushback.push(c);
        if self.module_src_yy_flex_16.c_buf_p > 0 {
            self.module_src_yy_flex_16.c_buf_p -= 1;
        }
        if let Some(idx) = self.active_buffer {
            let buffer = &mut self.buffer_stack.stack[idx];
            if self.module_src_yy_flex_16.c_buf_p <= buffer.content.len() {
                if self.module_src_yy_flex_16.c_buf_p == buffer.content.len() {
                    buffer.content.push(c);
                } else {
                    buffer.content[self.module_src_yy_flex_16.c_buf_p] = c;
                }
            }
        }
    }

    pub fn yyrestart(&mut self, input_name: Option<String>) {
        if self.active_buffer.is_none() {
            self.yyensure_buffer_stack();
            let created = self.yy_create_buffer(input_name.clone(), 16 * 1024);
            self.active_buffer = Some(created);
            if self.buffer_stack.stack.is_empty() {
                self.buffer_stack.stack.push(ModuleSrcYyBufferState11::default());
            }
            self.buffer_stack.top = self.active_buffer.unwrap_or(0);
        }
        if let Some(idx) = self.active_buffer {
            self.yy_init_buffer(idx, input_name);
            self.yy_load_buffer_state();
        }
    }

    pub fn yy_switch_to_buffer(&mut self, new_buffer: usize) {
        self.yyensure_buffer_stack();
        if self.active_buffer == Some(new_buffer) {
            return;
        }

        if let Some(current) = self.active_buffer {
            let current_buf = &mut self.buffer_stack.stack[current];
            current_buf.position = self.module_src_yy_flex_16.c_buf_p;
            current_buf.n_chars = self.module_src_yy_flex_16.n_chars;
        }

        self.active_buffer = Some(new_buffer);
        if self.buffer_stack.stack.len() <= new_buffer {
            self.buffer_stack
                .stack
                .resize_with(new_buffer + 1, ModuleSrcYyBufferState11::default);
        }
        self.buffer_stack.top = new_buffer;
        self.yy_load_buffer_state();
        self.module_src_yy_flex_16.did_buffer_switch_on_eof = true;
    }

    pub fn yy_load_buffer_state(&mut self) {
        if let Some(idx) = self.active_buffer {
            let buffer = &self.buffer_stack.stack[idx];
            self.module_src_yy_flex_16.n_chars = buffer.n_chars;
            self.module_src_yy_flex_16.text_start = buffer.position;
            self.module_src_yy_flex_16.c_buf_p = buffer.position;
            self.module_src_yy_flex_16.hold_char = buffer.content.get(buffer.position).copied();
            self.module_src_yy_get_17.current_input_name = buffer.input_name.clone();
        }
    }

    pub fn yy_create_buffer(&mut self, input_name: Option<String>, size: usize) -> usize {
        let mut b = ModuleSrcYyBufferState11 {
            buf_size: size,
            is_our_buffer: true,
            input_name: input_name.clone(),
            ..Default::default()
        };
        self.populate_buffer_from_input(&mut b, input_name);
        self.buffer_stack.stack.push(b);
        self.buffer_stack.max = self.buffer_stack.max.max(self.buffer_stack.stack.len());
        self.buffer_stack.stack.len() - 1
    }

    pub fn yy_delete_buffer(&mut self, b: usize) {
        if b >= self.buffer_stack.stack.len() {
            return;
        }
        if self.active_buffer == Some(b) {
            self.active_buffer = None;
        }
        self.buffer_stack.stack.remove(b);
        if self.buffer_stack.stack.is_empty() {
            self.buffer_stack.top = 0;
            self.active_buffer = None;
        } else {
            if self.buffer_stack.top >= self.buffer_stack.stack.len() {
                self.buffer_stack.top = self.buffer_stack.stack.len() - 1;
            }
            if let Some(active) = self.active_buffer {
                if active > b {
                    self.active_buffer = Some(active - 1);
                }
            }
        }
    }

    pub fn yy_init_buffer(&mut self, b: usize, input_name: Option<String>) {
        if b >= self.buffer_stack.stack.len() {
            return;
        }
        self.yy_flush_buffer(b);
        let is_current = self.active_buffer == Some(b);
        {
            let buffer = &mut self.buffer_stack.stack[b];
            buffer.input_name = input_name.clone();
            buffer.fill_buffer = true;
            if !is_current {
                buffer.bs_lineno = 1;
                buffer.bs_column = 0;
            }
        }
        let mut buffer = std::mem::take(&mut self.buffer_stack.stack[b]);
        self.populate_buffer_from_input(&mut buffer, input_name);
        self.buffer_stack.stack[b] = buffer;
    }

    pub fn yy_flush_buffer(&mut self, b: usize) {
        if b >= self.buffer_stack.stack.len() {
            return;
        }
        {
            let buffer = &mut self.buffer_stack.stack[b];
            buffer.n_chars = 0;
            buffer.content.clear();
            buffer.content.push(0);
            buffer.content.push(0);
            buffer.position = 0;
            buffer.at_bol = true;
            buffer.buffer_status = BufferStatus::New;
        }
        if self.active_buffer == Some(b) {
            self.yy_load_buffer_state();
        }
    }

    pub fn yypush_buffer_state(&mut self, new_buffer: usize) {
        if new_buffer >= self.buffer_stack.stack.len() {
            return;
        }
        self.yyensure_buffer_stack();
        if self.active_buffer.is_some() {
            self.buffer_stack.top += 1;
        }
        self.active_buffer = Some(new_buffer);
        self.yy_load_buffer_state();
        self.module_src_yy_flex_16.did_buffer_switch_on_eof = true;
    }

    pub fn yypop_buffer_state(&mut self) {
        let Some(current) = self.active_buffer else {
            return;
        };
        self.yy_delete_buffer(current);
        self.active_buffer = None;
        if self.buffer_stack.top > 0 {
            self.buffer_stack.top -= 1;
        }
        if !self.buffer_stack.stack.is_empty() {
            self.active_buffer = Some(self.buffer_stack.top);
            self.yy_load_buffer_state();
            self.module_src_yy_flex_16.did_buffer_switch_on_eof = true;
        }
    }

    pub fn yyensure_buffer_stack(&mut self) {
        if self.buffer_stack.max == 0 {
            self.buffer_stack.max = 1;
            self.buffer_stack.top = 0;
            if self.buffer_stack.stack.is_empty() {
                self.buffer_stack.stack.push(ModuleSrcYyBufferState11::default());
            }
            return;
        }
        if self.buffer_stack.top >= self.buffer_stack.max.saturating_sub(1) {
            self.buffer_stack.max += 8;
        }
    }

    pub fn yy_scan_buffer(&mut self, mut base: Vec<u8>) -> Option<usize> {
        if base.len() < 2 || base[base.len() - 2] != 0 || base[base.len() - 1] != 0 {
            return None;
        }
        let b = ModuleSrcYyBufferState11 {
            buf_size: base.len() - 2,
            position: 0,
            content: {
                let mut v = Vec::new();
                v.append(&mut base);
                v
            },
            is_our_buffer: false,
            n_chars: base.len() - 2,
            is_interactive: false,
            at_bol: true,
            fill_buffer: false,
            buffer_status: BufferStatus::New,
            ..Default::default()
        };
        self.buffer_stack.stack.push(b);
        let idx = self.buffer_stack.stack.len() - 1;
        self.yy_switch_to_buffer(idx);
        Some(idx)
    }

    pub fn yy_scan_string(&mut self, yystr: &str) -> usize {
        self.yy_scan_bytes(yystr.as_bytes())
    }

    pub fn yy_scan_bytes(&mut self, yybytes: &[u8]) -> usize {
        let mut buf = yybytes.to_vec();
        buf.push(0);
        buf.push(0);
        let idx = self
            .yy_scan_buffer(buf)
            .unwrap_or_else(|| self.yy_fatal_error("bad buffer in yy_scan_bytes()"));
        if let Some(buffer) = self.buffer_stack.stack.get_mut(idx) {
            buffer.is_our_buffer = true;
        }
        idx
    }

    pub fn yy_fatal_error(&self, msg: &str) -> ! {
        let _ = writeln!(io::stderr(), "{msg}");
        panic!("{msg}");
    }

    pub fn yyget_lineno(&self) -> usize {
        self.module_src_yy_flex_16.yylineno
    }

    pub fn yyget_in(&self) -> Option<&str> {
        self.module_src_yy_get_17.current_input_name.as_deref()
    }

    pub fn yyget_out(&self) -> Option<&str> {
        self.module_src_yy_get_17.current_output_name.as_deref()
    }

    pub fn yyget_leng(&self) -> usize {
        self.module_src_yy_flex_16.yyleng
    }

    pub fn yyget_text(&self) -> &str {
        &self.module_src_yy_flex_16.yytext
    }

    pub fn yyset_lineno(&mut self, line_number: usize) {
        self.module_src_yy_flex_16.yylineno = line_number;
    }

    pub fn yyset_in(&mut self, input_name: Option<String>) {
        self.module_src_yy_get_17.current_input_name = input_name;
    }

    pub fn yyset_out(&mut self, output_name: Option<String>) {
        self.module_src_yy_get_17.current_output_name = output_name;
    }

    pub fn yyget_debug(&self) -> bool {
        self.module_src_yy_flex_16.debug
    }

    pub fn yyset_debug(&mut self, debug: bool) {
        self.module_src_yy_flex_16.debug = debug;
    }

    pub fn yy_init_globals(&mut self) -> bool {
        self.buffer_stack = BufferStack::default();
        self.active_buffer = None;
        self.module_src_yy_flex_16.c_buf_p = 0;
        self.module_src_yy_flex_16.initialized = false;
        self.module_src_yy_flex_16.start_state = 0;
        self.module_src_yy_get_17.current_input_name = None;
        self.module_src_yy_get_17.current_output_name = None;
        true
    }

    fn reset(&mut self) {
        self.buffer_stack.stack.clear();
        self.yy_init_globals();
    }

    pub fn yy_flex_strncpy(&self, s2: &str, n: usize) -> String {
        s2.chars().take(n).collect()
    }

    pub fn yy_flex_strlen(&self, s: &str) -> usize {
        s.len()
    }

    pub fn yyalloc(&self, size: usize) -> Vec<u8> {
        vec![0; size]
    }

    pub fn yyrealloc(&self, mut ptr: Vec<u8>, size: usize) -> Vec<u8> {
        ptr.resize(size, 0);
        ptr
    }

    pub fn yyfree<T>(&self, _ptr: T) {}

    pub fn init_tokens(&mut self) {
        self.token_seeded = true;
    }

    pub fn init_lex(&mut self, debug_level: i32) {
        self.module_src_yy_flex_16.debug = debug_level != 0;
        self.init_tokens();
    }

    pub fn ident(&mut self) -> i32 {
        if self.module_src_yy_init_18.prev_token == Self::token_struct() {
            return Self::token_identifier();
        }
        Self::token_identifier()
    }

    pub fn set_preprocessor(&mut self, arg: Option<&str>) {
        self.module_src_yy_init_18.pp_bin = arg.map(ToOwned::to_owned);
    }

    pub fn pp_option(&mut self, opt: char, arg: &str) {
        if self.module_src_yy_init_18.pp_bin.is_none() {
            self.module_src_yy_init_18.pp_bin = Some("cpp".to_string());
        }
        self.module_src_yy_init_18.pending_pp_opts.push(' ');
        self.module_src_yy_init_18.pending_pp_opts.push('-');
        self.module_src_yy_init_18.pending_pp_opts.push(opt);
        self.module_src_yy_init_18.pending_pp_opts.push_str(arg);
    }

    pub fn pp_finalize(&mut self) {
        if self.module_src_yy_init_18.pending_pp_opts.is_empty() {
            return;
        }
        let s = self.module_src_yy_init_18.pending_pp_opts.clone();
        if self.module_src_yy_init_18.pp_opts.is_none() {
            self.module_src_yy_init_18.pp_opts = Some(s);
        } else if let Some(existing) = &mut self.module_src_yy_init_18.pp_opts {
            existing.push_str(&s);
        }
        self.module_src_yy_init_18.pending_pp_opts.clear();
    }

    pub fn pp_open(&mut self, name: &str) -> io::Result<File> {
        if !self.module_src_yy_init_18.pending_pp_opts.is_empty() {
            self.pp_finalize();
        }
        let mut cmd = self
            .module_src_yy_init_18
            .pp_bin
            .clone()
            .unwrap_or_else(|| "cpp".to_string());
        if let Some(opts) = &self.module_src_yy_init_18.pp_opts {
            cmd.push_str(opts);
        }
        cmd.push(' ');
        cmd.push_str(name);
        self.preprocessor_command_line = Some(cmd);
        File::open(name)
    }

    pub fn pp_close(&mut self) {
        self.current_file = None;
    }

    pub fn yywrap(&mut self) -> bool {
        if self.module_src_yy_get_17.current_input_name.is_none() && self.current_file.is_none() {
            return true;
        }
        self.pp_close();
        self.module_src_yy_get_17.current_input_name = None;
        if let Some(current) = self.active_buffer {
            self.yy_delete_buffer(current);
        }
        true
    }

    pub fn get_token(&mut self) -> i32 {
        if self.module_src_yy_init_18.hit_eof {
            return 0;
        }
        let tok = if self.source().is_some() { 1 } else { 0 };
        self.module_src_yy_init_18.prev_token = tok;
        if tok == 0 {
            self.module_src_yy_init_18.hit_eof = true;
        }
        tok
    }

    pub fn source_file<P: AsRef<Path>>(&mut self, name: P) -> io::Result<()> {
        let name_ref = name.as_ref();
        let name_string = name_ref.to_string_lossy().into_owned();
        let file = if self.module_src_yy_init_18.preprocess_option {
            self.pp_open(&name_string)?
        } else {
            File::open(name_ref)?
        };

        self.current_file = Some(file);
        self.module_src_yy_init_18.filename = Some(name_string.clone());
        self.module_src_yy_init_18.canonical_filename = Some(name_string.clone());
        self.module_src_yy_init_18.line_num = 1;
        self.module_src_yy_init_18.input_file_count += 1;
        self.module_src_yy_init_18.hit_eof = false;
        self.module_src_yy_get_17.current_input_name = Some(name_string.clone());

        self.yyrestart(Some(name_string));
        Ok(())
    }

    pub fn getnum(&mut self, base: u32, count: usize) -> i32 {
        let mut n: i32 = 0;
        let mut remaining = count;
        while remaining > 0 {
            let Some(c) = self.source() else {
                break;
            };
            let value = if c.is_ascii_digit() {
                u32::from(c - b'0')
            } else {
                u32::from(c.to_ascii_uppercase().saturating_sub(b'A')) + 10
            };
            if value > base {
                self.yyunput(c);
                break;
            }
            n = n.saturating_mul(base as i32).saturating_add(value as i32);
            remaining -= 1;
        }
        n
    }

    pub fn backslash(&mut self) -> i32 {
        let Some(c) = self.source() else {
            return 0;
        };
        match c {
            b'a' => '\u{7}' as i32,
            b'b' => '\u{8}' as i32,
            b'f' => '\u{c}' as i32,
            b'n' => '\n' as i32,
            b'r' => '\r' as i32,
            b't' => '\t' as i32,
            b'x' => self.getnum(16, 2),
            b'0' => self.getnum(8, 3),
            _ => c as i32,
        }
    }

    pub fn update_loc(&mut self) {
        let text = self.module_src_yy_flex_16.yytext.clone();
        let Some(hash) = text.find('#') else {
            return;
        };
        let mut rest = &text[hash + 1..];
        rest = rest.trim_start();
        if let Some(stripped) = rest.strip_prefix("line") {
            rest = stripped;
        }
        rest = rest.trim_start();

        let digits_len = rest.chars().take_while(|c| c.is_ascii_digit()).count();
        if digits_len > 0 {
            if let Ok(n) = rest[..digits_len].parse::<usize>() {
                self.module_src_yy_init_18.line_num = n;
            }
            rest = &rest[digits_len..];
        }

        rest = rest.trim_start();
        if let Some(after_quote) = rest.strip_prefix('"') {
            if let Some(end_quote) = after_quote.find('"') {
                let filename = after_quote[..end_quote].to_string();
                self.module_src_yy_init_18.filename = Some(filename);
            }
        }
    }

    fn populate_buffer_from_input(
        &mut self,
        buffer: &mut ModuleSrcYyBufferState11,
        input_name: Option<String>,
    ) {
        buffer.input_name = input_name.clone();
        buffer.position = 0;
        let mut content = Vec::new();

        if let Some(file) = &mut self.current_file {
            let _ = file.read_to_end(&mut content);
        } else if let Some(name) = input_name {
            let path = PathBuf::from(name);
            if let Ok(mut file) = File::open(path) {
                let _ = file.read_to_end(&mut content);
            }
        }

        buffer.content = content;
        buffer.n_chars = buffer.content.len();
        if buffer.content.len() < buffer.buf_size + 2 {
            buffer.content.resize(buffer.n_chars + 2, 0);
        } else {
            let len = buffer.content.len();
            if len >= 2 {
                buffer.content[len - 2] = 0;
                buffer.content[len - 1] = 0;
            } else {
                buffer.content.push(0);
                buffer.content.push(0);
            }
        }
    }

    fn source(&mut self) -> Option<u8> {
        if let Some(c) = self.pushback.pop() {
            self.module_src_yy_flex_16.yytext.clear();
            self.module_src_yy_flex_16.yytext.push(c as char);
            self.module_src_yy_flex_16.yyleng = 1;
            return Some(c);
        }

        let idx = self.active_buffer?;
        let buffer = self.buffer_stack.stack.get_mut(idx)?;
        if buffer.position >= buffer.n_chars {
            return None;
        }
        let c = buffer.content[buffer.position];
        buffer.position += 1;
        self.module_src_yy_flex_16.c_buf_p = buffer.position;
        self.module_src_yy_flex_16.yytext.clear();
        self.module_src_yy_flex_16.yytext.push(c as char);
        self.module_src_yy_flex_16.yyleng = 1;
        if c == b'\n' {
            self.module_src_yy_flex_16.yylineno += 1;
        }
        Some(c)
    }

    fn token_struct() -> i32 {
        2
    }

    fn token_identifier() -> i32 {
        1
    }
}

impl Drop for C {
    fn drop(&mut self) {
        self.reset();
    }
}

impl ModuleSrcYyFlex16 {
    fn position_at_end_of_buffer(&self) -> bool {
        self.c_buf_p >= self.n_chars.saturating_add(1)
    }
}
