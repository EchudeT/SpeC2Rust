use std::collections::HashMap;
use std::io::{self, Write};
use std::process;

const WRDSF_SHOWERR: u32 = 1 << 0;
const WRDSF_ENOMEMABRT: u32 = 1 << 1;
const WRDSF_REUSE: u32 = 1 << 2;
const WRDSF_APPEND: u32 = 1 << 3;
const WRDSF_NOVAR: u32 = 1 << 4;
const WRDSF_NOCMD: u32 = 1 << 5;
const WRDSF_ENV: u32 = 1 << 6;
const WRDSF_ENV_KV: u32 = 1 << 7;
const WRDSF_GETVAR: u32 = 1 << 8;
const WRDSF_CLOSURE: u32 = 1 << 9;
const WRDSF_DELIM: u32 = 1 << 10;
const WRDSF_ALLOC_DIE: u32 = 1 << 11;
const WRDSF_ERROR: u32 = 1 << 12;
const WRDSF_DEBUG: u32 = 1 << 13;
const WRDSF_SHOWDBG: u32 = 1 << 14;
const WRDSF_OPTIONS: u32 = 1 << 15;
const WRDSF_DOOFFS: u32 = 1 << 16;
const WRDSF_COMMENT: u32 = 1 << 17;
const WRDSF_ESCAPE: u32 = 1 << 18;
const WRDSF_CESCAPES: u32 = 1 << 19;
const WRDSF_RETURN_DELIMS: u32 = 1 << 20;

const WRDSO_MAXWORDS: u32 = 1 << 0;
const WRDSO_NAMECHAR: u32 = 1 << 1;
const WRDSO_PARAMV: u32 = 1 << 2;
const WRDSO_OESC_QUOTE: u32 = 1 << 3;
const WRDSO_OESC_WORD: u32 = 1 << 4;
const WRDSO_XESC_QUOTE: u32 = 1 << 5;
const WRDSO_XESC_WORD: u32 = 1 << 6;

const WRDSX_WORD: usize = 0;
const WRDSX_QUOTE: usize = 1;

const WRDSE_EOF: i32 = 0;
const WRDSE_USAGE: i32 = 1;
const WRDSE_NOSPACE: i32 = 2;
const WRDSE_USERERR: i32 = 3;
const WRDSE_UNDEF: i32 = 4;
const WRDSE_GLOBERR: i32 = 5;
const WRDSE_BADPARAM: i32 = 6;

const ALLOC_INIT: usize = 16;
const ALLOC_INCR: usize = 8;

const WSNF_WORD: u32 = 1 << 0;
const WSNF_NULL: u32 = 1 << 1;
const WSNF_QUOTE: u32 = 1 << 2;
const WSNF_NOEXPAND: u32 = 1 << 3;
const WSNF_JOIN: u32 = 1 << 4;
const WSNF_SEXP: u32 = 1 << 5;
const WSNF_DELIM: u32 = 1 << 6;

const WS_ESC_C: usize = 0;
const WS_ESC_C_WS: usize = 1;

const WORDSPLIT_ESCAPE: [&str; 2] = ["abfnrtv\\\"'", "abfnrtv\\\"' "];

#[derive(Clone, Debug, Default)]
pub struct WordsplitNode {
    pub flags: u32,
    pub text: String,
    pub range: Option<(usize, usize)>,
}

#[derive(Default)]
pub struct Wordsplit {
    ws_flags: u32,
    ws_options: u32,
    ws_namechar: String,
    ws_errno: i32,
    ws_errctx: Option<String>,
    ws_usererr: Option<String>,
    ws_input: String,
    ws_len: usize,
    ws_offs: usize,
    ws_wordv: Vec<String>,
    ws_wordc: usize,
    ws_wordn: usize,
    ws_delim: String,
    ws_sep: [char; 2],
    ws_comment: Option<String>,
    ws_closure: Option<String>,
    ws_escape: [String; 2],
    ws_paramv: Vec<String>,
    ws_paramc: usize,
    ws_paramidx: usize,
    ws_paramsiz: usize,
    ws_parambuf: Vec<String>,
    ws_envidx: usize,
    ws_envsiz: usize,
    ws_envbuf: Vec<(String, String)>,
    ws_wordi: usize,
    ws_endp: usize,
    ws_lvl: usize,
    ws_head: Vec<WordsplitNode>,
    ws_tail: Option<usize>,
    ws_debug_enabled: bool,
    env_map: HashMap<String, String>,
    words_cache: Vec<String>,
}

impl Wordsplit {
    pub fn node(flags: u32, text: impl Into<String>) -> WordsplitNode {
        WordsplitNode {
            flags,
            text: text.into(),
            range: None,
        }
    }

