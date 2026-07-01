use std::io::{self, Write};

const WRDSF_SHOWERR: u32 = 1 << 0;
const WRDSF_ENOMEMABRT: u32 = 1 << 1;
const WRDSF_REUSE: u32 = 1 << 2;
const WRDSF_APPEND: u32 = 1 << 3;
const WRDSF_ALLOC_DIE: u32 = 1 << 4;
const WRDSF_ERROR: u32 = 1 << 5;
const WRDSF_NOVAR: u32 = 1 << 6;
const WRDSF_ENV: u32 = 1 << 7;
const WRDSF_ENV_KV: u32 = 1 << 8;
const WRDSF_GETVAR: u32 = 1 << 9;
const WRDSF_NOCMD: u32 = 1 << 10;
const WRDSF_CLOSURE: u32 = 1 << 11;
const WRDSF_DEBUG: u32 = 1 << 12;
const WRDSF_SHOWDBG: u32 = 1 << 13;
const WRDSF_OPTIONS: u32 = 1 << 14;
const WRDSF_DOOFFS: u32 = 1 << 15;
const WRDSF_DELIM: u32 = 1 << 16;
const WRDSF_COMMENT: u32 = 1 << 17;
const WRDSF_ESCAPE: u32 = 1 << 18;
const WRDSF_CESCAPES: u32 = 1 << 19;
const WRDSF_QUOTE: u32 = 1 << 20;
const WRDSF_SQUOTE: u32 = 1 << 21;
const WRDSF_DQUOTE: u32 = 1 << 22;
const WRDSF_SED_EXPR: u32 = 1 << 23;
const WRDSF_RETURN_DELIMS: u32 = 1 << 24;
const WRDSF_SQUEEZE_DELIMS: u32 = 1 << 25;
const WRDSF_INCREMENTAL: u32 = 1 << 26;
const WRDSF_NOSPLIT: u32 = 1 << 27;
const WRDSF_WS: u32 = 1 << 28;
const WRDSF_UNDEF: u32 = 1 << 29;
const WRDSF_WARNUNDEF: u32 = 1 << 30;
const WRDSF_KEEPUNDEF: u32 = 1 << 31;

const WRDSO_NAMECHAR: u32 = 1 << 0;
const WRDSO_PARAMV: u32 = 1 << 1;
const WRDSO_PARAM_NEGIDX: u32 = 1 << 2;
const WRDSO_GETVARPREF: u32 = 1 << 3;
const WRDSO_NOVARSPLIT: u32 = 1 << 4;
const WRDSO_NOCMDSPLIT: u32 = 1 << 5;
const WRDSO_MAXWORDS: u32 = 1 << 6;
const WRDSO_RETDELNOTEMPTY: u32 = 1 << 7;
const WRDSO_OESC_QUOTE: u32 = 1 << 8;
const WRDSO_OESC_WORD: u32 = 1 << 9;
const WRDSO_XESC_QUOTE: u32 = 1 << 10;
const WRDSO_XESC_WORD: u32 = 1 << 11;
const WRDSO_BSKEEP_QUOTE: u32 = 1 << 12;
const WRDSO_BSKEEP_WORD: u32 = 1 << 13;
const WRDSO_DOTGLOB: u32 = 1 << 14;
const WRDSO_NULLGLOB: u32 = 1 << 15;
const WRDSO_FAILGLOB: u32 = 1 << 16;

const WRDSX_WORD: usize = 0;
const WRDSX_QUOTE: usize = 1;

const WSNF_WORD: u32 = 1 << 0;
const WSNF_NULL: u32 = 1 << 1;
const WSNF_QUOTE: u32 = 1 << 2;
const WSNF_NOEXPAND: u32 = 1 << 3;
const WSNF_JOIN: u32 = 1 << 4;
const WSNF_SEXP: u32 = 1 << 5;
const WSNF_DELIM: u32 = 1 << 6;
const WSNF_CONST: u32 = 1 << 7;
const WSNF_EMPTYOK: u32 = 1 << 8;

const WRDSE_OK: i32 = 0;
const WRDSE_EOF: i32 = 1;
const WRDSE_USAGE: i32 = 2;
const WRDSE_NOSPACE: i32 = 3;
const WRDSE_UNDEF: i32 = 4;
const WRDSE_BADPARAM: i32 = 5;
const WRDSE_QUOTE: i32 = 6;
const WRDSE_CBRACE: i32 = 7;
const WRDSE_PAREN: i32 = 8;
const WRDSE_GLOBERR: i32 = 9;
const WRDSE_USERERR: i32 = 10;
const WRDSE_NOINPUT: i32 = 11;

const WRDS_OK: i32 = 0;
const WRDS_ERR: i32 = -1;
const WRDS_EOF: i32 = 1;

const ALLOC_INIT: usize = 16;
const ALLOC_INCR: usize = 16;
const WORDSPLIT_ENV_INIT: usize = 16;

const EXPOPT_COALESCE: u32 = 1 << 0;
const EXPOPT_NEG: u32 = 1 << 1;
const EXPOPT_ALLOF: u32 = 1 << 2;

#[derive(Clone, Default)]
pub struct WordsplitNode {
    flags: u32,
    prev: Option<usize>,
    next: Option<usize>,
    kind: NodeKind,
}

#[derive(Clone, Default)]
enum NodeKind {
    #[default]
    Null,
    Segment { beg: usize, end: usize },
    Word(String),
}

#[derive(Clone)]
pub struct Wordsplit {
    ws_flags: u32,
    ws_options: u32,
    ws_errno: i32,
    ws_errctx: Option<String>,
    ws_usererr: Option<String>,
    ws_input: String,
    ws_len: usize,
    ws_delim: String,
    ws_comment: Option<String>,
    ws_namechar: Option<String>,
    ws_escape: [String; 2],
    ws_sep: String,
    ws_endp: usize,
    ws_wordi: usize,
    ws_offs: usize,
    ws_wordc: usize,
    ws_wordn: usize,
    ws_wordv: Vec<Option<String>>,
    ws_head: Option<usize>,
    ws_tail: Option<usize>,
    nodes: Vec<Option<WordsplitNode>>,
    ws_lvl: i32,
    ws_maxwords: usize,
    ws_env: Option<Vec<String>>,
    ws_envbuf: Option<Vec<String>>,
    ws_envidx: usize,
    ws_envsiz: usize,
    ws_paramv: Vec<String>,
    ws_parambuf: Option<Vec<String>>,
    ws_paramc: usize,
    ws_paramidx: usize,
    ws_paramsiz: usize,
    ws_closure: Option<String>,
}

struct ExpTab {
    descr: &'static str,
    flag: u32,
    opt: u32,
    expansion: Option<fn(&mut Wordsplit) -> i32>,
}

