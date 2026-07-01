use crate::help::{Context, Help, OptionDef};
use std::cmp::Ordering;
use std::process;

const PARSEOPT_SINGLE_DASH: u32 = 1 << 0;
const PARSEOPT_IN_ORDER: u32 = 1 << 1;
const PARSEOPT_NO_ERREXIT: u32 = 1 << 2;
const PARSEOPT_IGNORE_ERRORS: u32 = 1 << 3;
const PARSEOPT_NONOPT_ARG: u32 = 1 << 4;
const PARSEOPT_EARLY: u32 = 1 << 5;
const PARSEOPT_ARGV0: u32 = 1 << 6;
const PARSEOPT_NO_STDOPT: u32 = 1 << 7;
const PARSEOPT_INITIALIZED: u32 = 1 << 8;
const PARSEOPT_USER_MASK: u32 = PARSEOPT_SINGLE_DASH
    | PARSEOPT_IN_ORDER
    | PARSEOPT_NO_ERREXIT
    | PARSEOPT_IGNORE_ERRORS
    | PARSEOPT_ARGV0
    | PARSEOPT_NO_STDOPT;

const OPTFLAG_ALIAS: u32 = 1 << 0;
const OPTFLAG_BOOL: u32 = 1 << 1;
const OPTFLAG_ARG_OPTIONAL: u32 = 1 << 2;
const OPTFLAG_EARLY: u32 = 1 << 3;
const OPTFLAG_IS_SET: u32 = 1 << 4;
const OPTFLAG_HIDDEN: u32 = 1 << 5;
const OPTFLAG_SUBLIST: u32 = 1 << 6;

const OPT_NEXT: i32 = -4;
const OPT_ERR: i32 = -3;
const OPT_END: i32 = -2;
const OPT_ARG: i32 = -1;

const PO_MSG_INFO: i32 = 0;
const PO_MSG_ERR: i32 = 1;

const EX_USAGE: i32 = 64;

const SCANF_V: u32 = 1 << 0;

const DEFAULT_NEGATION_PREFIX: &str = "no-";

#[derive(Clone, Copy, PartialEq, Eq)]
enum NegMatch {
    NotTried,
    NoMatch,
    Inexact,
    Exact,
}

type SetoptHook = fn(&mut Parseopt, i32, Option<&str>);
type InitHook = fn(&mut Parseopt);
type ErrorHook = fn(&Parseopt, i32, &str);
type OptSetHook = fn(&mut Parseopt, usize, Option<&str>) -> bool;

#[derive(Clone)]
struct InternalOpt {
    def: OptionDef,
    code: i32,
    flags: u32,
    group_id: usize,
    sequence: usize,
    set_hook: Option<OptSetHook>,
}

pub struct Parseopt {
    pub help: Context,
    flags: u32,
    ex_usage: i32,
    program_name: Option<String>,
    negation: Option<String>,
    error_hook: Option<ErrorHook>,
    setopt_hook: Option<SetoptHook>,
    init_hook: Option<InitHook>,
    original_groups: Vec<Vec<OptionDef>>,
    optgrp: Vec<Vec<InternalOpt>>,
    optptr: Vec<(usize, usize)>,
    optidx: Vec<usize>,
    optcount: usize,
    arg_set: Option<usize>,
    argc: usize,
    argv_data: Vec<String>,
    argi: usize,
    arg_start: usize,
    arg_count: usize,
    permuted: bool,
    eopt: bool,
    dash_count: usize,
    optname: Option<String>,
}