    pub fn wordsplit_02() -> u32 {
        WSNF_WORD | WSNF_NULL | WSNF_QUOTE | WSNF_NOEXPAND | WSNF_JOIN | WSNF_SEXP | WSNF_DELIM
    }

    pub fn wordsplit_03() -> u32 {
        WRDSF_SHOWERR | WRDSF_REUSE | WRDSF_APPEND | WRDSF_RETURN_DELIMS
    }

    pub fn wordsplit_04() -> u32 {
        WRDSO_NAMECHAR
            | WRDSO_PARAMV
            | WRDSO_OESC_QUOTE
            | WRDSO_OESC_WORD
            | WRDSO_XESC_QUOTE
            | WRDSO_XESC_WORD
    }

    pub fn wordsplit_05() -> [usize; 2] {
        [WS_ESC_C, WS_ESC_C_WS]
    }

    pub fn init_0() -> Self {
        Self {
            ws_delim: " \t\n".to_string(),
            ws_sep: [' ', '\0'],
            ..Self::default()
        }
    }

    pub fn new(input: &str, flags: u32) -> Result<Self, i32> {
        let mut ws = Self::init_0();
        ws.ws_input = input.to_string();
        ws.ws_len = input.len();
        let rc = ws.init_state(input, input.len(), flags);
        if rc == 0 {
            Ok(ws)
        } else {
            Err(rc)
        }
    }

    fn init_state(&mut self, input: &str, len: usize, flags: u32) -> i32 {
        self.ws_flags = flags;

        if self.ws_flags & WRDSF_NOVAR == 0 {
            self.ws_envidx = 0;
            self.ws_envsiz = 0;
            self.ws_envbuf.clear();
        }

        if self.ws_flags & WRDSF_NOCMD == 0 && self.ws_closure.is_none() {
            return self.wsplt_seterr(WRDSE_USAGE);
        }

        if self.ws_flags & WRDSF_SHOWDBG != 0
            && self.ws_flags & WRDSF_DEBUG == 0
            && self.ws_flags & WRDSF_SHOWERR == 0
        {
            self.ws_flags &= !WRDSF_SHOWDBG;
        }

        self.ws_input = input.to_string();
        self.ws_len = len;

        if self.ws_flags & WRDSF_DOOFFS == 0 {
            self.ws_offs = 0;
        }

        if self.ws_flags & WRDSF_DELIM == 0 {
            self.ws_delim = " \t\n".to_string();
        }

        self.ws_sep = [self.ws_delim.chars().next().unwrap_or(' '), '\0'];

        if self.ws_flags & WRDSF_COMMENT == 0 {
            self.ws_comment = None;
        }

        if self.ws_flags & WRDSF_CLOSURE == 0 {
            self.ws_closure = None;
        }

        if self.ws_flags & WRDSF_OPTIONS == 0 {
            self.ws_options = 0;
        }

        if self.ws_flags & WRDSF_ESCAPE != 0 {
            if self.ws_escape[WRDSX_WORD].is_empty() {
                self.ws_escape[WRDSX_WORD] = String::new();
            }
            if self.ws_escape[WRDSX_QUOTE].is_empty() {
                self.ws_escape[WRDSX_QUOTE] = String::new();
            }
        } else if self.ws_flags & WRDSF_CESCAPES != 0 {
            self.ws_escape[WRDSX_WORD] = WORDSPLIT_ESCAPE[WS_ESC_C_WS].to_string();
            self.ws_escape[WRDSX_QUOTE] = WORDSPLIT_ESCAPE[WS_ESC_C].to_string();
            self.ws_options |=
                WRDSO_OESC_QUOTE | WRDSO_OESC_WORD | WRDSO_XESC_QUOTE | WRDSO_XESC_WORD;
        } else {
            self.ws_escape[WRDSX_WORD].clear();
            self.ws_escape[WRDSX_QUOTE].clear();
        }

        if self.ws_options & WRDSO_PARAMV == 0 {
            self.ws_paramv.clear();
            self.ws_paramc = 0;
        }
        self.ws_paramidx = 0;
        self.ws_paramsiz = 0;
        self.ws_parambuf.clear();

        if self.ws_options & WRDSO_NAMECHAR != 0 {
            if self.ws_namechar.chars().any(|c| "${}*@-+?=".contains(c)) {
                return self.wsplt_seterr(WRDSE_USAGE);
            }
        } else {
            self.ws_namechar.clear();
        }

        self.ws_endp = 0;
        self.ws_wordi = 0;

        if self.ws_flags & WRDSF_REUSE != 0 {
            self.free_nodes();
        }
        self.ws_head.clear();
        self.ws_tail = None;
        self.ws_errctx = None;
        self.reset_init0();
        0
    }