impl Default for Wordsplit {
    fn default() -> Self {
        Self {
            ws_flags: 0,
            ws_options: 0,
            ws_errno: WRDSE_OK,
            ws_errctx: None,
            ws_usererr: None,
            ws_input: String::new(),
            ws_len: 0,
            ws_delim: " \t\n".to_string(),
            ws_comment: None,
            ws_namechar: None,
            ws_escape: [String::new(), String::new()],
            ws_sep: " ".to_string(),
            ws_endp: 0,
            ws_wordi: 0,
            ws_offs: 0,
            ws_wordc: 0,
            ws_wordn: 0,
            ws_wordv: Vec::new(),
            ws_head: None,
            ws_tail: None,
            nodes: Vec::new(),
            ws_lvl: 0,
            ws_maxwords: 0,
            ws_env: None,
            ws_envbuf: None,
            ws_envidx: 0,
            ws_envsiz: 0,
            ws_paramv: Vec::new(),
            ws_parambuf: None,
            ws_paramc: 0,
            ws_paramidx: 0,
            ws_paramsiz: 0,
            ws_closure: None,
        }
    }
}

impl Wordsplit {
    pub fn node() -> WordsplitNode {
        WordsplitNode::default()
    }

    pub fn wordsplit_node_06() -> WordsplitNode {
        Self::node()
    }

    pub fn wordsplit_04() -> Self {
        Self::default()
    }

    pub fn wordsplit_05() -> Self {
        Self::default()
    }

    pub fn wordsplit_c_07() -> [String; 2] {
        [String::new(), String::new()]
    }

    pub fn wordsplit_01() -> Vec<String> {
        Vec::new()
    }

    pub fn wordsplit_02() -> Vec<String> {
        Vec::new()
    }

    pub fn quote_hex() -> bool {
        true
    }

    pub fn is_name_char(&self, c: char) -> bool {
        c.is_ascii_alphanumeric()
            || c == '_'
            || ((self.ws_options & WRDSO_NAMECHAR) != 0
                && self
                    .ws_namechar
                    .as_deref()
                    .map(|s| s.contains(c))
                    .unwrap_or(false))
    }

    pub fn wsplt_alloc_die(&self) -> ! {
        self.wsplt_error("memory exhausted");
        panic!("memory exhausted");
    }

    pub fn wsplt_error(&self, message: &str) {
        let _ = writeln!(io::stderr(), "{message}");
    }

    pub fn wsplt_seterr(&mut self, ec: i32) -> i32 {
        self.ws_errno = ec;
        if (self.ws_flags & WRDSF_SHOWERR) != 0 {
            self.perror();
        }
        ec
    }

    pub fn wsplt_nomem(&mut self) -> i32 {
        self.ws_errno = WRDSE_NOSPACE;
        if (self.ws_flags & WRDSF_ENOMEMABRT) != 0 {
            self.wsplt_alloc_die();
        }
        if (self.ws_flags & WRDSF_SHOWERR) != 0 {
            self.perror();
        }
        if (self.ws_flags & WRDSF_REUSE) == 0 {
            self.free_words();
            self.free_envbuf();
            self.free_parambuf();
        }
        self.free_nodes();
        self.ws_errno
    }

    pub fn wsplt_store_errctx(&mut self, s: &str, len: usize) {
        let take = len.min(s.len());
        self.ws_errctx = Some(s[..take].to_string());
    }

    pub fn wsplt_setctxerr(&mut self, ec: i32, s: &str, len: usize) -> i32 {
        self.wsplt_store_errctx(s, len);
        self.wsplt_seterr(ec)
    }

    pub fn wsplt_subsplit(
        &mut self,
        child: &mut Wordsplit,
        s: &str,
        len: usize,
        mut flags: u32,
        finalize: bool,
    ) -> i32 {
        child.ws_delim = self.ws_delim.clone();
        child.ws_options = self.ws_options & !WRDSO_MAXWORDS;
        child.ws_namechar = self.ws_namechar.clone();
        child.ws_closure = self.ws_closure.clone();

        if (flags & WRDSF_NOVAR) == 0 {
            if let Some(env) = &self.ws_env {
                child.ws_env = Some(env.clone());
            }
            flags |= self.ws_flags & (WRDSF_ENV | WRDSF_ENV_KV | WRDSF_GETVAR);
        }
        if (flags & WRDSF_NOCMD) == 0 {
            let _ = self.ws_flags;
        }
        flags |= WRDSF_DELIM
            | WRDSF_ALLOC_DIE
            | WRDSF_ERROR
            | WRDSF_DEBUG
            | (self.ws_flags & (WRDSF_SHOWDBG | WRDSF_SHOWERR | WRDSF_OPTIONS));

        let rc = child.new(&s[..len.min(s.len())], len.min(s.len()), flags);
        if rc != 0 {
            return rc;
        }
        child.ws_lvl = self.ws_lvl + 1;
        let rc = child.process_list(0);
        if rc != 0 {
            child.free_nodes();
            return rc;
        }
        if finalize {
            let rc = child.finish();
            child.free_nodes();
            return rc;
        }
        0
    }

    pub fn wsplt_seterr_sub(&mut self, child: &mut Wordsplit) {
        self.ws_errno = child.ws_errno;
        self.ws_usererr = child.ws_usererr.take();
        self.ws_errctx = child.ws_errctx.take();
    }

