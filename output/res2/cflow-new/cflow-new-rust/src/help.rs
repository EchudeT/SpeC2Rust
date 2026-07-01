use std::collections::BTreeMap;
use std::io::{self, Write};

pub struct Help;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UsageVarDef {
    pub name: String,
    pub is_bool: bool,
    pub value: Option<String>,
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Format {
    pub right_margin: usize,
    pub usage_indent: usize,
    pub long_opt_col: usize,
    pub dup_args: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OptionDef {
    pub name: String,
    pub argdoc: Option<String>,
    pub doc: Option<String>,
    pub is_alias: bool,
    pub is_hidden: bool,
    pub is_bool: bool,
    pub arg_optional: bool,
    pub is_option: bool,
    pub is_group_header: bool,
    pub group: i32,
    pub sort_key: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Context05 {
    pub label: String,
    pub first: usize,
    pub count: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Context {
    pub program_name: String,
    pub negation: String,
    pub argdoc: Option<String>,
    pub program_args: Vec<String>,
    pub single_dash: bool,
    pub options: Vec<OptionDef>,
    pub usage_vars: Vec<UsageVarDef>,
    pub format: Format,
    pub sorted_indices: Vec<usize>,
    pub argsused: bool,
    pub groups: Vec<Context05>,
    pub version_hook_output: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OptSort {
    pub indices: Vec<usize>,
    pub offset: usize,
}

impl Help {
    pub fn usage_var_def(name: impl Into<String>, is_bool: bool, value: Option<String>) -> UsageVarDef {
        UsageVarDef {
            name: name.into(),
            is_bool,
            value: value.clone(),
            enabled: value.is_some(),
        }
    }

    pub fn format() -> Format {
        Format {
            right_margin: 79,
            usage_indent: 12,
            long_opt_col: 24,
            dup_args: false,
        }
    }

    pub fn context(program_name: impl Into<String>) -> Context {
        Context {
            program_name: program_name.into(),
            negation: "no-".to_string(),
            format: Self::format(),
            ..Context::default()
        }
    }

    pub fn context_05(label: impl Into<String>, first: usize, count: usize) -> Context05 {
        Context05 {
            label: label.into(),
            first,
            count,
        }
    }

    pub fn fd(context: &mut Context, mut writer: impl Write) -> io::Result<()> {
        Self::init_usage_vars(context);
        Self::print_option_group(&mut writer, context)
    }

    pub fn parseopt_version_fd(context: &mut Context, mut writer: impl Write) -> io::Result<()> {
        if context.version_hook_output.is_none() {
            return Ok(());
        }
        Self::init_usage_vars(context);
        if let Some(text) = &context.version_hook_output {
            writer.write_all(text.as_bytes())?;
        }
        Ok(())
    }

    pub fn set_usage_var(context: &mut Context, text: &str) -> usize {
        let len = text.find([',', '=']).unwrap_or(text.len());
        let (name_start, boolval) = if len > 3 && text.starts_with("no-") {
            (3usize, false)
        } else {
            (0usize, true)
        };
        let name_end = len;
        let name = &text[name_start..name_end];
        let tail = &text[len..];

        if let Some(var) = context.usage_vars.iter_mut().find(|v| v.name == name) {
            if var.is_bool {
                if tail.starts_with('=') {
                    return len;
                }
                var.enabled = boolval;
                if !boolval {
                    var.value = Some("false".to_string());
                } else if var.value.is_none() {
                    var.value = Some("true".to_string());
                }
            } else if let Some(rest) = tail.strip_prefix('=') {
                let value_len = rest.find(',').unwrap_or(rest.len());
                let value = &rest[..value_len];
                if !value.is_empty() {
                    var.value = Some(value.to_string());
                    var.enabled = true;
                }
                return len + 1 + value_len;
            } else {
                return len;
            }
        } else if tail.starts_with('=') {
            let extra = tail[1..].find(',').unwrap_or(tail.len().saturating_sub(1));
            return len + 1 + extra;
        }

        len
    }

    pub fn init_usage_vars(context: &mut Context) {
        let Ok(fmt) = std::env::var("ARGP_HELP_FMT") else {
            return;
        };
        if fmt.is_empty() {
            return;
        }

        let mut rest = fmt.as_str();
        loop {
            let used = Self::set_usage_var(context, rest);
            if used >= rest.len() {
                break;
            }
            let next = &rest[used..];
            if next.is_empty() {
                break;
            }
            if let Some(stripped) = next.strip_prefix(',') {
                rest = stripped;
            } else {
                break;
            }
        }
    }

    pub fn print_arg(option: &OptionDef, delim: char, argsused: &mut bool) -> String {
        if let Some(argdoc) = &option.argdoc {
            *argsused = true;
            if option.arg_optional {
                format!("{delim}[{argdoc}]")
            } else {
                format!("{delim}{argdoc}")
            }
        } else {
            String::new()
        }
    }

    pub fn opt_unalias<'a>(options: &'a [OptionDef], index: usize) -> &'a OptionDef {
        let mut i = index;
        while i > 0 && options[i].is_alias {
            i -= 1;
        }
        &options[i]
    }

    pub fn parseopt_usage_std(context: &Context) -> String {
        let mut out = String::new();

        let short_noarg: Vec<&OptionDef> = context
            .options
            .iter()
            .filter(|opt| {
                opt.name.chars().count() == 1 && opt.argdoc.is_none() && !opt.is_hidden
            })
            .collect();

        if !short_noarg.is_empty() {
            out.push_str("[-");
            for opt in short_noarg {
                out.push_str(&opt.name);
            }
            out.push(']');
        }

        for (idx, opt) in context.options.iter().enumerate() {
            let orig = Self::opt_unalias(&context.options, idx);
            if opt.name.chars().count() == 1 && orig.argdoc.is_some() && !orig.is_hidden {
                out.push_str(" [-");
                out.push_str(&opt.name);
                out.push(' ');
                out.push_str(orig.argdoc.as_deref().unwrap_or_default());
                out.push(']');
            }
        }

        for (idx, opt) in context.options.iter().enumerate() {
            let orig = Self::opt_unalias(&context.options, idx);
            if opt.name.chars().count() > 1 && !opt.is_hidden {
                out.push_str(" [--");
                if opt.is_bool {
                    out.push('[');
                    out.push_str(&context.negation);
                    out.push(']');
                }
                out.push_str(&opt.name);
                if let Some(argdoc) = &orig.argdoc {
                    out.push('=');
                    if opt.arg_optional {
                        out.push('[');
                    }
                    out.push_str(argdoc);
                    if opt.arg_optional {
                        out.push(']');
                    }
                }
                out.push(']');
            }
        }

        out
    }

    pub fn parseopt_usage_sdash(context: &Context) -> String {
        let mut parts = Vec::new();

        for (idx, opt) in context.options.iter().enumerate() {
            if opt.is_option && !opt.is_hidden {
                let orig = Self::opt_unalias(&context.options, idx);
                let mut part = String::from("[-");
                if opt.is_bool {
                    part.push('[');
                    part.push_str(&context.negation);
                    part.push(']');
                }
                part.push_str(&opt.name);
                if let Some(argdoc) = &orig.argdoc {
                    part.push(' ');
                    if opt.arg_optional {
                        part.push('[');
                    }
                    part.push_str(argdoc);
                    if opt.arg_optional {
                        part.push(']');
                    }
                }
                part.push(']');
                parts.push(part);
            }
        }

        parts.join(" ")
    }

    pub fn parseopt_usage_fd(context: &mut Context, mut writer: impl Write) -> io::Result<()> {
        Self::init_usage_vars(context);
        write!(writer, "Usage: {} ", context.program_name)?;
        let usage = if context.single_dash {
            Self::parseopt_usage_sdash(context)
        } else {
            Self::parseopt_usage_std(context)
        };
        write!(writer, "{usage}")?;

        if let Some(argdoc) = &context.argdoc {
            writeln!(writer, " {argdoc}")?;
        }

        for program_arg in &context.program_args {
            writeln!(
                writer,
                "or: {} [OPTIONS...] {}",
                context.program_name, program_arg
            )?;
        }

        Ok(())
    }

    pub fn min_usize(a: usize, b: usize) -> usize {
        a.min(b)
    }

    pub fn merge<F>(
        options: &[OptionDef],
        source: &[usize],
        work: &mut [usize],
        left: usize,
        right: usize,
        end: usize,
        cmp: &F,
    ) where
        F: Fn(&[OptionDef], &[usize], usize, usize) -> i32,
    {
        let mut i = left;
        let mut j = right;

        for slot in work.iter_mut().take(end).skip(left) {
            if i < right && (j >= end || cmp(options, source, i, j) <= 0) {
                *slot = source[i];
                i += 1;
            } else {
                *slot = source[j];
                j += 1;
            }
        }
    }

    pub fn optsort<F>(options: &[OptionDef], indices: &mut [usize], cmp: &F) -> bool
    where
        F: Fn(&[OptionDef], &[usize], usize, usize) -> i32,
    {
        let n = indices.len();
        if n == 0 {
            return true;
        }

        let mut tmp = vec![0usize; n];
        let mut a = indices.to_vec();
        let mut b = tmp.clone();
        let mut width = 1usize;

        while width < n {
            let mut i = 0usize;
            while i < n {
                Self::merge(
                    options,
                    &a,
                    &mut b,
                    i,
                    Self::min_usize(i + width, n),
                    Self::min_usize(i + 2 * width, n),
                    cmp,
                );
                i += 2 * width;
            }
            std::mem::swap(&mut a, &mut b);
            width <<= 1;
        }

        indices.copy_from_slice(&a);
        true
    }

    pub fn compare_options(options: &[OptionDef], indices: &[usize], i: usize, j: usize) -> i32 {
        let a = &options[indices[i]];
        let b = &options[indices[j]];

        let a_hidden = a.is_hidden;
        let b_hidden = b.is_hidden;
        if a_hidden != b_hidden {
            return if a_hidden { 1 } else { -1 };
        }

        let a_group = a.group;
        let b_group = b.group;
        if a_group != b_group {
            return a_group.cmp(&b_group) as i32;
        }

        let a_short = a.name.chars().count() == 1;
        let b_short = b.name.chars().count() == 1;
        if a_short != b_short {
            return if a_short { -1 } else { 1 };
        }

        let an = a.sort_key.as_deref().unwrap_or(&a.name);
        let bn = b.sort_key.as_deref().unwrap_or(&b.name);
        match an.cmp(bn) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }

    pub fn sort_names(options: &[OptionDef], indices: &mut [usize], i: usize, j: usize) {
        let mut left = i;
        let mut right = j;
        while right > left {
            if Self::compare_options(options, indices, left, right) <= 0 {
                break;
            }
            indices.swap(left, right);
            right -= 1;
            left = i;
        }
    }

    pub fn set_head_group(options: &mut [OptionDef], indices: &[usize], offset: usize, i: usize, n: usize) {
        if offset + i >= indices.len() {
            return;
        }
        let head_idx = indices[offset + i];
        let head_group = options.get(head_idx).map(|o| o.group).unwrap_or_default();
        for pos in i..n {
            if let Some(opt) = indices.get(offset + pos).and_then(|idx| options.get_mut(*idx)) {
                opt.group = head_group;
            }
        }
    }

    pub fn sort_group(ops: &mut OptSort, options: &mut [OptionDef]) {
        if ops.offset >= ops.indices.len() {
            return;
        }

        let start = ops.offset;
        let current_group = options[ops.indices[start]].group;
        let mut end = start + 1;
        while end < ops.indices.len() && options[ops.indices[end]].group == current_group {
            end += 1;
        }

        let rel_end = end - start;
        let slice = &mut ops.indices[start..end];
        let _ = Self::optsort(options, slice, &Self::compare_options);

        if rel_end > 1 {
            Self::sort_names(options, slice, 0, rel_end - 1);
        }
        Self::set_head_group(options, &ops.indices, 0, start, end);
        ops.offset = end;
    }

    pub fn sort_options(context: &mut Context) {
        let mut indices: Vec<usize> = (0..context.options.len()).collect();
        context.sorted_indices.clear();

        if indices.is_empty() {
            return;
        }

        let mut by_group: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
        for idx in indices.drain(..) {
            by_group.entry(context.options[idx].group).or_default().push(idx);
        }

        let mut ops = OptSort::default();
        for (_, mut group_indices) in by_group {
            group_indices.sort_by(|a, b| {
                let probe = vec![*a, *b];
                match Self::compare_options(&context.options, &probe, 0, 1) {
                    n if n < 0 => std::cmp::Ordering::Less,
                    0 => std::cmp::Ordering::Equal,
                    _ => std::cmp::Ordering::Greater,
                }
            });
            ops.indices.extend(group_indices);
        }

        ops.offset = 0;
        while ops.offset < ops.indices.len() {
            Self::sort_group(&mut ops, &mut context.options);
        }

        context.sorted_indices = ops.indices;
    }

    pub fn print_option_std(
        context: &mut Context,
        current: &OptionDef,
        i: usize,
        next: usize,
    ) -> String {
        let mut out = String::new();
        let mut delim = None::<char>;
        let mut wrote_any = false;
        let mut cursor = i;

        while cursor < next {
            let opt = &context.options[context.sorted_indices[cursor]];
            if opt.is_hidden {
                cursor += 1;
                continue;
            }
            if opt.name.chars().count() > 1 {
                break;
            }
            if wrote_any {
                out.push_str(", ");
            }
            out.push('-');
            out.push_str(&opt.name);
            delim = Some(' ');
            if context.format.dup_args {
                out.push_str(&Self::print_arg(current, ' ', &mut context.argsused));
            }
            wrote_any = true;
            cursor += 1;
        }

        while cursor < next {
            let opt = &context.options[context.sorted_indices[cursor]];
            if !opt.is_hidden {
                if wrote_any {
                    out.push_str(", ");
                }
                break;
            }
            cursor += 1;
        }

        let mut long_wrote = false;
        while cursor < next {
            let opt = &context.options[context.sorted_indices[cursor]];
            if !opt.is_hidden {
                if long_wrote {
                    out.push_str(", ");
                }
                out.push_str("--");
                if opt.is_bool {
                    out.push('[');
                    out.push_str(&context.negation);
                    out.push(']');
                }
                out.push_str(&opt.name);
                delim = Some('=');
                if context.format.dup_args {
                    out.push_str(&Self::print_arg(current, '=', &mut context.argsused));
                }
                long_wrote = true;
            }
            cursor += 1;
        }

        if let Some(d) = delim {
            if !context.format.dup_args {
                out.push_str(&Self::print_arg(current, d, &mut context.argsused));
            }
        }

        out
    }

    pub fn print_option_sdash(
        context: &mut Context,
        current: &OptionDef,
        i: usize,
        next: usize,
    ) -> String {
        let mut out = String::new();
        let mut wrote_any = false;
        let mut delim = None::<char>;

        for cursor in i..next {
            let opt = &context.options[context.sorted_indices[cursor]];
            if !opt.is_hidden {
                if wrote_any {
                    out.push_str(", ");
                }
                out.push('-');
                if opt.is_bool {
                    out.push('[');
                    out.push_str(&context.negation);
                    out.push(']');
                }
                out.push_str(&opt.name);
                delim = Some('=');
                if context.format.dup_args {
                    out.push_str(&Self::print_arg(current, '=', &mut context.argsused));
                }
                wrote_any = true;
            }
        }

        if let Some(d) = delim {
            if !context.format.dup_args {
                out.push_str(&Self::print_arg(current, d, &mut context.argsused));
            }
        }

        out
    }

    pub fn print_option(context: &mut Context, i: usize) -> usize {
        if i >= context.sorted_indices.len() {
            return i;
        }

        let current_index = context.sorted_indices[i];
        let current = context.options[current_index].clone();
        let mut next = i + 1;
        while next < context.sorted_indices.len() {
            let other = &context.options[context.sorted_indices[next]];
            if other.group != current.group || other.argdoc != current.argdoc || other.doc != current.doc {
                break;
            }
            next += 1;
        }

        let _rendered = if context.single_dash {
            Self::print_option_sdash(context, &current, i, next)
        } else {
            Self::print_option_std(context, &current, i, next)
        };

        next
    }

    pub fn print_option_group(mut writer: impl Write, context: &mut Context) -> io::Result<()> {
        Self::sort_options(context);

        let mut i = 0usize;
        while i < context.sorted_indices.len() {
            let idx = context.sorted_indices[i];
            let current = context.options[idx].clone();
            let mut next = i + 1;
            while next < context.sorted_indices.len() {
                let other = &context.options[context.sorted_indices[next]];
                if other.group != current.group || other.argdoc != current.argdoc || other.doc != current.doc {
                    break;
                }
                next += 1;
            }

            let rendered = if context.single_dash {
                Self::print_option_sdash(context, &current, i, next)
            } else {
                Self::print_option_std(context, &current, i, next)
            };

            if !rendered.is_empty() || current.doc.is_some() {
                if let Some(doc) = &current.doc {
                    writeln!(writer, "{rendered}\t{doc}")?;
                } else {
                    writeln!(writer, "{rendered}")?;
                }
            }

            i = next;
        }

        context.sorted_indices.clear();
        Ok(())
    }
}