    fn reset_init0(&mut self) {
        if self.ws_flags & WRDSF_REUSE != 0 {
            if self.ws_flags & WRDSF_APPEND == 0 {
                self.free_words();
            }
            self.clearerr();
        } else {
            self.ws_wordv.clear();
            self.ws_wordc = 0;
            self.ws_wordn = 0;
        }
        self.ws_errno = 0;
    }

    pub fn is_name_char(&self, c: char) -> bool {
        c.is_ascii_alphanumeric()
            || c == '_'
            || (self.ws_options & WRDSO_NAMECHAR != 0 && self.ws_namechar.contains(c))
    }

    pub fn wsplt_alloc_die(&self) -> ! {
        self.wsplt_error("memory exhausted");
        process::abort();
    }

    pub fn wsplt_error(&self, msg: &str) {
        let _ = writeln!(io::stderr(), "{msg}");
    }

    pub fn wsplt_seterr(&mut self, ec: i32) -> i32 {
        self.ws_errno = ec;
        if self.ws_flags & WRDSF_SHOWERR != 0 {
            self.perror();
        }
        ec
    }

    pub fn wsplt_nomem(&mut self) -> i32 {
        self.ws_errno = WRDSE_NOSPACE;
        if self.ws_flags & WRDSF_ENOMEMABRT != 0 {
            self.wsplt_alloc_die();
        }
        if self.ws_flags & WRDSF_SHOWERR != 0 {
            self.perror();
        }
        if self.ws_flags & WRDSF_REUSE == 0 {
            self.free_words();
            self.free_envbuf();
            self.free_parambuf();
            self.clearerr();
        }
        self.free_nodes();
        self.ws_errno
    }

    pub fn wsplt_store_errctx(&mut self, s: &str, len: usize) {
        let clipped = s.chars().take(len).collect::<String>();
        self.ws_errctx = Some(clipped);
    }

    pub fn wsplt_setctxerr(&mut self, ec: i32, s: &str, len: usize) -> i32 {
        self.wsplt_store_errctx(s, len);
        self.wsplt_seterr(ec)
    }

    pub fn wsplt_subsplit(
        &mut self,
        wss: &mut Wordsplit,
        s: &str,
        len: usize,
        mut flags: u32,
        finalize: bool,
    ) -> i32 {
        wss.ws_delim = self.ws_delim.clone();
        wss.ws_debug_enabled = self.ws_debug_enabled;

        if flags & WRDSF_NOVAR == 0 {
            wss.env_map = self.env_map.clone();
            flags |= self.ws_flags & (WRDSF_ENV | WRDSF_ENV_KV | WRDSF_GETVAR);
        }
        if flags & WRDSF_NOCMD == 0 {
            wss.ws_closure = self.ws_closure.clone();
        }
        if (flags & (WRDSF_NOVAR | WRDSF_NOCMD)) != (WRDSF_NOVAR | WRDSF_NOCMD) {
            flags |= self.ws_flags & WRDSF_CLOSURE;
        }

        wss.ws_options = self.ws_options & !WRDSO_MAXWORDS;
        wss.ws_namechar = self.ws_namechar.clone();

        flags |= WRDSF_DELIM
            | WRDSF_ALLOC_DIE
            | WRDSF_ERROR
            | WRDSF_DEBUG
            | (self.ws_flags & (WRDSF_SHOWDBG | WRDSF_SHOWERR | WRDSF_OPTIONS));

        let rc = wss.init_state(s, len, flags);
        if rc != 0 {
            return rc;
        }
        wss.ws_lvl = self.ws_lvl + 1;
        let rc = wss.process_list(0);
        if rc != 0 {
            wss.free_nodes();
            return rc;
        }
        if finalize {
            let rc = wss.finish();
            wss.free_nodes();
            return rc;
        }
        0
    }

    pub fn wsplt_seterr_sub(&mut self, wss: &mut Wordsplit) {
        if self.ws_errno == WRDSE_USERERR {
            self.ws_usererr = None;
        }
        self.ws_errno = wss.ws_errno;
        if wss.ws_errno == WRDSE_USERERR {
            self.ws_usererr = wss.ws_usererr.take();
            wss.ws_errno = WRDSE_EOF;
        }
        self.ws_errctx = wss.ws_errctx.take();
    }

