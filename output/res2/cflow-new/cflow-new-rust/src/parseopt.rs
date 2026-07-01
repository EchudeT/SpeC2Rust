use crate::help::{Context as HelpContext, Help, OptionDef};
use std::io::{self, Write};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ParseAction {
    None,
    Help,
    Usage,
    Version,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ArgDisposition {
    None,
    Required,
    Optional,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NegMatch {
    NotTried,
    NoMatch,
    Exact,
    Inexact,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StepCode {
    Next,
    End,
    Arg,
    Err,
    Code(i32),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct OptDef {
    code: i32,
    name: String,
    argdoc: Option<String>,
    doc: Option<String>,
    flags_is_alias: bool,
    flags_is_bool: bool,
    flags_arg_optional: bool,
    flags_is_set: bool,
    flags_early: bool,
    hidden: bool,
    group: i32,
    alias_origin: Option<usize>,
    action: ParseAction,
}

impl OptDef {
    fn takes_argument(&self) -> bool {
        self.argdoc.is_some()
    }

    fn short_name(&self) -> Option<char> {
        let mut chars = self.name.chars();
        let first = chars.next()?;
        if chars.next().is_none() {
            Some(first)
        } else {
            None
        }
    }

    fn to_help_option(&self) -> OptionDef {
        OptionDef {
            name: self.name.clone(),
            argdoc: self.argdoc.clone(),
            doc: self.doc.clone(),
            is_alias: self.flags_is_alias,
            is_hidden: self.hidden,
            is_bool: self.flags_is_bool,
            arg_optional: self.flags_arg_optional,
            is_option: true,
            is_group_header: false,
            group: self.group,
            sort_key: Some(self.name.clone()),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Parseopt03 {
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Parseopt02 {
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parseopt {
    argv_data: Vec<String>,
    options: Vec<OptDef>,
    option_order: Vec<usize>,
    argi: usize,
    arg_start: usize,
    arg_count: usize,
    permuted: bool,
    end_of_options: bool,
    dash_count: usize,
    optname: Option<String>,
    negation: String,
    single_dash: bool,
    in_order: bool,
    nonopt_arg: bool,
    early: bool,
    no_errexit: bool,
    ignore_errors: bool,
    program_name: Option<String>,
    ex_usage: i32,
    last_error: Option<String>,
    current_nonopt: Option<usize>,
    arg_handler: Option<usize>,
    help_context: HelpContext,
    parseopt_03_state: Parseopt03,
    parseopt_02_state: Parseopt02,
}

impl Default for Parseopt {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl Parseopt {
    pub fn option_dash(&self, opt: &str) -> &'static str {
        if self.single_dash || opt.chars().count() == 1 {
            "-"
        } else {
            "--"
        }
    }

    pub fn option_find_short(
        &mut self,
        optname: &str,
    ) -> Option<(usize, usize, Option<String>, bool)> {
        let ch = optname.chars().next()?;
        for &slot in &self.option_order {
            let candidate = &self.options[slot];
            if candidate.short_name() == Some(ch) {
                let orig_idx = slot;
                let mut idx = slot;
                while self.options[idx].flags_is_alias {
                    if idx == 0 {
                        break;
                    }
                    idx -= 1;
                }

                if self.options[idx].takes_argument() && optname.len() > ch.len_utf8() {
                    let tail = optname[ch.len_utf8()..].to_string();
                    self.optname = Some(ch.to_string());
                    return Some((idx, orig_idx, Some(tail), true));
                }

                let value = if self.options[idx].flags_is_bool {
                    Some("1".to_string())
                } else {
                    None
                };
                return Some((idx, orig_idx, value, false));
            }
        }

        self.error(format!("unrecognized option '-{}'", ch));
        None
    }

    pub fn negmatch(&self, opt_index: usize, optstr: &str, optlen: usize) -> bool {
        !matches!(self.negmatch_state(opt_index, optstr, optlen), NegMatch::NoMatch)
    }

    fn negmatch_state(&self, opt_index: usize, optstr: &str, optlen: usize) -> NegMatch {
        let Some(opt) = self.options.get(opt_index) else {
            return NegMatch::NoMatch;
        };
        if opt.flags_is_bool {
            let neglen = self.negation.len();
            let len = opt.name.len();
            if optlen <= neglen + len
                && optstr.get(..neglen) == Some(self.negation.as_str())
                && optstr
                    .get(neglen..neglen + optlen.saturating_sub(neglen))
                    == opt.name.get(..optlen.saturating_sub(neglen))
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
        optname: &str,
    ) -> Option<(usize, usize, Option<String>, bool)> {
        let optlen = optname.find('=').unwrap_or(optname.len());
        let mut found = 0usize;
        let mut found_idx = None::<usize>;
        let mut negated = false;

        for order_idx in 0..self.option_order.len() {
            let slot = self.option_order[order_idx];
            let namelen = self.options[slot].name.len();
            let mut neg = NegMatch::NotTried;

            if namelen == 1 && !self.single_dash {
                continue;
            }

            let name_match = {
                let opt = &self.options[slot];
                optlen <= namelen && opt.name.get(..optlen) == Some(&optname[..optlen])
            };
            if !name_match {
                neg = self.negmatch_state(slot, optname, optlen);
            }

            if name_match || !matches!(neg, NegMatch::NoMatch) {
                match found {
                    0 => {
                        found_idx = Some(slot);
                        found = 1;
                        negated = matches!(neg, NegMatch::Exact | NegMatch::Inexact);
                        if optlen == namelen || matches!(neg, NegMatch::Exact) {
                            break;
                        }
                    }
                    1 => {
                        found = 2;
                        self.error(format!(
                            "option '{}{}' is ambiguous; possibilities:",
                            ["", "-", "--"][self.dash_count.min(2)],
                            &optname[..optlen]
                        ));
                        if let Some(prev_idx) = found_idx {
                            let prev_name = self.options[prev_idx].name.clone();
                            self.error(format!(
                                "{}{}{}",
                                ["", "-", "--"][self.dash_count.min(2)],
                                if negated { self.negation.as_str() } else { "" },
                                prev_name
                            ));
                            if !negated
                                && !matches!(
                                    self.negmatch_state(prev_idx, optname, optlen),
                                    NegMatch::NoMatch
                                )
                            {
                                self.error(format!(
                                    "{}{}{}",
                                    ["", "-", "--"][self.dash_count.min(2)],
                                    self.negation,
                                    prev_name
                                ));
                            }
                        }
                        let opt_name = self.options[slot].name.clone();
                        self.error(format!(
                            "{}{}{}",
                            ["", "-", "--"][self.dash_count.min(2)],
                            if matches!(neg, NegMatch::Exact | NegMatch::Inexact) {
                                self.negation.as_str()
                            } else {
                                ""
                            },
                            opt_name
                        ));
                    }
                    _ => {
                        let opt_name = self.options[slot].name.clone();
                        self.error(format!(
                            "{}{}{}",
                            ["", "-", "--"][self.dash_count.min(2)],
                            if matches!(neg, NegMatch::Exact | NegMatch::Inexact) {
                                self.negation.as_str()
                            } else {
                                ""
                            },
                            opt_name
                        ));
                    }
                }
            }
        }

        match found {
            0 => {
                self.error(format!(
                    "unrecognized option '{}{}'",
                    ["", "-", "--"][self.dash_count.min(2)],
                    &optname[..optlen]
                ));
                None
            }
            1 => {
                let orig_idx = found_idx?;
                let mut idx = orig_idx;
                while self.options[idx].flags_is_alias {
                    if idx == 0 {
                        break;
                    }
                    idx -= 1;
                }

                if optname.as_bytes().get(optlen) == Some(&b'=') {
                    Some((idx, orig_idx, Some(optname[optlen + 1..].to_string()), true))
                } else {
                    let arg = if self.options[idx].flags_is_bool {
                        Some(if negated { "0" } else { "1" }.to_string())
                    } else {
                        None
                    };
                    Some((idx, orig_idx, arg, false))
                }
            }
            _ => None,
        }
    }

    pub fn permute(&mut self) {
        if self.in_order || self.arg_count == 0 {
            return;
        }

        let n = self.argi.saturating_sub(self.arg_start + self.arg_count);
        if n == 0 || n > 2 {
            return;
        }

        let base = self.arg_start + self.arg_count;
        let mut saved = Vec::with_capacity(n);
        for i in 0..n {
            saved.push(self.argv_data[base + i].clone());
        }

        let src_start = self.arg_start;
        let src_end = self.arg_start + self.arg_count;
        let moved: Vec<String> = self.argv_data[src_start..src_end].to_vec();
        for (i, item) in moved.into_iter().enumerate() {
            self.argv_data[src_start + n + i] = item;
        }

        for (i, item) in saved.into_iter().enumerate() {
            self.argv_data[self.arg_start + i] = item;
        }

        self.arg_start += n;
        self.permuted = true;
    }

    pub fn lookahead(&self) -> Option<&str> {
        if !self.single_dash
            && self.dash_count == 1
            && self.optname.as_ref().is_some_and(|s| s.chars().nth(1).is_some())
        {
            self.optname.as_deref()
        } else {
            self.argv_data.get(self.argi).map(String::as_str)
        }
    }

    pub fn skip(&mut self) {
        if self.argi == self.argv_data.len() || self.end_of_options {
            return;
        }
        self.argi += 1;
    }

    pub fn next_internal(&mut self, ret_arg: &mut Option<String>) -> i32 {
        *ret_arg = None;
        let mut opt_arg: Option<String>;

        if !self.single_dash
            && self.dash_count == 1
            && self.optname.as_ref().is_some_and(|s| s.chars().nth(1).is_some())
        {
            if let Some(name) = self.optname.clone() {
                self.optname = Some(name.chars().skip(1).collect());
            }
        } else {
            self.permute();

            loop {
                self.dash_count = 0;

                if self.argi == self.argv_data.len() || self.end_of_options {
                    break;
                }

                let arg = self.argv_data[self.argi].clone();
                self.argi += 1;

                if let Some(stripped1) = arg.strip_prefix('-') {
                    let mut current = stripped1;
                    self.dash_count += 1;
                    if let Some(stripped2) = current.strip_prefix('-') {
                        current = stripped2;
                        self.dash_count += 1;
                        if current.is_empty() {
                            self.dash_count = 0;
                            self.permute();
                            self.end_of_options = true;
                            break;
                        }
                        if self.single_dash {
                            current = &arg[1..];
                            self.dash_count -= 1;
                        }
                    }
                    self.optname = Some(current.to_string());
                    break;
                } else if self.in_order {
                    self.argi = self.argi.saturating_sub(1);
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
            let optname = self.optname.clone().unwrap_or_default();
            let found = if self.dash_count == 2 || self.single_dash {
                self.option_find_long(&optname)
            } else {
                self.option_find_short(&optname)
            };

            let Some((opt_idx, orig_idx, mut found_arg, has_arg)) = found else {
                return if self.no_errexit {
                    StepCode::Err.to_i32()
                } else {
                    StepCode::End.to_i32()
                };
            };

            opt_arg = found_arg.take();

            if self.options[opt_idx].takes_argument() {
                if opt_arg.is_none() {
                    if self.options[opt_idx].flags_arg_optional {
                        opt_arg = None;
                    } else if self.argi == self.argv_data.len() {
                        let dash = self.option_dash(&self.options[orig_idx].name);
                        self.error(format!(
                            "option '{}{}' requires argument",
                            dash, self.options[orig_idx].name
                        ));
                        return if self.no_errexit {
                            StepCode::Err.to_i32()
                        } else {
                            StepCode::End.to_i32()
                        };
                    } else {
                        opt_arg = self.argv_data.get(self.argi).cloned();
                        self.argi += 1;
                    }
                }
                *ret_arg = opt_arg.clone();
            } else if has_arg {
                let dash = self.option_dash(&self.options[orig_idx].name);
                self.error(format!(
                    "option '{}{}' does not take argument",
                    dash, self.options[orig_idx].name
                ));
                return if self.no_errexit {
                    StepCode::Err.to_i32()
                } else {
                    StepCode::End.to_i32()
                };
            }

            if self.early ^ self.options[opt_idx].flags_early {
                return StepCode::Next.to_i32();
            }

            self.options[opt_idx].flags_is_set = true;

            match self.options[opt_idx].action {
                ParseAction::Help => {
                    self.set_help();
                    return StepCode::Next.to_i32();
                }
                ParseAction::Usage => {
                    self.set_usage();
                    return StepCode::Next.to_i32();
                }
                ParseAction::Version => {
                    self.set_version();
                    return StepCode::Next.to_i32();
                }
                ParseAction::None => {}
            }

            return StepCode::Code(self.options[opt_idx].code).to_i32();
        }

        if !self.permuted {
            self.arg_start = self.argi.saturating_sub(self.arg_count);
        }

        opt_arg = self.argv_data.get(self.arg_start).cloned();

        if opt_arg.is_some() && self.nonopt_arg {
            self.argi += 1;
            if self.early {
                return StepCode::Next.to_i32();
            }

            if let Some(handler_idx) = self.arg_handler {
                self.options[handler_idx].flags_is_set = true;
                return StepCode::Next.to_i32();
            }

            *ret_arg = opt_arg;
            return StepCode::Arg.to_i32();
        }

        *ret_arg = opt_arg;
        StepCode::End.to_i32()
    }

    pub fn next(&mut self, ret_arg: &mut Option<String>) -> i32 {
        loop {
            let rc = self.next_internal(ret_arg);
            if rc != StepCode::Next.to_i32() {
                return rc;
            }
        }
    }

    pub fn argv(&self) -> (usize, Vec<String>) {
        let start = self.arg_start.min(self.argv_data.len());
        (
            self.argv_data.len().saturating_sub(start),
            self.argv_data[start..].to_vec(),
        )
    }

    pub fn error(&mut self, message: impl Into<String>) {
        if self.ignore_errors {
            return;
        }
        let msg = message.into();
        self.last_error = Some(msg.clone());
        let mut stderr = io::stderr().lock();
        if let Some(program_name) = &self.program_name {
            let _ = write!(stderr, "{program_name}: ");
        }
        let _ = writeln!(stderr, "{msg}");
    }

    pub fn find_short_name(&self, index: usize) -> i32 {
        let mut i = index;
        loop {
            if let Some(opt) = self.options.get(i) {
                if let Some(ch) = opt.short_name() {
                    return ch as i32;
                }
            } else {
                return 0;
            }

            i += 1;
            let Some(next) = self.options.get(i) else {
                return 0;
            };
            if !next.flags_is_alias {
                return 0;
            }
        }
    }

    pub fn optidx_slot(&self, n: usize, opt_index: usize) -> i32 {
        let code = self.options.get(opt_index).map(|o| o.code);
        if let Some(code) = code {
            for i in 0..n.min(self.option_order.len()) {
                let idx = self.option_order[i];
                if self.options.get(idx).map(|o| o.code) == Some(code) {
                    return i as i32;
                }
            }
        }
        -1
    }

    pub fn collect_optdef(&mut self, opt_index: usize, n: usize) -> usize {
        let Some(opt) = self.options.get(opt_index).cloned() else {
            return n;
        };

        if self.optidx_slot(n, opt_index) >= 0 {
            return n;
        }

        self.options[opt_index] = opt;
        self.option_order.push(opt_index);
        n + 1
    }

    pub fn prepare_optdef(&mut self, opt_index: usize, scan_flags: &mut i32) {
        let short_code = self.find_short_name(opt_index);
        let Some(opt) = self.options.get_mut(opt_index) else {
            return;
        };

        if opt.code == 0 {
            opt.code = short_code;
        }

        if opt.name == "help" {
            opt.action = ParseAction::Help;
        } else if opt.name == "usage" {
            opt.action = ParseAction::Usage;
        } else if opt.name == "version" {
            opt.action = ParseAction::Version;
        }

        if opt.short_name().is_some() {
            *scan_flags |= 0x01;
        } else {
            *scan_flags |= 0x02;
        }
    }

    pub fn set_help(&mut self) {
        self.sync_help_context();
        let mut buffer = Vec::new();
        let _ = Help::fd(&mut self.help_context, &mut buffer);
        self.last_error = Some(String::from_utf8_lossy(&buffer).into_owned());
    }

    pub fn set_usage(&mut self) {
        self.sync_help_context();
        let mut buffer = Vec::new();
        let _ = Help::parseopt_usage_fd(&mut self.help_context, &mut buffer);
        self.last_error = Some(String::from_utf8_lossy(&buffer).into_owned());
    }

    pub fn set_version(&mut self) {
        self.sync_help_context();
        let mut buffer = Vec::new();
        let _ = Help::parseopt_version_fd(&mut self.help_context, &mut buffer);
        self.last_error = Some(String::from_utf8_lossy(&buffer).into_owned());
    }

    pub fn optgroup(&self, i: usize) -> Option<&OptionDef> {
        self.help_context.options.get(i)
    }

    pub fn init_0(&mut self) -> i32 {
        self.argi = 0;
        self.arg_start = 0;
        self.arg_count = 0;
        self.permuted = false;
        self.end_of_options = false;
        self.dash_count = 0;
        self.optname = None;
        self.last_error = None;
        self.option_order.clear();

        let mut scan_flags = 0;
        for i in 0..self.options.len() {
            self.prepare_optdef(i, &mut scan_flags);
        }
        let mut n = 0usize;
        for i in 0..self.options.len() {
            n = self.collect_optdef(i, n);
        }
        self.sync_help_context();
        0
    }

    pub fn new(argv: Vec<String>) -> Self {
        let program_name = argv.first().cloned().unwrap_or_default();
        let mut po = Self {
            argv_data: argv,
            options: Vec::new(),
            option_order: Vec::new(),
            argi: 0,
            arg_start: 0,
            arg_count: 0,
            permuted: false,
            end_of_options: false,
            dash_count: 0,
            optname: None,
            negation: "no-".to_string(),
            single_dash: false,
            in_order: false,
            nonopt_arg: false,
            early: false,
            no_errexit: true,
            ignore_errors: false,
            program_name: if program_name.is_empty() {
                None
            } else {
                Some(program_name.clone())
            },
            ex_usage: 64,
            last_error: None,
            current_nonopt: None,
            arg_handler: None,
            help_context: Help::context(program_name),
            parseopt_03_state: Parseopt03::default(),
            parseopt_02_state: Parseopt02::default(),
        };
        let _ = po.init_0();
        po
    }

    pub fn parse(&mut self) -> i32 {
        let mut p = None::<String>;
        let mut last = StepCode::End.to_i32();
        loop {
            let c = self.next(&mut p);
            last = c;
            if c <= 0 {
                break;
            }
        }
        last
    }

    pub fn getopt(&mut self, argc: usize, argv: &[String]) -> i32 {
        self.argv_data = argv.iter().take(argc).cloned().collect();
        self.program_name = self.argv_data.first().cloned();
        self.help_context.program_name = self.program_name.clone().unwrap_or_default();
        let _ = self.init_0();
        self.parse()
    }

    pub fn optdef_by_code(&self, code: i32) -> Option<usize> {
        self.options.iter().position(|opt| opt.code == code)
    }

    pub fn optdef_by_name(&self, name: &str) -> Option<usize> {
        self.options.iter().position(|opt| opt.name == name)
    }

    pub fn is_set(&self, code: i32) -> i32 {
        match self.optdef_by_code(code) {
            Some(idx) => i32::from(self.options[idx].flags_is_set),
            None => -1,
        }
    }

    pub fn parseopt_03() -> Parseopt03 {
        Parseopt03 { enabled: true }
    }

    pub fn parseopt_02() -> Parseopt02 {
        Parseopt02 { enabled: true }
    }

    fn sync_help_context(&mut self) {
        self.help_context.negation = self.negation.clone();
        self.help_context.single_dash = self.single_dash;
        self.help_context.options = self.options.iter().map(OptDef::to_help_option).collect();
    }
}

impl Drop for Parseopt {
    fn drop(&mut self) {
        self.options.clear();
        self.option_order.clear();
        self.argv_data.clear();
        self.last_error = None;
        self.optname = None;
    }
}

impl StepCode {
    fn to_i32(self) -> i32 {
        match self {
            StepCode::Next => -2,
            StepCode::End => 0,
            StepCode::Arg => -1,
            StepCode::Err => -3,
            StepCode::Code(v) => v,
        }
    }
}
