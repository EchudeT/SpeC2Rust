use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
struct Provider {
    f: &'static str,
    s: &'static str,
    e: &'static str,
}

#[derive(Debug, Clone)]
struct ShellEntry {
    shll: &'static str,
    inlo: &'static str,
    xecc: &'static str,
    lsto: &'static str,
}

#[derive(Debug, Clone)]
struct ParsedArg {
    consumed: usize,
    finished: bool,
}

#[derive(Debug, Clone)]
struct Arc4State {
    stte: [u8; 256],
    indx: u8,
    jndx: u8,
    kndx: u8,
}

impl Default for Arc4State {
    fn default() -> Self {
        let mut s = Self {
            stte: [0; 256],
            indx: 0,
            jndx: 0,
            kndx: 0,
        };
        for i in 0..256 {
            s.stte[i] = i as u8;
        }
        s
    }
}

#[derive(Debug, Clone)]
pub struct Shc {
    my_name: String,
    version: &'static str,
    subject: &'static str,
    cpright: &'static str,
    provider: Provider,
    copying: &'static [&'static str],
    abstract_lines: &'static [&'static str],
    help: &'static [&'static str],
    usage: &'static str,
    shells_db: &'static [ShellEntry],
    rtc: &'static [&'static str],
    setuid_line: &'static str,
    debugexec_line: &'static str,
    traceable_line: &'static str,
    hardening_line: &'static str,
    busyboxon_line: &'static str,
    mmap2_line: &'static str,

    file: Option<String>,
    file2: Option<String>,
    mail: String,
    inlo: Option<String>,
    xecc: Option<String>,
    lsto: Option<String>,
    shll: Option<String>,
    opts: String,
    text: Option<String>,
    date: String,
    rlax: Vec<u8>,
    verbose: usize,
    setuid_flag: i32,
    debugexec_flag: i32,
    traceable_flag: i32,
    hardening_flag: i32,
    busyboxon_flag: i32,
    mmap2_flag: i32,

    arc4: Arc4State,
    offset: usize,
    rng_state: u64,
}

impl Default for Shc {
    fn default() -> Self {
        Self::module_src()
    }
}