    pub fn alloc_space(&mut self, count: usize) -> i32 {
        let offs = if self.ws_flags & WRDSF_DOOFFS != 0 {
            self.ws_offs
        } else {
            0
        };
        if self.ws_wordv.is_empty() {
            let newalloc = if offs + count > ALLOC_INIT {
                offs + count
            } else {
                ALLOC_INIT
            };
            self.ws_wordv.reserve(newalloc);
            self.ws_wordn = newalloc;
            return 0;
        }
        if self.ws_wordn < offs + self.ws_wordc + count {
            let newalloc = offs + self.ws_wordc + count.max(ALLOC_INCR);
            self.ws_wordv
                .reserve(newalloc.saturating_sub(self.ws_wordv.capacity()));
            self.ws_wordn = newalloc;
        }
        0
    }

    pub fn wsnode_flagstr(&self, flags: u32) -> String {
        let mut out = String::with_capacity(6);
        if flags & WSNF_WORD != 0 {
            out.push('w');
        } else if flags & WSNF_NULL != 0 {
            out.push('n');
        } else {
            out.push('-');
        }
        out.push(if flags & WSNF_QUOTE != 0 { 'q' } else { '-' });
        out.push(if flags & WSNF_NOEXPAND != 0 { 'E' } else { '-' });
        out.push(if flags & WSNF_JOIN != 0 { 'j' } else { '-' });
        out.push(if flags & WSNF_SEXP != 0 { 's' } else { '-' });
        out.push(if flags & WSNF_DELIM != 0 { 'd' } else { '-' });
        out
    }

    pub fn wsnode_ptr<'a>(&'a self, p: &'a WordsplitNode) -> &'a str {
        if p.flags & WSNF_NULL != 0 {
            ""
        } else if p.flags & WSNF_WORD != 0 {
            &p.text
        } else if let Some((beg, end)) = p.range {
            self.ws_input.get(beg..end).unwrap_or("")
        } else {
            &p.text
        }
    }

    pub fn wsnode_len(&self, p: &WordsplitNode) -> usize {
        if p.flags & WSNF_NULL != 0 {
            0
        } else if p.flags & WSNF_WORD != 0 {
            p.text.len()
        } else if let Some((beg, end)) = p.range {
            end.saturating_sub(beg)
        } else {
            p.text.len()
        }
    }

    pub fn new_node(&mut self) -> Result<usize, i32> {
        self.ws_head.push(WordsplitNode::default());
        self.ws_tail = Some(self.ws_head.len() - 1);
        Ok(self.ws_head.len() - 1)
    }

    pub fn remove_node(&mut self, index: usize) {
        if index < self.ws_head.len() {
            self.ws_head.remove(index);
            self.ws_tail = self.ws_head.len().checked_sub(1);
        }
    }

    pub fn wsnode_append(&mut self, node: WordsplitNode) {
        self.ws_head.push(node);
        self.ws_tail = Some(self.ws_head.len() - 1);
    }

    pub fn wsnode_remove(&mut self, index: usize) {
        self.remove_node(index);
    }

    pub fn wsnode_tail(&self) -> Option<usize> {
        self.ws_tail
    }

    pub fn wsnode_insert(&mut self, node: WordsplitNode, anchor: usize, before: bool) {
        let pos = if before {
            anchor
        } else {
            anchor.saturating_add(1)
        };
        if pos >= self.ws_head.len() {
            self.wsnode_append(node);
        } else {
            self.ws_head.insert(pos, node);
            self.ws_tail = Some(self.ws_head.len() - 1);
        }
    }

    pub fn add_segm(&mut self, beg: usize, end: usize, flg: u32) -> i32 {
        let text = self.ws_input.get(beg..end).unwrap_or("").to_string();
        self.wsnode_append(WordsplitNode {
            flags: flg,
            text,
            range: Some((beg, end)),
        });
        0
    }

    pub fn free_nodes(&mut self) {
        self.ws_head.clear();
        self.ws_tail = None;
    }

    pub fn dump_nodes(&self) {
        for (n, p) in self.ws_head.iter().enumerate() {
            let _ = writeln!(
                io::stderr(),
                "{}: flags={} text={:?}",
                n,
                self.wsnode_flagstr(p.flags),
                self.wsnode_ptr(p)
            );
        }
    }

    pub fn coalesce_segment(&mut self, index: usize) -> i32 {
        if index + 1 >= self.ws_head.len() {
            return 0;
        }
        let can_merge = {
            let a = &self.ws_head[index];
            let b = &self.ws_head[index + 1];
            a.flags & WSNF_DELIM == 0 && b.flags & WSNF_DELIM == 0
        };
        if can_merge {
            let right = self.ws_head.remove(index + 1);
            self.ws_head[index].text.push_str(&right.text);
            self.ws_head[index].flags |= right.flags & (WSNF_QUOTE | WSNF_JOIN | WSNF_SEXP);
            self.ws_head[index].range = None;
            self.ws_tail = Some(self.ws_head.len() - 1);
        }
        0
    }