    pub fn init_0(&mut self) {
        if (self.ws_flags & WRDSF_REUSE) != 0 {
            if (self.ws_flags & WRDSF_APPEND) == 0 {
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

    pub fn new(&mut self, input: &str, len: usize, flags: u32) -> i32 {
        self.ws_flags = flags;
        self.ws_input = input[..len.min(input.len())].to_string();
        self.ws_len = self.ws_input.len();

        if (self.ws_flags & WRDSF_NOVAR) == 0 {
            self.ws_envidx = 0;
            self.ws_envsiz = 0;
            self.ws_envbuf = None;
        }

        if (self.ws_flags & WRDSF_NOCMD) == 0 {
            // Rust port keeps command expansion optional; no callback validation here.
        }

        if (self.ws_flags & WRDSF_DOOFFS) == 0 {
            self.ws_offs = 0;
        }

        if (self.ws_flags & WRDSF_DELIM) == 0 {
            self.ws_delim = " \t\n".to_string();
        }
        self.ws_sep = self.ws_delim.chars().next().unwrap_or(' ').to_string();

        if (self.ws_flags & WRDSF_COMMENT) == 0 {
            self.ws_comment = None;
        }

        if (self.ws_flags & WRDSF_OPTIONS) == 0 {
            self.ws_options = 0;
        }

        if (self.ws_flags & WRDSF_CESCAPES) != 0 {
            self.ws_escape[WRDSX_WORD] = "\\\\\"\"ttnnrr".to_string();
            self.ws_escape[WRDSX_QUOTE] = "\\\\\"\"ttnnrr".to_string();
            self.ws_options |=
                WRDSO_OESC_QUOTE | WRDSO_OESC_WORD | WRDSO_XESC_QUOTE | WRDSO_XESC_WORD;
        } else if (self.ws_flags & WRDSF_ESCAPE) == 0 {
            self.ws_escape[WRDSX_WORD].clear();
            self.ws_escape[WRDSX_QUOTE].clear();
        }

        if (self.ws_options & WRDSO_PARAMV) == 0 {
            self.ws_paramv.clear();
            self.ws_paramc = 0;
        }
        self.ws_paramidx = 0;
        self.ws_paramsiz = 0;
        self.ws_parambuf = None;

        if (self.ws_options & WRDSO_NAMECHAR) != 0 {
            if let Some(namechar) = &self.ws_namechar {
                if namechar.chars().any(|c| "${}*@-+?=".contains(c)) {
                    return self.wsplt_seterr(WRDSE_USAGE);
                }
            }
        } else {
            self.ws_namechar = None;
        }

        self.ws_endp = 0;
        self.ws_wordi = 0;

        if (self.ws_flags & WRDSF_REUSE) != 0 {
            self.free_nodes();
        }
        self.ws_head = None;
        self.ws_tail = None;
        self.ws_errctx = None;
        self.init_0();
        0
    }

    pub fn alloc_space(&mut self, count: usize) -> i32 {
        let offs = if (self.ws_flags & WRDSF_DOOFFS) != 0 {
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
            self.ws_wordv.resize(newalloc, None);
            self.ws_wordn = newalloc;
            0
        } else if self.ws_wordn < needed {
            let newalloc = offs + self.ws_wordc + if count > ALLOC_INCR { count } else { ALLOC_INCR };
            self.ws_wordv.resize(newalloc, None);
            self.ws_wordn = newalloc;
            0
        } else {
            0
        }
    }

    pub fn wsnode_flagstr(flags: u32) -> String {
        let mut s = String::with_capacity(6);
        s.push(if (flags & WSNF_WORD) != 0 {
            'w'
        } else if (flags & WSNF_NULL) != 0 {
            'n'
        } else {
            '-'
        });
        s.push(if (flags & WSNF_QUOTE) != 0 { 'q' } else { '-' });
        s.push(if (flags & WSNF_NOEXPAND) != 0 { 'E' } else { '-' });
        s.push(if (flags & WSNF_JOIN) != 0 { 'j' } else { '-' });
        s.push(if (flags & WSNF_SEXP) != 0 { 's' } else { '-' });
        s.push(if (flags & WSNF_DELIM) != 0 { 'd' } else { '-' });
        s
    }

    pub fn wsnode_ptr(&self, idx: usize) -> String {
        let Some(node) = self.nodes.get(idx).and_then(|n| n.as_ref()) else {
            return String::new();
        };
        if (node.flags & WSNF_NULL) != 0 {
            String::new()
        } else if (node.flags & WSNF_WORD) != 0 {
            match &node.kind {
                NodeKind::Word(s) => s.clone(),
                _ => String::new(),
            }
        } else {
            match node.kind {
                NodeKind::Segment { beg, end } => self.ws_input[beg.min(self.ws_input.len())..end.min(self.ws_input.len())].to_string(),
                _ => String::new(),
            }
        }
    }

    pub fn wsnode_len(&self, idx: usize) -> usize {
        let Some(node) = self.nodes.get(idx).and_then(|n| n.as_ref()) else {
            return 0;
        };
        if (node.flags & WSNF_NULL) != 0 {
            0
        } else if (node.flags & WSNF_WORD) != 0 {
            match &node.kind {
                NodeKind::Word(s) => s.len(),
                _ => 0,
            }
        } else {
            match node.kind {
                NodeKind::Segment { beg, end } => end.saturating_sub(beg),
                _ => 0,
            }
        }
    }

    pub fn alloc_node(&mut self) -> Result<usize, i32> {
        self.nodes.push(Some(WordsplitNode::default()));
        Ok(self.nodes.len() - 1)
    }

    fn release_node(&mut self, idx: usize) {
        if let Some(slot) = self.nodes.get_mut(idx) {
            *slot = None;
        }
    }

    pub fn wsnode_append(&mut self, idx: usize) {
        if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
            node.next = None;
            node.prev = self.ws_tail;
        }
        if let Some(tail) = self.ws_tail {
            if let Some(tail_node) = self.nodes.get_mut(tail).and_then(|n| n.as_mut()) {
                tail_node.next = Some(idx);
            }
        } else {
            self.ws_head = Some(idx);
        }
        self.ws_tail = Some(idx);
    }

    pub fn wsnode_remove(&mut self, idx: usize) {
        let Some(node) = self.nodes.get(idx).and_then(|n| n.as_ref()).cloned() else {
            return;
        };

        if let Some(prev) = node.prev {
            if let Some(prev_node) = self.nodes.get_mut(prev).and_then(|n| n.as_mut()) {
                prev_node.next = node.next;
                if node.next.is_none() {
                    prev_node.flags &= !WSNF_JOIN;
                }
            }
        } else {
            self.ws_head = node.next;
        }

        if let Some(next) = node.next {
            if let Some(next_node) = self.nodes.get_mut(next).and_then(|n| n.as_mut()) {
                next_node.prev = node.prev;
            }
        } else {
            self.ws_tail = node.prev;
        }

        self.release_node(idx);
    }

    pub fn wsnode_tail(&self, mut idx: usize) -> usize {
        while let Some(next) = self
            .nodes
            .get(idx)
            .and_then(|n| n.as_ref())
            .and_then(|n| n.next)
        {
            idx = next;
        }
        idx
    }

    pub fn wsnode_insert(&mut self, idx: usize, anchor: usize, before: bool) {
        if self.ws_head.is_none() {
            if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                node.next = None;
                node.prev = None;
            }
            self.ws_head = Some(idx);
            self.ws_tail = Some(idx);
        } else if before {
            let anchor_prev = self
                .nodes
                .get(anchor)
                .and_then(|n| n.as_ref())
                .and_then(|n| n.prev);
            if let Some(prev) = anchor_prev {
                self.wsnode_insert(idx, prev, false);
            } else {
                let tail = self.wsnode_tail(idx);
                if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                    node.prev = None;
                }
                if let Some(tail_node) = self.nodes.get_mut(tail).and_then(|n| n.as_mut()) {
                    tail_node.next = Some(anchor);
                }
                if let Some(anchor_node) = self.nodes.get_mut(anchor).and_then(|n| n.as_mut()) {
                    anchor_node.prev = Some(tail);
                }
                self.ws_head = Some(idx);
            }
        } else {
            let tail = self.wsnode_tail(idx);
            let p = self
                .nodes
                .get(anchor)
                .and_then(|n| n.as_ref())
                .and_then(|n| n.next);
            if let Some(next) = p {
                if let Some(next_node) = self.nodes.get_mut(next).and_then(|n| n.as_mut()) {
                    next_node.prev = Some(tail);
                }
            } else {
                self.ws_tail = Some(tail);
            }
            if let Some(tail_node) = self.nodes.get_mut(tail).and_then(|n| n.as_mut()) {
                tail_node.next = p;
            }
            if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                node.prev = Some(anchor);
            }
            if let Some(anchor_node) = self.nodes.get_mut(anchor).and_then(|n| n.as_mut()) {
                anchor_node.next = Some(idx);
            }
        }
    }

    pub fn add_segm(&mut self, beg: usize, end: usize, flg: u32) -> i32 {
        if end == beg && (flg & WSNF_EMPTYOK) == 0 {
            return 0;
        }
        let Ok(idx) = self.alloc_node() else {
            return self.wsplt_nomem();
        };
        if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
            node.flags = flg & !(WSNF_WORD | WSNF_EMPTYOK);
            node.kind = NodeKind::Segment { beg, end };
        }
        self.wsnode_append(idx);
        0
    }

    pub fn free_nodes(&mut self) {
        self.nodes.clear();
        self.ws_head = None;
        self.ws_tail = None;
    }

    pub fn clear_nodes(&mut self) {
        self.free_nodes();
    }

    pub fn dump_nodes(&self) {
        let mut p = self.ws_head;
        let mut n = 0usize;
        while let Some(idx) = p {
            if let Some(node) = self.nodes.get(idx).and_then(|x| x.as_ref()) {
                let txt = self.wsnode_ptr(idx);
                self.wsplt_error(&format!(
                    "({:02}) {:4}: {:04x} ({}):{};",
                    self.ws_lvl,
                    n,
                    node.flags,
                    Self::wsnode_flagstr(node.flags),
                    txt
                ));
                p = node.next;
                n += 1;
            } else {
                break;
            }
        }
    }

    pub fn coalesce_segment(&mut self, node: usize) -> i32 {
        let Some(first) = self.nodes.get(node).and_then(|n| n.as_ref()) else {
            return 0;
        };
        if (first.flags & WSNF_JOIN) == 0 {
            return 0;
        }

        let mut p = Some(node);
        let mut len = 0usize;
        while let Some(idx) = p {
            len += self.wsnode_len(idx);
            let joined = self
                .nodes
                .get(idx)
                .and_then(|n| n.as_ref())
                .map(|n| (n.flags & WSNF_JOIN) != 0)
                .unwrap_or(false);
            if !joined {
                break;
            }
            p = self.nodes.get(idx).and_then(|n| n.as_ref()).and_then(|n| n.next);
        }
        let end = p;

        let mut buf = String::with_capacity(len);
        let mut p = Some(node);
        let mut stop = false;
        while let Some(idx) = p {
            let next = self.nodes.get(idx).and_then(|n| n.as_ref()).and_then(|n| n.next);
            let s = self.wsnode_ptr(idx);
            buf.push_str(&s);
            if idx != node {
                let pflags = self
                    .nodes
                    .get(idx)
                    .and_then(|n| n.as_ref())
                    .map(|n| n.flags)
                    .unwrap_or(0);
                if let Some(node0) = self.nodes.get_mut(node).and_then(|n| n.as_mut()) {
                    node0.flags |= pflags & WSNF_QUOTE;
                }
                stop = Some(idx) == end;
                self.wsnode_remove(idx);
            }
            if stop {
                break;
            }
            p = next;
        }

        if let Some(node0) = self.nodes.get_mut(node).and_then(|n| n.as_mut()) {
            node0.flags &= !WSNF_JOIN;
            node0.flags |= WSNF_WORD;
            node0.kind = NodeKind::Word(buf);
        }
        0
    }

    pub fn wsnode_quoteremoval(&mut self) -> i32 {
        let mut p = self.ws_head;
        while let Some(idx) = p {
            let next = self.nodes.get(idx).and_then(|n| n.as_ref()).and_then(|n| n.next);
            let flags = self
                .nodes
                .get(idx)
                .and_then(|n| n.as_ref())
                .map(|n| n.flags)
                .unwrap_or(0);
            if (flags & WSNF_NOEXPAND) == 0 {
                let strv = self.wsnode_ptr(idx);
                let slen = self.wsnode_len(idx);
                let src = strv;
                let mut newstr = String::new();
                self.string_unquote_copy((flags & WSNF_QUOTE) != 0, &mut newstr, &src, slen);
                if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                    node.kind = NodeKind::Word(newstr);
                    node.flags |= WSNF_WORD;
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
            let flags = self
                .nodes
                .get(idx)
                .and_then(|n| n.as_ref())
                .map(|n| n.flags)
                .unwrap_or(0);
            if (flags & WSNF_JOIN) != 0 && self.coalesce_segment(idx) != 0 {
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
            while let Some(idx) = np {
                let next = self.nodes.get(idx).and_then(|n| n.as_ref()).and_then(|n| n.next);
                if next.is_some() {
                    if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                        node.flags |= WSNF_JOIN;
                    }
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

        'again: loop {
            let mut p = self.ws_head;
            n = 0;

            while let Some(idx) = p {
                let next = self.nodes.get(idx).and_then(|n| n.as_ref()).and_then(|n| n.next);
                let flags = self
                    .nodes
                    .get(idx)
                    .and_then(|n| n.as_ref())
                    .map(|n| n.flags)
                    .unwrap_or(0);

                if (flags & WSNF_DELIM) != 0 {
                    if (self.ws_flags & WRDSF_SQUEEZE_DELIMS) != 0 {
                        if let Some(next_idx) = next {
                            let next_flags = self
                                .nodes
                                .get(next_idx)
                                .and_then(|n| n.as_ref())
                                .map(|n| n.flags)
                                .unwrap_or(0);
                            if (next_flags & WSNF_DELIM) != 0
                                && self.wsnode_ptr(idx).chars().next()
                                    == self.wsnode_ptr(next_idx).chars().next()
                            {
                                self.wsnode_remove(idx);
                                p = Some(next_idx);
                                continue;
                            }
                        } else if (self.ws_flags & WRDSF_INCREMENTAL) != 0 {
                            break;
                        }
                    } else if let Some(next_idx) = next {
                        let next_flags = self
                            .nodes
                            .get(next_idx)
                            .and_then(|n| n.as_ref())
                            .map(|n| n.flags)
                            .unwrap_or(0);
                        if (next_flags & WSNF_DELIM) != 0
                            && (self.ws_options & WRDSO_RETDELNOTEMPTY) == 0
                        {
                            let Ok(nulidx) = self.alloc_node() else {
                                return self.wsplt_nomem();
                            };
                            if let Some(node) = self.nodes.get_mut(nulidx).and_then(|n| n.as_mut()) {
                                node.flags = WSNF_NULL | WSNF_NOEXPAND;
                            }
                            self.wsnode_insert(nulidx, idx, false);
                        }
                    }

                    if (self.ws_options & WRDSO_MAXWORDS) != 0
                        && (self.ws_flags & WRDSF_RETURN_DELIMS) == 0
                    {
                        self.wsnode_remove(idx);
                        p = next;
                        continue;
                    }
                }

                n += 1;

                if (self.ws_options & WRDSO_MAXWORDS) != 0 && (self.ws_wordi + n == self.ws_maxwords)
                {
                    break;
                }

                if (self.ws_flags & WRDSF_INCREMENTAL) != 0 {
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
                if (self.ws_flags & WRDSF_INCREMENTAL) != 0 {
                    if self.ws_endp < self.ws_len {
                        let rc = self.process_list(self.skip_delim());
                        if rc != 0 {
                            return rc;
                        }
                        continue 'again;
                    } else {
                        self.ws_errno = WRDSE_EOF;
                        return WRDSE_EOF;
                    }
                }

                if (self.ws_flags & WRDSF_NOSPLIT) != 0 {
                    if self.add_segm(0, 0, WSNF_EMPTYOK) != 0 {
                        return self.ws_errno;
                    }
                    n = 1;
                }
            }

            if self.alloc_space(n + 1) != 0 {
                return self.ws_errno;
            }

            while let Some(head) = self.ws_head {
                let s = self.wsnode_ptr(head);
                let slot = self.ws_offs + self.ws_wordc;
                if slot >= self.ws_wordv.len() {
                    self.ws_wordv.resize(slot + 1, None);
                }
                self.ws_wordv[slot] = Some(s);
                self.wsnode_remove(head);
                self.ws_wordc += 1;
                self.ws_wordi += 1;
                if (self.ws_flags & WRDSF_INCREMENTAL) != 0 {
                    break;
                }
            }
            let term = self.ws_offs + self.ws_wordc;
            if term >= self.ws_wordv.len() {
                self.ws_wordv.resize(term + 1, None);
            }
            self.ws_wordv[term] = None;
            return 0;
        }
    }

    pub fn append(&mut self, argv: &[String]) -> i32 {
        let rc = self.alloc_space(self.ws_wordc + argv.len() + 1);
        if rc != 0 {
            return rc;
        }
        for (i, arg) in argv.iter().enumerate() {
            let idx = self.ws_offs + self.ws_wordc + i;
            if idx >= self.ws_wordv.len() {
                self.ws_wordv.resize(idx + 1, None);
            }
            self.ws_wordv[idx] = Some(arg.clone());
        }
        self.ws_wordc += argv.len();
        let idx = self.ws_offs + self.ws_wordc;
        if idx >= self.ws_wordv.len() {
            self.ws_wordv.resize(idx + 1, None);
        }
        self.ws_wordv[idx] = None;
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
        let Ok(newidx) = self.alloc_node() else {
            return 1;
        };
        self.wsnode_insert(newidx, *ptail, false);
        let is_word = self
            .nodes
            .get(node)
            .and_then(|n| n.as_ref())
            .map(|n| (n.flags & WSNF_WORD) != 0)
            .unwrap_or(false);

        if is_word {
            let s = self.wsnode_ptr(node);
            let end = (beg + len).min(s.len());
            let sub = s[beg.min(s.len())..end].to_string();
            if let Some(newnode) = self.nodes.get_mut(newidx).and_then(|n| n.as_mut()) {
                newnode.flags = WSNF_WORD | flg;
                newnode.kind = NodeKind::Word(sub);
            }
        } else {
            let base = match self.nodes.get(node).and_then(|n| n.as_ref()).map(|n| &n.kind) {
                Some(NodeKind::Segment { beg, .. }) => *beg
,
                _ => 0,
            };
            if let Some(newnode) = self.nodes.get_mut(newidx).and_then(|n| n.as_mut()) {
                newnode.flags = flg;
                newnode.kind = NodeKind::Segment {
                    beg: base + beg,
                    end: base + beg + len,
                };
            }
        }
        *ptail = newidx;
        0
    }


    pub fn wsplt_env_find(&self, name: &str, len: usize) -> Option<String> {
        let key = &name[..len.min(name.len())];
        self.ws_env.as_ref().and_then(|env| {
            env.iter().find_map(|item| {
                let (k, v) = item.split_once('=')?;
                if k == key {
                    Some(v.to_string())
                } else {
                    None
                }
            })
        })
    }

    pub fn wsplt_env_lookup(&mut self, name: &str, len: usize, ret: &mut Option<String>) -> i32 {
        *ret = self.wsplt_env_find(name, len);
        if ret.is_some() {
            0
        } else {
            WRDSE_UNDEF
        }
    }

    pub fn wsplt_env_getvar(&mut self, name: &str, len: usize, ret: &mut Option<String>) -> i32 {
        self.wsplt_env_lookup(name, len, ret)
    }

    pub fn wsplt_assign_var(&mut self, name: &str, namelen: usize, value: &str) -> i32 {
        let key = &name[..namelen.min(name.len())];
        let entry = format!("{key}={value}");
        let env = self.ws_env.get_or_insert_with(Vec::new);
        if let Some(pos) = env.iter().position(|s| s.starts_with(&format!("{key}="))) {
            env[pos] = entry;
        } else {
            env.push(entry);
        }
        0
    }

    pub fn wsplt_assign_param(&mut self, param_idx: i32, value: &str) -> i32 {
        if param_idx < 0 {
            return self.wsplt_seterr(WRDSE_BADPARAM);
        }
        let idx = param_idx as usize;
        if self.ws_paramv.len() <= idx {
            self.ws_paramv.resize(idx + 1, String::new());
        }
        self.ws_paramv[idx] = value.to_string();
        self.ws_paramc = self.ws_paramv.len();
        0
    }

    pub fn expvar_recover(
        &mut self,
        s: &str,
        ptail: &mut usize,
        pend: &mut usize,
        flg: u32,
    ) -> i32 {
        let start = *pend;
        let rc = self.node_split_prefix(ptail, *ptail, start, s.len().saturating_sub(start), flg);
        if rc == 0 {
            *pend = s.len();
        }
        rc
    }

    pub fn expand_paramv(&mut self, ptail: &mut usize, flg: u32, _q: i32) -> i32 {
        let items = self.ws_paramv.clone();
        for item in items {
            let idx = match self.alloc_node() {
                Ok(v) => v,
                Err(_) => return self.wsplt_nomem(),
            };
            if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                node.flags = WSNF_WORD | flg;
                node.kind = NodeKind::Word(item);
            }
            self.wsnode_insert(idx, *ptail, false);
            *ptail = idx;
        }
        0
    }

    pub fn expvar(
        &mut self,
        s: &str,
        len: usize,
        ptail: &mut usize,
        pend: &mut usize,
        flg: u32,
    ) -> i32 {
        let bytes = s.as_bytes();
        if bytes.is_empty() || bytes[0] != b'$' {
            return self.wsplt_seterr(WRDSE_BADPARAM);
        }
        if len < 2 {
            *pend = len;
            return self.wsplt_seterr(WRDSE_BADPARAM);
        }

        if bytes[1] == b'{' {
            if let Some(end) = crate::parser::find_closing_paren(s, 1, len, "{}") {
                let name = &s[2..end];
                let mut val = None;
                let rc = self.wsplt_env_getvar(name, name.len(), &mut val);
                if rc == 0 {
                    let idx = match self.alloc_node() {
                        Ok(v) => v,
                        Err(_) => return self.wsplt_nomem(),
                    };
                    if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                        node.flags = WSNF_WORD | flg;
                        node.kind = NodeKind::Word(val.unwrap_or_default());
                    }
                    self.wsnode_insert(idx, *ptail, false);
                    *ptail = idx;
                    *pend = end + 1;
                    return 0;
                }
                *pend = end + 1;
                return rc;
            }
            return self.wsplt_seterr(WRDSE_CBRACE);
        }

        let mut end = 1usize;
        while end < len && end < s.len() && self.is_name_char(s.as_bytes()[end] as char) {
            end += 1;
        }
        if end == 1 && end < s.len() && s.as_bytes()[end].is_ascii_digit() {
            end += 1;
        }
        let name = &s[1..end.min(s.len())];
        let mut val = None;
        let rc = self.wsplt_env_getvar(name, name.len(), &mut val);
        if rc == 0 {
            let idx = match self.alloc_node() {
                Ok(v) => v,
                Err(_) => return self.wsplt_nomem(),
            };
            if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                node.flags = WSNF_WORD | flg;
                node.kind = NodeKind::Word(val.unwrap_or_default());
            }
            self.wsnode_insert(idx, *ptail, false);
            *ptail = idx;
        }
        *pend = end;
        rc
    }

    pub fn begin_var_p(c: char) -> bool {
        "{#@*".contains(c) || c.is_ascii_alphabetic() || c == '_' || c.is_ascii_digit()
    }

    pub fn node_expand(
        &mut self,
        node: usize,
        beg_p: fn(char) -> bool,
        ws_exp_fn: fn(&mut Wordsplit, &str, usize, &mut usize, &mut usize, u32) -> i32,
    ) -> i32 {
        let text = self.wsnode_ptr(node);
        let mut out_head: Option<usize> = None;
        let mut out_tail: Option<usize> = None;
        let mut i = 0usize;

        while i < text.len() {
            let ch = text.as_bytes()[i] as char;
            if beg_p(ch) {
                let mut tmp_tail = out_tail.unwrap_or(node);
                let mut pend = i;
                let rc = ws_exp_fn(self, &text[i - 1..], text.len() - (i - 1), &mut tmp_tail, &mut pend, 0);
                if rc == 0 {
                    if out_head.is_none() {
                        out_head = Some(tmp_tail);
                    }
                    out_tail = Some(tmp_tail);
                    i += pend.saturating_sub(1);
                    continue;
                }
            }
            let idx = match self.alloc_node() {
                Ok(v) => v,
                Err(_) => return self.wsplt_nomem(),
            };
            if let Some(n) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                n.flags = WSNF_WORD;
                n.kind = NodeKind::Word(ch.to_string());
            }
            if let Some(tail) = out_tail {
                self.wsnode_insert(idx, tail, false);
            } else {
                self.wsnode_insert(idx, node, true);
                out_head = Some(idx);
            }
            out_tail = Some(idx);
            i += 1;
        }

        if out_head.is_some() {
            self.wsnode_remove(node);
        }
        0
    }

    pub fn wsnode_nullelim(&mut self) {
        let mut p = self.ws_head;
        while let Some(idx) = p {
            let next = self.nodes.get(idx).and_then(|n| n.as_ref()).and_then(|n| n.next);
            let remove = self
                .nodes
                .get(idx)
                .and_then(|n| n.as_ref())
                .map(|n| (n.flags & WSNF_NULL) != 0)
                .unwrap_or(false);
            if remove {
                self.wsnode_remove(idx);
            }
            p = next;
        }
    }

    pub fn varexp(&mut self) -> i32 {
        let mut p = self.ws_head;
        while let Some(idx) = p {
            let next = self.nodes.get(idx).and_then(|n| n.as_ref()).and_then(|n| n.next);
            let flags = self
                .nodes
                .get(idx)
                .and_then(|n| n.as_ref())
                .map(|n| n.flags)
                .unwrap_or(0);
            if (flags & WSNF_NOEXPAND) == 0 {
                let s = self.wsnode_ptr(idx);
                if s.contains('$') {
                    let rc = self.node_expand(idx, Self::begin_var_p, Wordsplit::expvar);
                    if rc != 0 {
                        return rc;
                    }
                }
            }
            p = next;
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
        ptail: &mut usize,
        pend: &mut usize,
        flg: u32,
    ) -> i32 {
        if !s.starts_with("$(") {
            return self.wsplt_seterr(WRDSE_PAREN);
        }
        let Some(end) = crate::parser::find_closing_paren(s, 1, len, "()") else {
            return self.wsplt_seterr(WRDSE_PAREN);
        };
        let body = &s[2..end];
        let idx = match self.alloc_node() {
            Ok(v) => v,
            Err(_) => return self.wsplt_nomem(),
        };
        if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
            node.flags = WSNF_WORD | flg;
            node.kind = NodeKind::Word(body.trim().to_string());
        }
        self.wsnode_insert(idx, *ptail, false);
        *ptail = idx;
        *pend = end + 1;
        0
    }

    pub fn cmdexp(&mut self) -> i32 {
        let mut p = self.ws_head;
        while let Some(idx) = p {
            let next = self.nodes.get(idx).and_then(|n| n.as_ref()).and_then(|n| n.next);
            let flags = self
                .nodes
                .get(idx)
                .and_then(|n| n.as_ref())
                .map(|n| n.flags)
                .unwrap_or(0);
            if (flags & WSNF_NOEXPAND) == 0 {
                let s = self.wsnode_ptr(idx);
                if s.contains("$(") {
                    let rc = self.node_expand(idx, Self::begin_cmd_p, Wordsplit::expcmd);
                    if rc != 0 {
                        return rc;
                    }
                }
            }
            p = next;
        }
        self.wsnode_nullelim();
        0
    }

    pub fn trimws(&mut self) -> i32 {
        let mut p = self.ws_head;
        while let Some(idx) = p {
            let next = self.nodes.get(idx).and_then(|n| n.as_ref()).and_then(|n| n.next);
            let s = self.wsnode_ptr(idx).trim().to_string();
            if s.is_empty() {
                self.wsnode_remove(idx);
            } else if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                node.kind = NodeKind::Word(s);
                node.flags |= WSNF_WORD;
            }
            p = next;
        }
        self.wsnode_nullelim();
        0
    }

    pub fn tildexpand(&mut self) -> i32 {
        let home = std::env::var("HOME").unwrap_or_default();
        let mut p = self.ws_head;
        while let Some(idx) = p {
            let next = self.nodes.get(idx).and_then(|n| n.as_ref()).and_then(|n| n.next);
            let s = self.wsnode_ptr(idx);
            if let Some(rest) = s.strip_prefix('~') {
                if let Some(node) = self.nodes.get_mut(idx).and_then(|n| n.as_mut()) {
                    node.kind = NodeKind::Word(format!("{home}{rest}"));
                    node.flags |= WSNF_WORD;
                }
            }
            p = next;
        }
        0
    }

    pub fn is_glob(s: &str, l: i32) -> bool {
        let end = if l < 0 { s.len() } else { (l as usize).min(s.len()) };
        let bytes = &s.as_bytes()[..end];
        let mut i = 0usize;
        while i < bytes.len() {
            match bytes[i] {
                b'\\' => i += 2,
                b'*' | b'?' | b'[' => return true,
                _ => i += 1,
            }
        }
        false
    }

    pub fn pathexpand(&mut self) -> i32 {
        if (self.ws_options & WRDSO_NULLGLOB) == 0 && (self.ws_options & WRDSO_FAILGLOB) == 0 {
            return 0;
        }
        let mut p = self.ws_head;
        while let Some(idx) = p {
            let next = self.nodes.get(idx).and_then(|n| n.as_ref()).and_then(|n| n.next);
            let s = self.wsnode_ptr(idx);
            if Self::is_glob(&s, s.len() as i32) {
                if (self.ws_options & WRDSO_FAILGLOB) != 0 {
                    return self.wsplt_setctxerr(WRDSE_GLOBERR, &s, s.len());
                }
                if (self.ws_options & WRDSO_NULLGLOB) != 0 {
                    self.wsnode_remove(idx);
                }
            }
            p = next;
        }
        0
    }

    pub fn skip_sed_expr(command: &str, i: usize, len: usize) -> i32 {
        let bytes = command.as_bytes();
        if i + 2 >= len || i + 2 >= bytes.len() {
            return i as i32;
        }
        let delim = bytes[i + 1];
        let mut pos = i + 2;
        let mut parts = 0usize;
        while pos < len && pos < bytes.len() {
            match bytes[pos] {
                b'\\' => pos += 2,
                c if c == delim => {
                    parts += 1;
                    pos += 1;
                    if parts >= 2 {
                        break;
                    }
                }
                _ => pos += 1,
            }
        }
        pos as i32
    }

    pub fn skip_delim_internal(&self, return_delims: bool) -> usize {
        if return_delims {
            self.ws_endp
        } else {
            self.ws_endp + 1
        }
    }

    pub fn skip_delim(&self) -> usize {
        self.skip_delim_internal((self.ws_flags & WRDSF_RETURN_DELIMS) != 0)
    }

    pub fn skip_delim_real(&self) -> usize {
        self.skip_delim_internal((self.ws_flags & WRDSF_RETURN_DELIMS) != 0)
    }

    pub fn scan_qstring(&self, start: usize, end: &mut usize) -> Option<usize> {
        let bytes = self.ws_input.as_bytes();
        let quote = *bytes.get(start)?;
        let mut i = start + 1;
        while i < self.ws_len && i < bytes.len() {
            if bytes[i] == b'\\' && quote == b'"' {
                i += 2;
                continue;
            }
            if bytes[i] == quote {
                *end = i;
                return Some(i);
            }
            i += 1;
        }
        None
    }

    pub fn scan_word(&mut self, start: usize, consume_all: i32) -> i32 {
        let bytes = self.ws_input.as_bytes();
        let mut i = start;
        let mut quote = false;
        while i < self.ws_len && i < bytes.len() {
            let c = bytes[i];
            if (self.ws_flags & WRDSF_COMMENT) != 0
                && self
                    .ws_comment
                    .as_deref()
                    .map(|s| s.as_bytes().contains(&c))
                    .unwrap_or(false)
            {
                break;
            }
            if self.ws_delim.as_bytes().contains(&c) && consume_all == 0 {
                break;
            }
            if c == b'\'' || c == b'"' {
                let mut end = i;
                if self.scan_qstring(i, &mut end).is_none() {
                    return self.wsplt_seterr(WRDSE_QUOTE);
                }
                quote = true;
                i = end + 1;
                continue;
            }
            if c == b'\\' {
                i += 2;
                continue;
            }
            if (self.ws_flags & WRDSF_SED_EXPR) != 0 && c == b's' && i + 1 < self.ws_len {
                let n = Self::skip_sed_expr(&self.ws_input, i, self.ws_len);
                if n as usize > i {
                    i = n as usize;
                    continue;
                }
            }
            i += 1;
        }
        self.ws_endp = i;
        let mut flg = 0u32;
        if quote {
            flg |= WSNF_QUOTE;
        }
        if i == start && i < self.ws_len && self.ws_delim.as_bytes().contains(&bytes[i]) {
            flg |= WSNF_DELIM;
            self.add_segm(i, i + 1, flg)
        } else {
            self.add_segm(start, i, flg)
        }
    }

    pub fn parse_num(pval: &mut i32, src: &str, base: i32, cnt: i32) -> i32 {
        if cnt <= 0 || !(2..=16).contains(&base) {
            return 1;
        }
        let mut value: i32 = 0;
        let mut seen = 0;
        for ch in src.chars().take(cnt as usize) {
            let Some(d) = ch.to_digit(base as u32) else {
                break;
            };
            value = value.saturating_mul(base).saturating_add(d as i32);
            seen += 1;
        }
        if seen == 0 {
            1
        } else {
            *pval = value;
            0
        }
    }

    pub fn c_quoted_length(strv: &str, quote_hex: i32, quote: &mut i32) -> usize {
        let mut len = 0usize;
        *quote = 0;
        for b in strv.bytes() {
            let q = Self::c_quote_char(b as i32);
            if q != b as i32 {
                *quote = 1;
                if q == b'x' as i32 && quote_hex != 0 {
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

    pub fn wsplt_unquote_char(transtab: &str, c: i32) -> i32 {
        let bytes = transtab.as_bytes();
        let mut i = 0usize;
        while i + 1 < bytes.len() {
            if bytes[i] as i32 == c {
                return bytes[i + 1] as i32;
            }
            i += 2;
        }
        c
    }

    pub fn wsplt_quote_char(transtab: &str, c: i32) -> i32 {
        let bytes = transtab.as_bytes();
        let mut i = 0usize;
        while i + 1 < bytes.len() {
            if bytes[i + 1] as i32 == c {
                return bytes[i] as i32;
            }
            i += 2;
        }
        c
    }

    pub fn c_unquote_char(c: i32) -> i32 {
        Self::wsplt_unquote_char("\\\\\"\"ttnnrr", c)
    }

    pub fn c_quote_char(c: i32) -> i32 {
        Self::wsplt_quote_char("\\\\\"\"ttnnrr", c)
    }

    pub fn string_unquote_copy(
        &self,
        inquote: bool,
        dst: &mut String,
        src: &str,
        n: usize,
    ) {
        dst.clear();
        let bytes = src.as_bytes();
        let mut i = 0usize;
        while i < n && i < bytes.len() {
            if bytes[i] == b'\\' && i + 1 < n && i + 1 < bytes.len() {
                let c = if inquote {
                    Self::c_unquote_char(bytes[i + 1] as i32)
                } else {
                    Self::wsplt_unquote_char(&self.ws_escape[WRDSX_WORD], bytes[i + 1] as i32)
                };
                dst.push(char::from_u32(c as u32).unwrap_or('\0'));
                i += 2;
            } else {
                dst.push(bytes[i] as char);
                i += 1;
            }
        }
    }

    pub fn c_quote_copy(dst: &mut String, src: &str, quote_hex: i32) {
        dst.clear();
        for b in src.bytes() {
            let q = Self::c_quote_char(b as i32);
            if q != b as i32 {
                dst.push('\\');
                if q == b'x' as i32 && quote_hex != 0 {
                    dst.push('x');
                    dst.push_str(&format!("{:02x}", b));
                } else {
                    dst.push(char::from_u32(q as u32).unwrap_or('\\'));
                }
            } else {
                dst.push(b as char);
            }
        }
    }

    pub fn exptab_matches(p: &str, wsp: &Wordsplit) -> bool {
        match p {
            "novar" => (wsp.ws_flags & WRDSF_NOVAR) == 0,
            "nocmd" => (wsp.ws_flags & WRDSF_NOCMD) == 0,
            "tilde" => true,
            "path" => true,
            "trimws" => true,
            "coalesce" => true,
            _ => true,
        }
    }

    pub fn process_list(&mut self, start: usize) -> i32 {
        let input_len = self.ws_input.len();
        let mut i = start;
        while i < self.ws_len && i < input_len {
            let is_delim = self
                .ws_input
                .as_bytes()
                .get(i)
                .copied()
                .map(|b| self.ws_delim.as_bytes().contains(&b))
                .unwrap_or(false);
            if is_delim {
                self.ws_endp = i;
                let rc = self.add_segm(i, i + 1, WSNF_DELIM);
                if rc != 0 {
                    return rc;
                }
                i += 1;
                continue;
            }
            let rc = self.scan_word(i, 0);
            if rc != 0 {
                return rc;
            }
            i = self.ws_endp;
            if i == start {
                break;
            }
            let is_delim = i < self.ws_len
                && self
                    .ws_input
                    .as_bytes()
                    .get(i)
                    .copied()
                    .map(|b| self.ws_delim.as_bytes().contains(&b))
                    .unwrap_or(false);
            if is_delim {
                self.ws_endp = i;
                let rc = self.add_segm(i, i + 1, WSNF_DELIM);
                if rc != 0 {
                    return rc;
                }
                i += 1;
            }
        }

        let exptab = [
            ExpTab {
                descr: "var",
                flag: WRDSF_NOVAR,
                opt: 0,
                expansion: Some(Wordsplit::varexp),
            },
            ExpTab {
                descr: "cmd",
                flag: WRDSF_NOCMD,
                opt: 0,
                expansion: Some(Wordsplit::cmdexp),
            },
            ExpTab {
                descr: "tilde",
                flag: 0,
                opt: 0,
                expansion: Some(Wordsplit::tildexpand),
            },
            ExpTab {
                descr: "path",
                flag: 0,
                opt: 0,
                expansion: Some(Wordsplit::pathexpand),
            },
            ExpTab {
                descr: "trimws",
                flag: 0,
                opt: 0,
                expansion: Some(Wordsplit::trimws),
            },
        ];

        for exp in exptab {
            let _ = exp.descr;
            if (self.ws_flags & exp.flag) == 0
                && (self.ws_options & exp.opt) == exp.opt
                && exp.expansion.is_some()
            {
                let rc = (exp.expansion.unwrap())(self);
                if rc != 0 {
                    return rc;
                }
            }
        }

        if self.wsnode_quoteremoval() != 0 {
            return self.ws_errno;
        }
        self.wsnode_coalesce()
    }

    pub fn run(&mut self, command: &str, length: usize, flags: u32, lvl: i32) -> i32 {
        let rc = self.new(command, length, flags);
        if rc != 0 {
            return rc;
        }
        self.ws_lvl = lvl;
        if self.ws_len == 0 {
            return self.wsplt_seterr(WRDSE_NOINPUT);
        }
        let rc = self.process_list(0);
        if rc != 0 {
            return rc;
        }
        self.finish()
    }

    pub fn len(&mut self, command: &str, length: usize, flags: u32) -> i32 {
        self.run(command, length, flags, 0)
    }

    pub fn wordsplit(&mut self, command: &str, flags: u32) -> i32 {
        self.len(command, command.len(), flags)
    }

    pub fn free_words(&mut self) {
        self.ws_wordv.clear();
        self.ws_wordc = 0;
        self.ws_wordn = 0;
    }

    pub fn free_envbuf(&mut self) {
        self.ws_envbuf = None;
        self.ws_envidx = 0;
        self.ws_envsiz = 0;
    }

    pub fn free_parambuf(&mut self) {
        self.ws_parambuf = None;
        self.ws_paramidx = 0;
        self.ws_paramsiz = 0;
    }

    pub fn clearerr(&mut self) {
        self.ws_errno = WRDSE_OK;
        self.ws_errctx = None;
        self.ws_usererr = None;
    }

    pub fn get_words(&self) -> Vec<String> {
        self.ws_wordv
            .iter()
            .filter_map(|s| s.clone())
            .collect()
    }

    pub fn strerror(&self) -> &'static str {
        match self.ws_errno {
            WRDSE_OK => "no error",
            WRDSE_EOF => "unexpected end of input",
            WRDSE_USAGE => "invalid wordsplit usage",
            WRDSE_NOSPACE => "memory exhausted",
            WRDSE_UNDEF => "undefined variable",
            WRDSE_BADPARAM => "invalid parameter reference",
            WRDSE_QUOTE => "unterminated quote",
            WRDSE_CBRACE => "missing closing brace",
            WRDSE_PAREN => "missing closing parenthesis",
            WRDSE_GLOBERR => "glob expansion failed",
            WRDSE_USERERR => "user-defined error",
            WRDSE_NOINPUT => "no input",
            _ => "wordsplit error",
        }
    }

    pub fn perror(&self) {
        let mut msg = self.strerror().to_string();
        if let Some(ctx) = &self.ws_errctx {
            msg.push_str(": ");
            msg.push_str(ctx);
        }
        if let Some(user) = &self.ws_usererr {
            msg.push_str(": ");
            msg.push_str(user);
        }
        self.wsplt_error(&msg);
    }

}
impl Drop for Wordsplit {
    fn drop(&mut self) {
        self.clearerr();
        self.free_words();
        self.free_envbuf();
        self.free_parambuf();
        self.free_nodes();
    }
}