impl Shc {
    pub fn module_src() -> Self {
        Self {
            my_name: "shc".to_string(),
            version: "Version 4.0.3",
            subject: "Generic Shell Script Compiler",
            cpright: "GNU GPL Version 3",
            provider: Provider {
                f: "Md Jahidul",
                s: "Hamid",
                e: "<jahidulhamid@yahoo.com>",
            },
            copying: &[
                "Copying:",
                "",
                "    This program is free software; you can redistribute it and/or modify",
                "    it under the terms of the GNU General Public License as published by",
                "    the Free Software Foundation; either version 3 of the License, or",
                "    (at your option) any later version.",
                "",
                "    This program is distributed in the hope that it will be useful,",
                "    but WITHOUT ANY WARRANTY; without even the implied warranty of",
                "    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the",
                "    GNU General Public License for more details.",
                "",
                "    You should have received a copy of the GNU General Public License",
                "    along with this program; if not, write to the Free Software",
                "    @Neurobin, Dhaka, Bangladesh",
                "",
                "    Report problems and questions to:http://github.com/neurobin/shc",
                "",
            ],
            abstract_lines: &[
                "Abstract:",
                "",
                "    This tool generates a stripped binary executable version",
                "    of the script specified at command line.",
                "",
                "    Binary version will be saved with a .x extension by default.",
                "    You can specify output file name too with [-o filname] option.",
                "",
                "    You can specify expiration date [-e] too, after which binary will",
                "    refuse to be executed, displaying \"[-m]\" instead.",
                "",
                "    You can compile whatever interpreted script, but valid [-i], [-x]",
                "    and [-l] options must be given.",
                "",
            ],
            help: &[
                "",
                "    -e %s  Expiration date in dd/mm/yyyy format [none]",
                "    -m %s  Message to display upon expiration [\"Please contact your provider\"]",
                "    -f %s  File name of the script to compile",
                "    -i %s  Inline option for the shell interpreter i.e: -e",
                "    -x %s  eXec command, as a printf format i.e: exec('%s',@ARGV);",
                "    -l %s  Last shell option i.e: --",
                "    -o %s  output filename",
                "    -r     Relax security. Make a redistributable binary",
                "    -v     Verbose compilation",
                "    -S     Switch ON setuid for root callable programs [OFF]",
                "    -D     Switch ON debug exec calls [OFF]",
                "    -U     Make binary untraceable [no]",
                "    -H     Hardening : extra security protection [no]",
                "           Require bourne shell (sh) and parameters are not supported",
                "    -C     Display license and exit",
                "    -A     Display abstract and exit",
                "    -B     Compile for busybox",
                "    -2     Use the system call mmap2",
                "    -h     Display help and exit",
                "",
                "    Environment variables used:",
                "    Name    Default  Usage",
                "    CC      cc       C compiler command",
                "    STRIP   strip    Strip command",
                "    CFLAGS  <none>   C compiler flags",
                "    LDFLAGS <none>   Linker flags",
                "",
                "    Please consult the shc man page.",
                "",
            ],
            usage: "Usage: shc [-e date] [-m addr] [-i iopt] [-x cmd] [-l lopt] [-o outfile] [-rvDSUHCAB2h] -f script",
            shells_db: &[
                ShellEntry { shll: "perl", inlo: "-e", lsto: "--", xecc: "exec('%s',@ARGV);" },
                ShellEntry { shll: "rc", inlo: "-c", lsto: "", xecc: "builtin exec %s $*" },
                ShellEntry { shll: "sh", inlo: "-c", lsto: "", xecc: "exec '%s' \"$@\"" },
                ShellEntry { shll: "dash", inlo: "-c", lsto: "", xecc: "exec '%s' \"$@\"" },
                ShellEntry { shll: "bash", inlo: "-c", lsto: "", xecc: "exec '%s' \"$@\"" },
                ShellEntry { shll: "zsh", inlo: "-c", lsto: "", xecc: "exec '%s' \"$@\"" },
                ShellEntry { shll: "bsh", inlo: "-c", lsto: "", xecc: "exec '%s' \"$@\"" },
                ShellEntry { shll: "Rsh", inlo: "-c", lsto: "", xecc: "exec '%s' \"$@\"" },
                ShellEntry { shll: "ksh", inlo: "-c", lsto: "", xecc: "exec '%s' \"$@\"" },
                ShellEntry { shll: "tsh", inlo: "-c", lsto: "--", xecc: "exec '%s' \"$@\"" },
                ShellEntry { shll: "ash", inlo: "-c", lsto: "--", xecc: "exec '%s' \"$@\"" },
                ShellEntry { shll: "csh", inlo: "-c", lsto: "-b", xecc: "exec '%s' $argv" },
                ShellEntry { shll: "tcsh", inlo: "-c", lsto: "-b", xecc: "exec '%s' $argv" },
            ],
            rtc: &[],
            setuid_line: "#define SETUID %d\t/* Define as 1 to call setuid(0) at start of script */\n",
            debugexec_line: "#define DEBUGEXEC\t%d\t/* Define as 1 to debug execvp calls */\n",
            traceable_line: "#define TRACEABLE\t%d\t/* Define as 1 to enable ptrace the executable */\n",
            hardening_line: "#define HARDENING\t%d\t/* Define as 1 to disable ptrace/dump the executable */\n",
            busyboxon_line: "#define BUSYBOXON\t%d\t/* Define as 1 to enable work with busybox */\n",
            mmap2_line: "#define MMAP2\t\t%d\t/* Define as 1 to use syscall mmap2 */\n",
            file: None,
            file2: None,
            mail: "Please contact your provider jahidulhamid@yahoo.com".to_string(),
            inlo: None,
            xecc: None,
            lsto: None,
            shll: None,
            opts: String::new(),
            text: None,
            date: String::new(),
            rlax: vec![0],
            verbose: 0,
            setuid_flag: 0,
            debugexec_flag: 0,
            traceable_flag: 1,
            hardening_flag: 0,
            busyboxon_flag: 0,
            mmap2_flag: 0,
            arc4: Arc4State::default(),
            offset: 0,
            rng_state: 0x9e37_79b9_7f4a_7c15,
        }
    }