    pub fn wsnode_quoteremoval(&mut self) -> i32 {
        let quoted: Vec<bool> = self
            .ws_head
            .iter()
            .map(|n| n.flags & WSNF_QUOTE != 0)
            .collect();
        let texts: Vec<String> = self.ws_head.iter().map(|n| n.text.clone()).collect();

        for (idx, node) in self.ws_head.iter_mut().enumerate() {
            if node.flags & WSNF_NULL != 0 {
                continue;
            }
            node.text = Self::string_unquote_copy_static(&self.ws_escape, quoted[idx], &texts[idx], texts[idx].len());
            node.flags |= WSNF_WORD;
            node.range = None;
        }
        0
    }

    pub fn wsnode_coalesce(&mut self) -> i32 {
        let mut i = 0;
        while i + 1 < self.ws_head.len() {
            let rc = self.coalesce_segment(i);
            if rc != 0 {
                return rc;
            }
            i += 1;
        }
        0
    }

    pub fn wsnode_tail_coalesce(&mut self) -> i32 {
        if self.ws_head.len() >= 2 {
            self.coalesce_segment(self.ws_head.len() - 2)
        } else {
            0
        }
    }

    pub fn finish(&mut self) -> i32 {
        let _ = self.wsnode_tail_coalesce();
        let _ = self.wsnode_quoteremoval();
        let needed = self.ws_head.len() + 1;
        let rc = self.alloc_space(needed);
        if rc != 0 {
            return rc;
        }
        let words: Vec<String> = self
            .ws_head
            .iter()
            .filter(|node| node.flags & WSNF_DELIM == 0)
            .map(|node| self.wsnode_ptr(node).to_string())
            .collect();
        self.ws_wordv.extend(words);
        self.ws_wordc = self.ws_wordv.len();
        self.words_cache = self.ws_wordv.clone();
        0
    }

    pub fn append(&mut self, argv: &[String]) -> i32 {
        let rc = self.alloc_space(self.ws_wordc + argv.len() + 1);
        if rc != 0 {
            return rc;
        }
        self.ws_wordv.extend(argv.iter().cloned());
        self.ws_wordc = self.ws_wordv.len();
        self.words_cache = self.ws_wordv.clone();
        0
    }

    pub fn node_split_prefix(&mut self, node: usize, beg: usize, len: usize, flg: u32) -> i32 {
        if node >= self.ws_head.len() {
            return WRDSE_USAGE;
        }
        let original = self.ws_head[node].clone();
        let text = self.wsnode_ptr(&original).to_string();
        let prefix = text
            .get(beg..beg.saturating_add(len))
            .unwrap_or("")
            .to_string();
        let suffix = text.get(beg.saturating_add(len)..).unwrap_or("").to_string();
        self.ws_head[node].text = suffix;
        self.ws_head[node].flags = original.flags;
        self.ws_head.insert(
            node,
            WordsplitNode {
                flags: flg,
                text: prefix,
                range: None,
            },
        );
        self.ws_tail = Some(self.ws_head.len() - 1);
        0
    }

    pub fn find_matching_paren(
        &self,
        s: &str,
        i: usize,
        len: usize,
        paren: &str,
    ) -> Option<usize> {
        let pair: Vec<char> = paren.chars().collect();
        if pair.len() < 2 {
            return None;
        }
        let open = pair[0];
        let close = pair[1];
        let bytes: Vec<char> = s.chars().collect();
        let mut depth = 1usize;
        let mut pos = i;
        while pos < len && pos < bytes.len() {
            let c = bytes[pos];
            if c == '\\' {
                pos += 2;
                continue;
            }
            if c == open {
                depth += 1;
            } else if c == close {
                depth -= 1;
                if depth == 0 {
                    return Some(pos);
                }
            }
            pos += 1;
        }
        None
    }

    pub fn wsplt_env_find(&self, name: &str, len: usize) -> Option<String> {
        let key = name.chars().take(len).collect::<String>();
        self.env_map.get(&key).cloned()
    }

    pub fn wsplt_env_lookup(&self, name: &str, len: usize) -> Result<String, i32> {
        self.wsplt_env_find(name, len).ok_or(WRDSE_UNDEF)
    }

    pub fn wsplt_env_getvar(&self, name: &str, len: usize) -> Result<String, i32> {
        self.wsplt_env_lookup(name, len)
    }

    pub fn wsplt_assign_var(&mut self, name: &str, namelen: usize, value: &str) -> i32 {
        let key = name.chars().take(namelen).collect::<String>();
        self.env_map.insert(key.clone(), value.to_string());
        if let Some(slot) = self.ws_envbuf.iter_mut().find(|(k, _)| *k == key) {
            slot.1 = value.to_string();
        } else {
            self.ws_envbuf.push((key, value.to_string()));
        }
        self.ws_envidx = self.ws_envbuf.len();
        self.ws_envsiz = self.ws_envbuf.len();
        0
    }

