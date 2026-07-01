use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseMessagePriority {
    Error,
    Info,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseOutcome {
    Next,
    End,
    Error,
    Argument,
    Code(i32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NegMatch {
    NotTried,
    NoMatch,
    Inexact,
    Exact,
}

#[derive(Clone, Debug, Default)]
pub struct ParseoptOption {
    pub name: String,
    pub code: i32,
    pub flags: u32,
    pub arg_doc: Option<String>,
    pub help: Option<String>,
    pub usage: Option<String>,
    pub group: Option<String>,
    pub canonical: Option<usize>,
    pub aliases: Vec<usize>,
    pub set_count: usize,
    pub takes_non_option: bool,
}

const FLAG_ALIAS: u32 = 1 << 0;
const FLAG_BOOL: u32 = 1 << 1;
const FLAG_ARG_OPTIONAL: u32 = 1 << 2;
const FLAG_EARLY: u32 = 1 << 3;
const FLAG_IS_SET: u32 = 1 << 4;
const FLAG_HIDDEN: u32 = 1 << 5;
const FLAG_SUBLIST: u32 = 1 << 6;

const FLAG_SINGLE_DASH: u32 = 1 << 16;
const FLAG_IN_ORDER: u32 = 1 << 17;
const FLAG_EARLY_PASS: u32 = 1 << 18;
const FLAG_NONOPT_ARG: u32 = 1 << 19;
const FLAG_INITIALIZED: u32 = 1 << 20;
const FLAG_NO_ERREXIT: u32 = 1 << 21;
const FLAG_IGNORE_ERRORS: u32 = 1 << 22;
const FLAG_ARGV0: u32 = 1 << 23;
const FLAG_NO_STDOPT: u32 = 1 << 24;

const DEFAULT_EX_USAGE: i32 = 64;

pub struct Parseopt {
    flags: u32,
    ex_usage: i32,
    argc: usize,
    argv: Vec<String>,
    argi: usize,
    arg_start: usize,
    arg_count: usize,
    permuted: bool,
    eopt: bool,
    dash_count: usize,
    optname: Option<String>,
    opt_buf: String,
    program_name: Option<String>,
    negation: Option<String>,
    options: Vec<ParseoptOption>,
    optidx: Vec<usize>,
    groups: Vec<Vec<ParseoptOption>>,
    arg_set: Option<usize>,
    version_enabled: bool,
    help_enabled: bool,
    usage_enabled: bool,
    errors: Vec<String>,
    pending_value: Option<String>,
    setopt: Option<Box<dyn FnMut(i32, Option<&str>)>>,
    init_hook: Option<Box<dyn FnMut(&mut Parseopt)>>,
}

impl Parseopt {
    pub fn init_0() -> Self {
        Self {
            flags: 0,
            ex_usage: 0,
            argc: 0,
            argv: Vec::new(),
            argi: 0,
            arg_start: 0,
            arg_count: 0,
            permuted: false,
            eopt: false,
            dash_count: 0,
            optname: None,
            opt_buf: String::new(),
            program_name: None,
            negation: None,
            options: Vec::new(),
            optidx: Vec::new(),
            groups: Vec::new(),
            arg_set: None,
            version_enabled: false,
            help_enabled: false,
            usage_enabled: false,
            errors: Vec::new(),
            pending_value: None,
            setopt: None,
            init_hook: None,
        }
    }

    pub fn new() -> Self {
        Self::init_0()
    }

    pub fn option_dash(&self, opt: &ParseoptOption) -> &'static str {
        if (self.flags & FLAG_SINGLE_DASH) != 0 || opt.name.chars().count() == 1 {
            "-"
        } else {
            "--"
        }
    }

    pub fn option_find_short(
        &mut self,
        orig_opt: &mut Option<usize>,
        argptr: &mut Option<String>,
        exparg: &mut bool,
    ) -> Option<usize> {
        let current = self.optname.clone().unwrap_or_default();
        let ch = current.chars().next()?;
        for sorted in 0..self.optidx.len() {
            let idx = self.optidx[sorted];
            let opt = &self.options[idx];
            if opt.name.chars().count() == 1 && opt.name.starts_with(ch) {
                *orig_opt = Some(idx);
                let canonical = self.canonical_index(idx);
                if self.options[canonical].arg_doc.is_some() && current.chars().nth(1).is_some() {
                    *exparg = true;
                    let rest = current.chars().skip(1).collect::<String>();
                    *argptr = Some(rest);
                    self.opt_buf.clear();
                    self.opt_buf.push(ch);
                    self.optname = Some(self.opt_buf.clone());
                } else {
                    *exparg = false;
                    if (self.options[canonical].flags & FLAG_BOOL) != 0 {
                        *argptr = Some(String::from("1"));
                    } else {
                        *argptr = None;
                    }
                }
                return Some(canonical);
            }
        }
        self.error(
            ParseMessagePriority::Error,
            format_args!("unrecognized option '-{}'", ch),
        );
        None
    }

    pub fn negmatch(
        &self,
        opt: &ParseoptOption,
        optstr: &str,
        optlen: usize,
    ) -> NegMatch {
        if (opt.flags & FLAG_BOOL) != 0 {
            let neg = self.negation.as_deref().unwrap_or("");
            let neglen = neg.len();
            let len = opt.name.len();
            if optlen <= neglen + len
                && optstr.get(..neglen) == Some(neg)
                && optstr.get(neglen..optlen) == Some(&opt.name[..optlen.saturating_sub(neglen)])
            {
                if optlen == neglen + len {
                    return NegMatch::Exact;
                }
                return NegMatch::Inexact;
            }
        }
        NegMatch::NoMatch
    }

    pub fn option_find_long(
        &mut self,
        orig_opt: &mut Option<usize>,
        argptr: &mut Option<String>,
        exparg: &mut bool,
    ) -> Option<usize> {
        let optname = self.optname.clone().unwrap_or_default();
        let optlen = optname.find('=').unwrap_or(optname.len());
        let mut found = 0usize;
        let mut found_opt: Option<usize> = None;
        let mut negated = false;

        for sorted in 0..self.optidx.len() {
            let idx = self.optidx[sorted];
            let opt = &self.options[idx];
            let opt_name = opt.name.clone();
            let namelen = opt_name.len();
            let mut neg = NegMatch::NotTried;

            if namelen == 1 && (self.flags & FLAG_SINGLE_DASH) == 0 {
                continue;
            }

            let prefix_match = optlen <= namelen && opt_name.get(..optlen) == optname.get(..optlen);
            if !prefix_match {
                neg = self.negmatch(opt, &optname, optlen);
            }

            if prefix_match || neg != NegMatch::NoMatch {
                match found {
                    0 => {
                        found_opt = Some(idx);
                        found = 1;
                        negated = matches!(neg, NegMatch::Inexact | NegMatch::Exact);
                        if optlen == namelen || neg == NegMatch::Exact {
                            break;
                        }
                    }
                    1 => {
                        found = 2;
                        let dash = if self.dash_count == 1 { "-" } else { "--" };
                        let negation = self.negation.clone().unwrap_or_default();
                        self.error(
                            ParseMessagePriority::Error,
                            format_args!(
                                "option '{}{}' is ambiguous; possibilities:",
                                dash,
                                &optname[..optlen]
                            ),
                        );
                        if let Some(prev) = found_opt {
                            let prev_name = self.options[prev].name.clone();
                            self.error(
                                ParseMessagePriority::Info,
                                format_args!(
                                    "{}{}{}",
                                    dash,
                                    if negated { negation.as_str() } else { "" },
                                    prev_name
                                ),
                            );
                            if !negated
                                && self.negmatch(&self.options[prev], &optname, optlen)
                                    != NegMatch::NoMatch
                            {
                                self.error(
                                    ParseMessagePriority::Info,
                                    format_args!("{}{}{}", dash, negation.as_str(), prev_name),
                                );
                            }
                        }
                        self.error(
                            ParseMessagePriority::Info,
                            format_args!(
                                "{}{}{}",
                                dash,
                                if matches!(neg, NegMatch::Inexact | NegMatch::Exact) {
                                    negation.as_str()
                                } else {
                                    ""
                                },
                                opt_name
                            ),
                        );
                    }
                    _ => {
                        let dash = if self.dash_count == 1 { "-" } else { "--" };
                        let negation = self.negation.clone().unwrap_or_default();
                        self.error(
                            ParseMessagePriority::Info,
                            format_args!(
                                "{}{}{}",
                                dash,
                                if matches!(neg, NegMatch::Inexact | NegMatch::Exact) {
                                    negation.as_str()
                                } else {
                                    ""
                                },
                                opt_name
                            ),
                        );
                    }
                }
            }
        }

        match found {
            0 => {
                self.error(
                    ParseMessagePriority::Error,
                    format_args!(
                        "unrecognized option '{}{}'",
                        if self.dash_count == 1 { "-" } else { "--" },
                        &optname[..optlen]
                    ),
                );
                None
            }
            1 => {
                let idx = found_opt?;
                *orig_opt = Some(idx);
                let canonical = self.canonical_index(idx);
                if optname.as_bytes().get(optlen) == Some(&b'=') {
                    *argptr = Some(optname[optlen + 1..].to_string());
                    *exparg = true;
                } else {
                    *exparg = false;
                    if (self.options[canonical].flags & FLAG_BOOL) != 0 {
                        *argptr = Some(if negated { "0" } else { "1" }.to_string());
                    } else {
                        *argptr = None;
                    }
                }
                Some(canonical)
            }
            _ => None,
        }
    }

    pub fn permute(&mut self) {
        if (self.flags & FLAG_IN_ORDER) == 0 && self.arg_count != 0 {
            let n = self.argi.saturating_sub(self.arg_start + self.arg_count);
            assert!(n <= 2);
            let shift = n;
            if shift != 0 {
                let start = self.arg_start;
                let end = self.arg_start + self.arg_count + shift;
                self.argv[start..end].rotate_right(shift);
            }

            self.arg_start += n;
            self.permuted = true;

            self.arg_start += n;
            self.permuted = true;
        }
    }

    pub fn lookahead(&self) -> Option<&str> {
        if (self.flags & FLAG_SINGLE_DASH) == 0
            && self.dash_count == 1
            && self.optname.as_deref().is_some()
            && self
                .optname
                .as_deref()
                .map(|s| s.chars().nth(1).is_some())
                .unwrap_or(false)
        {
            self.optname.as_deref()
        } else {
            self.argv.get(self.argi).map(String::as_str)
        }
    }

    pub fn skip(&mut self) {
        if self.argi == self.argc || self.eopt {
            return;
        }
        self.argi += 1;
    }

    pub fn next_internal(&mut self) -> ParseOutcome {
        let mut ret_arg: Option<String> = None;

        if (self.flags & FLAG_SINGLE_DASH) == 0
            && self.dash_count == 1
            && self.optname.as_deref().is_some()
            && self
                .optname
                .as_deref()
                .map(|s| s.chars().nth(1).is_some())
                .unwrap_or(false)
        {
            if let Some(name) = self.optname.clone() {
                self.optname = Some(name.chars().skip(1).collect());
            }
        } else {
            self.permute();

            loop {
                self.dash_count = 0;

                if self.argi == self.argc || self.eopt {
                    break;
                }

                let arg = self.argv[self.argi].clone();
                self.argi += 1;

                if let Some(stripped) = arg.strip_prefix('-') {
                    let mut rest = stripped.to_string();
                    self.dash_count += 1;
                    if let Some(double) = rest.strip_prefix('-') {
                        rest = double.to_string();
                        self.dash_count += 1;
                        if rest.is_empty() {
                            self.dash_count = 0;
                            self.permute();
                            self.eopt = true;
                            break;
                        }
                        if (self.flags & FLAG_SINGLE_DASH) != 0 {
                            rest.insert(0, '-');
                            self.dash_count -= 1;
                        }
                    }
                    self.optname = Some(rest);
                    break;
                } else if (self.flags & FLAG_IN_ORDER) != 0 {
                    self.argi -= 1;
                    break;
                } else {
                    if !self.permuted && self.arg_count == 0 {
                        self.arg_start = self.argi - 1;
                    }
                    self.arg_count += 1;
                }
            }
        }

        if self.dash_count != 0 {
            let mut orig_opt = None;
            let mut opt_arg = None;
            let mut has_arg = false;

            let opt = if self.dash_count == 2 || (self.flags & FLAG_SINGLE_DASH) != 0 {
                self.option_find_long(&mut orig_opt, &mut opt_arg, &mut has_arg)
            } else {
                self.option_find_short(&mut orig_opt, &mut opt_arg, &mut has_arg)
            };

            let Some(opt_idx) = opt else {
                return if (self.flags & FLAG_NO_ERREXIT) != 0 {
                    ParseOutcome::Error
                } else {
                    ParseOutcome::Error
                };
            };

            let orig_idx = orig_opt.unwrap_or(opt_idx);
            let dash = if self.dash_count == 1 { "-" } else { "--" };
            let orig_name = self.options[orig_idx].name.clone();

            if self.options[opt_idx].arg_doc.is_some() {
                if opt_arg.is_none() {
                    if (self.options[opt_idx].flags & FLAG_ARG_OPTIONAL) != 0 {
                        opt_arg = None;
                    } else if self.argi == self.argc {
                        self.error(
                            ParseMessagePriority::Error,
                            format_args!("option '{}{}' requires argument", dash, orig_name),
                        );
                        return ParseOutcome::Error;
                    } else {
                        opt_arg = self.argv.get(self.argi).cloned();
                        self.argi += 1;
                    }
                }
                if self.setopt.is_none() {
                    ret_arg = opt_arg.clone();
                }
            } else if has_arg {
                self.error(
                    ParseMessagePriority::Error,
                    format_args!("option '{}{}' does not take argument", dash, orig_name),
                );
                return ParseOutcome::Error;
            }

            let early_session = (self.flags & FLAG_EARLY_PASS) != 0;
            let early_opt = (self.options[opt_idx].flags & FLAG_EARLY) != 0;
            if early_session ^ early_opt {
                return ParseOutcome::Next;
            }

            self.options[opt_idx].flags |= FLAG_IS_SET;
            self.options[opt_idx].set_count += 1;

            if let Some(cb) = self.setopt.as_mut() {
                let code = self.options[opt_idx].code;
                cb(code, opt_arg.as_deref());
                return ParseOutcome::Next;
            }

            self.pending_value = opt_arg.clone();
            return ParseOutcome::Code(self.options[opt_idx].code);
        }

        if !self.permuted {
            self.arg_start = self.argi.saturating_sub(self.arg_count);
        }

        let opt_arg = self.argv.get(self.arg_start).cloned();

        if opt_arg.is_some() && (self.flags & FLAG_NONOPT_ARG) != 0 {
            self.argi += 1;
            if (self.flags & FLAG_EARLY_PASS) != 0 {
                return ParseOutcome::Next;
            }
            self.pending_value = opt_arg.clone();
            return ParseOutcome::Argument;
        }

        self.pending_value = opt_arg.clone();
        ret_arg = opt_arg;
        let _ = ret_arg;
        ParseOutcome::End
    }

    pub fn next(&mut self) -> ParseOutcome {
        loop {
            let rc = self.next_internal();
            if rc != ParseOutcome::Next {
                return rc;
            }
        }
    }

    pub fn argv(&self) -> (&[String], usize) {
        let start = self.arg_start.min(self.argv.len());
        (&self.argv[start..], self.argc.saturating_sub(self.arg_start))
    }

    pub fn error(&mut self, pri: ParseMessagePriority, args: fmt::Arguments<'_>) {
        if (self.flags & FLAG_IGNORE_ERRORS) != 0 {
            return;
        }

        let mut msg = String::new();
        if self.program_name.is_some() && pri == ParseMessagePriority::Error {
            msg.push_str(self.program_name.as_deref().unwrap_or_default());
            msg.push_str(": ");
        }
        msg.push_str(&args.to_string());
        self.errors.push(msg.clone());
        eprintln!("{msg}");
    }

    pub fn find_short_name(&self, start: usize) -> i32 {
        let mut idx = start;
        loop {
            if let Some(opt) = self.options.get(idx) {
                if opt.name.chars().count() == 1 {
                    return opt.name.chars().next().map(|c| c as i32).unwrap_or(0);
                }
                idx += 1;
                if idx >= self.options.len() || (self.options[idx].flags & FLAG_ALIAS) == 0 {
                    break;
                }
            } else {
                break;
            }
        }
        0
    }

    pub fn optidx_slot(&self, n: usize, opt: &ParseoptOption) -> usize {
        if n == 0 {
            return 0;
        }
        let mut i = 0usize;
        let mut j = n;
        while i < j {
            let m = i + (j - i) / 2;
            let cmp_name = &self.options[self.optidx[m]].name;
            if cmp_name.cmp(&opt.name) != Ordering::Greater {
                i = m + 1;
            } else {
                j = m;
            }
        }
        i
    }

    pub fn collect_optdef(&mut self, group_index: usize, mut n: usize) -> usize {
        if let Some(group) = self.groups.get(group_index) {
            for opt in group {
                let global_index = self
                    .options
                    .iter()
                    .position(|o| o.name == opt.name && o.code == opt.code && o.flags == opt.flags)
                    .unwrap_or_else(|| {
                        self.options.push(opt.clone());
                        self.options.len() - 1
                    });
                let j = self.optidx_slot(n, &self.options[global_index]);
                self.optidx.insert(j, global_index);
                n += 1;
            }
        }
        n
    }

    pub fn prepare_optdef(&mut self, group_index: usize, scan_flags: &mut i32) {
        if let Some(group) = self.groups.get_mut(group_index) {
            if let Some(first) = group.first_mut() {
                first.flags &= !(FLAG_ALIAS | FLAG_SUBLIST);
            }

            for i in 0..group.len() {
                let opt = &mut group[i];
                let opt_flags = opt.flags;
                let opt_name = opt.name.clone();
                let opt_code = opt.code;
                self.options.push(opt.clone());
                let idx = self.options.len() - 1;

                if opt_code == ParseOutcome::Argument.code_value() {
                    self.options[idx].flags = FLAG_HIDDEN;
                    self.flags |= FLAG_NONOPT_ARG;
                    self.arg_set = Some(idx);
                    continue;
                }

                if self.version_enabled && opt_name == "V" {
                    *scan_flags |= 1;
                }
            }
        }

        let group_len = self.groups.get(group_index).map(|g| g.len()).unwrap_or(0);
        let start_idx = self.options.len().saturating_sub(group_len);
        for offset in 0..group_len {
            let idx = start_idx + offset;
            let opt_flags = self.options[idx].flags;
            let short_code = if (opt_flags & FLAG_ALIAS) == 0 && self.options[idx].code == 0 {
                Some(self.find_short_name(idx))
            } else {
                None
            };

            if (opt_flags & FLAG_ALIAS) == 0 {
                self.options[idx].flags &= !FLAG_IS_SET;
                if let Some(code) = short_code {
                    self.options[idx].code = code;
                }
                if (self.options[idx].flags & FLAG_BOOL) != 0 {
                    self.options[idx].flags &= !FLAG_ARG_OPTIONAL;
                    self.options[idx].arg_doc = None;
                    if self.negation.is_none() {
                        self.negation = Some(String::from("no-"));
                    }
                }
                if (self.options[idx].flags & FLAG_EARLY) != 0 {
                    self.flags |= FLAG_EARLY_PASS;
                }
            }
        }
    }

    pub fn set_help(&mut self, _opt_index: usize, _arg: Option<&str>) -> i32 {
        self.help_enabled = true;
        0
    }

    pub fn set_usage(&mut self, _opt_index: usize, _arg: Option<&str>) -> i32 {
        self.usage_enabled = true;
        0
    }

    pub fn set_version(&mut self, _opt_index: usize, _arg: Option<&str>) -> i32 {
        self.version_enabled = true;
        0
    }

    pub fn optgroup(&self, i: usize) -> Option<&[ParseoptOption]> {
        self.groups.get(i).map(Vec::as_slice)
    }

    pub fn parse(&mut self) -> ParseOutcome {
        let mut c = self.next();
        if matches!(c, ParseOutcome::Code(v) if v > 0) {
            assert!(self.setopt.is_some());
            loop {
                if let ParseOutcome::Code(code) = c {
                    let arg_owned = self.pending_value.clone();
                    if let Some(cb) = self.setopt.as_mut() {
                        cb(code, arg_owned.as_deref());
                    }
                }
                c = self.next();
                if !matches!(c, ParseOutcome::Code(v) if v > 0) {
                    break;
                }
            }
        }
        c
    }

    pub fn getopt(&mut self, argc: usize, argv: &[String]) -> ParseOutcome {
        if self.parseopt_02(argc, argv).is_err() {
            return ParseOutcome::Error;
        }

        if (self.flags & FLAG_EARLY_PASS) != 0 {
            let c = self.parse();
            if c != ParseOutcome::End {
                return c;
            }
            self.flags &= !FLAG_EARLY_PASS;
            self.argi = if (self.flags & FLAG_ARGV0) != 0 { 0 } else { 1 };
            self.eopt = false;
            self.arg_start = self.argi;
            self.arg_count = 0;
            self.permuted = false;
        }

        if let Some(mut hook) = self.init_hook.take() {
            hook(self);
            self.init_hook = Some(hook);
        }

        self.parse()
    }

    pub fn optdef_by_code(&self, code: i32) -> Option<&ParseoptOption> {
        self.optidx
            .iter()
            .filter_map(|&idx| self.options.get(idx))
            .find(|opt| opt.code == code)
    }

    pub fn optdef_by_name(&self, name: &str) -> Option<&ParseoptOption> {
        self.optidx.iter().find_map(|&idx| {
            let opt = self.options.get(idx)?;
            if opt.name == name {
                let canon = self.canonical_index(idx);
                self.options.get(canon)
            } else {
                None
            }
        })
    }

    pub fn is_set(&self, code: i32) -> Option<bool> {
        self.optdef_by_code(code)
            .map(|opt| (opt.flags & FLAG_IS_SET) != 0)
    }

    pub fn parseopt_03(&self) -> (usize, usize, bool) {
        (self.argi, self.arg_start, self.permuted)
    }

    pub fn parseopt_02(&mut self, argc: usize, argv: &[String]) -> Result<(), ()> {
        if (self.flags & FLAG_INITIALIZED) == 0 {
            self.init_internal()?;
        }

        self.argc = argc;
        self.argv = argv.to_vec();
        self.argi = 0;

        if (self.flags & FLAG_ARGV0) == 0 {
            self.argi += 1;
            if self.program_name.is_none() && !self.argv.is_empty() {
                let mut p = self.argv[0].rsplit('/').next().unwrap_or(&self.argv[0]).to_string();
                if p.len() > 3 && p.starts_with("lt-") {
                    p = p[3..].to_string();
                }
                self.program_name = Some(p);
            }
        }

        self.eopt = false;
        self.arg_start = self.argi;
        self.arg_count = 0;
        self.permuted = false;

        Ok(())
    }

    fn init_internal(&mut self) -> Result<(), ()> {
        let mut sf = if (self.flags & FLAG_SINGLE_DASH) != 0 { 1 } else { 0 };

        if self.ex_usage == 0 {
            self.ex_usage = DEFAULT_EX_USAGE;
        }

        self.optcount_reset();

        for i in 0..self.groups.len() {
            self.prepare_optdef(i, &mut sf);
        }

        if !(self.flags & FLAG_NO_STDOPT) != 0 {
            self.set_help(0, None);
            self.set_usage(0, None);
        }

        self.optidx.clear();
        let count = self.options.len();
        for n in 0..count {
            let slot = self.optidx_slot(n, &self.options[n]);
            self.optidx.insert(slot, n);
        }

        if (self.flags & FLAG_NONOPT_ARG) != 0 {
            self.flags |= FLAG_IN_ORDER;
        }

        self.flags |= FLAG_INITIALIZED;
        Ok(())
    }

    fn canonical_index(&self, idx: usize) -> usize {
        self.options
            .get(idx)
            .and_then(|o| o.canonical)
            .unwrap_or(idx)
    }

    fn optcount_reset(&mut self) {
        self.options.clear();
        self.optidx.clear();
        self.arg_set = None;
    }
}

impl ParseOutcome {
    fn code_value(self) -> i32 {
        match self {
            ParseOutcome::Argument => -2,
            ParseOutcome::Error => -1,
            ParseOutcome::End => 0,
            ParseOutcome::Next => -3,
            ParseOutcome::Code(v) => v,
        }
    }
}

impl Drop for Parseopt {
    fn drop(&mut self) {
        if (self.flags & FLAG_INITIALIZED) != 0 {
            self.groups.clear();
            self.options.clear();
            self.optidx.clear();
            self.flags &= !FLAG_INITIALIZED;
        }
    }
}