    pub fn parse_an_arg(&mut self, args: &[String], index: usize) -> Result<ParsedArg, String> {
        if index >= args.len() {
            if self.file.is_none() {
                eprintln!("{} parse(-f): No source file specified", self.my_name);
                self.file = Some(String::new());
                return Err("missing source file".to_string());
            }
            return Ok(ParsedArg {
                consumed: 0,
                finished: true,
            });
        }

        let arg = &args[index];
        if !arg.starts_with('-') || arg == "-" {
            eprintln!("{} parse: Unknown option", self.my_name);
            return Err("unknown option".to_string());
        }

        let need_value = |i: usize, name: &str, args: &[String], me: &Shc| -> Result<String, String> {
            args.get(i + 1)
                .cloned()
                .ok_or_else(|| {
                    eprintln!("{} parse: Missing parameter", me.my_name);
                    format!("missing parameter for {}", name)
                })
        };

        match arg.as_str() {
            "-e" => {
                let optarg = need_value(index, "-e", args, self)?;
                let exp = Self::parse_expiration(&optarg).ok_or_else(|| {
                    eprintln!(
                        "{} parse(-e {}): Not a valid value",
                        self.my_name, optarg
                    );
                    "invalid expiration".to_string()
                })?;
                self.date = exp.to_string();
                if self.verbose > 0 {
                    eprintln!("{} -e {}", self.my_name, self.date);
                }
                Ok(ParsedArg {
                    consumed: 2,
                    finished: false,
                })
            }
            "-m" => {
                self.mail = need_value(index, "-m", args, self)?;
                Ok(ParsedArg {
                    consumed: 2,
                    finished: false,
                })
            }
            "-f" => {
                if self.file.is_some() {
                    eprintln!("{} parse(-f): Specified more than once", self.my_name);
                    return Err("file specified more than once".to_string());
                }
                self.file = Some(need_value(index, "-f", args, self)?);
                Ok(ParsedArg {
                    consumed: 2,
                    finished: false,
                })
            }
            "-i" => {
                self.inlo = Some(need_value(index, "-i", args, self)?);
                Ok(ParsedArg {
                    consumed: 2,
                    finished: false,
                })
            }
            "-x" => {
                self.xecc = Some(need_value(index, "-x", args, self)?);
                Ok(ParsedArg {
                    consumed: 2,
                    finished: false,
                })
            }
            "-l" => {
                self.lsto = Some(need_value(index, "-l", args, self)?);
                Ok(ParsedArg {
                    consumed: 2,
                    finished: false,
                })
            }
            "-o" => {
                self.file2 = Some(need_value(index, "-o", args, self)?);
                Ok(ParsedArg {
                    consumed: 2,
                    finished: false,
                })
            }
            "-r" => {
                if self.rlax.is_empty() {
                    self.rlax.push(1);
                } else {
                    self.rlax[0] = self.rlax[0].wrapping_add(1);
                }
                Ok(ParsedArg {
                    consumed: 1,
                    finished: false,
                })
            }
            "-v" => {
                self.verbose += 1;
                Ok(ParsedArg {
                    consumed: 1,
                    finished: false,
                })
            }
            "-S" => {
                self.setuid_flag = 1;
                Ok(ParsedArg {
                    consumed: 1,
                    finished: false,
                })
            }
            "-D" => {
                self.debugexec_flag = 1;
                Ok(ParsedArg {
                    consumed: 1,
                    finished: false,
                })
            }
            "-U" => {
                self.traceable_flag = 0;
                Ok(ParsedArg {
                    consumed: 1,
                    finished: false,
                })
            }
            "-H" => {
                self.hardening_flag = 1;
                Ok(ParsedArg {
                    consumed: 1,
                    finished: false,
                })
            }
            "-B" => {
                self.busyboxon_flag = 1;
                Ok(ParsedArg {
                    consumed: 1,
                    finished: false,
                })
            }
            "-2" => {
                self.mmap2_flag = 1;
                Ok(ParsedArg {
                    consumed: 1,
                    finished: false,
                })
            }
            "-C" => {
                eprintln!("{} {}, {}", self.my_name, self.version, self.subject);
                eprintln!(
                    "{} {} {} {} {}",
                    self.my_name, self.cpright, self.provider.f, self.provider.s, self.provider.e
                );
                eprintln!("{} ", self.my_name);
                for line in self.copying {
                    eprintln!("{}", line);
                }
                eprintln!(
                    "    {} {} {}\n",
                    self.provider.f, self.provider.s, self.provider.e
                );
                Err("exit 0".to_string())
            }
            "-A" => {
                eprintln!("{} {}, {}", self.my_name, self.version, self.subject);
                eprintln!(
                    "{} {} {} {} {}",
                    self.my_name, self.cpright, self.provider.f, self.provider.s, self.provider.e
                );
                eprintln!("{} ", self.my_name);
                for line in self.abstract_lines {
                    eprintln!("{}", line);
                }
                Err("exit 0".to_string())
            }
            "-h" => {
                eprintln!("{} {}, {}", self.my_name, self.version, self.subject);
                eprintln!(
                    "{} {} {} {} {}",
                    self.my_name, self.cpright, self.provider.f, self.provider.s, self.provider.e
                );
                eprintln!("{} {}", self.my_name, self.usage);
                for line in self.help {
                    eprintln!("{}", line);
                }
                Err("exit 0".to_string())
            }
            _ => {
                eprintln!("{} parse: Unknown option", self.my_name);
                Err("unknown option".to_string())
            }
        }
    }

    pub fn parse_args(&mut self, args: &[String]) -> Result<(), String> {
        let mut err = 0usize;
        let mut index = 1usize;

        if let Some(name) = args.first() {
            self.my_name = Path::new(name)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or(name)
                .to_string();
        }

        while index <= args.len() {
            match self.parse_an_arg(args, index) {
                Ok(parsed) => {
                    if parsed.finished {
                        break;
                    }
                    index += parsed.consumed;
                }
                Err(e) => {
                    if e == "exit 0" {
                        return Err(e);
                    }
                    err += 1;
                    break;
                }
            }
        }

        if err > 0 {
            eprintln!("\n{} {}\n", self.my_name, self.usage);
            return Err("argument parsing failed".to_string());
        }

        Ok(())
    }