    pub fn wsplt_assign_param(&mut self, param_idx: usize, value: &str) -> i32 {
        if param_idx > 1_000_000 {
            return self.wsplt_seterr(WRDSE_BADPARAM);
        }
        if self.ws_paramv.len() <= param_idx {
            self.ws_paramv.resize(param_idx + 1, String::new());
        }
        self.ws_paramv[param_idx] = value.to_string();
        self.ws_paramc = self.ws_paramv.len();
        0
    }

    pub fn expvar_recover(
        &mut self,
        s: &str,
        ptail: &mut Option<usize>,
        pend: &mut usize,
        flg: u32,
    ) -> i32 {
        let node = WordsplitNode {
            flags: flg | WSNF_WORD,
            text: s.to_string(),
            range: None,
        };
        self.wsnode_append(node);
        *ptail = self.wsnode_tail();
        *pend = s.len();
        0
    }

    pub fn expand_paramv(&mut self, ptail: &mut Option<usize>, flg: u32, _q: bool) -> i32 {
        for p in self.ws_paramv.clone() {
            self.wsnode_append(WordsplitNode {
                flags: flg | WSNF_WORD,
                text: p,
                range: None,
            });
        }
        *ptail = self.wsnode_tail();
        0
    }

    pub fn expvar(
        &mut self,
        s: &str,
        len: usize,
        ptail: &mut Option<usize>,
        pend: &mut usize,
        flg: u32,
    ) -> i32 {
        let name = s.chars().take(len).collect::<String>();
        match self.wsplt_env_getvar(&name, name.len()) {
            Ok(value) => {
                self.wsnode_append(WordsplitNode {
                    flags: flg | WSNF_WORD,
                    text: value,
                    range: None,
                });
                *ptail = self.wsnode_tail();
                *pend = len;
                0
            }
            Err(ec) => self.wsplt_setctxerr(ec, s, len),
        }
    }

    pub fn begin_var_p(c: char) -> bool {
        "{#@*".contains(c) || c.is_ascii_alphabetic() || c == '_' || c.is_ascii_digit()
    }

    pub fn node_expand<F1, F2>(&mut self, node: usize, beg_p: F1, mut exp_fn: F2) -> i32
    where
        F1: Fn(char) -> bool,
        F2: FnMut(&mut Self, &str, usize, &mut Option<usize>, &mut usize, u32) -> i32,
    {
        if node >= self.ws_head.len() {
            return 0;
        }
        let text = self.wsnode_ptr(&self.ws_head[node]).to_string();
        if let Some(ch) = text.chars().next() {
            if beg_p(ch) {
                let mut tail = None;
                let mut end = 0;
                return exp_fn(self, &text, text.len(), &mut tail, &mut end, self.ws_head[node].flags);
            }
        }
        0
    }

    pub fn wsnode_nullelim(&mut self) {
        self.ws_head.retain(|n| n.flags & WSNF_NULL == 0);
        self.ws_tail = self.ws_head.len().checked_sub(1);
    }

    pub fn varexp(&mut self) -> i32 {
        let indices: Vec<usize> = (0..self.ws_head.len()).collect();
        for idx in indices {
            let _ = self.node_expand(idx, Self::begin_var_p, |w, s, len, t, e, f| {
                w.expvar(s, len, t, e, f)
            });
        }
        self.wsnode_nullelim();
        0
    }

    pub fn begin_cmd_p(c: char) -> bool {
        c == '('
    }

    pub fn expcmd(
        &mut self,
        s: &str,
        len: usize,
        ptail: &mut Option<usize>,
        pend: &mut usize,
        flg: u32,
    ) -> i32 {
        let text = s.chars().take(len).collect::<String>();
        self.wsnode_append(WordsplitNode {
            flags: flg | WSNF_WORD,
            text,
            range: None,
        });
        *ptail = self.wsnode_tail();
        *pend = len;
        0
    }

    pub fn cmdexp(&mut self) -> i32 {
        let indices: Vec<usize> = (0..self.ws_head.len()).collect();
        for idx in indices {
            let _ = self.node_expand(idx, Self::begin_cmd_p, |w, s, len, t, e, f| {
                w.expcmd(s, len, t, e, f)
            });
        }
        self.wsnode_nullelim();
        0
    }

    pub fn trimws(&mut self) -> i32 {
        for node in &mut self.ws_head {
            node.text = node.text.trim().to_string();
        }
        self.wsnode_nullelim();
        0
    }

