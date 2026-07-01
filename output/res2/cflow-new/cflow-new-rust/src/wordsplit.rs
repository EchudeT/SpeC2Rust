use std::env;
use std::fmt;
use std::io::{self, Write};
use std::path::PathBuf;

const WRDSE_OK: i32 = 0;
const WRDSE_EOF: i32 = 1;
const WRDSE_QUOTE: i32 = 2;
const WRDSE_USAGE: i32 = 3;
const WRDSE_NOSPACE: i32 = 4;
const WRDSE_UNDEF: i32 = 5;
const WRDSE_CBRACE: i32 = 6;
const WRDSE_PAREN: i32 = 7;
const WRDSE_BADPARAM: i32 = 8;
const WRDSE_NOINPUT: i32 = 9;
const WRDSE_GLOBERR: i32 = 10;
const WRDSE_USERERR: i32 = 11;

const WRDSF_SHOWERR: u64 = 1 << 0;
const WRDSF_ENOMEMABRT: u64 = 1 << 1;
const WRDSF_REUSE: u64 = 1 << 2;
const WRDSF_APPEND: u64 = 1 << 3;
const WRDSF_ALLOC_DIE: u64 = 1 << 4;
const WRDSF_ERROR: u64 = 1 << 5;
const WRDSF_NOVAR: u64 = 1 << 6;
const WRDSF_NOCMD: u64 = 1 << 7;
const WRDSF_ENV: u64 = 1 << 8;
const WRDSF_ENV_KV: u64 = 1 << 9;
const WRDSF_GETVAR: u64 = 1 << 10;
const WRDSF_CLOSURE: u64 = 1 << 11;
const WRDSF_DEBUG: u64 = 1 << 12;
const WRDSF_SHOWDBG: u64 = 1 << 13;
const WRDSF_OPTIONS: u64 = 1 << 14;
const WRDSF_DELIM: u64 = 1 << 15;
const WRDSF_DOOFFS: u64 = 1 << 16;
const WRDSF_COMMENT: u64 = 1 << 17;
const WRDSF_ESCAPE: u64 = 1 << 18;
const WRDSF_CESCAPES: u64 = 1 << 19;
const WRDSF_QUOTE: u64 = 1 << 20;
const WRDSF_SQUOTE: u64 = 1 << 21;
const WRDSF_DQUOTE: u64 = 1 << 22;
const WRDSF_RETURN_DELIMS: u64 = 1 << 23;
const WRDSF_SQUEEZE_DELIMS: u64 = 1 << 24;
const WRDSF_INCREMENTAL: u64 = 1 << 25;
const WRDSF_NOSPLIT: u64 = 1 << 26;
const WRDSF_WS: u64 = 1 << 27;
const WRDSF_SED_EXPR: u64 = 1 << 28;
const WRDSF_UNDEF: u64 = 1 << 29;
const WRDSF_WARNUNDEF: u64 = 1 << 30;
const WRDSF_KEEPUNDEF: u64 = 1 << 31;

const WRDSO_NAMECHAR: u64 = 1 << 0;
const WRDSO_PARAMV: u64 = 1 << 1;
const WRDSO_PARAM_NEGIDX: u64 = 1 << 2;
const WRDSO_GETVARPREF: u64 = 1 << 3;
const WRDSO_MAXWORDS: u64 = 1 << 4;
const WRDSO_RETDELNOTEMPTY: u64 = 1 << 5;
const WRDSO_DOTGLOB: u64 = 1 << 6;
const WRDSO_NULLGLOB: u64 = 1 << 7;
const WRDSO_FAILGLOB: u64 = 1 << 8;
const WRDSO_NOVARSPLIT: u64 = 1 << 9;
const WRDSO_NOCMDSPLIT: u64 = 1 << 10;
const WRDSO_OESC_QUOTE: u64 = 1 << 11;
const WRDSO_OESC_WORD: u64 = 1 << 12;
const WRDSO_XESC_QUOTE: u64 = 1 << 13;
const WRDSO_XESC_WORD: u64 = 1 << 14;

const _WSNF_WORD: u32 = 1 << 0;
const _WSNF_NULL: u32 = 1 << 1;
const _WSNF_QUOTE: u32 = 1 << 2;
const _WSNF_NOEXPAND: u32 = 1 << 3;
const _WSNF_JOIN: u32 = 1 << 4;
const _WSNF_SEXP: u32 = 1 << 5;
const _WSNF_DELIM: u32 = 1 << 6;
const _WSNF_EMPTYOK: u32 = 1 << 7;
const _WSNF_CONST: u32 = 1 << 8;

const WRDSX_WORD: usize = 0;
const WRDSX_QUOTE: usize = 1;

const _WRDS_OK: i32 = 0;
const _WRDS_ERR: i32 = 1;
const _WRDS_EOF: i32 = 2;

const ALLOC_INIT: usize = 16;
const ALLOC_INCR: usize = 16;
const WORDSPLIT_ENV_INIT: usize = 16;

#[derive(Clone, Default)]
pub struct WordsplitNode06 {
    pub flags: u32,
    pub word: Option<String>,
    pub segm_beg: usize,
    pub segm_end: usize,
    pub next: Option<usize>,
    pub prev: Option<usize>,
}

#[derive(Clone, Default)]
pub struct WordsplitNode {
    pub flags: u32,
    pub word: Option<String>,
    pub segm_beg: usize,
    pub segm_end: usize,
}

#[derive(Clone, Default)]
pub struct Wordsplit05 {
    pub enabled: bool,
}

#[derive(Clone, Default)]
pub struct Wordsplit04 {
    pub enabled: bool,
}

#[derive(Clone, Default)]
pub struct WordsplitC07 {
    pub enabled: bool,
}

#[derive(Clone, Default)]
pub struct Wordsplit03 {
    pub enabled: bool,
}

#[derive(Clone, Default)]
pub struct Wordsplit01 {
    pub enabled: bool,
}

type ErrorFn = fn(&str);
type AllocDieFn = fn(&Wordsplit) -> !;
type GetVarFn = fn(&str, usize) -> Result<Option<String>, i32>;
type CommandFn = fn(&str, &[String]) -> Result<Option<String>, i32>;

pub struct Wordsplit {
    ws_flags: u64,
    ws_options: u64,
    ws_namechar: String,
    ws_delim: String,
    ws_sep: String,
    ws_comment: Option<String>,
    ws_escape: [String; 2],
    ws_input: String,
    ws_len: usize,
    ws_endp: usize,
    ws_offs: usize,
    ws_wordi: usize,
    ws_wordc: usize,
    ws_wordn: usize,
    ws_wordv: Vec<Option<String>>,
    ws_errno: i32,
    ws_errctx: Option<String>,
    ws_usererr: Option<String>,
    ws_head: Option<usize>,
    ws_tail: Option<usize>,
    nodes: Vec<Option<WordsplitNode06>>,
    ws_debug: ErrorFn,
    ws_error_cb: ErrorFn,
    ws_alloc_die_cb: AllocDieFn,
    ws_env: Vec<String>,
    ws_env_kv: bool,
    ws_envbuf: Vec<String>,
    ws_envidx: usize,
    ws_envsiz: usize,
    ws_getvar: Option<GetVarFn>,
    ws_command: Option<CommandFn>,
    ws_paramv: Vec<String>,
    ws_parambuf: Vec<String>,
    ws_paramc: usize,
    ws_paramidx: usize,
    ws_paramsiz: usize,
    ws_closure: Option<String>,
    ws_lvl: usize,
    ws_maxwords: usize,
}

