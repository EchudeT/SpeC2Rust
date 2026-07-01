use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub struct C {
    active_buffer: Option<YyBufferState>,
    buffer_stack: Vec<YyBufferState>,
    yy_n_chars: usize,
    yy_c_buf_p: usize,
    yytext_ptr: usize,
    yy_hold_char: char,
    yy_start: usize,
    yy_last_accepting_state: usize,
    yy_last_accepting_cpos: usize,
    yy_did_buffer_switch_on_eof: bool,
    yy_init: bool,
    yy_flex_debug: i32,
    yyin: Option<PathBuf>,
    yyout: Option<PathBuf>,
    yyleng: usize,
    yytext: String,
    yylineno: usize,
    preprocess_option: bool,
    pp_bin: Option<String>,
    pp_opts: Option<String>,
    opt_stack: Option<String>,
    hit_eof: bool,
    prev_token: i32,
    filename: Option<String>,
    canonical_filename: Option<String>,
    line_num: usize,
    input_file_count: usize,
    source_content: String,
    source_pos: usize,
    string_pool: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct YyBufferState {
    pub yy_ch_buf: Vec<char>,
    pub yy_buf_pos: usize,
    pub yy_buf_size: usize,
    pub yy_n_chars: usize,
    pub yy_is_our_buffer: bool,
    pub yy_input_file: Option<PathBuf>,
    pub yy_is_interactive: bool,
    pub yy_at_bol: bool,
    pub yy_fill_buffer: bool,
    pub yy_buffer_status: BufferStatus,
    pub yy_bs_lineno: usize,
    pub yy_bs_column: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BufferStatus {
    New,
    Normal,
    EofPending,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct YyTransInfo {
    pub verify: i32,
    pub advance: i32,
}

pub type ModuleSrcYyBufferState11 = YyBufferState;
pub type YyGet = usize;

#[derive(Clone, Debug, Default)]
pub struct ModuleSrcYyGet17 {
    pub debug: i32,
    pub in_path: Option<PathBuf>,
    pub out_path: Option<PathBuf>,
    pub leng: usize,
    pub text: String,
    pub lineno: usize,
}

#[derive(Clone, Debug, Default)]
pub struct ModuleSrcYyInit18 {
    pub debug_level: i32,
    pub initialized_tokens: usize,
}

#[derive(Clone, Debug, Default)]
pub struct ModuleSrcYyScan19 {
    pub original_len: usize,
    pub allocated_len: usize,
    pub owns_buffer: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ModuleSrcYyFlex16 {
    pub copied: usize,
    pub measured: usize,
}

impl Default for YyBufferState {
    fn default() -> Self {
        Self {
            yy_ch_buf: vec!['\0', '\0'],
            yy_buf_pos: 0,
            yy_buf_size: 0,
            yy_n_chars: 0,
            yy_is_our_buffer: true,
            yy_input_file: None,
            yy_is_interactive: false,
            yy_at_bol: true,
            yy_fill_buffer: true,
            yy_buffer_status: BufferStatus::New,
            yy_bs_lineno: 1,
            yy_bs_column: 0,
        }
    }
}

impl Default for C {
    fn default() -> Self {
        let mut c = Self {
            active_buffer: None,
            buffer_stack: Vec::new(),
            yy_n_chars: 0,
            yy_c_buf_p: 0,
            yytext_ptr: 0,
            yy_hold_char: '\0',
            yy_start: 0,
            yy_last_accepting_state: 0,
            yy_last_accepting_cpos: 0,
            yy_did_buffer_switch_on_eof: false,
            yy_init: false,
            yy_flex_debug: 0,
            yyin: None,
            yyout: None,
            yyleng: 0,
            yytext: String::new(),
            yylineno: 1,
            preprocess_option: false,
            pp_bin: None,
            pp_opts: None,
            opt_stack: None,
            hit_eof: false,
            prev_token: 0,
            filename: None,
            canonical_filename: None,
            line_num: 1,
            input_file_count: 0,
            source_content: String::new(),
            source_pos: 0,
            string_pool: Vec::new(),
        };
        let _ = c.yy_init_globals();
        c
    }
}

impl C {
    pub fn r#if() -> bool {
        true
    }

    pub fn yy_get_next_buffer(&mut self) -> i32 {
        const EOB_ACT_CONTINUE_SCAN: i32 = 0;
        const EOB_ACT_END_OF_FILE: i32 = 1;
        const EOB_ACT_LAST_MATCH: i32 = 2;

        let Some(buffer) = self.active_buffer.as_mut() else {
            return EOB_ACT_END_OF_FILE;
        };

        if self.yy_c_buf_p > self.yy_n_chars.saturating_add(1) {
            self.yy_fatal_error("fatal flex scanner internal error--end of buffer missed");
        }

        if !buffer.yy_fill_buffer {
            if self.yy_c_buf_p.saturating_sub(self.yytext_ptr) == 1 {
                return EOB_ACT_END_OF_FILE;
            }
            return EOB_ACT_LAST_MATCH;
        }

        let number_to_move = self.yy_c_buf_p.saturating_sub(self.yytext_ptr).saturating_sub(1);

        if buffer.yy_buffer_status == BufferStatus::EofPending {
            buffer.yy_n_chars = 0;
            self.yy_n_chars = 0;
        } else {
            let remaining = self
                .source_content
                .chars()
                .skip(self.source_pos)
                .take(buffer.yy_buf_size.saturating_sub(number_to_move).saturating_sub(1))
                .collect::<Vec<_>>();

            if number_to_move > 0 && self.yytext_ptr < buffer.yy_ch_buf.len() {
                let carried = buffer
                    .yy_ch_buf
                    .iter()
                    .skip(self.yytext_ptr)
                    .take(number_to_move)
                    .copied()
                    .collect::<Vec<_>>();
                for (i, ch) in carried.into_iter().enumerate() {
                    if i < buffer.yy_ch_buf.len() {
                        buffer.yy_ch_buf[i] = ch;
                    }
                }
            }

            let needed = number_to_move + remaining.len() + 2;
            if needed > buffer.yy_ch_buf.len() {
                buffer.yy_ch_buf.resize(needed, '\0');
                buffer.yy_buf_size = needed.saturating_sub(2);
            }

            for (i, ch) in remaining.iter().copied().enumerate() {
                buffer.yy_ch_buf[number_to_move + i] = ch;
            }

            self.source_pos += remaining.len();
            buffer.yy_n_chars = remaining.len();
            self.yy_n_chars = remaining.len();
        }

        let ret_val = if self.yy_n_chars == 0 {
            if number_to_move == 0 {
                self.yyrestart(self.yyin.clone());
                EOB_ACT_END_OF_FILE
            } else {
                if let Some(buffer) = self.active_buffer.as_mut() {
                    buffer.yy_buffer_status = BufferStatus::EofPending;
                }
                EOB_ACT_LAST_MATCH
            }
        } else {
            EOB_ACT_CONTINUE_SCAN
        };

        if let Some(buffer) = self.active_buffer.as_mut() {
            self.yy_n_chars += number_to_move;
            if buffer.yy_ch_buf.len() < self.yy_n_chars + 2 {
                buffer.yy_ch_buf.resize(self.yy_n_chars + 2, '\0');
                buffer.yy_buf_size = buffer.yy_ch_buf.len().saturating_sub(2);
            }
            buffer.yy_ch_buf[self.yy_n_chars] = '\0';
            buffer.yy_ch_buf[self.yy_n_chars + 1] = '\0';
            self.yytext_ptr = 0;
            buffer.yy_buf_pos = self.yy_c_buf_p.min(self.yy_n_chars);
        }

        ret_val
    }

    pub fn yy_get_previous_state(&mut self) -> usize {
        let mut current_state = self.yy_start;
        if self
            .active_buffer
            .as_ref()
            .map(|b| b.yy_at_bol)
            .unwrap_or(false)
        {
            current_state += 1;
        }

        let Some(buffer) = self.active_buffer.as_ref() else {
            return current_state;
        };

        for pos in self.yytext_ptr..self.yy_c_buf_p.min(buffer.yy_ch_buf.len()) {
            let ch = buffer.yy_ch_buf[pos];
            if ch != '\0' {
                self.yy_last_accepting_state = current_state;
                self.yy_last_accepting_cpos = pos;
                current_state = current_state.wrapping_add(ch as usize % 7 + 1);
            }
        }

        current_state
    }

    pub fn yy_try_nul_trans(&mut self, yy_current_state: usize) -> usize {
        self.yy_last_accepting_state = yy_current_state;
        self.yy_last_accepting_cpos = self.yy_c_buf_p;
        if yy_current_state == 202 {
            0
        } else {
            yy_current_state.saturating_add(1)
        }
    }

    pub fn yyunput(&mut self, c: char, yy_bp: usize) {
        let Some(buffer) = self.active_buffer.as_mut() else {
            return;
        };

        let mut yy_cp = self.yy_c_buf_p;
        if yy_cp < buffer.yy_ch_buf.len() {
            buffer.yy_ch_buf[yy_cp] = self.yy_hold_char;
        }

        if yy_cp < 2 {
            let old_len = buffer.yy_ch_buf.len();
            let grow_to = old_len + buffer.yy_buf_size.max(2) + 2;
            let mut new_buf = vec!['\0'; grow_to];
            let offset = grow_to - old_len;
            for (idx, ch) in buffer.yy_ch_buf.iter().copied().enumerate() {
                new_buf[offset + idx] = ch;
            }
            buffer.yy_ch_buf = new_buf;
            yy_cp += offset;
            self.yy_n_chars = buffer.yy_buf_size;
            buffer.yy_n_chars = buffer.yy_buf_size;
            if yy_cp < 2 {
                self.yy_fatal_error("flex scanner push-back overflow");
            }
            self.yytext_ptr = yy_bp + offset;
        } else {
            self.yytext_ptr = yy_bp;
        }

        yy_cp = yy_cp.saturating_sub(1);
        if yy_cp >= buffer.yy_ch_buf.len() {
            buffer.yy_ch_buf.resize(yy_cp + 1, '\0');
        }
        buffer.yy_ch_buf[yy_cp] = c;
        self.yy_hold_char = buffer.yy_ch_buf[yy_cp];
        self.yy_c_buf_p = yy_cp;
        buffer.yy_buf_pos = yy_cp;
    }

    pub fn yyrestart(&mut self, input_file: Option<PathBuf>) {
        if self.active_buffer.is_none() {
            self.yyensure_buffer_stack();
            self.active_buffer = Some(self.yy_create_buffer(self.yyin.clone(), 16_384));
        }
        let current = self.active_buffer.clone();
        if let Some(buf) = current {
            let mut refreshed = buf;
            self.yy_init_buffer(&mut refreshed, input_file);
            self.active_buffer = Some(refreshed);
            self.yy_load_buffer_state();
        }
    }

    pub fn yy_switch_to_buffer(&mut self, new_buffer: YyBufferState) {
        self.yyensure_buffer_stack();

        if self
            .active_buffer
            .as_ref()
            .zip(Some(&new_buffer))
            .is_some_and(|(a, b)| a.yy_ch_buf == b.yy_ch_buf && a.yy_buf_pos == b.yy_buf_pos)
        {
            return;
        }

        if let Some(current) = self.active_buffer.as_mut() {
            if self.yy_c_buf_p < current.yy_ch_buf.len() {
                current.yy_ch_buf[self.yy_c_buf_p] = self.yy_hold_char;
            }
            current.yy_buf_pos = self.yy_c_buf_p;
            current.yy_n_chars = self.yy_n_chars;
        }

        if self.buffer_stack.is_empty() {
            self.buffer_stack.push(new_buffer.clone());
        } else if let Some(top) = self.buffer_stack.last_mut() {
            *top = new_buffer.clone();
        }

        self.active_buffer = Some(new_buffer);
        self.yy_load_buffer_state();
        self.yy_did_buffer_switch_on_eof = true;
    }

    pub fn yy_load_buffer_state(&mut self) {
        if let Some(buffer) = self.active_buffer.as_ref() {
            self.yy_n_chars = buffer.yy_n_chars;
            self.yytext_ptr = buffer.yy_buf_pos;
            self.yy_c_buf_p = buffer.yy_buf_pos;
            self.yyin = buffer.yy_input_file.clone();
            self.yy_hold_char = buffer
                .yy_ch_buf
                .get(self.yy_c_buf_p)
                .copied()
                .unwrap_or('\0');
        }
    }

    pub fn yy_create_buffer(&mut self, file: Option<PathBuf>, size: usize) -> YyBufferState {
        let mut b = YyBufferState {
            yy_buf_size: size,
            yy_ch_buf: vec!['\0'; size + 2],
            yy_is_our_buffer: true,
            ..Default::default()
        };
        self.yy_init_buffer(&mut b, file);
        b
    }

    pub fn yy_delete_buffer(&mut self, b: Option<YyBufferState>) {
        let Some(buffer) = b else {
            return;
        };

        let clear_current = self
            .active_buffer
            .as_ref()
            .is_some_and(|cur| cur.yy_ch_buf == buffer.yy_ch_buf && cur.yy_buf_pos == buffer.yy_buf_pos);

        if clear_current {
            self.active_buffer = None;
        }

        self.buffer_stack
            .retain(|item| !(item.yy_ch_buf == buffer.yy_ch_buf && item.yy_buf_pos == buffer.yy_buf_pos));
    }

    pub fn yy_init_buffer(&mut self, b: &mut YyBufferState, file: Option<PathBuf>) {
        self.yy_flush_buffer(Some(b));
        b.yy_input_file = file;
        b.yy_fill_buffer = true;

        let is_current = self
            .active_buffer
            .as_ref()
            .is_some_and(|cur| cur.yy_ch_buf == b.yy_ch_buf && cur.yy_buf_pos == b.yy_buf_pos);

        if !is_current {
            b.yy_bs_lineno = 1;
            b.yy_bs_column = 0;
        }

        b.yy_is_interactive = false;
    }

    pub fn yy_flush_buffer(&mut self, buffer: Option<&mut YyBufferState>) {
        let Some(b) = buffer else {
            return;
        };

        b.yy_n_chars = 0;
        if b.yy_ch_buf.len() < 2 {
            b.yy_ch_buf.resize(2, '\0');
        }
        b.yy_ch_buf[0] = '\0';
        b.yy_ch_buf[1] = '\0';
        b.yy_buf_pos = 0;
        b.yy_at_bol = true;
        b.yy_buffer_status = BufferStatus::New;

        let is_current = self
            .active_buffer
            .as_ref()
            .is_some_and(|cur| cur.yy_ch_buf == b.yy_ch_buf && cur.yy_buf_pos == b.yy_buf_pos);

        if is_current {
            self.yy_load_buffer_state();
        }
    }

    pub fn yypush_buffer_state(&mut self, new_buffer: Option<YyBufferState>) {
        let Some(new_buffer) = new_buffer else {
            return;
        };

        self.yyensure_buffer_stack();

        if let Some(current) = self.active_buffer.as_mut() {
            if self.yy_c_buf_p < current.yy_ch_buf.len() {
                current.yy_ch_buf[self.yy_c_buf_p] = self.yy_hold_char;
            }
            current.yy_buf_pos = self.yy_c_buf_p;
            current.yy_n_chars = self.yy_n_chars;
            self.buffer_stack.push(current.clone());
        }

        self.active_buffer = Some(new_buffer);
        self.yy_load_buffer_state();
        self.yy_did_buffer_switch_on_eof = true;
    }

    pub fn yypop_buffer_state(&mut self) {
        if self.active_buffer.is_none() {
            return;
        }

        self.active_buffer = None;
        if !self.buffer_stack.is_empty() {
            self.buffer_stack.pop();
        }

        if let Some(last) = self.buffer_stack.last().cloned() {
            self.active_buffer = Some(last);
            self.yy_load_buffer_state();
            self.yy_did_buffer_switch_on_eof = true;
        }
    }

    pub fn yyensure_buffer_stack(&mut self) {
        if self.buffer_stack.is_empty() {
            self.buffer_stack.reserve(1);
        } else if self.buffer_stack.len() + 1 >= self.buffer_stack.capacity() {
            self.buffer_stack.reserve(8);
        }
    }

    pub fn yy_scan_buffer(&mut self, mut base: Vec<char>, size: usize) -> Option<YyBufferState> {
        if size < 2 || base.get(size - 2) != Some(&'\0') || base.get(size - 1) != Some(&'\0') {
            return None;
        }

        if base.len() < size {
            base.resize(size, '\0');
        }

        let b = YyBufferState {
            yy_buf_size: size - 2,
            yy_buf_pos: 0,
            yy_ch_buf: base,
            yy_is_our_buffer: false,
            yy_input_file: None,
            yy_n_chars: size - 2,
            yy_is_interactive: false,
            yy_at_bol: true,
            yy_fill_buffer: false,
            yy_buffer_status: BufferStatus::New,
            yy_bs_lineno: 1,
            yy_bs_column: 0,
        };

        self.yy_switch_to_buffer(b.clone());
        Some(b)
    }

    pub fn yy_scan_string(&mut self, yystr: &str) -> Option<YyBufferState> {
        self.yy_scan_bytes(yystr.as_bytes())
    }

    pub fn yy_scan_bytes(&mut self, yybytes: &[u8]) -> Option<YyBufferState> {
        let mut buf = yybytes.iter().map(|b| *b as char).collect::<Vec<_>>();
        buf.push('\0');
        buf.push('\0');
        let mut b = self.yy_scan_buffer(buf, yybytes.len() + 2)?;
        b.yy_is_our_buffer = true;
        self.active_buffer = Some(b.clone());
        Some(b)
    }

    pub fn yy_fatal_error(&self, msg: &str) -> ! {
        let _ = writeln!(io::stderr(), "{msg}");
        panic!("{msg}");
    }

    pub fn yyget_lineno(&self) -> usize {
        self.yylineno
    }

    pub fn yyget_in(&self) -> Option<&Path> {
        self.yyin.as_deref()
    }

    pub fn yyget_out(&self) -> Option<&Path> {
        self.yyout.as_deref()
    }

    pub fn yyget_leng(&self) -> usize {
        self.yyleng
    }

    pub fn yyget_text(&self) -> &str {
        &self.yytext
    }

    pub fn yyset_lineno(&mut self, line_number: usize) {
        self.yylineno = line_number;
    }

    pub fn yyset_in(&mut self, in_path: Option<PathBuf>) {
        self.yyin = in_path;
    }

    pub fn yyset_out(&mut self, out_path: Option<PathBuf>) {
        self.yyout = out_path;
    }

    pub fn yyget_debug(&self) -> i32 {
        self.yy_flex_debug
    }

    pub fn yyset_debug(&mut self, debug: i32) {
        self.yy_flex_debug = debug;
    }

    pub fn yy_init_globals(&mut self) -> i32 {
        self.buffer_stack.clear();
        self.active_buffer = None;
        self.yy_c_buf_p = 0;
        self.yy_init = false;
        self.yy_start = 0;
        self.yyin = None;
        self.yyout = None;
        0
    }

    fn reset(&mut self) {
        while self.active_buffer.is_some() {
            let current = self.active_buffer.clone();
            self.yy_delete_buffer(current);
            self.active_buffer = None;
            self.yypop_buffer_state();
        }

        self.buffer_stack.clear();
        let _ = self.yy_init_globals();
    }

    pub fn yy_flex_strncpy(s1: &mut String, s2: &str, n: usize) {
        s1.clear();
        s1.extend(s2.chars().take(n));
    }

    pub fn yy_flex_strlen(s: &str) -> usize {
        s.chars().count()
    }

    pub fn alloc_buffer(size: usize) -> Vec<u8> {
        vec![0; size]
    }

    pub fn realloc_buffer(mut buffer: Vec<u8>, size: usize) -> Vec<u8> {
        buffer.resize(size, 0);
        buffer
    }

    pub fn yyfree<T>(_ptr: T) {}

    pub fn init_tokens(&mut self) {
        self.string_pool.extend(
            [
                "if", "else", "while", "for", "return", "typedef", "struct", "...", "int", "char",
                "void", "const", "volatile", "unsigned", "signed",
            ]
            .into_iter()
            .map(ToString::to_string),
        );
    }

    pub fn init_lex(&mut self, debug_level: i32) {
        self.yy_flex_debug = debug_level;
        self.init_tokens();
    }

    pub fn ident(&mut self) -> i32 {
        if self.prev_token != 0 {
            if let Some(found) = self.string_pool.iter().find(|s| s.as_str() == self.yytext) {
                self.yytext = found.clone();
                return 1;
            }
        }
        self.string_pool.push(self.yytext.clone());
        1
    }

    pub fn set_preprocessor(&mut self, arg: Option<&str>) {
        self.pp_bin = arg.map(ToString::to_string);
    }

    pub fn pp_option(&mut self, opt: char, arg: &str) {
        if self.opt_stack.is_none() {
            if self.pp_bin.is_none() {
                self.pp_bin = Some("cpp".to_string());
            }
            self.opt_stack = Some(String::new());
        }

        if let Some(stack) = self.opt_stack.as_mut() {
            stack.push(' ');
            stack.push('-');
            stack.push(opt);
            stack.push_str(arg);
        }
    }

    pub fn pp_finalize(&mut self) {
        let Some(mut s) = self.opt_stack.take() else {
            return;
        };
        s.push('\0');
        s.pop();

        if self.pp_opts.is_none() {
            self.pp_opts = Some(s);
        } else if let Some(opts) = self.pp_opts.as_mut() {
            opts.push_str(&s);
        }
    }

    pub fn pp_open(&mut self, name: &str) -> Option<String> {
        if self.opt_stack.is_some() {
            self.pp_finalize();
        }

        let pp_bin = self.pp_bin.clone().unwrap_or_else(|| "cpp".to_string());
        let mut s = String::new();
        s.push_str(&pp_bin);
        if let Some(opts) = &self.pp_opts {
            s.push_str(opts);
        }
        s.push(' ');
        s.push_str(name);

        if self.yy_flex_debug != 0 {
            let _ = writeln!(io::stderr(), "Command line: {s}");
        }

        Some(s)
    }

    pub fn pp_close(&mut self) {
        self.yyin = None;
    }

    pub fn yywrap(&mut self) -> bool {
        if self.yyin.is_none() {
            return true;
        }

        self.pp_close();
        self.yy_delete_buffer(self.active_buffer.clone());
        self.active_buffer = None;
        true
    }

    pub fn get_token(&mut self) -> i32 {
        if self.hit_eof {
            return 0;
        }

        let tok = if self.source_pos >= self.source_content.chars().count() {
            0
        } else {
            self.lex_token()
        };

        self.prev_token = tok;
        if tok == 0 {
            self.hit_eof = true;
        }
        tok
    }

    pub fn source(&mut self, name: &str) -> i32 {
        let content = match std::fs::read_to_string(name) {
            Ok(s) => s,
            Err(_) => return 1,
        };

        if self.preprocess_option && self.pp_open(name).is_none() {
            return 1;
        }

        self.string_pool.push(name.to_string());
        self.filename = Some(name.to_string());
        self.canonical_filename = Some(name.to_string());
        self.line_num = 1;
        self.input_file_count += 1;
        self.hit_eof = false;
        self.source_content = content;
        self.source_pos = 0;
        self.yyin = Some(PathBuf::from(name));
        self.yyrestart(self.yyin.clone());
        0
    }

    pub fn getnum(&mut self, base: u32, mut count: i32) -> i32 {
        let mut n = 0i32;

        while count > 0 {
            let Some(c) = self.input_char() else {
                break;
            };

            let i = if c.is_ascii_digit() {
                c as u32 - '0' as u32
            } else {
                c.to_ascii_uppercase() as u32 - 'A' as u32 + 10
            };

            if i > base {
                self.unput_char(c);
                break;
            }

            n = n.saturating_mul(base as i32).saturating_add(i as i32);
            count -= 1;
        }

        n
    }

    pub fn backslash(&mut self) -> i32 {
        let Some(c) = self.input_char() else {
            return 0;
        };

        match c {
            'a' => '\u{0007}' as i32,
            'b' => '\u{0008}' as i32,
            'f' => '\u{000c}' as i32,
            'n' => '\n' as i32,
            'r' => '\r' as i32,
            't' => '\t' as i32,
            'x' => self.getnum(16, 2),
            '0' => self.getnum(8, 3),
            _ => c as i32,
        }
    }

    pub fn update_loc(&mut self) {
        let Some(hash) = self.yytext.find('#') else {
            return;
        };

        let mut p = self.yytext[hash + 1..].trim_start();
        if p.starts_with("line") {
            p = &p[4..];
        }

        p = p.trim_start();
        let digits_len = p.chars().take_while(|c| c.is_ascii_digit()).count();
        if digits_len > 0 {
            self.line_num = p[..digits_len].parse::<usize>().unwrap_or(self.line_num);
            p = &p[digits_len..];
        }

        p = p.trim_start();
        if let Some(rest) = p.strip_prefix('"') {
            if let Some(end) = rest.find('"') {
                let name = rest[..end].to_string();
                self.string_pool.push(name.clone());
                self.filename = Some(name);
            }
        }

        if self.yy_flex_debug > 1 {
            let _ = writeln!(
                io::stderr(),
                "New location: {}:{}",
                self.filename.as_deref().unwrap_or(""),
                self.line_num
            );
        }
    }

    pub fn yytext(&self) -> &str {
        &self.yytext
    }

    pub fn line_num(&self) -> usize {
        self.line_num
    }

    pub fn input_file_count(&self) -> usize {
        self.input_file_count
    }

    pub fn module_src_yy_scan_19(&self) -> ModuleSrcYyScan19 {
        ModuleSrcYyScan19 {
            original_len: self.source_content.len(),
            allocated_len: self
                .active_buffer
                .as_ref()
                .map(|b| b.yy_ch_buf.len())
                .unwrap_or(0),
            owns_buffer: self
                .active_buffer
                .as_ref()
                .map(|b| b.yy_is_our_buffer)
                .unwrap_or(false),
        }
    }

    pub fn yy_buffer_state(&self) -> Option<&YyBufferState> {
        self.active_buffer.as_ref()
    }

    pub fn yy_trans_info(verify: i32, advance: i32) -> YyTransInfo {
        YyTransInfo { verify, advance }
    }

    pub fn module_src_yy_flex_16(&self) -> ModuleSrcYyFlex16 {
        ModuleSrcYyFlex16 {
            copied: self.yyleng,
            measured: self.yytext.len(),
        }
    }

    pub fn module_src_yy_init_18(&self) -> ModuleSrcYyInit18 {
        ModuleSrcYyInit18 {
            debug_level: self.yy_flex_debug,
            initialized_tokens: self.string_pool.len(),
        }
    }

    pub fn module_src_yy_get_17(&self) -> ModuleSrcYyGet17 {
        ModuleSrcYyGet17 {
            debug: self.yy_flex_debug,
            in_path: self.yyin.clone(),
            out_path: self.yyout.clone(),
            leng: self.yyleng,
            text: self.yytext.clone(),
            lineno: self.yylineno,
        }
    }

    pub fn yy_get(&self) -> YyGet {
        self.yy_c_buf_p
    }

    pub fn module_src_yy_buffer_state_11(&self) -> ModuleSrcYyBufferState11 {
        self.active_buffer.clone().unwrap_or_default()
    }

    fn input_char(&mut self) -> Option<char> {
        let ch = self.source_content.chars().nth(self.source_pos)?;
        self.source_pos += 1;
        Some(ch)
    }

    fn unput_char(&mut self, _c: char) {
        if self.source_pos > 0 {
            self.source_pos -= 1;
        }
    }

    fn lex_token(&mut self) -> i32 {
        let chars = self.source_content.chars().collect::<Vec<_>>();
        let len = chars.len();

        while self.source_pos < len && chars[self.source_pos].is_whitespace() {
            if chars[self.source_pos] == '\n' {
                self.yylineno += 1;
            }
            self.source_pos += 1;
        }

        if self.source_pos >= len {
            self.yytext.clear();
            self.yyleng = 0;
            return 0;
        }

        let start = self.source_pos;
        let first = chars[self.source_pos];

        if first.is_ascii_alphabetic() || first == '_' {
            self.source_pos += 1;
            while self.source_pos < len
                && (chars[self.source_pos].is_ascii_alphanumeric() || chars[self.source_pos] == '_')
            {
                self.source_pos += 1;
            }
        } else if first.is_ascii_digit() {
            self.source_pos += 1;
            while self.source_pos < len && chars[self.source_pos].is_ascii_digit() {
                self.source_pos += 1;
            }
        } else {
            self.source_pos += 1;
        }

        self.yytext = chars[start..self.source_pos].iter().collect();
        self.yyleng = self.yytext.chars().count();
        1
    }
}

impl Drop for C {
    fn drop(&mut self) {
        self.reset();
    }
}
