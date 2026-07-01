use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::c_strcasecmp::CStrcasecmp;
use crate::localcharset::Localcharset;
use crate::propername_lite::PropernameLite;
use crate::progname::Progname;

#[derive(Clone, Debug)]
struct LineNumberState {
    line_buf: Vec<u8>,
    line_num_start: usize,
    line_num_end: usize,
    line_num_print: usize,
}

impl LineNumberState {
    fn new() -> Self {
        let mut line_buf = b"     0\t".to_vec();
        let line_num_end = 5;
        let line_num_start = 5;
        let line_num_print = 0;
        line_buf[line_num_end] = b'0';
        Self {
            line_buf,
            line_num_start,
            line_num_end,
            line_num_print,
        }
    }

    fn printable_bytes(&self) -> &[u8] {
        &self.line_buf[self.line_num_print..]
    }
}

#[derive(Clone, Debug, Default)]
struct OutputState {
    pending: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Default)]
struct CopyOptions {
    show_nonprinting: bool,
    show_tabs: bool,
    number: bool,
    number_nonblank: bool,
    show_ends: bool,
    squeeze_blank: bool,
}

#[derive(Clone, Debug)]
struct TransformState {
    line_numbers: LineNumberState,
    newlines2: i32,
    pending_cr: bool,
}

impl Default for TransformState {
    fn default() -> Self {
        Self {
            line_numbers: LineNumberState::new(),
            newlines2: 0,
            pending_cr: false,
        }
    }
}

pub struct Cat {
    transform: TransformState,
    output: OutputState,
}

impl Cat {
    pub fn print_usage(status: i32) -> ! {
        let program = env::args().next().unwrap_or_else(|| "cat".to_string());
        let mut out = io::stdout().lock();

        if status != 0 {
            let _ = writeln!(
                io::stderr().lock(),
                "Try '{} --help' for more information.",
                program
            );
        } else {
            let _ = writeln!(out, "Usage: {} [OPTION]... [FILE]...", program);
            let _ = writeln!(out, "Concatenate FILE(s) to standard output.");
            let _ = writeln!(out);
            let _ = writeln!(
                out,
                "  -A, --show-all           equivalent to -vET"
            );
            let _ = writeln!(
                out,
                "  -b, --number-nonblank    number nonempty output lines, overrides -n"
            );
            let _ = writeln!(out, "  -e                       equivalent to -vE");
            let _ = writeln!(
                out,
                "  -E, --show-ends          display $ at end of each line"
            );
            let _ = writeln!(out, "  -n, --number             number all output lines");
            let _ = writeln!(
                out,
                "  -s, --squeeze-blank      suppress repeated empty output lines"
            );
            let _ = writeln!(out, "  -t                       equivalent to -vT");
            let _ = writeln!(
                out,
                "  -T, --show-tabs          display TAB characters as ^I"
            );
            let _ = writeln!(out, "  -u                       (ignored)");
            let _ = writeln!(
                out,
                "  -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB"
            );
            let _ = writeln!(out, "      --help     display this help and exit");
            let _ = writeln!(out, "      --version  output version information and exit");
            let _ = writeln!(out);
            let _ = writeln!(out, "Examples:");
            let _ = writeln!(
                out,
                "  {} f - g  Output f's contents, then standard input, then g's contents.",
                program
            );
            let _ = writeln!(out, "  {}        Copy standard input to standard output.", program);
        }

        std::process::exit(status);
    }

    pub fn next_line_num(state: &mut Cat) {
        Self::next_line_num_state(&mut state.transform.line_numbers);
    }

    fn next_line_num_state(state: &mut LineNumberState) {
        let mut endp = state.line_num_end;
        loop {
            let current = state.line_buf[endp];
            if current < b'9' {
                state.line_buf[endp] = current + 1;
                return;
            }
            state.line_buf[endp] = b'0';
            if endp == state.line_num_start {
                break;
            }
            endp -= 1;
        }

        if state.line_num_start > 0 {
            state.line_num_start -= 1;
            state.line_buf[state.line_num_start] = b'1';
        } else {
            state.line_buf[0] = b'>';
        }

        if state.line_num_start < state.line_num_print {
            state.line_num_print -= 1;
        }
    }