impl Default for Wordsplit {
    fn default() -> Self {
        Self::new()
    }
}

impl Wordsplit {
    pub fn wordsplit_05() -> Wordsplit05 {
        Wordsplit05 { enabled: true }
    }

    pub fn wordsplit_04() -> Wordsplit04 {
        Wordsplit04 { enabled: true }
    }

    pub fn wordsplit_node_06() -> WordsplitNode06 {
        WordsplitNode06::default()
    }

    pub fn node() -> WordsplitNode {
        WordsplitNode::default()
    }

    pub fn wordsplit_c_07() -> WordsplitC07 {
        WordsplitC07 { enabled: true }
    }

    pub fn quote_hex() -> bool {
        true
    }

    pub fn wordsplit_03() -> Wordsplit03 {
        Wordsplit03 { enabled: true }
    }

    pub fn wordsplit_01() -> Wordsplit01 {
        Wordsplit01 { enabled: true }
    }

    pub fn new() -> Self {
        Self {
            ws_flags: 0,
            ws_options: 0,
            ws_namechar: String::new(),
            ws_delim: " \t\n".to_string(),
            ws_sep: " ".to_string(),
            ws_comment: None,
            ws_escape: [
                String::new(),
                String::new(),
            ],
            ws_input: String::new(),
            ws_len: 0,
            ws_endp: 0,
            ws_offs: 0,
            ws_wordi: 0,
            ws_wordc: 0,
            ws_wordn: 0,
            ws_wordv: Vec::new(),
            ws_errno: WRDSE_OK,
            ws_errctx: None,
            ws_usererr: None,
            ws_head: None,
            ws_tail: None,
            nodes: Vec::new(),
            ws_debug: Self::wsplt_error,
            ws_error_cb: Self::wsplt_error,
            ws_alloc_die_cb: Self::wsplt_alloc_die,
            ws_env: Vec::new(),
            ws_env_kv: false,
            ws_envbuf: Vec::new(),
            ws_envidx: 0,
            ws_envsiz: 0,
            ws_getvar: None,
            ws_command: None,
            ws_paramv: Vec::new(),
            ws_parambuf: Vec::new(),
            ws_paramc: 0,
            ws_paramidx: 0,
            ws_paramsiz: 0,
            ws_closure: None,
            ws_lvl: 0,
            ws_maxwords: 0,
        }
    }

    pub fn init_0(&mut self) {
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
            || ((self.ws_options & WRDSO_NAMECHAR) != 0 && self.ws_namechar.contains(c))
    }

    pub fn wsplt_alloc_die(&self) -> ! {
        (self.ws_error_cb)("memory exhausted");
        panic!("memory exhausted")
    }

