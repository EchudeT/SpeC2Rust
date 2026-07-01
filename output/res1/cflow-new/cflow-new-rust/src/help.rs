use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::env;
use std::io::{self, Write};

pub struct Help;

#[derive(Clone, Debug, Default)]
pub struct UsageVarDef {
    pub name: String,
    pub requires_value: bool,
    pub bool_value: Option<bool>,
    pub number_value: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct Format {
    pub right_margin: usize,
    pub usage_indent: usize,
    pub long_option_column: usize,
    pub duplicate_args: bool,
}

impl Default for Format {
    fn default() -> Self {
        Self {
            right_margin: 79,
            usage_indent: 12,
            long_option_column: 24,
            duplicate_args: false,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Context05 {
    pub args_used: bool,
    pub current_group: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct Context {
    pub program_name: String,
    pub negation: String,
    pub single_dash: bool,
    pub args_used: bool,
    pub usage_tail: Option<String>,
    pub program_args: Vec<String>,
    pub groups: Vec<OptionGroup>,
    pub format: Format,
    pub idx: Vec<usize>,
    pub nidx: usize,
}

#[derive(Clone, Debug, Default)]
pub struct SortBy {
    pub group_name: Option<String>,
    pub stable_index: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Optsort {
    pub options: Vec<HelpOption>,
    pub idx: Vec<usize>,
    pub opti: usize,
}

#[derive(Clone, Debug, Default)]
pub struct OptionGroup {
    pub heading: Option<String>,
    pub options: Vec<HelpOption>,
}

#[derive(Clone, Debug, Default)]
pub struct HelpOption {
    pub name: String,
    pub aliases: Vec<String>,
    pub short: bool,
    pub hidden: bool,
    pub is_bool: bool,
    pub arg_doc: Option<String>,
    pub arg_optional: bool,
    pub doc: Option<String>,
    pub group: Option<String>,
    pub canonical_index: usize,
    pub display_order: usize,
    pub head: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct HelpFd {
    pub writer: Vec<u8>,
}

impl HelpFd {
    pub fn into_string(self) -> String {
        String::from_utf8_lossy(&self.writer).into_owned()
    }
}

impl Write for HelpFd {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Help {
    pub fn usage_var_def(name: impl Into<String>, requires_value: bool) -> UsageVarDef {
        UsageVarDef {
            name: name.into(),
            requires_value,
            bool_value: if requires_value { None } else { Some(false) },
            number_value: None,
        }
    }

    pub fn format() -> Format {
        Format::default()
    }

    pub fn context_05() -> Context05 {
        Context05::default()
    }

    pub fn context(program_name: impl Into<String>) -> Context {
        Context {
            program_name: program_name.into(),
            negation: "no-".to_string(),
            single_dash: false,
            args_used: false,
            usage_tail: None,
            program_args: Vec::new(),
            groups: Vec::new(),
            format: Self::format(),
            idx: Vec::new(),
            nidx: 0,
        }
    }

    pub fn sort_by(group_name: Option<String>, stable_index: usize) -> SortBy {
        SortBy {
            group_name,
            stable_index,
        }
    }

    pub fn fd() -> HelpFd {
        HelpFd::default()
    }

    pub fn set_usage_var(
        vars: &mut BTreeMap<String, UsageVarDef>,
        text: &str,
    ) -> Result<usize, String> {
        let mut boolval = true;
        let mut body = text;

        if let Some(rest) = body.strip_prefix("no-") {
            boolval = false;
            body = rest;
        }

        let split = body
            .find(|c| c == ',' || c == '=')
            .unwrap_or(body.len());
        let name = &body[..split];
        let mut end = split;

        if let Some(var) = vars.get_mut(name) {
            let next = body.as_bytes().get(split).copied().map(char::from);
            if !var.requires_value {
                if next == Some('=') {
                    return Err(format!(
                        "error in ARGP_HELP_FMT: improper usage of [no-]{}",
                        name
                    ));
                }
                var.bool_value = Some(boolval);
            } else if next == Some('=') {
                let value_text = &body[split + 1..];
                let consumed = value_text.find(',').unwrap_or(value_text.len());
                let raw = &value_text[..consumed];
                end = split + 1 + consumed;
                let parsed = raw.parse::<u32>().map_err(|_| {
                    format!(
                        "error in ARGP_HELP_FMT: bad value for {} (near {})",
                        name,
                        if raw.is_empty() { "end" } else { raw }
                    )
                })?;
                var.number_value = Some(parsed);
            } else {
                return Err(format!(
                    "{}: ARGP_HELP_FMT parameter requires a value",
                    name
                ));
            }
        } else {
            if body.as_bytes().get(split).copied().map(char::from) == Some('=') {
                let rest = &body[split..];
                end = split + rest.find(',').unwrap_or(rest.len());
            }
            return Err(format!("{}: unknown ARGP_HELP_FMT parameter", name));
        }

        Ok(if text.starts_with("no-") { end + 3 } else { end })
    }

    pub fn init_usage_vars(
        vars: &mut BTreeMap<String, UsageVarDef>,
    ) -> Result<(), String> {
        let Some(fmt) = env::var_os("ARGP_HELP_FMT") else {
            return Ok(());
        };
        let fmt = fmt.to_string_lossy();
        if fmt.is_empty() {
            return Ok(());
        }

        let mut pos = 0usize;
        while pos < fmt.len() {
            let consumed = Self::set_usage_var(vars, &fmt[pos..])?;
            pos += consumed;
            match fmt.as_bytes().get(pos).copied().map(char::from) {
                None => break,
                Some(',') => pos += 1,
                Some(_) => {
                    return Err(format!(
                        "ARGP_HELP_FMT: missing delimiter near {}",
                        &fmt[pos..]
                    ))
                }
            }
        }
        Ok(())
    }

    pub fn print_arg(out: &mut dyn Write, opt: &HelpOption, delim: char, argsused: &mut bool) {
        if let Some(arg) = &opt.arg_doc {
            *argsused = true;
            let text = if opt.arg_optional {
                format!("{delim}[{arg}]")
            } else {
                format!("{delim}{arg}")
            };
            let _ = write!(out, "{text}");
        }
    }

    pub fn opt_unalias<'a>(options: &'a [HelpOption], opt: &'a HelpOption) -> &'a HelpOption {
        options
            .get(opt.canonical_index)
            .unwrap_or(opt)
    }

    pub fn parseopt_usage_std(out: &mut dyn Write, ctx: &Context) {
        let all = ctx.groups.iter().flat_map(|g| g.options.iter()).collect::<Vec<_>>();
        let all_options = all.iter().cloned().cloned().collect::<Vec<_>>();

        let shorts_without_args = all
            .iter()
            .filter(|opt| opt.short && !opt.hidden && opt.arg_doc.is_none())
            .collect::<Vec<_>>();
        if !shorts_without_args.is_empty() {
            let _ = write!(out, "[-");
            for opt in shorts_without_args {
                let _ = write!(out, "{}", opt.name);
            }
            let _ = write!(out, "]");
        }

        for opt in all
            .iter()
            .filter(|opt| opt.short && !opt.hidden && opt.arg_doc.is_some())
        {
            let orig = Self::opt_unalias(&all_options, opt);
            let _ = write!(
                out,
                " [-{} {}]",
                opt.name,
                orig.arg_doc.as_deref().unwrap_or_default()
            );
        }

        for opt in all.iter().filter(|opt| !opt.short && !opt.hidden) {
            let orig = Self::opt_unalias(&all_options, opt);
            let _ = write!(out, " [--");
            if opt.is_bool {
                let _ = write!(out, "[{}]", ctx.negation);
            }
            let _ = write!(out, "{}", opt.name);
            if let Some(arg) = &orig.arg_doc {
                let _ = write!(out, "=");
                if opt.arg_optional {
                    let _ = write!(out, "[");
                }
                let _ = write!(out, "{arg}");
                if opt.arg_optional {
                    let _ = write!(out, "]");
                }
            }
            let _ = write!(out, "]");
        }
    }

    pub fn parseopt_usage_sdash(out: &mut dyn Write, ctx: &Context) {
        let all = ctx.groups.iter().flat_map(|g| g.options.iter()).collect::<Vec<_>>();
        let all_options = all.iter().cloned().cloned().collect::<Vec<_>>();
        let filtered = all
            .iter()
            .filter(|opt| !opt.hidden)
            .collect::<Vec<_>>();

        for (i, opt) in filtered.iter().enumerate() {
            if i > 0 {
                let _ = write!(out, " ");
            }
            let orig = Self::opt_unalias(&all_options, opt);
            let _ = write!(out, "[-");
            if opt.is_bool {
                let _ = write!(out, "[{}]", ctx.negation);
            }
            let _ = write!(out, "{}", opt.name);
            if let Some(arg) = &orig.arg_doc {
                let _ = write!(out, " ");
                if opt.arg_optional {
                    let _ = write!(out, "[");
                }
                let _ = write!(out, "{arg}");
                if opt.arg_optional {
                    let _ = write!(out, "]");
                }
            }
            let _ = write!(out, "]");
        }
    }

    pub fn parseopt_usage_fd(ctx: &Context, out: &mut dyn Write) {
        let _ = write!(out, "Usage: {} ", ctx.program_name);
        if ctx.single_dash {
            Self::parseopt_usage_sdash(out, ctx);
        } else {
            Self::parseopt_usage_std(out, ctx);
        }

        if let Some(tail) = &ctx.usage_tail {
            let _ = writeln!(out, " {tail}");
        } else {
            let _ = writeln!(out);
        }

        for arg in &ctx.program_args {
            let _ = writeln!(out, "or: {} [OPTIONS...] {}", ctx.program_name, arg);
        }
    }

    pub fn min_usize(a: usize, b: usize) -> usize {
        a.min(b)
    }

    pub fn merge_indices(
        options: &[HelpOption],
        source: &[usize],
        work: &mut [usize],
        left: usize,
        right: usize,
        end: usize,
    ) {
        let mut i = left;
        let mut j = right;
        for slot in work.iter_mut().take(end).skip(left) {
            if i < right
                && (j >= end || Self::compare_options(options, source, i, j) != Ordering::Greater)
            {
                *slot = source[i];
                i += 1;
            } else {
                *slot = source[j];
                j += 1;
            }
        }
    }

    pub fn sort_options_indices(ops: &mut Optsort, n: usize) -> Result<(), String> {
        let mut tmp = vec![0usize; n];
        let mut a = ops.idx[ops.opti..ops.opti + n].to_vec();
        let mut b = vec![0usize; n];
        let mut width = 1usize;

        while width < n {
            let mut i = 0usize;
            while i < n {
                Self::merge_indices(
                    &ops.options,
                    &a,
                    &mut b,
                    i,
                    Self::min_usize(i + width, n),
                    Self::min_usize(i + 2 * width, n),
                );
                i += 2 * width;
            }
            std::mem::swap(&mut a, &mut b);
            width <<= 1;
        }

        tmp.copy_from_slice(&a);
        ops.idx[ops.opti..ops.opti + n].copy_from_slice(&tmp);
        Ok(())
    }

    pub fn print_option_std(
        out: &mut dyn Write,
        ctx: &mut Context,
        current: &HelpOption,
        i: usize,
        next: usize,
    ) {
        let mut delim = None::<char>;
        let mut wrote = false;
        let mut pos = i;

        while pos < next {
            let idx = ctx.idx[pos];
            let opt = &ctx.groups.iter().flat_map(|g| g.options.iter()).collect::<Vec<_>>()[idx];
            if opt.hidden {
                pos += 1;
                continue;
            }
            if !opt.short {
                break;
            }
            if wrote {
                let _ = write!(out, ", ");
            }
            let _ = write!(out, "-{}", opt.name);
            delim = Some(' ');
            if ctx.format.duplicate_args {
                Self::print_arg(out, current, ' ', &mut ctx.args_used);
            }
            wrote = true;
            pos += 1;
        }

        while pos < next {
            let idx = ctx.idx[pos];
            let opt = &ctx.groups.iter().flat_map(|g| g.options.iter()).collect::<Vec<_>>()[idx];
            if !opt.hidden {
                if wrote {
                    let _ = write!(out, ", ");
                }
                break;
            }
            pos += 1;
        }

        let mut long_wrote = false;
        while pos < next {
            let idx = ctx.idx[pos];
            let opt = &ctx.groups.iter().flat_map(|g| g.options.iter()).collect::<Vec<_>>()[idx];
            if !opt.hidden {
                if long_wrote {
                    let _ = write!(out, ", ");
                }
                let _ = write!(out, "--");
                if opt.is_bool {
                    let _ = write!(out, "[{}]", ctx.negation);
                }
                let _ = write!(out, "{}", opt.name);
                delim = Some('=');
                if ctx.format.duplicate_args {
                    Self::print_arg(out, current, '=', &mut ctx.args_used);
                }
                long_wrote = true;
            }
            pos += 1;
        }

        if let Some(d) = delim {
            if !ctx.format.duplicate_args {
                Self::print_arg(out, current, d, &mut ctx.args_used);
            }
        }
    }

    pub fn print_option_sdash(
        out: &mut dyn Write,
        ctx: &mut Context,
        current: &HelpOption,
        i: usize,
        next: usize,
    ) {
        let mut wrote = false;
        let mut delim = None::<char>;

        for pos in i..next {
            let idx = ctx.idx[pos];
            let opt = &ctx.groups.iter().flat_map(|g| g.options.iter()).collect::<Vec<_>>()[idx];
            if !opt.hidden {
                if wrote {
                    let _ = write!(out, ", ");
                }
                let _ = write!(out, "-");
                if opt.is_bool {
                    let _ = write!(out, "[{}]", ctx.negation);
                }
                let _ = write!(out, "{}", opt.name);
                delim = Some('=');
                if ctx.format.duplicate_args {
                    Self::print_arg(out, current, '=', &mut ctx.args_used);
                }
                wrote = true;
            }
        }

        if let Some(d) = delim {
            if !ctx.format.duplicate_args {
                Self::print_arg(out, current, d, &mut ctx.args_used);
            }
        }
    }

    pub fn print_option(out: &mut dyn Write, ctx: &mut Context, i: usize) -> usize {
        let all = ctx
            .groups
            .iter()
            .flat_map(|g| g.options.iter())
            .cloned()
            .collect::<Vec<_>>();
        if i >= ctx.nidx || i >= all.len() {
            return ctx.nidx;
        }

        let current = &all[ctx.idx[i]];
        let head = current
            .head
            .clone()
            .or_else(|| current.group.clone())
            .unwrap_or_default();

        let mut next = i + 1;
        while next < ctx.nidx && next < all.len() {
            let opt = &all[ctx.idx[next]];
            let opt_head = opt
                .head
                .clone()
                .or_else(|| opt.group.clone())
                .unwrap_or_default();
            if opt_head != head {
                break;
            }
            next += 1;
        }

        if ctx.single_dash {
            Self::print_option_sdash(out, ctx, current, i, next);
        } else {
            Self::print_option_std(out, ctx, current, i, next);
        }

        if let Some(doc) = &current.doc {
            let pad = if ctx.format.long_option_column > 2 {
                ctx.format.long_option_column
            } else {
                2
            };
            let _ = writeln!(out, "{:width$}{}", "", doc, width = pad.saturating_sub(1));
        } else {
            let _ = writeln!(out);
        }

        next
    }

    pub fn compare_options(options: &[HelpOption], idx: &[usize], i: usize, j: usize) -> Ordering {
        let a = &options[idx[i]];
        let b = &options[idx[j]];

        match (a.hidden, b.hidden) {
            (false, true) => Ordering::Less,
            (true, false) => Ordering::Greater,
            _ => a
                .name
                .cmp(&b.name)
                .then_with(|| a.display_order.cmp(&b.display_order)),
        }
    }

    pub fn sort_names(ops: &mut Optsort, i: usize, j: usize) {
        let base_i = ops.opti + i;
        let mut cur_j = ops.opti + j;
        while cur_j > base_i {
            if Self::compare_options(&ops.options, &ops.idx, base_i, cur_j) != Ordering::Greater {
                break;
            }
            ops.idx.swap(base_i, cur_j);
            cur_j -= 1;
        }
    }

    pub fn set_head(ops: &mut Optsort, i: usize, n: usize) {
        let head_name = {
            let idx = ops.idx[ops.opti + i];
            ops.options[idx]
                .group
                .clone()
                .or_else(|| Some(ops.options[idx].name.clone()))
        };
        for off in i..n {
            let idx = ops.idx[ops.opti + off];
            ops.options[idx].head = head_name.clone();
        }
    }

    pub fn sort_group(ops: &mut Optsort) {
        let start = ops.opti;
        if start >= ops.idx.len() {
            return;
        }

        let group = ops.options[ops.idx[start]].group.clone();
        let mut end = start + 1;
        while end < ops.idx.len() && ops.options[ops.idx[end]].group == group {
            end += 1;
        }

        let n = end - start;
        for rel in 0..n {
            let idx = start + rel;
            ops.idx[idx] = idx;
            Self::sort_names(ops, 0, rel);
        }
        let _ = Self::sort_options_indices(ops, n);
        Self::set_head(ops, 0, n);
        ops.opti = end;
    }

    pub fn sort_options(ctx: &mut Context) {
        let all = ctx
            .groups
            .iter()
            .flat_map(|g| g.options.iter())
            .cloned()
            .collect::<Vec<_>>();

        let mut ops = Optsort {
            options: all,
            idx: Vec::new(),
            opti: 0,
        };

        ops.idx = (0..ops.options.len()).collect();
        ctx.idx = ops.idx.clone();
        ctx.nidx = ctx.idx.len();

        while ops.opti < ops.idx.len() {
            Self::sort_group(&mut ops);
        }

        ctx.idx = ops.idx;
        let mut flat_index = 0usize;
        for group in &mut ctx.groups {
            for opt in &mut group.options {
                if let Some(src) = ops.options.get(flat_index) {
                    opt.head = src.head.clone();
                }
                flat_index += 1;
            }
        }
    }

    pub fn print_option_group(out: &mut dyn Write, ctx: &mut Context) {
        Self::sort_options(ctx);
        let mut i = 0usize;
        while i < ctx.nidx {
            i = Self::print_option(out, ctx, i);
        }
        ctx.idx.clear();
    }

    pub fn parseopt_version_fd<F>(ctx: &Context, out: &mut dyn Write, mut version_hook: F)
    where
        F: FnMut(&mut dyn Write, &Context),
    {
        version_hook(out, ctx);
    }
}