    pub fn simple_cat<R: Read, W: Write>(
        reader: &mut R,
        writer: &mut W,
        bufsize: usize,
    ) -> io::Result<bool> {
        let mut buf = vec![0_u8; bufsize.max(1)];
        loop {
            let n_read = reader.read(&mut buf)?;
            if n_read == 0 {
                return Ok(true);
            }
            writer.write_all(&buf[..n_read])?;
        }
    }

    fn stdout_is_append_mode() -> bool {
        let fdinfo = match fs::read_to_string("/proc/self/fdinfo/1") {
            Ok(s) => s,
            Err(_) => return false,
        };
        for line in fdinfo.lines() {
            if let Some(rest) = line.strip_prefix("flags:\t") {
                if let Ok(flags) = u32::from_str_radix(rest.trim(), 8) {
                    return (flags & 0o2000) != 0;
                }
            }
        }
        false
    }

    fn fd_pos(fd: i32) -> Option<u64> {
        let path = format!("/proc/self/fdinfo/{fd}");
        let fdinfo = fs::read_to_string(path).ok()?;
        for line in fdinfo.lines() {
            if let Some(rest) = line.strip_prefix("pos:\t") {
                if let Ok(pos) = rest.trim().parse::<u64>() {
                    return Some(pos);
                }
            }
        }
        None
    }

    fn stdin_is_exhausting_stdout() -> bool {
        let stdin_meta = match fs::metadata("/proc/self/fd/0") {
            Ok(m) => m,
            Err(_) => return false,
        };
        let stdout_meta = match fs::metadata("/proc/self/fd/1") {
            Ok(m) => m,
            Err(_) => return false,
        };
        if !stdin_meta.is_file() || !stdout_meta.is_file() {
            return false;
        }
        if !Self::same_regular_file(&stdin_meta, &stdout_meta) {
            return false;
        }
        Self::stdout_is_append_mode()
    }

    pub fn write_pending<W: Write>(state: &mut Cat, writer: &mut W) -> io::Result<()> {
        Self::write_pending_state(&mut state.output, writer)
    }

    fn write_pending_state<W: Write>(state: &mut OutputState, writer: &mut W) -> io::Result<()> {
        if !state.pending.is_empty() {
            writer.write_all(&state.pending)?;
            state.pending.clear();
        }
        Ok(())
    }

    pub fn cat<R: Read, W: Write>(
        state: &mut Cat,
        reader: &mut R,
        writer: &mut W,
        options: CopyOptions,
    ) -> io::Result<bool> {
        Self::cat_with_state(&mut state.transform, &mut state.output, reader, writer, options)
    }