impl Parseopt {
    pub fn option_dash(&self, opt: &OptionDef) -> &'static str {
        if (self.flags & PARSEOPT_SINGLE_DASH) != 0 || opt.name.chars().count() == 1 {
            "-"
        } else {
            "--"
        }
    }

    pub fn option_find_short(
        &mut self,
        current: &str,
    ) -> Option<(usize, usize, Option<String>, bool)> {
        let ch = current.chars().next()?;
        for pos in 0..self.optcount {
            let (g, i) = self.optptr[self.optidx[pos]];
            let name = self.optgrp[g][i].def.name.clone();
            if name.chars().count() == 1 && name.chars().next() == Some(ch) {
                let orig = (g, i);
                let canonical = self.unalias_index(g, i);
                let opt = &self.optgrp[canonical.0][canonical.1];
                if opt.def.argdoc.is_some() && current.len() > ch.len_utf8() {
                    let attached = current[ch.len_utf8()..].to_string();
                    self.optname = Some(ch.to_string());
                    return Some((orig.0, orig.1, Some(attached), true));
                }
                let arg = if (opt.flags & OPTFLAG_BOOL) != 0 {
                    Some("1".to_string())
                } else {
                    None
                };
                return Some((orig.0, orig.1, arg, false));
            }
        }

        self.error(
            PO_MSG_ERR,
            &format!("unrecognized option '-{}'", ch),
        );
        None
    }

    pub fn negmatch(&self, opt: &OptionDef, optstr: &str, optlen: usize) -> bool {
        self.negmatch_kind(opt, optstr, optlen) != NegMatch::NoMatch
    }

    pub fn option_find_long(
        &mut self,
        current: &str,
    ) -> Option<(usize, usize, Option<String>, bool)> {
        let optlen = current.find('=').unwrap_or(current.len());
        let mut found = 0usize;
        let mut found_opt: Option<(usize, usize)> = None;
        let mut negated = false;

        for pos in 0..self.optcount {
            let (g, i) = self.optptr[self.optidx[pos]];
            let opt = &self.optgrp[g][i];
            let namelen = opt.def.name.len();
            let mut neg = NegMatch::NotTried;

            if namelen == 1 && (self.flags & PARSEOPT_SINGLE_DASH) == 0 {
                continue;
            }

            let prefix_match = optlen <= namelen && opt.def.name[..optlen] == current[..optlen];
            let matched = if prefix_match {
                true
            } else {
                neg = self.negmatch_kind(&opt.def, current, optlen);
                neg != NegMatch::NoMatch
            };

            if matched {
                match found {
                    0 => {
                        found_opt = Some((g, i));
                        found = 1;
                        negated = matches!(neg, NegMatch::Inexact | NegMatch::Exact);
                        if optlen == namelen || neg == NegMatch::Exact {
                            break;
                        }
                    }
                    1 => {
                        found = 2;
                        if let Some((fg, fi)) = found_opt {
                            let found_name = self.optgrp[fg][fi].def.name.clone();
                            self.error(
                                PO_MSG_ERR,
                                &format!(
                                    "option '{}{}' is ambiguous; possibilities:",
                                    if self.dash_count == 1 { "-" } else { "--" },
                                    &current[..optlen]
                                ),
                            );
                            self.error(
                                PO_MSG_INFO,
                                &format!(
                                    "{}{}{}",
                                    if self.dash_count == 1 { "-" } else { "--" },
                                    if negated {
                                        self.negation.as_deref().unwrap_or(DEFAULT_NEGATION_PREFIX)
                                    } else {
                                        ""
                                    },
                                    found_name
                                ),
                            );
                            if !negated {
                                let neg_prefix =
                                    self.negation.as_deref().unwrap_or(DEFAULT_NEGATION_PREFIX);
                                if self.negmatch_kind(&self.optgrp[fg][fi].def, current, optlen)
                                    != NegMatch::NoMatch
                                {
                                    self.error(
                                        PO_MSG_INFO,
                                        &format!(
                                            "{}{}{}",
                                            if self.dash_count == 1 { "-" } else { "--" },
                                            neg_prefix,
                                            self.optgrp[fg][fi].def.name
                                        ),
                                    );
                                }
                            }
                        }
                        let neg_prefix = self.negation.as_deref().unwrap_or(DEFAULT_NEGATION_PREFIX);
                        self.error(
                            PO_MSG_INFO,
                            &format!(
                                "{}{}{}",
                                if self.dash_count == 1 { "-" } else { "--" },
                                if matches!(neg, NegMatch::Inexact | NegMatch::Exact) {
                                    neg_prefix
                                } else {
                                    ""
                                },
                                self.optgrp[g][i].def.name
                            ),
                        );
                    }
                    _ => {
                        let neg_prefix = self.negation.as_deref().unwrap_or(DEFAULT_NEGATION_PREFIX);
                        self.error(
                            PO_MSG_INFO,
                            &format!(
                                "{}{}{}",
                                if self.dash_count == 1 { "-" } else { "--" },
                                if matches!(neg, NegMatch::Inexact | NegMatch::Exact) {
                                    neg_prefix
                                } else {
                                    ""
                                },
                                self.optgrp[g][i].def.name
                            ),
                        );
                    }
                }
            }
        }

        match found {
            0 => {
                self.error(
                    PO_MSG_ERR,
                    &format!(
                        "unrecognized option '{}{}'",
                        if self.dash_count == 1 { "-" } else { "--" },
                        &current[..optlen]
                    ),
                );
                None
            }
            1 => {
                let (og, oi) = found_opt?;
                let (cg, ci) = self.unalias_index(og, oi);
                let arg = if current.as_bytes().get(optlen) == Some(&b'=') {
                    Some(current[optlen + 1..].to_string())
                } else if (self.optgrp[cg][ci].flags & OPTFLAG_BOOL) != 0 {
                    Some(if negated { "0" } else { "1" }.to_string())
                } else {
                    None
                };
                let has_arg = current.as_bytes().get(optlen) == Some(&b'=');
                Some((og, oi, arg, has_arg))
            }
            _ => None,
        }
    }

    pub fn permute(&mut self) {
        if (self.flags & PARSEOPT_IN_ORDER) == 0 && self.arg_count != 0 {
            let n = self.argi.saturating_sub(self.arg_start + self.arg_count);
            assert!(n <= 2);

            let mut save = Vec::with_capacity(n);
            for idx in 0..n {
                save.push(self.argv_data[self.arg_start + self.arg_count + idx].clone());
            }

            if self.arg_count > 0 {
                self.argv_data.copy_within(
                    self.arg_start..self.arg_start + self.arg_count,
                    self.arg_start + n,
                );
            }

            for (idx, value) in save.into_iter().enumerate() {
                self.argv_data[self.arg_start + idx] = value;
            }

            self.arg_start += n;
            self.permuted = true;
        }
    }

    pub fn lookahead(&self) -> Option<&str> {
        if (self.flags & PARSEOPT_SINGLE_DASH) == 0
            && self.dash_count == 1
            && self.optname.as_deref().is_some_and(|s| s.len() > 1)
        {
            self.optname.as_deref()
        } else {
            self.argv_data.get(self.argi).map(String::as_str)
        }
    }

    pub fn skip(&mut self) {
        if self.argi == self.argc || self.eopt {
            return;
        }
        self.argi += 1;
    }

    pub fn next_internal(&mut self, ret_arg: &mut Option<String>) -> i32 {
        let mut opt_arg: Option<String>;

        if (self.flags & PARSEOPT_SINGLE_DASH) == 0
            && self.dash_count == 1
            && self.optname.as_deref().is_some_and(|s| s.len() > 1)
        {
            if let Some(name) = self.optname.clone() {
                let mut chars = name.chars();
                chars.next();
                self.optname = Some(chars.collect());
            }
        } else {
            self.permute();

            loop {
                self.dash_count = 0;

                if self.argi == self.argc || self.eopt {
                    break;
                }

                let arg = self.argv_data[self.argi].clone();
                self.argi += 1;

                if let Some(stripped) = arg.strip_prefix('-') {
                    let mut current = stripped;
                    self.dash_count += 1;
                    if let Some(rest) = current.strip_prefix('-') {
                        current = rest;
                        self.dash_count += 1;
                        if current.is_empty() {
                            self.dash_count = 0;
                            self.permute();
                            self.eopt = true;
                            break;
                        }
                        if (self.flags & PARSEOPT_SINGLE_DASH) != 0 {
                            current = &arg[1..];
                            self.dash_count -= 1;
                        }
                    }
                    self.optname = Some(current.to_string());
                    break;
                } else if (self.flags & PARSEOPT_IN_ORDER) != 0 {
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
            let current = self.optname.clone().unwrap_or_default();
            let found = if self.dash_count == 2 || (self.flags & PARSEOPT_SINGLE_DASH) != 0 {
                self.option_find_long(&current)
            } else {
                self.option_find_short(&current)
            };

            let Some((orig_g, orig_i, mut found_arg, has_arg)) = found else {
                if (self.flags & PARSEOPT_NO_ERREXIT) != 0 {
                    return OPT_ERR;
                }
                process::exit(self.ex_usage);
            };

            let (g, i) = self.unalias_index(orig_g, orig_i);
            let opt_requires_arg = self.optgrp[g][i].def.argdoc.is_some();
            let opt_flags = self.optgrp[g][i].flags;
            let opt_has_set = self.optgrp[g][i].set_hook.is_some();
            let orig_name = self.optgrp[orig_g][orig_i].def.name.clone();

            if opt_requires_arg {
                if found_arg.is_some() {
                } else if (opt_flags & OPTFLAG_ARG_OPTIONAL) != 0 {
                    found_arg = None;
                } else if self.argi == self.argc {
                    self.error(
                        PO_MSG_ERR,
                        &format!(
                            "option '{}{}' requires argument",
                            if self.dash_count == 1 { "-" } else { "--" },
                            orig_name
                        ),
                    );
                    if (self.flags & PARSEOPT_NO_ERREXIT) != 0 {
                        return OPT_ERR;
                    }
                    process::exit(self.ex_usage);
                } else {
                    found_arg = self.argv_data.get(self.argi).cloned();
                    self.argi += 1;
                }
                if !opt_has_set {
                    *ret_arg = found_arg.clone();
                }
            } else if has_arg {
                self.error(
                    PO_MSG_ERR,
                    &format!(
                        "option '{}{}' does not take argument",
                        if self.dash_count == 1 { "-" } else { "--" },
                        orig_name
                    ),
                );
                if (self.flags & PARSEOPT_NO_ERREXIT) != 0 {
                    return OPT_ERR;
                }
                process::exit(self.ex_usage);
            }

            if (((self.flags & PARSEOPT_EARLY) != 0) as u8) ^ (((opt_flags & OPTFLAG_EARLY) != 0) as u8) != 0
            {
                return OPT_NEXT;
            }

            self.optgrp[g][i].flags |= OPTFLAG_IS_SET;

            if let Some(hook) = self.optgrp[g][i].set_hook {
                if hook(self, i, found_arg.as_deref()) {
                    self.error(
                        PO_MSG_ERR,
                        &format!(
                            "bad value for option '{}{}'",
                            if self.dash_count == 1 { "-" } else { "--" },
                            orig_name
                        ),
                    );
                    if (self.flags & PARSEOPT_NO_ERREXIT) != 0 {
                        return OPT_ERR;
                    }
                    process::exit(self.ex_usage);
                }
                return OPT_NEXT;
            }

            return self.optgrp[g][i].code;
        }

        if !self.permuted {
            self.arg_start = self.argi.saturating_sub(self.arg_count);
        }

        opt_arg = self.argv_data.get(self.arg_start).cloned();

        if opt_arg.is_some() && (self.flags & PARSEOPT_NONOPT_ARG) != 0 {
            self.argi += 1;
            if (self.flags & PARSEOPT_EARLY) != 0 {
                return OPT_NEXT;
            }
            if let Some(arg_set) = self.arg_set {
                let (g, i) = self.optptr[arg_set];
                if let Some(hook) = self.optgrp[g][i].set_hook {
                    if hook(self, i, opt_arg.as_deref()) {
                        if let Some(arg) = opt_arg.as_deref() {
                            self.error(PO_MSG_ERR, &format!("invalid argument '{}'", arg));
                        }
                        if (self.flags & PARSEOPT_NO_ERREXIT) != 0 {
                            return OPT_ERR;
                        }
                        process::exit(self.ex_usage);
                    }
                }
                return OPT_NEXT;
            } else {
                *ret_arg = opt_arg;
                return OPT_ARG;
            }
        }

        *ret_arg = opt_arg;
        OPT_END
    }

    pub fn next(&mut self, ret_arg: &mut Option<String>) -> i32 {
        let mut rc;
        loop {
            rc = self.next_internal(ret_arg);
            if rc != OPT_NEXT {
                break;
            }
        }
        rc
    }

    pub fn argv(&self) -> (usize, Vec<String>) {
        let args = if self.arg_start <= self.argv_data.len() {
            self.argv_data[self.arg_start..].to_vec()
        } else {
            Vec::new()
        };
        (self.argc.saturating_sub(self.arg_start), args)
    }

    pub fn error(&self, pri: i32, message: &str) {
        if let Some(hook) = self.error_hook {
            hook(self, pri, message);
            return;
        }
        if (self.flags & PARSEOPT_IGNORE_ERRORS) != 0 {
            return;
        }
        if pri == PO_MSG_ERR {
            if let Some(program_name) = &self.program_name {
                eprintln!("{program_name}: {message}");
                return;
            }
        }
        eprintln!("{message}");
    }

    pub fn find_short_name(&self, group: &[OptionDef], start: usize) -> i32 {
        let mut i = start;
        loop {
            let opt = match group.get(i) {
                Some(opt) => opt,
                None => return 0,
            };
            if opt.name.chars().count() == 1 {
                return opt.name.chars().next().map(|c| c as i32).unwrap_or(0);
            }
            i += 1;
            let next = match group.get(i) {
                Some(next) => next,
                None => return 0,
            };
            if !next.alias {
                return 0;
            }
        }
    }

    pub fn optidx_slot(&self, n: usize, opt: &InternalOpt) -> usize {
        if n == 0 {
            return 0;
        }
        let mut i = 0usize;
        let mut j = n;
        while i < j {
            let m = i + (j - i) / 2;
            let (g, oi) = self.optptr[self.optidx[m]];
            match self.optgrp[g][oi].def.name.cmp(&opt.def.name) {
                Ordering::Less | Ordering::Equal => i = m + 1,
                Ordering::Greater => j = m,
            }
        }
        i
    }

    pub fn collect_optdef(&mut self, group_index: usize, mut n: usize) -> usize {
        let len = self.optgrp[group_index].len();
        for i in 0..len {
            let is_option = self.is_option(group_index, i);
            if is_option {
                self.optptr[n] = (group_index, i);
                let slot = {
                    let opt = &self.optgrp[group_index][i];
                    self.optidx_slot(n, opt)
                };
                if slot < n {
                    self.optidx.copy_within(slot..n, slot + 1);
                }
                self.optidx[slot] = n;
                n += 1;
            }
        }
        n
    }

    pub fn prepare_optdef(&mut self, group: &mut [OptionDef], scan_flags: &mut u32) {
        if let Some(first) = group.first_mut() {
            first.alias = false;
        }

        for i in 0..group.len() {
            if !self.optiondef_is_option(&group[i]) {
                continue;
            }

            self.optcount += 1;

            if group[i].group == OPT_ARG {
                group[i].hidden = true;
                self.flags |= PARSEOPT_NONOPT_ARG;
                continue;
            }

            if self.help.version_hook.is_some() && group[i].name == "V" {
                *scan_flags |= SCANF_V;
            }

            if !group[i].alias {
                if group[i].bool_flag {
                    group[i].arg_optional = false;
                    group[i].argdoc = None;
                    if self.negation.is_none() {
                        self.negation = Some(DEFAULT_NEGATION_PREFIX.to_string());
                    }
                }
            }
        }
    }

    pub fn set_help(&mut self, _opt_index: usize, _arg: Option<&str>) -> bool {
        let mut ctx = self.help.clone();
        let _ = Help::fd(&mut ctx, &mut std::io::stdout());
        process::exit(0);
    }

    pub fn set_usage(&mut self, _opt_index: usize, _arg: Option<&str>) -> bool {
        let mut ctx = self.help.clone();
        let _ = Help::parseopt_usage_fd(&mut ctx, &mut std::io::stdout());
        process::exit(0);
    }

    pub fn set_version(&mut self, _opt_index: usize, _arg: Option<&str>) -> bool {
        let mut ctx = self.help.clone();
        let _ = Help::parseopt_version_fd(&mut ctx, &mut std::io::stdout());
        process::exit(0);
    }

    pub fn optgroup(&self, i: usize) -> Option<&[OptionDef]> {
        self.original_groups.get(i).map(Vec::as_slice)
    }

    pub fn init_0(&mut self) -> Result<(), ()> {
        let mut sf = if (self.flags & PARSEOPT_SINGLE_DASH) != 0 {
            SCANF_V
        } else {
            0
        };

        self.flags &= PARSEOPT_USER_MASK;
        if self.ex_usage == 0 {
            self.ex_usage = EX_USAGE;
        }
        if self.error_hook.is_none() {
            self.error_hook = Some(Self::default_error_hook);
        }

        let mut all_groups = self.original_groups.clone();
        if (self.flags & PARSEOPT_NO_STDOPT) == 0 {
            all_groups.push(Self::standard_options());
        }
        if self.help.version_hook.is_some() {
            all_groups.push(if (sf & SCANF_V) != 0 {
                Self::version_options_short()
            } else {
                Self::version_options_long()
            });
        }

        self.optcount = 0;
        self.arg_set = None;
        self.optgrp.clear();

        for (group_id, mut group) in all_groups.into_iter().enumerate() {
            self.prepare_optdef(&mut group, &mut sf);
            let mut converted = Vec::with_capacity(group.len());
            for (sequence, def) in group.into_iter().enumerate() {
                let mut flags = 0u32;
                if def.alias {
                    flags |= OPTFLAG_ALIAS;
                }
                if def.bool_flag {
                    flags |= OPTFLAG_BOOL;
                }
                if def.arg_optional {
                    flags |= OPTFLAG_ARG_OPTIONAL;
                }
                if def.hidden {
                    flags |= OPTFLAG_HIDDEN;
                }
                let mut code = if def.group == OPT_ARG { OPT_ARG } else { 0 };
                if !def.alias && code == 0 {
                    code = self.find_short_name(
                        self.original_groups
                            .get(group_id)
                            .map(Vec::as_slice)
                            .unwrap_or(&[]),
                        sequence,
                    );
                }
                if def.bool_flag && self.help.version_hook.is_none() && def.name == "help" {
                    flags &= !OPTFLAG_ARG_OPTIONAL;
                }

                let set_hook = if def.name == "help" {
                    Some(Self::help_hook as OptSetHook)
                } else if def.name == "usage" {
                    Some(Self::usage_hook as OptSetHook)
                } else if def.name == "version" || def.name == "V" {
                    Some(Self::version_hook as OptSetHook)
                } else {
                    None
                };

                converted.push(InternalOpt {
                    def,
                    code,
                    flags,
                    group_id,
                    sequence,
                    set_hook,
                });
            }
            self.optgrp.push(converted);
        }

        self.optptr = vec![(0, 0); self.optcount];
        self.optidx = vec![0; self.optcount];

        let mut j = 0usize;
        for i in 0..self.optgrp.len() {
            j = self.collect_optdef(i, j);
        }
        assert!(j == self.optcount);

        if (self.flags & PARSEOPT_NONOPT_ARG) != 0 {
            self.flags |= PARSEOPT_IN_ORDER;
        }

        self.flags |= PARSEOPT_INITIALIZED;
        Ok(())
    }

    pub fn new(help: Context, argc: usize, argv: Vec<String>) -> Self {
        let mut po = Self {
            help,
            flags: 0,
            ex_usage: EX_USAGE,
            program_name: None,
            negation: None,
            error_hook: None,
            setopt_hook: None,
            init_hook: None,
            original_groups: Vec::new(),
            optgrp: Vec::new(),
            optptr: Vec::new(),
            optidx: Vec::new(),
            optcount: 0,
            arg_set: None,
            argc,
            argv_data: argv,
            argi: 0,
            arg_start: 0,
            arg_count: 0,
            permuted: false,
            eopt: false,
            dash_count: 0,
            optname: None,
        };
        let _ = po.init_0();
        po.argc = argc;
        po
    }

    pub fn parse(&mut self) -> i32 {
        let mut c;
        let mut p = None;
        c = self.next(&mut p);
        if c > 0 {
            let hook = self
                .setopt_hook
                .expect("parse requires a registered option callback");
            loop {
                hook(self, c, p.as_deref());
                c = self.next(&mut p);
                if c <= 0 {
                    break;
                }
            }
        }
        c
    }

    pub fn getopt(&mut self, argc: usize, argv: Vec<String>) -> i32 {
        if self.parseopt_02(argc, argv).is_err() {
            return OPT_ERR;
        }

        if (self.flags & PARSEOPT_EARLY) != 0 {
            let c = self.parse();
            if c != OPT_END {
                return c;
            }
            self.flags &= !PARSEOPT_EARLY;
            self.argi = if (self.flags & PARSEOPT_ARGV0) != 0 { 0 } else { 1 };
            self.eopt = false;
            self.arg_start = self.argi;
            self.arg_count = 0;
            self.permuted = false;
        }
        if let Some(hook) = self.init_hook {
            hook(self);
        }
        self.parse()
    }

    pub fn optdef_by_code(&self, code: i32) -> Option<&OptionDef> {
        for i in 0..self.optcount {
            let (g, oi) = self.optptr[self.optidx[i]];
            let opt = &self.optgrp[g][oi];
            if opt.code == code {
                return Some(&opt.def);
            }
        }
        None
    }

    pub fn optdef_by_name(&self, name: &str) -> Option<&OptionDef> {
        for i in 0..self.optcount {
            let (g, oi) = self.optptr[self.optidx[i]];
            if self.optgrp[g][oi].def.name == name {
                let (cg, ci) = self.unalias_index(g, oi);
                return Some(&self.optgrp[cg][ci].def);
            }
        }
        None
    }

    pub fn is_set(&self, code: i32) -> i32 {
        for i in 0..self.optcount {
            let (g, oi) = self.optptr[self.optidx[i]];
            let opt = &self.optgrp[g][oi];
            if opt.code == code {
                return if (opt.flags & OPTFLAG_IS_SET) != 0 { 1 } else { 0 };
            }
        }
        -1
    }

    pub fn parseopt_03(&mut self) -> i32 {
        self.parse()
    }

    pub fn parseopt_02(&mut self, argc: usize, argv: Vec<String>) -> Result<(), ()> {
        if (self.flags & PARSEOPT_INITIALIZED) == 0 {
            self.init_0()?;
        }

        self.argc = argc;
        self.argv_data = argv;
        self.argi = 0;

        if (self.flags & PARSEOPT_ARGV0) == 0 {
            self.argi += 1;
            if self.program_name.is_none() {
                if let Some(argv0) = self.argv_data.first() {
                    let mut p = argv0.rsplit('/').next().unwrap_or(argv0).to_string();
                    if p.len() > 3 && p.starts_with("lt-") {
                        p = p[3..].to_string();
                    }
                    self.program_name = Some(p);
                }
            }
        }

        self.eopt = false;
        self.arg_start = self.argi;
        self.arg_count = 0;
        self.permuted = false;

        Ok(())
    }

    fn default_error_hook(po: &Parseopt, pri: i32, message: &str) {
        if (po.flags & PARSEOPT_IGNORE_ERRORS) != 0 {
            return;
        }
        if po.program_name.is_some() && pri == PO_MSG_ERR {
            eprintln!("{}: {}", po.program_name.as_deref().unwrap_or_default(), message);
        } else {
            eprintln!("{message}");
        }
    }

    fn help_hook(po: &mut Parseopt, opt_index: usize, arg: Option<&str>) -> bool {
        po.set_help(opt_index, arg)
    }

    fn usage_hook(po: &mut Parseopt, opt_index: usize, arg: Option<&str>) -> bool {
        po.set_usage(opt_index, arg)
    }

    fn version_hook(po: &mut Parseopt, opt_index: usize, arg: Option<&str>) -> bool {
        po.set_version(opt_index, arg)
    }

    fn optiondef_is_option(&self, opt: &OptionDef) -> bool {
        !opt.name.is_empty()
    }

    fn is_option(&self, group: usize, index: usize) -> bool {
        self.optgrp
            .get(group)
            .and_then(|g| g.get(index))
            .map(|o| !o.def.name.is_empty())
            .unwrap_or(false)
    }

    fn unalias_index(&self, group: usize, index: usize) -> (usize, usize) {
        let mut i = index;
        while i > 0 && (self.optgrp[group][i].flags & OPTFLAG_ALIAS) != 0 {
            i -= 1;
        }
        (group, i)
    }

    fn negmatch_kind(&self, opt: &OptionDef, optstr: &str, optlen: usize) -> NegMatch {
        if !opt.bool_flag {
            return NegMatch::NoMatch;
        }
        let neg = self
            .negation
            .as_deref()
            .unwrap_or(DEFAULT_NEGATION_PREFIX);
        let neglen = neg.len();
        let len = opt.name.len();
        if optlen <= neglen + len
            && optstr.get(..neglen) == Some(neg)
            && optstr.get(neglen..optlen) == Some(&opt.name[..optlen - neglen])
        {
            if optlen == neglen + len {
                NegMatch::Exact
            } else {
                NegMatch::Inexact
            }
        } else {
            NegMatch::NoMatch
        }
    }

    fn standard_options() -> Vec<OptionDef> {
        vec![
            OptionDef {
                name: "help".to_string(),
                argdoc: None,
                doc: Some("display this help and exit".to_string()),
                hidden: false,
                alias: false,
                bool_flag: true,
                arg_optional: false,
                group: 0,
                head: None,
            },
            OptionDef {
                name: "usage".to_string(),
                argdoc: None,
                doc: Some("display a short usage message and exit".to_string()),
                hidden: false,
                alias: false,
                bool_flag: true,
                arg_optional: false,
                group: 0,
                head: None,
            },
        ]
    }

    fn version_options_short() -> Vec<OptionDef> {
        vec![OptionDef {
            name: "version".to_string(),
            argdoc: None,
            doc: Some("output version information and exit".to_string()),
            hidden: false,
            alias: false,
            bool_flag: true,
            arg_optional: false,
            group: 0,
            head: None,
        }]
    }

    fn version_options_long() -> Vec<OptionDef> {
        vec![
            OptionDef {
                name: "version".to_string(),
                argdoc: None,
                doc: Some("output version information and exit".to_string()),
                hidden: false,
                alias: false,
                bool_flag: true,
                arg_optional: false,
                group: 0,
                head: None,
            },
            OptionDef {
                name: "V".to_string(),
                argdoc: None,
                doc: Some("output version information and exit".to_string()),
                hidden: false,
                alias: true,
                bool_flag: true,
                arg_optional: false,
                group: 0,
                head: None,
            },
        ]
    }
}

impl Drop for Parseopt {
    fn drop(&mut self) {
        if (self.flags & PARSEOPT_INITIALIZED) != 0 {
            self.optgrp.clear();
            self.optptr.clear();
            self.optidx.clear();
            self.flags &= !PARSEOPT_INITIALIZED;
        }
    }
}