    pub fn tildexpand(&mut self) -> i32 {
        for node in &mut self.ws_head {
            if let Some(rest) = node.text.strip_prefix('~') {
                let home = self
                    .env_map
                    .get("HOME")
                    .cloned()
                    .unwrap_or_else(|| "~".to_string());
                node.text = format!("{home}{rest}");
            }
        }
        0
    }

    pub fn tilde_expand(input: &str) -> Option<String> {
        if let Some(rest) = input.strip_prefix("~/") {
            let home = std::env::var("HOME").ok()?;
            return Some(format!("{home}/{rest}"));
        }
        if input == "~" {
            return std::env::var("HOME").ok();
        }
        Some(input.to_string())
    }

    pub fn is_glob(s: &str, l: usize) -> bool {
        s.chars().take(l).any(|c| matches!(c, '*' | '?' | '['))
    }

    pub fn pathexpand(&mut self) -> i32 {
        for idx in 0..self.ws_head.len() {
            let text = self.ws_head[idx].text.clone();
            let len = text.len();
            if Self::is_glob(&text, len) {
                return self.wsplt_setctxerr(WRDSE_GLOBERR, &text, len);
            }
        }
        0
    }
    pub fn get_words(&self) -> &[String] {
        &self.words_cache
    }

    pub fn free_words(&mut self) {
        self.ws_wordv.clear();
        self.ws_wordc = 0;
        self.ws_wordn = 0;
        self.words_cache.clear();
    }

    pub fn free_envbuf(&mut self) {
        self.ws_envbuf.clear();
        self.ws_envidx = 0;
        self.ws_envsiz = 0;
    }

    pub fn free_parambuf(&mut self) {
        self.ws_parambuf.clear();
        self.ws_paramidx = 0;
        self.ws_paramsiz = 0;
    }

    pub fn clearerr(&mut self) {
        self.ws_errno = WRDSE_EOF;
        self.ws_errctx = None;
        self.ws_usererr = None;
    }