    fn cat_with_state<R: Read, W: Write>(
        transform: &mut TransformState,
        output: &mut OutputState,
        reader: &mut R,
        writer: &mut W,
        options: CopyOptions,
    ) -> io::Result<bool> {
        let mut at_line_start = transform.newlines2 >= 0;
        let mut blank_run = if transform.newlines2 > 0 {
            transform.newlines2 as usize
        } else {
            0
        };
        let mut buf = [0u8; 8192];

        loop {
            let n_read = reader.read(&mut buf)?;
            if n_read == 0 {
                break;
            }

            for &byte in &buf[..n_read] {
                if at_line_start {
                    let is_blank_line = byte == b'\n';

                    if is_blank_line {
                        blank_run += 1;
                        if options.squeeze_blank && blank_run >= 2 {
                            continue;
                        }
                        if options.number && !options.number_nonblank {
                            Self::next_line_num_state(&mut transform.line_numbers);
                            output
                                .pending
                                .extend_from_slice(transform.line_numbers.printable_bytes());
                        }
                    } else {
                        blank_run = 0;
                        if options.number {
                            Self::next_line_num_state(&mut transform.line_numbers);
                            output
                                .pending
                                .extend_from_slice(transform.line_numbers.printable_bytes());
                        }
                    }

                    at_line_start = false;
                }

                if options.show_nonprinting {
                    match byte {
                        b'\t' if !options.show_tabs => output.pending.push(b'\t'),
                        b'\n' => {
                            transform.newlines2 = -1;
                            if options.show_ends {
                                if transform.pending_cr {
                                    output.pending.extend_from_slice(b"^M");
                                    transform.pending_cr = false;
                                }
                                output.pending.push(b'$');
                            }
                            output.pending.push(b'\n');
                            at_line_start = true;
                        }
                        32..=126 => output.pending.push(byte),
                        127 => output.pending.extend_from_slice(b"^?"),
                        128..=255 => {
                            output.pending.extend_from_slice(b"M-");
                            if byte >= 160 {
                                if byte < 255 {
                                    output.pending.push(byte - 128);
                                } else {
                                    output.pending.extend_from_slice(b"^?");
                                }
                            } else {
                                output.pending.push(b'^');
                                output.pending.push(byte - 128 + 64);
                            }
                        }
                        _ => {
                            output.pending.push(b'^');
                            output.pending.push(byte + 64);
                        }
                    }
                } else {
                    match byte {
                        b'\t' if options.show_tabs => output.pending.extend_from_slice(b"^I"),
                        b'\n' => {
                            transform.newlines2 = -1;
                            if options.show_ends {
                                if transform.pending_cr {
                                    output.pending.extend_from_slice(b"^M");
                                    transform.pending_cr = false;
                                }
                                output.pending.push(b'$');
                            }
                            output.pending.push(b'\n');
                            at_line_start = true;
                        }
                        b'\r' => {
                            if options.show_ends {
                                transform.pending_cr = true;
                            } else {
                                output.pending.push(b'\r');
                            }
                        }
                        _ => {
                            if transform.pending_cr {
                                output.pending.push(b'\r');
                                transform.pending_cr = false;
                            }
                            output.pending.push(byte);
                        }
                    }
                }
            }

            Self::write_pending_state(output, writer)?;
        }

        transform.newlines2 = if at_line_start { blank_run as i32 } else { -1 };
        Ok(true)
    }

    pub fn copy_cat<R: Read, W: Write>(
        state: &mut Cat,
        reader: &mut R,
        writer: &mut W,
        options: CopyOptions,
    ) -> io::Result<bool> {
        if !(options.number
            || options.show_ends
            || options.show_nonprinting
            || options.show_tabs
            || options.squeeze_blank)
        {
            Self::simple_cat(reader, writer, 128 * 1024)
        } else {
            Self::cat(state, reader, writer, options)
        }
    }