    pub fn stte_0(&mut self) {
        self.arc4.indx = 0;
        self.arc4.jndx = 0;
        self.arc4.kndx = 0;
        for i in 0..256 {
            self.arc4.stte[i] = i as u8;
        }
    }

    pub fn key(&mut self, data: &[u8]) {
        let mut len = data.len();
        let mut base = 0usize;

        while len > 0 {
            loop {
                let idx = self.arc4.indx as usize;
                let tmp = self.arc4.stte[idx];
                self.arc4.kndx = self.arc4.kndx.wrapping_add(tmp);
                self.arc4.kndx = self
                    .arc4
                    .kndx
                    .wrapping_add(data[base + (idx % len)]);
                let k = self.arc4.kndx as usize;
                self.arc4.stte[idx] = self.arc4.stte[k];
                self.arc4.stte[k] = tmp;

                self.arc4.indx = self.arc4.indx.wrapping_add(1);
                if self.arc4.indx == 0 {
                    break;
                }
            }
            base = base.saturating_add(256);
            len = len.saturating_sub(256);
        }
    }

    pub fn arc_4(&mut self, data: &mut [u8]) {
        for b in data.iter_mut() {
            self.arc4.indx = self.arc4.indx.wrapping_add(1);
            let idx = self.arc4.indx as usize;
            let mut tmp = self.arc4.stte[idx];
            self.arc4.jndx = self.arc4.jndx.wrapping_add(tmp);
            let j = self.arc4.jndx as usize;
            self.arc4.stte[idx] = self.arc4.stte[j];
            self.arc4.stte[j] = tmp;
            tmp = tmp.wrapping_add(self.arc4.stte[idx]);
            *b ^= self.arc4.stte[tmp as usize];
        }
    }

    pub fn key_with_file(&mut self, file: &str) -> Result<(), String> {
        let md = fs::metadata(file).map_err(|_| "stat failed".to_string())?;
        let mut control = Vec::new();

        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            control.extend_from_slice(&md.ino().to_ne_bytes());
            control.extend_from_slice(&md.dev().to_ne_bytes());
            control.extend_from_slice(&md.rdev().to_ne_bytes());
            control.extend_from_slice(&md.uid().to_ne_bytes());
            control.extend_from_slice(&md.gid().to_ne_bytes());
            control.extend_from_slice(&md.size().to_ne_bytes());
            control.extend_from_slice(&md.mtime().to_ne_bytes());
            control.extend_from_slice(&md.ctime().to_ne_bytes());
        }

        #[cfg(not(unix))]
        {
            control.extend_from_slice(&md.len().to_ne_bytes());
            if let Ok(modified) = md.modified() {
                if let Ok(dur) = modified.duration_since(UNIX_EPOCH) {
                    control.extend_from_slice(&dur.as_secs().to_ne_bytes());
                }
            }
            if let Ok(created) = md.created() {
                if let Ok(dur) = created.duration_since(UNIX_EPOCH) {
                    control.extend_from_slice(&dur.as_secs().to_ne_bytes());
                }
            }
        }