    pub fn strerror(&self) -> &'static str {
        match self.ws_errno {
            WRDSE_EOF => "no error",
            WRDSE_USAGE => "invalid wordsplit usage",
            WRDSE_NOSPACE => "memory exhausted",
            WRDSE_USERERR => "user-defined error",
            WRDSE_UNDEF => "undefined variable",
            WRDSE_GLOBERR => "pattern expansion failed",
            WRDSE_BADPARAM => "invalid parameter reference",
            _ => "wordsplit error",
        }
    }

    pub fn perror(&self) {
        let mut msg = self.strerror().to_string();
        if let Some(ctx) = &self.ws_errctx {
            if !ctx.is_empty() {
                msg.push_str(": ");
                msg.push_str(ctx);
            }
        }
        self.wsplt_error(&msg);
    }

    fn string_unquote_copy_static(ws_escape: &[String; 2], quoted: bool, s: &str, len: usize) -> String {
        let chars: Vec<char> = s.chars().take(len).collect();
        if chars.is_empty() {
            return String::new();
        }
        let mut out = String::new();
        let mut i = 0usize;
        let mut delim = '\0';
        if quoted && matches!(chars[0], '\'' | '"') {
            delim = chars[0];
            i = 1;
        }
        let end = if quoted && delim != '\0' && chars.len() > i && chars[chars.len() - 1] == delim {
            chars.len() - 1
        } else {
            chars.len()
        };
        while i < end {
            let c = chars[i];
            if c == '\\' && i + 1 < end {
                let next = chars[i + 1];
                let transtab = if delim == '"' {
                    &ws_escape[WRDSX_QUOTE]
                } else {
                    &ws_escape[WRDSX_WORD]
                };
                if !transtab.is_empty() {
                    out.push(Self::wsplt_unquote_char(transtab, next));
                } else {
                    out.push(next);
                }
                i += 2;
            } else {
                out.push(c);
                i += 1;
            }
        }
        out
    }
    pub fn string_unquote_copy(&self, quoted: bool, s: &str, len: usize) -> String {
        Self::string_unquote_copy_static(&self.ws_escape, quoted, s, len)
    }

    pub fn process_list(&mut self, start: usize) -> i32 {
        let mut pos = start;
        let input_len = self.ws_input.len();
        while pos < input_len {
            while pos < input_len && self.ws_delim.contains(self.ws_input.as_bytes()[pos] as char) {
                pos += 1;
            }
            if pos >= input_len {
                break;
            }
            let rc = self.scan_word(pos, false);
            if rc != 0 {
                return rc;
            }
            let next = self.skip_delim();
            if next <= pos {
                pos += 1;
            } else {
                pos = next;
            }
        }
        let _ = self.varexp();
        let _ = self.cmdexp();
        let _ = self.trimws();
        let _ = self.tildexpand();
        self.pathexpand()
    }


    pub fn skip_sed_expr(command: &str, i: usize, len: usize) -> Option<usize> {
        let chars: Vec<char> = command.chars().collect();
        if i >= len || i >= chars.len() {
            return None;
        }
        if chars[i] != 's' || i + 1 >= len || i + 1 >= chars.len() {
            return None;
        }
        let delim = chars[i + 1];
        let mut pos = i + 2;
        let mut sections = 0;
        while pos < len && pos < chars.len() {
            if chars[pos] == '\\' {
                pos += 2;
                continue;
            }
            if chars[pos] == delim {
                sections += 1;
                if sections == 2 {
                    return Some(pos + 1);
                }
            }
            pos += 1;
        }
        None
    }

    pub fn skip_delim_internal(&self, return_delims: bool) -> usize {
        if return_delims {
            self.ws_endp
        } else {
            self.ws_endp + 1
        }
    }

    pub fn skip_delim(&self) -> usize {
        self.skip_delim_internal(self.ws_flags & WRDSF_RETURN_DELIMS != 0)
    }

    pub fn skip_delim_real(&self) -> usize {
        self.skip_delim_internal(self.ws_flags & WRDSF_RETURN_DELIMS != 0)
    }

    pub fn scan_qstring(&mut self, start: usize, end: &mut usize) -> i32 {
        let bytes = self.ws_input.as_bytes();
        if start >= bytes.len() {
            return WRDSE_USAGE;
        }
        let quote = bytes[start];
        let mut i = start + 1;
        while i < bytes.len() {
            if bytes[i] == b'\\' {
                i += 2;
                continue;
            }
            if bytes[i] == quote {
                *end = i + 1;
                return 0;
            }
            i += 1;
        }
        WRDSE_USAGE
    }

    pub fn scan_word(&mut self, start: usize, consume_all: bool) -> i32 {
        let input = self.ws_input.clone();
        let bytes = input.as_bytes();
        let mut i = start;
        while i < bytes.len() {
            let c = bytes[i] as char;
            if self.ws_delim.contains(c) && !consume_all {
                break;
            }
            if c == '"' || c == '\'' {
                let mut end = i;
                let rc = self.scan_qstring(i, &mut end);
                if rc != 0 {
                    return rc;
                }
                let _ = self.add_segm(i, end, WSNF_QUOTE);
                i = end;
                continue;
            }
            if c == '$' && i + 1 < bytes.len() && Wordsplit::begin_var_p(bytes[i + 1] as char) {
                let mut j = i + 1;
                while j < bytes.len() && Wordsplit::begin_var_p(bytes[j] as char) {
                    j += 1;
                }
                let _ = self.add_segm(i + 1, j, WSNF_WORD);
                i = j;
                continue;
            }
            i += 1;
        }
        if start < i {
            let _ = self.add_segm(start, i, 0);
        }
        self.ws_endp = i.saturating_sub(1);
        0
    }

    pub fn parse_num(src: &str, base: u32, cnt: usize) -> Result<i32, i32> {
        let frag: String = src.chars().take(cnt).collect();
        if frag.chars().count() != cnt {
            return Err(WRDSE_USAGE);
        }
        i32::from_str_radix(&frag, base).map_err(|_| WRDSE_USAGE)
    }

    pub fn c_quoted_length(s: &str, quote_hex: bool, quote: &mut bool) -> usize {
        let mut len = 0;
        for ch in s.chars() {
            let q = Self::c_quote_char(ch);
            if q != ch {
                len += 2;
                *quote = true;
            } else if quote_hex && ch.is_control() {
                len += 4;
                *quote = true;
            } else {
                len += ch.len_utf8();
            }
        }
        len
    }

    pub fn wsplt_unquote_char(transtab: &str, c: char) -> char {
        let from = "abfnrtv\\\"'";
        if let Some(pos) = transtab.chars().position(|x| x == c) {
            from.chars().nth(pos).unwrap_or(c)
        } else {
            c
        }
    }

    pub fn wsplt_quote_char(transtab: &str, c: char) -> char {
        let from = "abfnrtv\\\"'";
        if let Some(pos) = from.chars().position(|x| x == c) {
            transtab.chars().nth(pos).unwrap_or(c)
        } else {
            c
        }
    }

    pub fn c_unquote_char(c: char) -> char {
        Self::wsplt_unquote_char(WORDSPLIT_ESCAPE[WS_ESC_C], c)
    }

    pub fn c_quote_char(c: char) -> char {
        Self::wsplt_quote_char(WORDSPLIT_ESCAPE[WS_ESC_C], c)
    }
}