    fn same_regular_file(a: &fs::Metadata, b: &fs::Metadata) -> bool {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            a.ino() == b.ino() && a.dev() == b.dev()
        }
        #[cfg(not(unix))]
        {
            let _ = (a, b);
            false
        }
    }

    fn named_input_is_exhausting_stdout(file: &File) -> bool {
        let meta = match file.metadata() {
            Ok(m) => m,
            Err(_) => return false,
        };
        if !meta.is_file() {
            return false;
        }
        let stdout_meta = match fs::metadata("/proc/self/fd/1") {
            Ok(m) => m,
            Err(_) => return false,
        };
        if !stdout_meta.is_file() || !Self::same_regular_file(&meta, &stdout_meta) {
            return false;
        }

        if Self::stdout_is_append_mode() {
            return true;
        }

        let in_pos = match file.try_clone().and_then(|mut f| f.seek(SeekFrom::Current(0))) {
            Ok(pos) => pos,
            Err(_) => return false,
        };
        let out_pos = match Self::fd_pos(1) {
            Some(pos) => pos,
            None => return false,
        };

        in_pos < out_pos
    }

    pub fn main() -> i32 {
        let args = Self::args_os();
        let argv0 = args
            .first()
            .and_then(|s| s.to_str())
            .unwrap_or("cat")
            .to_string();
        Progname::set_program_name(Some(&argv0));

        let mut options = CopyOptions::default();
        let mut files: Vec<String> = Vec::new();

        for arg in args.into_iter().skip(1) {
            let text = arg.to_string_lossy().into_owned();
            if text == "--help" {
                Self::print_usage(0);
            } else if text == "--version" {
                println!("cat");
                return 0;
            } else if text == "--number-nonblank" {
                options.number = true;
                options.number_nonblank = true;
            } else if text == "--number" {
                options.number = true;
            } else if text == "--squeeze-blank" {
                options.squeeze_blank = true;
            } else if text == "--show-nonprinting" {
                options.show_nonprinting = true;
            } else if text == "--show-ends" {
                options.show_ends = true;
            } else if text == "--show-tabs" {
                options.show_tabs = true;
            } else if text == "--show-all" {
                options.show_nonprinting = true;
                options.show_ends = true;
                options.show_tabs = true;
            } else if text.starts_with('-') && text != "-" && !text.starts_with("--") {
                for ch in text.chars().skip(1) {
                    match ch {
                        'b' => {
                            options.number = true;
                            options.number_nonblank = true;
                        }
                        'e' => {
                            options.show_ends = true;
                            options.show_nonprinting = true;
                        }
                        'n' => options.number = true,
                        's' => options.squeeze_blank = true,
                        't' => {
                            options.show_tabs = true;
                            options.show_nonprinting = true;
                        }
                        'u' => {}
                        'v' => options.show_nonprinting = true,
                        'A' => {
                            options.show_nonprinting = true;
                            options.show_ends = true;
                            options.show_tabs = true;
                        }
                        'E' => options.show_ends = true,
                        'T' => options.show_tabs = true,
                        _ => Self::print_usage(1),
                    }
                }
            } else {
                files.push(text);
            }
        }

        if files.is_empty() {
            files.push("-".to_string());
        }

        let stdout = io::stdout();
        let mut writer = stdout.lock();
        let mut cat = Self {
            transform: TransformState::default(),
            output: OutputState::default(),
        };
        let mut ok = true;

        for infile in files {
            if infile == "-" {
                if Self::stdin_is_exhausting_stdout() {
                    eprintln!("{}: -: input file is output file", argv0);
                    ok = false;
                    continue;
                }
                let stdin = io::stdin();
                let mut reader = stdin.lock();
                if Self::copy_cat(&mut cat, &mut reader, &mut writer, options).is_err() {
                    eprintln!("{}: -: I/O error", argv0);
                    ok = false;
                }
                continue;
            }

            match File::open(Path::new(&infile)) {
                Ok(file) => {
                    if Self::named_input_is_exhausting_stdout(&file) {
                        eprintln!("{}: {}: input file is output file", argv0, infile);
                        ok = false;
                        continue;
                    }

                    let mut reader = BufReader::new(file);
                    if Self::copy_cat(&mut cat, &mut reader, &mut writer, options).is_err() {
                        eprintln!("{}: {}: I/O error", argv0, infile);
                        ok = false;
                    }
                }
                Err(err) => {
                    eprintln!("{}: {}: {}", argv0, infile, err);
                    ok = false;
                }
            }
        }

        if cat.transform.pending_cr {
            if writer.write_all(b"\r").is_err() {
                ok = false;
            }
        }

        if writer.flush().is_err() {
            ok = false;
        }

        if ok { 0 } else { 1 }
    }

    pub fn proper_name(name_ascii: &str, name_utf8: &str) -> String {
        let translated = PropernameLite::choose(name_ascii, name_utf8);
        if translated != name_ascii {
            translated.to_string()
        } else if CStrcasecmp::eq_ignore_case(&Localcharset::locale_charset(), "UTF-8") {
            name_utf8.to_string()
        } else {
            name_ascii.to_string()
        }
    }

    pub fn args_os() -> Vec<std::ffi::OsString> {
        env::args_os().collect()
    }

}
