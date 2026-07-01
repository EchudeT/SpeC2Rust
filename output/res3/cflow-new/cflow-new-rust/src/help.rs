use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};

pub struct Help;

#[derive(Clone, Debug, Default)]
pub struct UsageVarDef {
    pub name: String,
    pub requires_value: bool,
    pub value: Option<usize>,
    pub enabled: bool,
}

#[derive(Clone, Debug)]
pub struct Format {
    pub right_margin: usize,
    pub usage_indent: usize,
    pub long_opt_col: usize,
    pub dup_args: bool,
}

impl Default for Format {
    fn default() -> Self {
        Self {
            right_margin: 79,
            usage_indent: 12,
            long_opt_col: 24,
            dup_args: false,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct OptionDef {
    pub name: String,
    pub argdoc: Option<String>,
    pub doc: Option<String>,
    pub hidden: bool,
    pub alias: bool,
    pub bool_flag: bool,
    pub arg_optional: bool,
    pub group: i32,
    pub head: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct Context05 {
    pub key: String,
    pub order: usize,
}

#[derive(Clone, Debug, Default)]
pub struct ModuleSrcParseoptOptsort07 {
    pub indices: Vec<usize>,
    pub offset: usize,
}

#[derive(Clone, Debug, Default)]
pub struct SortBy {
    pub group: i32,
    pub name: String,
    pub index: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Context {
    pub program_name: String,
    pub negation: String,
    pub argdoc: Option<String>,
    pub program_args: Vec<String>,
    pub single_dash: bool,
    pub options: Vec<OptionDef>,
    pub version_hook: Option<String>,
    pub usage_vars: HashMap<String, UsageVarDef>,
    pub idx: Vec<usize>,
    pub nidx: usize,
    pub argsused: bool,
    pub format: Format,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct OptSort {
    pub cmp: Option<fn(&[OptionDef], &[usize], usize, usize) -> i32>,
    pub optv: Vec<OptionDef>,
    pub idx: Vec<usize>,
    pub opti: usize,
    pub optc: usize,
}

impl Help {
    pub fn usage_var_def() -> UsageVarDef {
        UsageVarDef::default()
    }

    pub fn format() -> Format {
        Format::default()
    }

    pub fn context() -> Context {
        Context::default()
    }

    pub fn context_05() -> Context05 {
        Context05::default()
    }

    pub fn module_src_parseopt_optsort_07() -> ModuleSrcParseoptOptsort07 {
        ModuleSrcParseoptOptsort07::default()
    }

    pub fn sort_by() -> SortBy {
        SortBy::default()
    }

    pub fn min_value(a: usize, b: usize) -> usize {
        a.min(b)
    }

    pub fn set_usage_var(po: &mut Context, text: &str) -> usize {
        let mut boolval = true;
        let mut slice = text;

        if let Some(rest) = slice.strip_prefix("no-") {
            boolval = false;
            slice = rest;
        }

        let split_at = slice
            .find([',', '='])
            .unwrap_or(slice.len());
        let name = &slice[..split_at];
        let mut end = split_at;

        match name {
            "dup-args" => {
                if slice.as_bytes().get(split_at) == Some(&b'=') {
                    po.errors.push(format!(
                        "error in ARGP_HELP_FMT: improper usage of [no-]{}",
                        name
                    ));
                } else {
                    po.format.dup_args = boolval;
                }
            }
            "rmargin" | "usage-indent" | "long-opt-col" => {
                if slice.as_bytes().get(split_at) == Some(&b'=') {
                    let value_text = &slice[split_at + 1..];
                    let consumed = value_text.find(',').unwrap_or(value_text.len());
                    let number_text = &value_text[..consumed];
                    end = split_at + 1 + consumed;
                    match number_text.parse::<usize>() {
                        Ok(value) => match name {
                            "rmargin" => po.format.right_margin = value,
                            "usage-indent" => po.format.usage_indent = value,
                            "long-opt-col" => po.format.long_opt_col = value,
                            _ => {}
                        },
                        Err(_) => po.errors.push(format!(
                            "error in ARGP_HELP_FMT: bad value for {} (near {})",
                            name,
                            if number_text.is_empty() { "end" } else { number_text }
                        )),
                    }
                } else {
                    po.errors
                        .push(format!("{}: ARGP_HELP_FMT parameter requires a value", name));
                }
            }
            "" => {}
            _ => {
                po.errors
                    .push(format!("{}: unknown ARGP_HELP_FMT parameter", name));
                if slice.as_bytes().get(split_at) == Some(&b'=') {
                    let tail = &slice[split_at..];
                    end = split_at + tail.find(',').unwrap_or(tail.len());
                }
            }
        }

        end
    }

    pub fn init_usage_vars(po: &mut Context) {
        let Some(fmt) = env::var("ARGP_HELP_FMT").ok() else {
            return;
        };
        if fmt.is_empty() {
            return;
        }

        let mut current = fmt.as_str();
        loop {
            let used = Self::set_usage_var(po, current);
            let rest = &current[used..];
            if rest.is_empty() {
                break;
            } else if let Some(next) = rest.strip_prefix(',') {
                current = next;
            } else {
                po.errors
                    .push(format!("ARGP_HELP_FMT: missing delimiter near {}", rest));
                break;
            }
        }
    }

    pub fn print_arg<W: Write>(
        writer: &mut W,
        opt: &OptionDef,
        delim: char,
        argsused: &mut bool,
    ) -> io::Result<()> {
        if let Some(argdoc) = &opt.argdoc {
            *argsused = true;
            if opt.arg_optional {
                write!(writer, "{}[{}]", delim, argdoc)?;
            } else {
                write!(writer, "{}{}", delim, argdoc)?;
            }
        }
        Ok(())
    }

    pub fn opt_unalias<'a>(options: &'a [OptionDef], index: usize) -> &'a OptionDef {
        let mut i = index;
        while i > 0 && options[i].alias {
            i -= 1;
        }
        &options[i]
    }

    pub fn parseopt_usage_std<W: Write>(po: &Context, writer: &mut W) -> io::Result<()> {
        let mut first_short_without_arg = None;
        for (i, opt) in po.options.iter().enumerate() {
            if opt.name.len() == 1 && opt.argdoc.is_none() && !opt.hidden {
                first_short_without_arg = Some(i);
                break;
            }
        }

        if let Some(mut i) = first_short_without_arg {
            write!(writer, "[-{}", po.options[i].name)?;
            i += 1;
            while i < po.options.len() {
                let opt = &po.options[i];
                if opt.name.len() == 1 && opt.argdoc.is_none() && !opt.hidden {
                    write!(writer, "{}", opt.name)?;
                }
                i += 1;
            }
            write!(writer, "]")?;
        }

        for (i, opt) in po.options.iter().enumerate() {
            let orig = Self::opt_unalias(&po.options, i);
            if opt.name.len() == 1 && orig.argdoc.is_some() && !orig.hidden {
                write!(writer, " [-{} ", opt.name)?;
                if let Some(argdoc) = &orig.argdoc {
                    write!(writer, "{}]", argdoc)?;
                }
            }
        }

        for (i, opt) in po.options.iter().enumerate() {
            let orig = Self::opt_unalias(&po.options, i);
            if opt.name.len() > 1 && !opt.hidden {
                write!(writer, " [--")?;
                if opt.bool_flag {
                    write!(writer, "[{}]", po.negation)?;
                }
                write!(writer, "{}", opt.name)?;
                if let Some(argdoc) = &orig.argdoc {
                    write!(writer, "=")?;
                    if opt.arg_optional {
                        write!(writer, "[")?;
                    }
                    write!(writer, "{argdoc}")?;
                    if opt.arg_optional {
                        write!(writer, "]")?;
                    }
                }
                write!(writer, "]")?;
            }
        }

        Ok(())
    }

    pub fn parseopt_usage_sdash<W: Write>(po: &Context, writer: &mut W) -> io::Result<()> {
        let mut printed = false;
        for (i, opt) in po.options.iter().enumerate() {
            if !opt.name.is_empty() && !opt.hidden {
                let orig = Self::opt_unalias(&po.options, i);
                if printed {
                    write!(writer, " ")?;
                }
                write!(writer, "[-")?;
                if opt.bool_flag {
                    write!(writer, "[{}]", po.negation)?;
                }
                write!(writer, "{}", opt.name)?;
                if let Some(argdoc) = &orig.argdoc {
                    write!(writer, " ")?;
                    if opt.arg_optional {
                        write!(writer, "[")?;
                    }
                    write!(writer, "{argdoc}")?;
                    if opt.arg_optional {
                        write!(writer, "]")?;
                    }
                }
                write!(writer, "]")?;
                printed = true;
            }
        }
        Ok(())
    }

    pub fn parseopt_usage_fd<W: Write>(po: &mut Context, writer: &mut W) -> io::Result<()> {
        Self::init_usage_vars(po);
        write!(writer, "Usage: {} ", po.program_name)?;
        if po.single_dash {
            Self::parseopt_usage_sdash(po, writer)?;
        } else {
            Self::parseopt_usage_std(po, writer)?;
        }

        if let Some(argdoc) = &po.argdoc {
            writeln!(writer, " {}", argdoc)?;
        } else {
            writeln!(writer)?;
        }

        for arg in &po.program_args {
            writeln!(writer, "or: {} [OPTIONS...] {}", po.program_name, arg)?;
        }

        Ok(())
    }

    pub fn merge_indices(
        optv: &[OptionDef],
        source: &[usize],
        work: &mut [usize],
        left: usize,
        right: usize,
        end: usize,
        cmp: fn(&[OptionDef], &[usize], usize, usize) -> i32,
    ) {
        let mut i = left;
        let mut j = right;
        for slot in work.iter_mut().take(end).skip(left) {
            if i < right && (j >= end || cmp(optv, source, i, j) <= 0) {
                *slot = source[i];
                i += 1;
            } else {
                *slot = source[j];
                j += 1;
            }
        }
    }

    pub fn sort_options_range(ops: &mut OptSort, n: usize) -> bool {
        let mut tmp = vec![0usize; n];
        let mut a = ops.idx[ops.opti..ops.opti + n].to_vec();
        let mut b = tmp.clone();
        let cmp = ops.cmp.unwrap_or(Self::compare_options);

        let mut width = 1usize;
        while width < n {
            let mut i = 0usize;
            while i < n {
                Self::merge_indices(
                    &ops.optv,
                    &a,
                    &mut b,
                    i,
                    Self::min_value(i + width, n),
                    Self::min_value(i + 2 * width, n),
                    cmp,
                );
                i += 2 * width;
            }
            std::mem::swap(&mut a, &mut b);
            width <<= 1;
        }

        for (dst, src) in ops.idx[ops.opti..ops.opti + n].iter_mut().zip(a.into_iter()) {
            *dst = src;
        }
        true
    }

    pub fn print_option_std<W: Write>(
        writer: &mut W,
        ctx: &mut Context,
        cur_opt: &OptionDef,
        mut i: usize,
        next: usize,
    ) -> io::Result<()> {
        let mut delim = None::<char>;
        let mut wrote_any = false;

        while i < next {
            let opt = &ctx.options[ctx.idx[i]];
            if opt.hidden {
                i += 1;
                continue;
            }
            if opt.name.len() != 1 {
                break;
            }
            if wrote_any {
                write!(writer, ", ")?;
            }
            write!(writer, "-{}", opt.name)?;
            delim = Some(' ');
            if ctx.format.dup_args {
                Self::print_arg(writer, cur_opt, ' ', &mut ctx.argsused)?;
            }
            wrote_any = true;
            i += 1;
        }

        while i < next {
            let opt = &ctx.options[ctx.idx[i]];
            if !opt.hidden {
                if wrote_any {
                    write!(writer, ", ")?;
                }
                break;
            }
            i += 1;
        }

        let mut long_started = false;
        while i < next {
            let opt = &ctx.options[ctx.idx[i]];
            if !opt.hidden {
                if long_started {
                    write!(writer, ", ")?;
                }
                write!(writer, "--")?;
                if opt.bool_flag {
                    write!(writer, "[{}]", ctx.negation)?;
                }
                write!(writer, "{}", opt.name)?;
                delim = Some('=');
                if ctx.format.dup_args {
                    Self::print_arg(writer, cur_opt, '=', &mut ctx.argsused)?;
                }
                long_started = true;
            }
            i += 1;
        }

        if let Some(d) = delim {
            if !ctx.format.dup_args {
                Self::print_arg(writer, cur_opt, d, &mut ctx.argsused)?;
            }
        }
        Ok(())
    }

    pub fn print_option_sdash<W: Write>(
        writer: &mut W,
        ctx: &mut Context,
        cur_opt: &OptionDef,
        mut i: usize,
        next: usize,
    ) -> io::Result<()> {
        let mut w = false;
        let mut delim = None::<char>;
        while i < next {
            let opt = &ctx.options[ctx.idx[i]];
            if !opt.hidden {
                if w {
                    write!(writer, ", ")?;
                }
                write!(writer, "-")?;
                if opt.bool_flag {
                    write!(writer, "[{}]", ctx.negation)?;
                }
                write!(writer, "{}", opt.name)?;
                delim = Some('=');
                if ctx.format.dup_args {
                    Self::print_arg(writer, cur_opt, '=', &mut ctx.argsused)?;
                }
                w = true;
            }
            i += 1;
        }
        if let Some(d) = delim {
            if !ctx.format.dup_args {
                Self::print_arg(writer, cur_opt, d, &mut ctx.argsused)?;
            }
        }
        Ok(())
    }

    pub fn print_option<W: Write>(
        writer: &mut W,
        ctx: &mut Context,
        i: usize,
    ) -> io::Result<usize> {
        if i >= ctx.nidx {
            return Ok(i);
        }

        let first = ctx.idx[i];
        let cur_opt = ctx.options[first].clone();
        let mut next = i + 1;

        while next < ctx.nidx {
            let a = &ctx.options[ctx.idx[next]];
            let same_group = a.group == cur_opt.group;
            let same_head = a.head == cur_opt.head;
            let same_argdoc = a.argdoc == cur_opt.argdoc;
            if same_group && same_head && same_argdoc {
                next += 1;
            } else {
                break;
            }
        }

        if ctx.single_dash {
            Self::print_option_sdash(writer, ctx, &cur_opt, i, next)?;
        } else {
            Self::print_option_std(writer, ctx, &cur_opt, i, next)?;
        }

        if let Some(doc) = &cur_opt.doc {
            write!(writer, "\t{}", doc)?;
        }
        writeln!(writer)?;
        Ok(next)
    }

    pub fn compare_options(optv: &[OptionDef], idx: &[usize], i: usize, j: usize) -> i32 {
        let a = &optv[idx[i]];
        let b = &optv[idx[j]];

        match a.group.cmp(&b.group) {
            Ordering::Less => return -1,
            Ordering::Greater => return 1,
            Ordering::Equal => {}
        }

        match (a.hidden, b.hidden) {
            (false, true) => return -1,
            (true, false) => return 1,
            _ => {}
        }

        let a_short = a.name.len() == 1;
        let b_short = b.name.len() == 1;
        match (a_short, b_short) {
            (true, false) => return -1,
            (false, true) => return 1,
            _ => {}
        }

        match a.name.cmp(&b.name) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => 0,
        }
    }

    pub fn sort_names(ops: &mut OptSort, i: usize, j: usize) {
        let base_i = i + ops.opti;
        let mut base_j = j + ops.opti;
        while base_j > base_i {
            if Self::compare_options(&ops.optv, &ops.idx, base_i, base_j) <= 0 {
                break;
            } else {
                ops.idx.swap(base_i, base_j);
            }
            base_j -= 1;
        }
    }

    pub fn set_head(ops: &mut OptSort, i: usize, n: usize) {
        if i >= n || ops.opti + i >= ops.idx.len() {
            return;
        }
        let head_name = ops.optv[ops.idx[ops.opti + i]].name.clone();
        for k in i..n {
            let idx = ops.idx[ops.opti + k];
            ops.optv[idx].head = Some(head_name.clone());
        }
    }

    pub fn sort_group(ops: &mut OptSort) {
        let start = ops.opti;
        if start >= ops.optc {
            return;
        }

        let group = ops.optv[ops.idx[start]].group;
        let mut end = start;
        while end < ops.optc && ops.optv[ops.idx[end]].group == group {
            end += 1;
        }

        let n = end - start;
        if n > 0 {
            let _ = Self::sort_options_range(ops, n);
            if n >= 2 {
                for j in 1..n {
                    Self::sort_names(ops, 0, j);
                }
            }
            Self::set_head(ops, 0, n);
        }
        ops.opti = end;
    }

    pub fn sort_options(ctx: &mut Context) {
        let optc = ctx.options.len();
        let mut opts = OptSort {
            cmp: Some(Self::compare_options),
            optv: ctx.options.clone(),
            idx: (0..optc).collect(),
            opti: 0,
            optc,
        };

        ctx.idx = opts.idx.clone();
        ctx.nidx = optc;

        while opts.opti < opts.optc {
            Self::sort_group(&mut opts);
        }

        ctx.idx = opts.idx;
        ctx.options = opts.optv;
        ctx.nidx = ctx.idx.len();
    }

    pub fn print_option_group<W: Write>(writer: &mut W, ctx: &mut Context) -> io::Result<()> {
        Self::sort_options(ctx);
        let mut i = 0usize;
        while i < ctx.nidx {
            i = Self::print_option(writer, ctx, i)?;
        }
        ctx.idx.clear();
        Ok(())
    }

    pub fn fd<W: Write>(po: &mut Context, writer: &mut W) -> io::Result<()> {
        Self::init_usage_vars(po);
        Self::parseopt_usage_fd(po, writer)?;
        if !po.options.is_empty() {
            writeln!(writer)?;
            writeln!(writer, "Options:")?;
            Self::print_option_group(writer, po)?;
        }
        Ok(())
    }

    pub fn parseopt_version_fd<W: Write>(po: &mut Context, writer: &mut W) -> io::Result<()> {
        if po.version_hook.is_none() {
            return Ok(());
        }
        Self::init_usage_vars(po);
        if let Some(text) = &po.version_hook {
            write!(writer, "{text}")?;
            if !text.ends_with('\n') {
                writeln!(writer)?;
            }
        }
        Ok(())
    }
}