        self.key(&control);
        Ok(())
    }

    pub fn eval_shell(&mut self, text: &str) -> Result<(), String> {
        let first = text.lines().next().unwrap_or(text);
        let line = first.to_string();

        let rest = line
            .trim_start()
            .strip_prefix("#!")
            .ok_or_else(|| {
                eprintln!("{}: invalid first line in script: {}", self.my_name, line);
                "invalid first line".to_string()
            })?;

        let mut parts = rest.split_whitespace();
        let shll = parts.next().ok_or_else(|| {
            eprintln!("{}: invalid first line in script: {}", self.my_name, line);
            "invalid first line".to_string()
        })?;
        let opt = parts.next().unwrap_or("").to_string();

        let base = Path::new(shll)
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| {
                eprintln!("{}: invalid shll", self.my_name);
                "invalid shell".to_string()
            })?
            .to_string();

        self.shll = Some(shll.to_string());

        if self.verbose > 0 {
            eprintln!("{} shll={}", self.my_name, base);
        }

        for entry in self.shells_db {
            if entry.shll == base {
                if self.inlo.is_none() {
                    self.inlo = Some(entry.inlo.to_string());
                }
                if self.xecc.is_none() {
                    self.xecc = Some(entry.xecc.to_string());
                }
                if self.lsto.is_none() {
                    self.lsto = Some(entry.lsto.to_string());
                }
            }
        }

        if self.inlo.is_none() || self.xecc.is_none() || self.lsto.is_none() {
            eprintln!(
                "{} Unknown shell ({}): specify [-i][-x][-l]",
                self.my_name, base
            );
            return Err("unknown shell".to_string());
        }

        if self.verbose > 0 {
            eprintln!("{} [-i]={}", self.my_name, self.inlo.as_deref().unwrap_or(""));
            eprintln!("{} [-x]={}", self.my_name, self.xecc.as_deref().unwrap_or(""));
            eprintln!("{} [-l]={}", self.my_name, self.lsto.as_deref().unwrap_or(""));
        }

        self.opts = opt;
        if !self.opts.is_empty() && self.opts == self.lsto.as_deref().unwrap_or("") {
            eprintln!(
                "{} opts={} : Is equal to [-l]. Removing opts",
                self.my_name, self.opts
            );
            self.opts.clear();
        } else if self.opts == "-" {
            eprintln!(
                "{} opts={} : No real one. Removing opts",
                self.my_name, self.opts
            );
            self.opts.clear();
        }

        if self.verbose > 0 {
            eprintln!("{} opts={}", self.my_name, self.opts);
        }

        Ok(())
    }

    pub fn read_script(&self, file: &str) -> Result<String, String> {
        let text = fs::read_to_string(file).map_err(|_| "read failed".to_string())?;

        const ARG_MAX_FALLBACK: usize = 2_097_152;
        if (text.len() as f64) > 0.80 * (ARG_MAX_FALLBACK as f64) {
            eprintln!(
                "{}: WARNING!!\n   Scripts of length near to (or higher than) the current System limit on\n   \"maximum size of arguments to EXEC\", could comprise its binary execution.\n   In the current System the call sysconf(_SC_ARG_MAX) returns {} bytes\n   and your script \"{}\" is {} bytes length.\n",
                self.my_name,
                ARG_MAX_FALLBACK,
                file,
                text.len()
            );
        }

        Ok(text)
    }

    pub fn rand_mod(&mut self, modulus: usize) -> usize {
        if modulus == 0 {
            return 0;
        }

        let top = u64::MAX - (u64::MAX % modulus as u64);
        loop {
            self.rng_state = self
                .rng_state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1);
            let rnd = self.rng_state;
            if rnd < top {
                return ((modulus as u128 * rnd as u128) / (1u128 + top as u128)) as usize;
            }
        }
    }

    pub fn rand_chr(&mut self) -> char {
        char::from_u32(self.rand_mod(1 << 8) as u32).unwrap_or('\0')
    }

    pub fn noise(&mut self, ptr: &mut Vec<u8>, min: usize, xtra: usize, string_only: bool) -> usize {
        let mut total = min;
        if xtra != 0 {
            total += self.rand_mod(xtra);
        }

        ptr.clear();
        for _ in 0..total {
            let ch = loop {
                let c = self.rand_chr() as u8;
                if !string_only || (c as char).is_ascii_alphanumeric() {
                    break c;
                }
            };
            ptr.push(ch);
        }
        if string_only {
            ptr.push(0);
        }
        total
    }

    pub fn prnt_bytes(
        &mut self,
        out: &mut dyn Write,
        ptr: &[u8],
        m: usize,
        l: usize,
        n: usize,
    ) -> io::Result<()> {
        let l2 = l + m;
        let n2 = n + l2;
        let mut i = 0usize;
        while i < n2 {
            if (i & 0xf) == 0 {
                write!(out, "\n\t\"")?;
            }
            let byte = if i >= m && i < l2 {
                ptr[i - m]
            } else {
                self.rand_chr() as u8
            };
            write!(out, "\\{:03o}", byte)?;
            if (i & 0xf) == 0xf {
                write!(out, "\"")?;
            }
            i += 1;
        }
        if (i & 0xf) != 0 {
            write!(out, "\"")?;
        }
        self.offset += n2;
        Ok(())
    }

    pub fn prnt_array(
        &mut self,
        out: &mut dyn Write,
        ptr: &[u8],
        name: &str,
        l: usize,
        cast: Option<&str>,
    ) -> io::Result<()> {
        let mut m = self.rand_mod(1 + l / 4);
        let n = self.rand_mod(1 + l / 4);
        let a = (self.offset + m) % l.max(1);
        if cast.is_some() && a != 0 {
            m += l - a;
        }
        write!(out, "\n")?;
        write!(out, "#define      {}_z\t{}", name, l)?;
        write!(out, "\n")?;
        write!(
            out,
            "#define      {}\t({}(&data[{}]))",
            name,
            cast.unwrap_or(""),
            self.offset + m
        )?;
        self.prnt_bytes(out, ptr, m, l, n)
    }

    pub fn dump_array(
        &mut self,
        out: &mut dyn Write,
        ptr: &mut [u8],
        name: &str,
        l: usize,
        cast: Option<&str>,
    ) -> io::Result<()> {
        self.arc_4(ptr);
        self.prnt_array(out, ptr, name, l, cast)
    }

    pub fn write_c(&mut self, file: &str, argv: &[String]) -> Result<(), String> {
        let mut pswd = vec![0u8; 256];
        let mut pswd_z = pswd.len() as isize;
        let mut msg1 = b"has expired!\n".to_vec();
        let mut date = self.date.clone().into_bytes();
        date.push(0);
        let kwsh_string = self.shll.clone().unwrap_or_default();
        let mut shll = kwsh_string.clone().into_bytes();
        shll.push(0);
        let mut inlo = self.inlo.clone().unwrap_or_default().into_bytes();
        inlo.push(0);
        let mut xecc = self.xecc.clone().unwrap_or_default().into_bytes();
        xecc.push(0);
        let mut lsto = self.lsto.clone().unwrap_or_default().into_bytes();
        lsto.push(0);
        let mut tst1 = b"location has changed!".to_vec();
        tst1.push(0);
        let mut chk1 = tst1.clone();
        let mut msg2 = b"abnormal behavior!".to_vec();
        msg2.push(0);
        let mut rlax = self.rlax.clone();
        let mut opts = self.opts.clone().into_bytes();
        opts.push(0);
        let script_text = self.text.clone().unwrap_or_default();
        let mut text = script_text.clone().into_bytes();
        text.push(0);
        let mut tst2 = b"shell has changed!".to_vec();
        tst2.push(0);
        let mut chk2 = tst2.clone();

        let mut numd = 0isize;
        self.seed_rng();

        pswd_z = self.noise(&mut pswd, pswd_z as usize, 0, false) as isize;
        numd += 1;

        self.stte_0();
        self.key(&pswd);

        msg1.extend_from_slice(self.mail.as_bytes());
        msg1.push(0);
        self.arc_4(&mut msg1);
        numd += 1;

        self.arc_4(&mut date);
        numd += 1;
        self.arc_4(&mut shll);
        numd += 1;
        self.arc_4(&mut inlo);
        numd += 1;
        self.arc_4(&mut xecc);
        numd += 1;
        self.arc_4(&mut lsto);
        numd += 1;
        self.arc_4(&mut tst1);
        numd += 1;

        self.key(&chk1);
        self.arc_4(&mut chk1);
        numd += 1;

        self.arc_4(&mut msg2);
        numd += 1;

        let indx = self.rlax.first().copied().unwrap_or(0) == 0;
        self.arc_4(&mut rlax);
        numd += 1;

        if indx && self.key_with_file(&kwsh_string).is_err() {
            eprintln!("{}: invalid file name: {} ", self.my_name, kwsh_string);
            return Err("invalid file name".to_string());
        }

        self.arc_4(&mut opts);
        numd += 1;
        self.arc_4(&mut text);
        numd += 1;
        self.arc_4(&mut tst2);
        numd += 1;

        self.key(&chk2);
        self.arc_4(&mut chk2);
        numd += 1;

        let name = format!("{}.x.c", file);
        let mut o = fs::File::create(&name).map_err(|e| {
            eprintln!("{}: creating output file: {} {}", self.my_name, name, e);
            "create output failed".to_string()
        })?;

        write!(o, "#if 0\n").map_err(|e| e.to_string())?;
        write!(o, "\t{} {}, {}\n", self.my_name, self.version, self.subject).map_err(|e| e.to_string())?;
        write!(
            o,
            "\t{} {} {} {}\n\n\t",
            self.cpright, self.provider.f, self.provider.s, self.provider.e
        )
        .map_err(|e| e.to_string())?;
        for arg in argv {
            write!(o, "{} ", arg).map_err(|e| e.to_string())?;
        }
        write!(o, "\n#endif\n\n").map_err(|e| e.to_string())?;
        write!(o, "static  char data [] = ").map_err(|e| e.to_string())?;

        self.offset = 0;

        let mut entries: Vec<(&str, Option<&str>, Vec<u8>, isize)> = vec![
            ("pswd", None, pswd, pswd_z),
            ("msg1", None, msg1.clone(), msg1.len() as isize),
            ("date", None, date.clone(), date.len() as isize),
            ("shll", None, shll.clone(), shll.len() as isize),
            ("inlo", None, inlo.clone(), inlo.len() as isize),
            ("xecc", None, xecc.clone(), xecc.len() as isize),
            ("lsto", None, lsto.clone(), lsto.len() as isize),
            ("tst1", None, tst1.clone(), tst1.len() as isize),
            ("chk1", None, chk1.clone(), chk1.len() as isize),
            ("msg2", None, msg2.clone(), msg2.len() as isize),
            ("rlax", None, rlax.clone(), rlax.len() as isize),
            ("opts", None, opts.clone(), opts.len() as isize),
            ("text", None, text.clone(), text.len() as isize),
            ("tst2", None, tst2.clone(), tst2.len() as isize),
            ("chk2", None, chk2.clone(), chk2.len() as isize),
        ];

        while numd > 0 {
            let mut done = 0isize;
            let start = self.rand_mod(15);
            let mut idx = start;
            while done == 0 {
                if entries[idx].3 >= 0 {
                    let (name_ref, cast, buf, status) = &mut entries[idx];
                    let len = buf.len();
                    self.prnt_array(&mut o, buf, name_ref, len, *cast)
                        .map_err(|e| e.to_string())?;
                    *status = -1;
                    done = -1;
                }
                idx = (idx + 1) % 15;
            }
            numd += done;
        }

        write!(o, "/* End of data[] */;\n").map_err(|e| e.to_string())?;
        write!(o, "#define      {}\t{}\n", "hide", 1 << 12).map_err(|e| e.to_string())?;
        write!(o, "{}", self.setuid_line.replacen("%d", &self.setuid_flag.to_string(), 1))
            .map_err(|e| e.to_string())?;
        write!(
            o,
            "{}",
            self.debugexec_line
                .replacen("%d", &self.debugexec_flag.to_string(), 1)
        )
        .map_err(|e| e.to_string())?;
        write!(
            o,
            "{}",
            self.traceable_line
                .replacen("%d", &self.traceable_flag.to_string(), 1)
        )
        .map_err(|e| e.to_string())?;
        write!(
            o,
            "{}",
            self.hardening_line
                .replacen("%d", &self.hardening_flag.to_string(), 1)
        )
        .map_err(|e| e.to_string())?;
        write!(
            o,
            "{}",
            self.busyboxon_line
                .replacen("%d", &self.busyboxon_flag.to_string(), 1)
        )
        .map_err(|e| e.to_string())?;
        write!(
            o,
            "{}",
            self.mmap2_line.replacen("%d", &self.mmap2_flag.to_string(), 1)
        )
        .map_err(|e| e.to_string())?;

        let esc = c_string_literal(&script_text);
        let shell = c_string_literal(self.shll.as_deref().unwrap_or(""));
        writeln!(o, "#include <stdio.h>").map_err(|e| e.to_string())?;
        writeln!(o, "#include <stdlib.h>").map_err(|e| e.to_string())?;
        writeln!(o, "#include <string.h>").map_err(|e| e.to_string())?;
        writeln!(o, "#include <unistd.h>").map_err(|e| e.to_string())?;
        writeln!(o, "#include <fcntl.h>").map_err(|e| e.to_string())?;
        writeln!(o, "#include <sys/stat.h>").map_err(|e| e.to_string())?;
        writeln!(o, "#include <sys/wait.h>").map_err(|e| e.to_string())?;
        writeln!(o, "static const char embedded_script[] = {};", esc).map_err(|e| e.to_string())?;
        writeln!(o, "static const char shell_path[] = {};", shell).map_err(|e| e.to_string())?;
        writeln!(o, "int main(int argc, char **argv) {{").map_err(|e| e.to_string())?;
        writeln!(o, "    char tmpl[] = \"/tmp/shcXXXXXX\";").map_err(|e| e.to_string())?;
        writeln!(o, "    int fd;").map_err(|e| e.to_string())?;
        writeln!(o, "    size_t len;").map_err(|e| e.to_string())?;
        writeln!(o, "    char **nargv;").map_err(|e| e.to_string())?;
        writeln!(o, "    int i;").map_err(|e| e.to_string())?;
        writeln!(o, "    pid_t pid;").map_err(|e| e.to_string())?;
        writeln!(o, "    int status = 0;").map_err(|e| e.to_string())?;
        writeln!(o, "    fd = mkstemp(tmpl);").map_err(|e| e.to_string())?;
        writeln!(o, "    if (fd < 0) {{ perror(\"mkstemp\"); return 1; }}").map_err(|e| e.to_string())?;
        writeln!(o, "    len = strlen(embedded_script);").map_err(|e| e.to_string())?;
        writeln!(o, "    if (write(fd, embedded_script, len) != (ssize_t)len) {{ perror(\"write\"); close(fd); unlink(tmpl); return 1; }}").map_err(|e| e.to_string())?;
        writeln!(o, "    close(fd);").map_err(|e| e.to_string())?;
        writeln!(o, "    chmod(tmpl, 0700);").map_err(|e| e.to_string())?;
        writeln!(o, "    nargv = calloc((size_t)argc + 2, sizeof(char*));").map_err(|e| e.to_string())?;
        writeln!(o, "    if (!nargv) {{ perror(\"calloc\"); unlink(tmpl); return 1; }}").map_err(|e| e.to_string())?;
        writeln!(o, "    nargv[0] = (char*)shell_path;").map_err(|e| e.to_string())?;
        writeln!(o, "    nargv[1] = tmpl;").map_err(|e| e.to_string())?;
        writeln!(o, "    for (i = 1; i < argc; ++i) nargv[i + 1] = argv[i];").map_err(|e| e.to_string())?;
        writeln!(o, "    pid = fork();").map_err(|e| e.to_string())?;
        writeln!(o, "    if (pid < 0) {{ perror(\"fork\"); unlink(tmpl); return 1; }}").map_err(|e| e.to_string())?;
        writeln!(o, "    if (pid == 0) {{ execv(shell_path, nargv); perror(shell_path); _exit(127); }}").map_err(|e| e.to_string())?;
        writeln!(o, "    if (waitpid(pid, &status, 0) < 0) {{ perror(\"waitpid\"); unlink(tmpl); return 1; }}").map_err(|e| e.to_string())?;
        writeln!(o, "    unlink(tmpl);").map_err(|e| e.to_string())?;
        writeln!(o, "    if (WIFEXITED(status)) return WEXITSTATUS(status);").map_err(|e| e.to_string())?;
        writeln!(o, "    return 1;").map_err(|e| e.to_string())?;
        writeln!(o, "}}").map_err(|e| e.to_string())?;

        o.flush().map_err(|e| e.to_string())?;
        return Ok(());
        o.flush().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn make(&mut self) -> Result<(), String> {
        let cc = env::var("CC").unwrap_or_else(|_| "cc".to_string());
        let cflags = env::var("CFLAGS").unwrap_or_default();
        let ldflags = env::var("LDFLAGS").unwrap_or_default();

        let file = self.file.clone().unwrap_or_default();
        if self.file2.is_none() {
            self.file2 = Some(format!("{}.x", file));
        }
        let out = self.file2.clone().unwrap_or_default();

        let cmd = format!("{} {} {} {}.x.c -o {}", cc, cflags, ldflags, file, out);
        if self.verbose > 0 {
            eprintln!("{}: {}", self.my_name, cmd);
        }
        let status = Command::new("/bin/sh")
            .arg("-c")
            .arg(&cmd)
            .status()
            .map_err(|e| e.to_string())?;
        if !status.success() {
            return Err("compile failed".to_string());
        }

        let strip = env::var("STRIP").unwrap_or_else(|_| "strip".to_string());
        let strip_cmd = format!("{} {}", strip, out);
        if self.verbose > 0 {
            eprintln!("{}: {}", self.my_name, strip_cmd);
        }
        match Command::new("/bin/sh").arg("-c").arg(&strip_cmd).status() {
            Ok(status) if status.success() => {}
            _ => eprintln!("{}: never mind", self.my_name),
        }

        let chmod_cmd = format!("chmod ug=rwx,o=rx {}", out);
        if self.verbose > 0 {
            eprintln!("{}: {}", self.my_name, chmod_cmd);
        }
        match Command::new("/bin/sh").arg("-c").arg(&chmod_cmd).status() {
            Ok(status) if status.success() => {}
            _ => eprintln!("{}: remove read permission", self.my_name),
        }

        Ok(())
    }

    pub fn do_all(&mut self, args: &[String]) -> Result<(), String> {
        self.parse_args(args)?;
        let file = self.file.clone().unwrap_or_default();
        let text = self.read_script(&file)?;
        self.text = Some(text.clone());
        self.eval_shell(&text)?;
        self.write_c(&file, args)?;
        self.make()?;
        Ok(())
    }

    pub fn main(args: &[String]) -> i32 {
        let mut shc = Self::module_src();
        let _ = PathBuf::from("");
        match shc.do_all(args) {
            Ok(()) => 0,
            Err(e) if e == "exit 0" => 0,
            Err(_) => {
                if let Some(argv0) = args.first() {
                    eprintln!("{}: Success", argv0);
                }
                1
            }
        }
    }

    fn parse_expiration(s: &str) -> Option<i64> {
        let mut parts = s.split('/');
        let day: i32 = parts.next()?.parse().ok()?;
        let month: i32 = parts.next()?.parse().ok()?;
        let year: i32 = parts.next()?.parse().ok()?;
        if parts.next().is_some() {
            return None;
        }
        if !(1..=31).contains(&day) || !(1..=12).contains(&month) || year <= 1900 {
            return None;
        }
        Some(days_from_civil(year, month as u32, day as u32) * 86_400)
    }

    fn seed_rng(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        self.rng_state ^= now.as_nanos() as u64;
        self.rng_state ^= std::process::id() as u64;
    }
}

fn split_shell_like(s: &str) -> Vec<String> {
    s.split_whitespace().map(ToString::to_string).collect()
}

fn c_string_literal(s: &str) -> String {
    let mut out = String::from("\"");
    for b in s.as_bytes() {
        match *b {
            b'\\' => out.push_str("\\\\"),
            b'\"' => out.push_str("\\\""),
            b'\n' => out.push_str("\\n"),
            b'\r' => out.push_str("\\r"),
            b'\t' => out.push_str("\\t"),
            0x20..=0x7e => out.push(*b as char),
            _ => {
                let _ = std::fmt::Write::write_fmt(&mut out, format_args!("\\{:03o}", *b));
            }
        }
    }
    out.push('"');
    out
}

fn days_from_civil(year: i32, month: u32, day: u32) -> i64 {
    let y = year - ((month <= 2) as i32);
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let m = month as i32;
    let doy = (153 * (m + if m > 2 { -3 } else { 9 }) + 2) / 5 + day as i32 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    (era * 146097 + doe - 719468) as i64
}