    pub fn wsplt_error(message: &str) {
        let _ = writeln!(io::stderr(), "{message}");
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
            (self.ws_alloc_die_cb)(self);
        }
        if self.ws_flags & WRDSF_SHOWERR != 0 {
            self.perror();
        }
        if self.ws_flags & WRDSF_REUSE == 0 {
            self.free_words();
            self.free_envbuf();
            self.free_parambuf();
        }
        self.free_nodes();
        self.ws_errno
    }

    pub fn wsplt_store_errctx(&mut self, text: &str, len: usize) {
        let end = len.min(text.len());
        self.ws_errctx = Some(text[..end].to_string());
    }

    pub fn wsplt_setctxerr(&mut self, ec: i32, text: &str, len: usize) -> i32 {
        self.wsplt_store_errctx(text, len);
        self.wsplt_seterr(ec)
    }

    pub fn wsplt_subsplit(
        &mut self,
        str_: &str,
        flags: u64,
        finalize: bool,
    ) -> Result<Wordsplit, i32> {
        let mut wss = Wordsplit::new();
        wss.ws_delim = self.ws_delim.clone();
        wss.ws_debug = self.ws_debug;
        wss.ws_error_cb = self.ws_error_cb;
        wss.ws_alloc_die_cb = self.ws_alloc_die_cb;

        let mut f = flags;
        if flags & WRDSF_NOVAR == 0 {
            wss.ws_env = self.ws_env.clone();
            wss.ws_env_kv = self.ws_env_kv;
            wss.ws_getvar = self.ws_getvar;
            f |= self.ws_flags & (WRDSF_ENV | WRDSF_ENV_KV | WRDSF_GETVAR);
        }
        if flags & WRDSF_NOCMD == 0 {
            wss.ws_command = self.ws_command;
        }
        if (flags & (WRDSF_NOVAR | WRDSF_NOCMD)) != (WRDSF_NOVAR | WRDSF_NOCMD) {
            wss.ws_closure = self.ws_closure.clone();
            f |= self.ws_flags & WRDSF_CLOSURE;
        }
        wss.ws_options = self.ws_options & !WRDSO_MAXWORDS;
        wss.ws_namechar = self.ws_namechar.clone();
        f |= WRDSF_DELIM
            | WRDSF_ALLOC_DIE
            | WRDSF_ERROR
            | WRDSF_DEBUG
            | (self.ws_flags & (WRDSF_SHOWDBG | WRDSF_SHOWERR | WRDSF_OPTIONS));

        let rc = wss.run(Some(str_), str_.len(), f, self.ws_lvl + 1, finalize);
        if rc != 0 && rc != WRDSE_EOF {
            return Err(rc);
        }
        Ok(wss)
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
        let needed = offs + self.ws_wordc + count;
        if self.ws_wordv.is_empty() {
            let newalloc = if offs + count > ALLOC_INIT {
                count
            } else {
                ALLOC_INIT
            };
            self.ws_wordv.resize_with(newalloc, || None);
            self.ws_wordn = newalloc;
            0
        } else if self.ws_wordn < needed {
            let newalloc = offs + self.ws_wordc + count.max(ALLOC_INCR);
            self.ws_wordv.resize_with(newalloc, || None);
            self.ws_wordn = newalloc;
            0
        } else {
            0
        }
    }

    pub fn wsnode_flagstr(flags: u32) -> String {
        let mut s = String::with_capacity(6);
        s.push(if flags & _WSNF_WORD != 0 {
            'w'
        } else if flags & _WSNF_NULL != 0 {
            'n'
        } else {
            '-'
        });
        s.push(if flags & _WSNF_QUOTE != 0 { 'q' } else { '-' });
        s.push(if flags & _WSNF_NOEXPAND != 0 { 'E' } else { '-' });
        s.push(if flags & _WSNF_JOIN != 0 { 'j' } else { '-' });
        s.push(if flags & _WSNF_SEXP != 0 { 's' } else { '-' });
        s.push(if flags & _WSNF_DELIM != 0 { 'd' } else { '-' });
        s
    }

    pub fn wsnode_ptr(&self, idx: usize) -> String {
        let Some(node) = self.nodes.get(idx).and_then(|n| n.as_ref()) else {
            return String::new();
        };
        if node.flags & _WSNF_NULL != 0 {
            String::new()
        } else if node.flags & _WSNF_WORD != 0 {
            node.word.clone().unwrap_or_default()
        } else {
            self.ws_input[node.segm_beg.min(self.ws_input.len())..node.segm_end.min(self.ws_input.len())]
                .to_string()
        }
    }

    pub fn wsnode_len(&self, idx: usize) -> usize {
        let Some(node) = self.nodes.get(idx).and_then(|n| n.as_ref()) else {
            return 0;
        };
        if node.flags & _WSNF_NULL != 0 {
            0
        } else if node.flags & _WSNF_WORD != 0 {
            node.word.as_deref().unwrap_or("").len()
        } else {
            node.segm_end.saturating_sub(node.segm_beg)
        }
    }

    pub fn new_node(&mut self) -> Result<usize, i32> {
        let idx = self.nodes.len();
        self.nodes.push(Some(WordsplitNode06::default()));
        Ok(idx)
    }

    pub fn remove_node(&mut self, idx: usize) {
        if idx < self.nodes.len() {
            self.nodes[idx] = None;
        }
    }

    pub fn wsnode_append(&mut self, idx: usize) {
        let tail = self.ws_tail;
        if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
            node.next = None;
            node.prev = tail;
        }
        if let Some(t) = tail {
            if let Some(tn) = self.nodes.get_mut(t).and_then(|n| n.as_mut()) {
                tn.next = Some(idx);
            }
        } else {
            self.ws_head = Some(idx);
        }
        self.ws_tail = Some(idx);
    }

    pub fn wsnode_remove(&mut self, idx: usize) {
        let (prev, next) = match self.nodes.get(idx).and_then(|n| n.as_ref()) {
            Some(n) => (n.prev, n.next),
            None => return,
        };

        if let Some(p) = prev {
            if let Some(pn) = self.nodes.get_mut(p).and_then(|n| n.as_mut()) {
                pn.next = next;
                if next.is_none() {
                    pn.flags &= !_WSNF_JOIN;
                }
            }
        } else {
            self.ws_head = next;
        }

        if let Some(n) = next {
            if let Some(nn) = self.nodes.get_mut(n).and_then(|x| x.as_mut()) {
                nn.prev = prev;
            }
        } else {
            self.ws_tail = prev;
        }

        self.remove_node(idx);
    }

    pub fn wsnode_tail(&self, mut idx: Option<usize>) -> Option<usize> {
        while let Some(i) = idx {
            let next = self
                .nodes
                .get(i)
                .and_then(|n| n.as_ref())
                .and_then(|n| n.next);
            if next.is_none() {
                return Some(i);
            }
            idx = next;
        }
        None
    }

    pub fn wsnode_insert(&mut self, idx: usize, anchor: Option<usize>, before: bool) {
        if self.ws_head.is_none() {
            if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                node.next = None;
                node.prev = None;
            }
            self.ws_head = Some(idx);
            self.ws_tail = self.wsnode_tail(Some(idx));
        } else if before {
            if let Some(a) = anchor {
                let prev = self.nodes.get(a).and_then(|n| n.as_ref()).and_then(|n| n.prev);
                if let Some(p) = prev {
                    self.wsnode_insert(idx, Some(p), false);
                } else {
                    let tail = self.wsnode_tail(Some(idx));
                    if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                        node.prev = None;
                    }
                    if let Some(t) = tail {
                        if let Some(tn) = self.nodes.get_mut(t).and_then(|n| n.as_mut()) {
                            tn.next = Some(a);
                        }
                    }
                    if let Some(an) = self.nodes.get_mut(a).and_then(|n| n.as_mut()) {
                        an.prev = tail;
                    }
                    self.ws_head = Some(idx);
                }
            }
        } else if let Some(a) = anchor {
            let p = self.nodes.get(a).and_then(|n| n.as_ref()).and_then(|n| n.next);
            let tail = self.wsnode_tail(Some(idx));
            if let Some(pi) = p {
                if let Some(pn) = self.nodes.get_mut(pi).and_then(|n| n.as_mut()) {
                    pn.prev = tail;
                }
            } else {
                self.ws_tail = tail;
            }
            if let Some(t) = tail {
                if let Some(tn) = self.nodes.get_mut(t).and_then(|n| n.as_mut()) {
                    tn.next = p;
                }
            }
            if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                node.prev = Some(a);
            }
            if let Some(an) = self.nodes.get_mut(a).and_then(|n| n.as_mut()) {
                an.next = Some(idx);
            }
        }
    }

    pub fn add_segm(&mut self, beg: usize, end: usize, flg: u32) -> i32 {
        if end == beg && flg & _WSNF_EMPTYOK == 0 {
            return 0;
        }
        let Ok(idx) = self.new_node() else {
            return self.wsplt_nomem();
        };
        if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
            node.flags = flg & !(_WSNF_WORD | _WSNF_EMPTYOK);
            node.segm_beg = beg;
            node.segm_end = end;
        }
        self.wsnode_append(idx);
        0
    }

    pub fn free_nodes(&mut self) {
        self.nodes.clear();
        self.ws_head = None;
        self.ws_tail = None;
    }

    pub fn dump_nodes(&self) {
        let mut n = 0usize;
        let mut p = self.ws_head;
        while let Some(idx) = p {
            if let Some(node) = self.nodes.get(idx).and_then(|x| x.as_ref()) {
                let text = if node.flags & _WSNF_WORD != 0 {
                    node.word.clone().unwrap_or_default()
                } else {
                    self.wsnode_ptr(idx)
                };
                (self.ws_debug)(&format!(
                    "({:02}) {:4}: {} ({}) :{};",
                    self.ws_lvl,
                    n,
                    node.flags,
                    Self::wsnode_flagstr(node.flags),
                    text
                ));
                p = node.next;
                n += 1;
            } else {
                break;
            }
        }
    }

    pub fn coalesce_segment(&mut self, node_idx: usize) -> i32 {
        let Some(node0) = self.nodes.get(node_idx).and_then(|n| n.as_ref()) else {
            return 0;
        };
        if node0.flags & _WSNF_JOIN == 0 {
            return 0;
        }

        let mut len = 0usize;
        let mut p = Some(node_idx);
        while let Some(i) = p {
            len += self.wsnode_len(i);
            let join = self
                .nodes
                .get(i)
                .and_then(|n| n.as_ref())
                .map(|n| n.flags & _WSNF_JOIN != 0)
                .unwrap_or(false);
            if !join {
                break;
            }
            p = self.nodes.get(i).and_then(|n| n.as_ref()).and_then(|n| n.next);
        }
        let end = p;

        let mut buf = String::with_capacity(len);
        let mut p = Some(node_idx);
        let mut stop = false;
        while let Some(i) = p {
            let next = self.nodes.get(i).and_then(|n| n.as_ref()).and_then(|n| n.next);
            let part = self.wsnode_ptr(i);
            buf.push_str(&part);
            if i != node_idx {
                let qf = self
                    .nodes
                    .get(i)
                    .and_then(|n| n.as_ref())
                    .map(|n| n.flags & _WSNF_QUOTE)
                    .unwrap_or(0);
                if let Some(n0) = self.nodes.get_mut(node_idx).and_then(|n| n.as_mut()) {
                    n0.flags |= qf;
                }
                stop = Some(i) == end;
                self.wsnode_remove(i);
            } else {
                stop = Some(i) == end;
            }
            if stop {
                break;
            }
            p = next;
        }

        if let Some(n0) = self.nodes.get_mut(node_idx).and_then(|n| n.as_mut()) {
            n0.flags &= !_WSNF_JOIN;
            n0.flags |= _WSNF_WORD;
            n0.word = Some(buf);
        }
        0
    }

    pub fn wsnode_quoteremoval(&mut self) -> i32 {
        let mut p = self.ws_head;
        while let Some(idx) = p {
            let next = self.nodes.get(idx).and_then(|n| n.as_ref()).and_then(|n| n.next);
            let noexpand = self
                .nodes
                .get(idx)
                .and_then(|n| n.as_ref())
                .map(|n| n.flags & _WSNF_NOEXPAND != 0)
                .unwrap_or(true);
            if !noexpand {
                let str_ = self.wsnode_ptr(idx);
                let slen = self.wsnode_len(idx);
                let quote = self
                    .nodes
                    .get(idx)
                    .and_then(|n| n.as_ref())
                    .map(|n| n.flags & _WSNF_QUOTE != 0)
                    .unwrap_or(false);
                let newstr = self.string_unquote_copy(quote as i32, &str_[..slen.min(str_.len())]);
                if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                    node.flags |= _WSNF_WORD;
                    node.word = Some(newstr);
                }
            }
            p = next;
        }
        0
    }

    pub fn wsnode_coalesce(&mut self) -> i32 {
        let mut p = self.ws_head;
        while let Some(idx) = p {
            let next = self.nodes.get(idx).and_then(|n| n.as_ref()).and_then(|n| n.next);
            let join = self
                .nodes
                .get(idx)
                .and_then(|n| n.as_ref())
                .map(|n| n.flags & _WSNF_JOIN != 0)
                .unwrap_or(false);
            if join && self.coalesce_segment(idx) != 0 {
                return 1;
            }
            p = next;
        }
        0
    }

    pub fn wsnode_tail_coalesce(&mut self, p: usize) -> i32 {
        let has_next = self
            .nodes
            .get(p)
            .and_then(|n| n.as_ref())
            .and_then(|n| n.next)
            .is_some();
        if has_next {
            let mut np = Some(p);
            while let Some(i) = np {
                let next = self.nodes.get(i).and_then(|n| n.as_ref()).and_then(|n| n.next);
                if next.is_none() {
                    break;
                }
                if let Some(node) = self.nodes.get_mut(i).and_then(|n| n.as_mut()) {
                    node.flags |= _WSNF_JOIN;
                }
                np = next;
            }
            if self.coalesce_segment(p) != 0 {
                return 1;
            }
        }
        0
    }

    pub fn finish(&mut self) -> i32 {
        let mut n = 0usize;
        let mut p = self.ws_head;

        while let Some(idx) = p {
            let next = self.nodes.get(idx).and_then(|n| n.as_ref()).and_then(|n| n.next);
            let flags = self.nodes.get(idx).and_then(|n| n.as_ref()).map(|n| n.flags).unwrap_or(0);
            if flags & _WSNF_DELIM != 0 {
                if self.ws_flags & WRDSF_SQUEEZE_DELIMS != 0 {
                    if let Some(nx) = next {
                        let nf = self.nodes.get(nx).and_then(|n| n.as_ref()).map(|n| n.flags).unwrap_or(0);
                        if nf & _WSNF_DELIM != 0 {
                            let a = self.wsnode_ptr(idx);
                            let b = self.wsnode_ptr(nx);
                            if a.chars().next() == b.chars().next() {
                                self.wsnode_remove(idx);
                                p = Some(nx);
                                continue;
                            }
                        }
                    }
                } else if let Some(nx) = next {
                    let nf = self.nodes.get(nx).and_then(|n| n.as_ref()).map(|n| n.flags).unwrap_or(0);
                    if nf & _WSNF_DELIM != 0 && self.ws_options & WRDSO_RETDELNOTEMPTY == 0 {
                        let Ok(nulnode) = self.new_node() else {
                            return self.wsplt_nomem();
                        };
                        if let Some(nn) = self.nodes.get_mut(nulnode).and_then(|n| n.as_mut()) {
                            nn.flags = _WSNF_NULL | _WSNF_NOEXPAND;
                        }
                        self.wsnode_insert(nulnode, Some(idx), false);
                    }
                }

                if self.ws_options & WRDSO_MAXWORDS != 0 && self.ws_flags & WRDSF_RETURN_DELIMS == 0 {
                    self.wsnode_remove(idx);
                    p = next;
                    continue;
                }
            }

            n += 1;
            if self.ws_options & WRDSO_MAXWORDS != 0 && self.ws_wordi + n == self.ws_maxwords {
                break;
            }

            if self.ws_flags & WRDSF_INCREMENTAL != 0 {
                p = None;
            } else {
                p = next;
            }
        }

        if let Some(idx) = p {
            if self.wsnode_tail_coalesce(idx) != 0 {
                return self.ws_errno;
            }
            n += 1;
        }

        if n == 0 {
            if self.ws_flags & WRDSF_NOSPLIT != 0 {
                if self.add_segm(0, 0, _WSNF_EMPTYOK) != 0 {
                    return self.ws_errno;
                }
                n = 1;
            } else if self.ws_flags & WRDSF_INCREMENTAL != 0 {
                if self.ws_endp < self.ws_len {
                    let rc = self.process_list(self.skip_delim());
                    if rc != 0 {
                        return rc;
                    }
                    return self.finish();
                } else {
                    self.ws_errno = WRDSE_EOF;
                    return WRDSE_EOF;
                }
            }
        }

        if self.alloc_space(n + 1) != 0 {
            return self.ws_errno;
        }

        while let Some(head) = self.ws_head {
            let str_ = self.wsnode_ptr(head);
            let slot = self.ws_offs + self.ws_wordc;
            if slot >= self.ws_wordv.len() {
                self.ws_wordv.resize_with(slot + 1, || None);
            }
            self.ws_wordv[slot] = Some(str_);
            self.wsnode_remove(head);
            self.ws_wordc += 1;
            self.ws_wordi += 1;
            if self.ws_flags & WRDSF_INCREMENTAL != 0 {
                break;
            }
        }

        let term = self.ws_offs + self.ws_wordc;
        if term >= self.ws_wordv.len() {
            self.ws_wordv.resize_with(term + 1, || None);
        }
        self.ws_wordv[term] = None;
        0
    }

    pub fn append(&mut self, argv: &[String]) -> i32 {
        let rc = self.alloc_space(self.ws_wordc + argv.len() + 1);
        if rc != 0 {
            return rc;
        }
        let base = self.ws_offs + self.ws_wordc;
        for (i, arg) in argv.iter().enumerate() {
            let idx = base + i;
            if idx >= self.ws_wordv.len() {
                self.ws_wordv.resize_with(idx + 1, || None);
            }
            self.ws_wordv[idx] = Some(arg.clone());
        }
        self.ws_wordc += argv.len();
        let end = self.ws_offs + self.ws_wordc;
        if end >= self.ws_wordv.len() {
            self.ws_wordv.resize_with(end + 1, || None);
        }
        self.ws_wordv[end] = None;
        0
    }

    pub fn node_split_prefix(
        &mut self,
        ptail: &mut usize,
        node: usize,
        beg: usize,
        len: usize,
        flg: u32,
    ) -> i32 {
        if len == 0 {
            return 0;
        }
        let Ok(newnode) = self.new_node() else {
            return 1;
        };
        self.wsnode_insert(newnode, Some(*ptail), false);

        let node_is_word = self
            .nodes
            .get(node)
            .and_then(|n| n.as_ref())
            .map(|n| n.flags & _WSNF_WORD != 0)
            .unwrap_or(false);

        if node_is_word {
            let str_ = self.wsnode_ptr(node);
            let end = (beg + len).min(str_.len());
            let newstr = str_[beg.min(str_.len())..end].to_string();
            if let Some(nn) = self.nodes.get_mut(newnode).and_then(|n| n.as_mut()) {
                nn.flags = _WSNF_WORD;
                nn.word = Some(newstr);
            }
        } else {
            let base = self
                .nodes
                .get(node)
                .and_then(|n| n.as_ref())
                .map(|n| n.segm_beg)
                .unwrap_or(0);
            if let Some(nn) = self.nodes.get_mut(newnode).and_then(|n| n.as_mut()) {
                nn.segm_beg = base + beg;
                nn.segm_end = nn.segm_beg + len;
            }
        }
        if let Some(nn) = self.nodes.get_mut(newnode).and_then(|n| n.as_mut()) {
            nn.flags |= flg;
        }
        *ptail = newnode;
        0
    }

    pub fn find_closing_paren(
        &self,
        s: &str,
        i: usize,
        len: usize,
        paren: &str,
    ) -> Option<usize> {
        let bytes = s.as_bytes();
        let pb = paren.as_bytes();
        if pb.len() != 2 {
            return None;
        }
        let mut state = 0u8;
        let mut level = 1usize;
        let mut k = i;
        while k < len && k < bytes.len() {
            match state {
                0 => match bytes[k] {
                    b'"' => state = 2,
                    b'\'' => state = 1,
                    c if c == pb[0] => level += 1,
                    c if c == pb[1] => {
                        level = level.saturating_sub(1);
                        if level == 0 {
                            return Some(k);
                        }
                    }
                    _ => {}
                },
                1 => {
                    if bytes[k] == b'\'' {
                        state = 0;
                    }
                }
                2 => {
                    if bytes[k] == b'\\' {
                        k += 1;
                    } else if bytes[k] == b'"' {
                        state = 0;
                    }
                }
                _ => {}
            }
            k += 1;
        }
        None
    }

    pub fn wsplt_env_find(&self, name: &str, len: usize
) -> Option<String> {
        let key = &name[..len.min(name.len())];
        if self.ws_env_kv {
            for item in &self.ws_env {
                if let Some((k, v)) = item.split_once('=') {
                    if k == key {
                        return Some(v.to_string());
                    }
                }
            }
            None
        } else {
            env::var(key).ok()
        }
    }

    pub fn wsplt_env_lookup(&mut self, name: &str, len: usize) -> Result<Option<String>, i32> {
        if let Some(v) = self.wsplt_env_find(name, len) {
            return Ok(Some(v));
        }
        if self.ws_options & WRDSO_GETVARPREF != 0 {
            let prefixed = format!("${}", &name[..len.min(name.len())]);
            if let Some(v) = self.wsplt_env_find(&prefixed, prefixed.len()) {
                return Ok(Some(v));
            }
        }
        Ok(None)
    }

    pub fn wsplt_env_getvar(&mut self, name: &str, len: usize) -> Result<Option<String>, i32> {
        if let Some(cb) = self.ws_getvar {
            cb(name, len)
        } else {
            self.wsplt_env_lookup(name, len)
        }
    }

    pub fn wsplt_assign_var(
        &mut self,
        name: &str,
        namelen: usize,
        value: &str,
    ) -> Result<(), i32> {
        let key = &name[..namelen.min(name.len())];
        let item = format!("{key}={value}");
        if self.ws_env_kv {
            for slot in &mut self.ws_env {
                if slot.split_once('=').map(|(k, _)| k == key).unwrap_or(false) {
                    *slot = item.clone();
                    return Ok(());
                }
            }
            self.ws_env.push(item);
        } else {
            self.ws_env.push(item);
        }
        Ok(())
    }

    pub fn wsplt_assign_param(&mut self, param_idx: isize, value: String) -> Result<(), i32> {
        let idx = if param_idx < 0 {
            if self.ws_options & WRDSO_PARAM_NEGIDX == 0 {
                return Err(self.wsplt_seterr(WRDSE_BADPARAM));
            }
            let base = self.ws_paramv.len() as isize + param_idx;
            if base < 0 {
                return Err(self.wsplt_seterr(WRDSE_BADPARAM));
            }
            base as usize
        } else {
            param_idx as usize
        };

        if idx >= self.ws_paramv.len() {
            self.ws_paramv.resize(idx + 1, String::new());
        }
        self.ws_paramv[idx] = value;
        self.ws_paramc = self.ws_paramv.len();
        Ok(())
    }

    pub fn expvar_recover(
        &mut self,
        text: &str,
        ptail: &mut usize,
        pend: &mut usize,
        flg: u32,
    ) -> i32 {
        let start = *pend;
        let mut end = start;
        for (off, ch) in text[start..].char_indices() {
            if ch == '}' || ch == ')' || ch.is_whitespace() {
                break;
            }
            end = start + off + ch.len_utf8();
        }
        if end > start {
            let _ = self.node_split_prefix(ptail, *ptail, 0, 0, flg);
            *pend = end;
        }
        0
    }

    pub fn expand_paramv(
        &mut self,
        ptail: &mut usize,
        flg: u32,
        _q: bool,
    ) -> i32 {
        let values = self.ws_paramv.clone();
        for value in values {
            let Ok(idx) = self.new_node() else {
                return self.wsplt_nomem();
            };
            if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                node.flags = _WSNF_WORD | flg;
                node.word = Some(value);
            }
            self.wsnode_insert(idx, Some(*ptail), false);
            *ptail = idx;
        }
        0
    }

    pub fn expvar(
        &mut self,
        str_: &str,
        len: usize,
        ptail: &mut usize,
        pend: &mut usize,
        flg: u32,
    ) -> i32 {
        let bytes = str_.as_bytes();
        if *pend >= len || *pend >= bytes.len() || bytes[*pend] != b'$' {
            return self.wsplt_setctxerr(WRDSE_BADPARAM, str_, len);
        }
        let mut i = *pend + 1;
        if i >= len || i >= bytes.len() {
            return self.wsplt_setctxerr(WRDSE_BADPARAM, str_, len);
        }

        if bytes[i] == b'{' {
            i += 1;
            let beg = i;
            while i < len && i < bytes.len() && self.is_name_char(bytes[i] as char) {
                i += 1;
            }
            if i >= len || i >= bytes.len() || bytes[i] != b'}' {
                return self.wsplt_setctxerr(WRDSE_CBRACE, str_, len);
            }
            let name = &str_[beg..i];
            *pend = i + 1;
            match self.wsplt_env_getvar(name, name.len()) {
                Ok(Some(val)) => {
                    let Ok(idx) = self.new_node() else {
                        return self.wsplt_nomem();
                    };
                    if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                        node.flags = _WSNF_WORD | flg;
                        node.word = Some(val);
                    }
                    self.wsnode_insert(idx, Some(*ptail), false);
                    *ptail = idx;
                    0
                }
                Ok(None) => {
                    if self.ws_flags & WRDSF_UNDEF != 0 {
                        self.wsplt_setctxerr(WRDSE_UNDEF, name, name.len())
                    } else if self.ws_flags & WRDSF_KEEPUNDEF != 0 {
                        let Ok(idx) = self.new_node() else {
                            return self.wsplt_nomem();
                        };
                        if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                            node.flags = _WSNF_WORD | flg;
                            node.word = Some(format!("${{{name}}}"));
                        }
                        self.wsnode_insert(idx, Some(*ptail), false);
                        *ptail = idx;
                        0
                    } else {
                        0
                    }
                }
                Err(ec) => self.wsplt_seterr(ec),
            }
        } else if bytes[i].is_ascii_digit() {
            let mut num = 0usize;
            while i < len && i < bytes.len() && bytes[i].is_ascii_digit() {
                num = num.saturating_mul(10).saturating_add((bytes[i] - b'0') as usize);
                i += 1;
            }
            *pend = i;
            let value = self.ws_paramv.get(num).cloned().unwrap_or_default();
            let Ok(idx) = self.new_node() else {
                return self.wsplt_nomem();
            };
            if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                node.flags = _WSNF_WORD | flg;
                node.word = Some(value);
            }
            self.wsnode_insert(idx, Some(*ptail), false);
            *ptail = idx;
            0
        } else if self.begin_var_p(bytes[i] as char) {
            let beg = i;
            while i < len && i < bytes.len() && self.is_name_char(bytes[i] as char) {
                i += 1;
            }
            let name = &str_[beg..i];
            *pend = i;
            match self.wsplt_env_getvar(name, name.len()) {
                Ok(Some(val)) => {
                    let Ok(idx) = self.new_node() else {
                        return self.wsplt_nomem();
                    };
                    if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                        node.flags = _WSNF_WORD | flg;
                        node.word = Some(val);
                    }
                    self.wsnode_insert(idx, Some(*ptail), false);
                    *ptail = idx;
                    0
                }
                Ok(None) => {
                    if self.ws_flags & WRDSF_KEEPUNDEF != 0 {
                        let Ok(idx) = self.new_node() else {
                            return self.wsplt_nomem();
                        };
                        if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                            node.flags = _WSNF_WORD | flg;
                            node.word = Some(format!("${name}"));
                        }
                        self.wsnode_insert(idx, Some(*ptail), false);
                        *ptail = idx;
                    }
                    0
                }
                Err(ec) => self.wsplt_seterr(ec),
            }
        } else {
            self.wsplt_setctxerr(WRDSE_BADPARAM, str_, len)
        }
    }

    pub fn begin_var_p(&self, c: char) -> bool {
        matches!(c, '{' | '#' | '@' | '*') || c.is_ascii_alphabetic() || c == '_' || c.is_ascii_digit()
    }

    pub fn node_expand(
        &mut self,
        node: usize,
        beg_p: fn(char) -> bool,
        ws_exp_fn: fn(&mut Wordsplit, &str, usize, &mut usize, &mut usize, u32) -> i32,
    ) -> i32 {
        let text = self.wsnode_ptr(node);
        let len = text.len();
        let mut tail = node;
        let mut pos = 0usize;
        let mut last = 0usize;

        while pos < len {
            let ch = text.as_bytes()[pos] as char;
            if ch == '$' && beg_p(text[pos + 1..].chars().next().unwrap_or('\0')) {
                if pos > last {
                    let _ = self.node_split_prefix(&mut tail, node, last, pos - last, 0);
                }
                let mut end = pos;
                let rc = ws_exp_fn(self, &text, len, &mut tail, &mut end, 0);
                if rc != 0 {
                    return rc;
                }
                pos = end;
                last = end;
            } else {
                pos += 1;
            }
        }

        if last == 0 {
            return 0;
        }
        if last < len {
            let _ = self.node_split_prefix(&mut tail, node, last, len - last, 0);
        }
        self.wsnode_remove(node);
        0
    }

    pub fn wsnode_nullelim(&mut self) {
        let mut p = self.ws_head;
        while let Some(idx) = p {
            let next = self.nodes.get(idx).and_then(|n| n.as_ref()).and_then(|n| n.next);
            let is_null = self
                .nodes
                .get(idx)
                .and_then(|n| n.as_ref())
                .map(|n| n.flags & _WSNF_NULL != 0)
                .unwrap_or(false);
            if is_null {
                self.wsnode_remove(idx);
            }
            p = next;
        }
    }

    pub fn varexp(&mut self) -> i32 {
        let mut p = self.ws_head;
        while let Some(idx) = p {
            let next = self.nodes.get(idx).and_then(|n| n.as_ref()).and_then(|n| n.next);
            let rc = self.node_expand(idx, Self::begin_var_char, Self::expvar_shim);
            if rc != 0 {
                return rc;
            }
            p = next;
        }
        self.wsnode_nullelim();
        0
    }

    fn begin_var_char(c: char) -> bool {
        matches!(c, '{' | '#' | '@' | '*') || c.is_ascii_alphabetic() || c == '_' || c.is_ascii_digit()
    }

    fn expvar_shim(
        wsp: &mut Wordsplit,
        str_: &str,
        len: usize,
        ptail: &mut usize,
        pend: &mut usize,
        flg: u32,
    ) -> i32 {
        wsp.expvar(str_, len, ptail, pend, flg)
    }

    pub fn begin_cmd_p(&self, c: char) -> bool {
        c == '('
    }

    pub fn expcmd(
        &mut self,
        str_: &str,
        len: usize,
        ptail: &mut usize,
        pend: &mut usize,
        flg: u32,
    ) -> i32 {
        let bytes = str_.as_bytes();
        if *pend + 1 >= len || bytes.get(*pend) != Some(&b'$') || bytes.get(*pend + 1) != Some(&b'(') {
            return self.wsplt_setctxerr(WRDSE_PAREN, str_, len);
        }
        let start = *pend + 2;
        let Some(end) = self.find_closing_paren(str_, start, len, "()") else {
            return self.wsplt_setctxerr(WRDSE_PAREN, str_, len);
        };
        let body = &str_[start..end];
        *pend = end + 1;

        let value = if let Some(cmd) = self.ws_command {
            match cmd(body, &self.ws_paramv) {
                Ok(Some(v)) => v,
                Ok(None) => String::new(),
                Err(ec) => return self.wsplt_seterr(ec),
            }
        } else {
            body.to_string()
        };

        let Ok(idx) = self.new_node() else {
            return self.wsplt_nomem();
        };
        if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
            node.flags = _WSNF_WORD | flg;
            node.word = Some(value);
        }
        self.wsnode_insert(idx, Some(*ptail), false);
        *ptail = idx;
        0
    }

    pub fn cmdexp(&mut self) -> i32 {
        let mut p = self.ws_head;
        while let Some(idx) = p {
            let next = self.nodes.get(idx).and_then(|n| n.as_ref()).and_then(|n| n.next);
            let text = self.wsnode_ptr(idx);
            if text.contains("$(") {
                let mut tail = idx;
                let mut pos = 0usize;
                let mut last = 0usize;
                let len = text.len();
                while pos < len {
                    if text.as_bytes()[pos] == b'$'
                        && pos + 1 < len
                        && self.begin_cmd_p(text.as_bytes()[pos + 1] as char)
                    {
                        if pos > last {
                            let _ = self.node_split_prefix(&mut tail, idx, last, pos - last, 0);
                        }
                        let mut end = pos;
                        let rc = self.expcmd(&text, len, &mut tail, &mut end, 0);
                        if rc != 0 {
                            return rc;
                        }
                        pos = end;
                        last = end;
                    } else {
                        pos += 1;
                    }
                }
                if last < len {
                    let _ = self.node_split_prefix(&mut tail, idx, last, len - last, 0);
                }
                if last > 0 {
                    self.wsnode_remove(idx);
                }
            }
            p = next;
        }
        self.wsnode_nullelim();
        0
    }

    pub fn trimws(&mut self) -> i32 {
        let mut changed = false;
        let mut p = self.ws_head;
        while let Some(idx) = p {
            let next = self.nodes.get(idx).and_then(|n| n.as_ref()).and_then(|n| n.next);
            let text = self.wsnode_ptr(idx);
            let trimmed = text.trim().to_string();
            if trimmed != text {
                changed = true;
                if trimmed.is_empty() {
                    if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                        node.flags = _WSNF_NULL;
                        node.word = Some(String::new());
                    }
                } else if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                    node.flags |= _WSNF_WORD;
                    node.word = Some(trimmed);
                }
            }
            p = next;
        }
        if changed {
            self.wsnode_nullelim();
        }
        0
    }

    pub fn tildexpand(&mut self) -> i32 {
        let home = env::var("HOME").unwrap_or_else(|_| String::from("~"));
        let mut p = self.ws_head;
        while let Some(idx) = p {
            let next = self.nodes.get(idx).and_then(|n| n.as_ref()).and_then(|n| n.next);
            let text = self.wsnode_ptr(idx);
            if let Some(rest) = text.strip_prefix('~') {
                let expanded = if rest.is_empty() || rest.starts_with('/') {
                    format!("{home}{rest}")
                } else {
                    text
                };
                if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                    node.flags |= _WSNF_WORD;
                    node.word = Some(expanded);
                }
            }
            p = next;
        }
        0
    }

    pub fn is_glob(&self, s: &str, l: usize) -> bool {
        let bytes = s.as_bytes();
        let mut i = 0usize;
        while i < l && i < bytes.len() {
            match bytes[i] {
                b'\\' => i += 1,
                b'*' | b'?' | b'[' => return true,
                _ => {}
            }
            i += 1;
        }
        false
    }

    pub fn pathexpand(&mut self) -> i32 {
        let mut p = self.ws_head;
        while let Some(idx) = p {
            let next = self.nodes.get(idx).and_then(|n| n.as_ref()).and_then(|n| n.next);
            let text = self.wsnode_ptr(idx);
            if self.is_glob(&text, text.len()) {
                if self.ws_options & WRDSO_FAILGLOB != 0 {
                    return self.wsplt_setctxerr(WRDSE_GLOBERR, &text, text.len());
                }
                if self.ws_options & WRDSO_NULLGLOB != 0 {
                    if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                        node.flags = _WSNF_NULL;
                        node.word = Some(String::new());
                    }
                }
            }
            p = next;
        }
        0
    }

    pub fn skip_sed_expr(&self, command: &str, i: usize, len: usize) -> Option<usize> {
        let bytes = command.as_bytes();
        if i >= len || i >= bytes.len() {
            return None;
        }
        let mut p = i;
        if bytes[p] == b's' && p + 1 < len {
            p += 1;
        }
        let delim = *bytes.get(p)?;
        p += 1;
        for _ in 0..3 {
            let mut found = false;
            while p < len && p < bytes.len() {
                if bytes[p] == b'\\' {
                    p += 2;
                    continue;
                }
                if bytes[p] == delim {
                    p += 1;
                    found = true;
                    break;
                }
                p += 1;
            }
            if !found {
                return None;
            }
        }
        while p < len && p < bytes.len() && bytes[p].is_ascii_alphabetic() {
            p += 1;
        }
        Some(p)
    }

    pub fn skip_delim_internal(&self, return_delims: bool) -> usize {
        if return_delims {
            self.ws_endp
        } else {
            self.ws_endp.saturating_add(1)
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
            return self.wsplt_seterr(WRDSE_QUOTE);
        }
        let quote = bytes[start];
        let mut i = start + 1;
        while i < self.ws_len && i < bytes.len() {
            if quote == b'"' && bytes[i] == b'\\' {
                i += 2;
                continue;
            }
            if bytes[i] == quote {
                *end = i + 1;
                return self.add_segm(start + 1, i, _WSNF_QUOTE);
            }
            i += 1;
        }
        let ctx = self.ws_input[start..].to_string();
        self.wsplt_setctxerr(WRDSE_QUOTE, &ctx, self.ws_len - start)
    }

    pub fn scan_word(&mut self, start: usize, consume_all: bool) -> i32 {
        let bytes = self.ws_input.as_bytes();
        let mut i = start;
        while i < self.ws_len && i < bytes.len() {
            let c = bytes[i];
            if (self.ws_flags & WRDSF_COMMENT != 0)
                && self.ws_comment.as_deref().unwrap_or("#").as_bytes().contains(&c)
            {
                break;
            }
            if self.ws_delim.as_bytes().contains(&c) && !consume_all {
                break;
            }
            if c == b'\'' || c == b'"' {
                if i > start {
                    let rc = self.add_segm(start, i, 0);
                    if rc != 0 {
                        return rc;
                    }
                }
                let mut end = i;
                let rc = self.scan_qstring(i, &mut end);
                if rc != 0 {
                    return rc;
                }
                self.ws_endp = end.saturating_sub(1);
                return 0;
            }
            if (self.ws_flags & WRDSF_SED_EXPR != 0) && c == b's' {
                if let Some(end) = self.skip_sed_expr(&self.ws_input, i, self.ws_len) {
                    i = end;
                    continue;
                }
            }
            if c == b'\\' && self.ws_flags & (WRDSF_ESCAPE | WRDSF_CESCAPES) != 0 {
                i += 2;
                continue;
            }
            i += 1;
        }
        self.ws_endp = i;
        self.add_segm(start, i, 0)
    }

    pub fn xtonum(&self, src: &str, base: u32, cnt: usize) -> Result<i32, i32> {
        let mut val: i32 = 0;
        let mut n = 0usize;
        for ch in src.chars().take(cnt) {
            let Some(d) = ch.to_digit(base) else {
                return Err(WRDSE_USAGE);
            };
            val = val
                .checked_mul(base as i32)
                .and_then(|v| v.checked_add(d as i32))
                .ok_or(WRDSE_USAGE)?;
            n += 1;
        }
        if n == 0 {
            Err(WRDSE_USAGE)
        } else {
            Ok(val)
        }
    }

    pub fn c_quoted_length(&self, str_: &str, quote_hex: bool, quote: &mut bool) -> usize {
        let mut len = 0usize;
        *quote = false;
        for b in str_.bytes() {
            let qc = self.c_quote_char(b as char);
            if qc != b as char {
                *quote = true;
                if quote_hex && !qc.is_control() && !qc.is_ascii() {
                    len += 4;
                } else {
                    len += 2;
                }
            } else {
                len += 1;
            }
        }
        len
    }

    pub fn wsplt_unquote_char(&self, transtab: &str, c: char) -> char {
        let tb = transtab.as_bytes();
        let mut i = 0usize;
        while i + 1 < tb.len() {
            if tb[i + 1] as char == c {
                return tb[i] as char;
            }
            i += 2;
        }
        c
    }

    pub fn wsplt_quote_char(&self, transtab: &str, c: char) -> char {
        let tb = transtab.as_bytes();
        let mut i = 0usize;
        while i + 1 < tb.len() {
            if tb[i] as char == c {
                return tb[i + 1] as char;
            }
            i += 2;
        }
        c
    }

    pub fn c_unquote_char(&self, c: char) -> char {
        self.wsplt_unquote_char(&self.ws_escape[WRDSX_QUOTE], c)
    }

    pub fn c_quote_char(&self, c: char) -> char {
        self.wsplt_quote_char(&self.ws_escape[WRDSX_QUOTE], c)
    }

    pub fn string_unquote_copy(&self, inquote: i32, src: &str) -> String {
        let mut out = String::with_capacity(src.len());
        let bytes = src.as_bytes();
        let mut i = 0usize;
        while i < bytes.len() {
            if bytes[i] == b'\\' && i + 1 < bytes.len() {
                let c = bytes[i + 1] as char;
                if self.ws_flags & WRDSF_CESCAPES != 0 {
                    match c {
                        'n' => out.push('\n'),
                        't' => out.push('\t'),
                        'r' => out.push('\r'),
                        'x' => {
                            let hex = &src[(i + 2).min(src.len())..(i + 4).min(src.len())];
                            if let Ok(v) = self.xtonum(hex, 16, hex.len()) {
                                if let Some(ch) = char::from_u32(v as u32) {
                                    out.push(ch);
                                    i += 2 + hex.len();
                                    continue;
                                }
                            }
                            out.push(c);
                        }
                        '0'..='7' => {
                            let end = (i + 4).min(src.len());
                            let oct = &src[i + 1..end];
                            let oct_len = oct
                                .chars()
                                .take_while(|ch| ('0'..='7').contains(ch))
                                .count()
                                .min(3);
                            if let Ok(v) = self.xtonum(&src[i + 1..i + 1 + oct_len], 8, oct_len) {
                                if let Some(ch) = char::from_u32(v as u32) {
                                    out.push(ch);
                                    i += 1 + oct_len;
                                    continue;
                                }
                            }
                            out.push(c);
                        }
                        _ => out.push(self.c_unquote_char(c)),
                    }
                } else if inquote != 0 {
                    out.push(self.wsplt_unquote_char(&self.ws_escape[WRDSX_QUOTE], c));
                } else {
                    out.push(self.wsplt_unquote_char(&self.ws_escape[WRDSX_WORD], c));
                }
                i += 2;
            } else {
                out.push(bytes[i] as char);
                i += 1;
            }
        }
        out
    }

    pub fn c_quote_copy(&self, src: &str, quote_hex: bool) -> String {
        let mut out = String::new();
        for ch in src.chars() {
            let q = self.c_quote_char(ch);
            if q != ch {
                out.push('\\');
                out.push(q);
            } else if quote_hex && ch.is_control() {
                out.push_str(&format!("\\x{:02x}", ch as u32));
            } else {
                out.push(ch);
            }
        }
        out
    }

    pub fn exptab_matches(&self, enabled: bool) -> bool {
        enabled
    }

    pub fn process_list(&mut self, start: usize) -> i32 {
        let input_len = self.ws_input.len();
        let mut i = start;

        if self.ws_flags & WRDSF_RETURN_DELIMS != 0
            && i < self.ws_len
            && i < input_len
            && self.ws_delim.as_bytes().contains(&self.ws_input.as_bytes()[i])
        {
            let rc = self.add_segm(i, i + 1, _WSNF_DELIM | _WSNF_CONST);
            if rc != 0 {
                return rc;
            }
            self.ws_endp = i;
            return self.finish();
        }

        while i < self.ws_len && i < input_len {
            while i < self.ws_len
                && i < input_len
                && self.ws_delim.as_bytes().contains(&self.ws_input.as_bytes()[i])
            {
                if self.ws_flags & WRDSF_RETURN_DELIMS != 0 {
                    let rc = self.add_segm(i, i + 1, _WSNF_DELIM | _WSNF_CONST);
                    if rc != 0 {
                        return rc;
                    }
                }
                i += 1;
                if self.ws_flags & WRDSF_INCREMENTAL != 0 {
                    self.ws_endp = i.saturating_sub(1);
                    return self.finish();
                }
            }
            if i >= self.ws_len || i >= input_len {
                break;
            }

            let rc = self.scan_word(i, self.ws_flags & WRDSF_NOSPLIT != 0);
            if rc != 0 {
                return rc;
            }
            i = self.ws_endp;
            if i < self.ws_len
                && i < input_len
                && !self.ws_delim.as_bytes().contains(&self.ws_input.as_bytes()[i])
            {
                i += 1;
            }
            if self.ws_flags & WRDSF_INCREMENTAL != 0 {
                break;
            }
        }

        self.ws_endp = i.min(self.ws_len);
        if self.ws_flags & WRDSF_SHOWDBG != 0 {
            self.dump_nodes();
        }
        self.wsnode_coalesce()
    }

    pub fn run(
        &mut self,
        command: Option<&str>,
        length: usize,
        flags: u64,
        lvl: usize,
        finalize: bool,
    ) -> i32 {
        self.ws_lvl = lvl;
        self.init_0();
        self.ws_flags = flags;
        self.ws_input = command.unwrap_or_default().to_string();
        self.ws_len = length.min(self.ws_input.len());
        self.ws_endp = 0;

        if command.is_none() {
            return self.wsplt_seterr(WRDSE_NOINPUT);
        }

        let rc = self.process_list(0);
        if rc != 0 {
            return rc;
        }
        if self.ws_flags & WRDSF_NOVAR == 0 {
            let rc = self.varexp();
            if rc != 0 {
                return rc;
            }
        }
        if self.ws_flags & WRDSF_NOCMD == 0 {
            let rc = self.cmdexp();
            if rc != 0 {
                return rc;
            }
        }
        let _ = self.tildexpand();
        let _ = self.pathexpand();
        let _ = self.wsnode_quoteremoval();

        if finalize {
            self.finish()
        } else {
            0
        }
    }

    pub fn len(&mut self, command: &str, length: usize, flags: u64) -> i32 {
        self.run(Some(command), length, flags, 0, true)
    }

    pub fn wordsplit(&mut self, command: &str, flags: u64) -> i32 {
        self.len(command, command.len(), flags)
    }

    pub fn free_words(&mut self) {
        self.ws_wordv.clear();
        self.ws_wordc = 0;
        self.ws_wordn = 0;
        self.ws_wordi = 0;
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
        self.ws_errno = WRDSE_OK;
        self.ws_errctx = None;
        self.ws_usererr = None;
    }

    pub fn get_words(&self) -> Vec<&str> {
        self.ws_wordv
            .iter()
            .take(self.ws_wordc)
            .filter_map(|s| s.as_deref())
            .collect()
    }

    pub fn strerror(&self) -> &'static str {
        match self.ws_errno {
            WRDSE_OK => "no error",
            WRDSE_EOF => "unexpected end of input",
            WRDSE_QUOTE => "unterminated quoted string",
            WRDSE_USAGE => "invalid usage",
            WRDSE_NOSPACE => "memory exhausted",
            WRDSE_UNDEF => "undefined variable",
            WRDSE_CBRACE => "missing closing brace",
            WRDSE_PAREN => "missing closing parenthesis",
            WRDSE_BADPARAM => "bad parameter expansion",
            WRDSE_NOINPUT => "no input",
            WRDSE_GLOBERR => "glob expansion failed",
            WRDSE_USERERR => "user-defined error",
            _ => "unknown error",
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
        if let Some(user) = &self.ws_usererr {
            if !user.is_empty() {
                msg.push_str(": ");
                msg.push_str(user);
            }
        }
        (self.ws_error_cb)(&msg);
    }
}

impl Drop for Wordsplit {
    fn drop(&mut self) {
        self.clearerr();
        self.free_nodes();
        self.free_words();
        self.free_envbuf();
        self.free_parambuf();
    }
}

impl fmt::Debug for Wordsplit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Wordsplit")
            .field("ws_flags", &self.ws_flags)
            .field("ws_options", &self.ws_options)
            .field("ws_input", &self.ws_input)
            .field("ws_wordc", &self.ws_wordc)
            .field("ws_errno", &self.ws_errno)
            .finish()
    }
}

#[allow(dead_code)]
fn _home_dir() -> Option<PathBuf> {
    env::var_os("HOME").map(PathBuf::from)
}
