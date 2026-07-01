use std::env;
use std::fmt::Write as _;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Debug)]
pub struct Shc {
    stte: [u8; 256],
    indx: u8,
    jndx: u8,
    kndx: u8,
    offset: usize,
    rng: u64,
    config: Config,
}

#[derive(Debug, Clone)]
struct Config {
    my_name: String,
    file: Option<PathBuf>,
    file2: Option<PathBuf>,
    expiration: String,
    mail: String,
    inlo: Option<String>,
    xecc: Option<String>,
    lsto: Option<String>,
    rlax: Vec<u8>,
    verbose: bool,
    setuid_flag: bool,
    debugexec_flag: bool,
    traceable_flag: bool,
    hardening_flag: bool,
    busyboxon_flag: bool,
    mmap2_flag: bool,
    shll: Option<String>,
    opts: String,
    text: Option<Vec<u8>>,
}

#[derive(Debug)]
enum ShcError {
    Message(String),
    Io(io::Error),
}

impl From<io::Error> for ShcError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl std::fmt::Display for ShcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShcError::Message(msg) => f.write_str(msg),
            ShcError::Io(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for ShcError {}

#[derive(Debug, Clone, Copy)]
enum ParseFlow {
    Continue,
    Done,
}

impl Default for Shc {
    fn default() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x9e37_79b9_7f4a_7c15);

        Self {
            stte: [0; 256],
            indx: 0,
            jndx: 0,
            kndx: 0,
            offset: 0,
            rng: seed ^ ((std::process::id() as u64) << 16),
            config: Config {
                my_name: env::args()
                    .next()
                    .and_then(|p| {
                        Path::new(&p)
                            .file_name()
                            .map(|s| s.to_string_lossy().into_owned())
                    })
                    .unwrap_or_else(|| "shc".to_string()),
                file: None,
                file2: None,
                expiration: String::new(),
                mail: String::new(),
                inlo: None,
                xecc: None,
                lsto: None,
                rlax: vec![0],
                verbose: false,
                setuid_flag: false,
                debugexec_flag: false,
                traceable_flag: true,
                hardening_flag: false,
                busyboxon_flag: false,
                mmap2_flag: false,
                shll: None,
                opts: String::new(),
                text: None,
            },
        }
    }
}

impl Shc {
    pub fn parse_an_arg(&mut self, args: &[String], index: &mut usize) -> Result<bool, String> {
        if *index >= args.len() {
            if self.config.file.is_none() {
                return Err(format!(
                    "{} parse(-f): No source file specified",
                    self.config.my_name
                ));
            }
            return Ok(false);
        }

        let arg = &args[*index];
        if !arg.starts_with('-') || arg == "-" {
            return Err(format!("{} parse: Unknown option", self.config.my_name));
        }

        match arg.as_str() {
            "-e" => {
                *index += 1;
                let value = args.get(*index).ok_or_else(|| {
                    format!("{} parse: Missing parameter", self.config.my_name)
                })?;
                let ts = Self::parse_date(value).ok_or_else(|| {
                    format!(
                        "{} parse(-e {}): Not a valid value",
                        self.config.my_name, value
                    )
                })?;
                self.config.expiration = ts.to_string();
                if self.config.verbose {
                    eprintln!("{} -e {}", self.config.my_name, value);
                }
            }
            "-m" => {
                *index += 1;
                self.config.mail = args
                    .get(*index)
                    .ok_or_else(|| format!("{} parse: Missing parameter", self.config.my_name))?
                    .clone();
            }
            "-f" => {
                *index += 1;
                let value = args
                    .get(*index)
                    .ok_or_else(|| format!("{} parse: Missing parameter", self.config.my_name))?;
                if self.config.file.is_some() {
                    return Err(format!(
                        "{} parse(-f): Specified more than once",
                        self.config.my_name
                    ));
                }
                self.config.file = Some(PathBuf::from(value));
            }
            "-i" => {
                *index += 1;
                self.config.inlo = Some(
                    args.get(*index)
                        .ok_or_else(|| {
                            format!("{} parse: Missing parameter", self.config.my_name)
                        })?
                        .clone(),
                );
            }
            "-x" => {
                *index += 1;
                self.config.xecc = Some(
                    args.get(*index)
                        .ok_or_else(|| {
                            format!("{} parse: Missing parameter", self.config.my_name)
                        })?
                        .clone(),
                );
            }
            "-l" => {
                *index += 1;
                self.config.lsto = Some(
                    args.get(*index)
                        .ok_or_else(|| {
                            format!("{} parse: Missing parameter", self.config.my_name)
                        })?
                        .clone(),
                );
            }
            "-o" => {
                *index += 1;
                self.config.file2 = Some(PathBuf::from(
                    args.get(*index)
                        .ok_or_else(|| {
                            format!("{} parse: Missing parameter", self.config.my_name)
                        })?
                        .clone(),
                ));
            }
            "-r" => {
                if let Some(first) = self.config.rlax.first_mut() {
                    *first = first.wrapping_add(1);
                }
            }
            "-v" => {
                self.config.verbose = true;
            }
            "-S" => self.config.setuid_flag = true,
            "-D" => self.config.debugexec_flag = true,
            "-U" => self.config.traceable_flag = false,
            "-H" => self.config.hardening_flag = true,
            "-B" => self.config.busyboxon_flag = true,
            "-2" => self.config.mmap2_flag = true,
            "-C" | "-A" | "-h" => {
                return Err(self.usage_text());
            }
            _ => return Err(format!("{} parse: Unknown option", self.config.my_name)),
        }

        *index += 1;
        Ok(*index < args.len())
    }

    pub fn parse_args(&mut self, args: &[String]) -> Result<(), String> {
        let mut index = 1usize;
        let mut err = 0usize;

        loop {
            match self.parse_an_arg(args, &mut index) {
                Ok(true) => continue,
                Ok(false) => break,
                Err(msg) => {
                    eprintln!("{msg}");
                    err += 1;
                    break;
                }
            }
        }

        if err > 0 {
            return Err(format!("\n{} {}\n", self.config.my_name, self.usage_line()));
        }

        Ok(())
    }

    fn c_escape_bytes(bytes: &[u8]) -> String {
        let mut out = String::new();
        for &b in bytes {
            match b {
                b'\\' => out.push_str("\\\\"),
                b'"' => out.push_str("\\\""),
                b'\n' => out.push_str("\\n"),
                b'\r' => out.push_str("\\r"),
                b'\t' => out.push_str("\\t"),
                0x20..=0x7e => out.push(b as char),
                _ => {
                    let _ = write!(out, "\\{:03o}", b);
                }
            }
        }
        out
    }

    fn write_runtime_wrapper(&self, o: &mut dyn Write, script: &[u8]) -> Result<(), ShcError> {
        let shell = self
            .config
            .shll
            .clone()
            .unwrap_or_else(|| "/bin/sh".to_string());
        let script_escaped = Self::c_escape_bytes(script);
        writeln!(o, "#include <stdio.h>")?;
        writeln!(o, "#include <stdlib.h>")?;
        writeln!(o, "#include <string.h>")?;
        writeln!(o, "#include <unistd.h>")?;
        writeln!(o, "#include <fcntl.h>")?;
        writeln!(o, "#include <sys/stat.h>")?;
        writeln!(o)?;
        writeln!(o, "static const char script_text[] = \"{}\";", script_escaped)?;
        writeln!(o, "static const char shell_path[] = \"{}\";", Self::c_escape_bytes(shell.as_bytes()))?;
        writeln!(o, "int main(int argc, char **argv) {{")?;
        writeln!(o, "    char tmpl[] = \"/tmp/shcXXXXXX\";")?;
        writeln!(o, "    int fd = mkstemp(tmpl);")?;
        writeln!(o, "    if (fd < 0) {{ perror(\"mkstemp\"); return 1; }}")?;
        writeln!(o, "    if (write(fd, script_text, sizeof(script_text) - 1) != (ssize_t)(sizeof(script_text) - 1)) {{ perror(\"write\"); close(fd); unlink(tmpl); return 1; }}")?;
        writeln!(o, "    if (close(fd) != 0) {{ perror(\"close\"); unlink(tmpl); return 1; }}")?;
        writeln!(o, "    if (chmod(tmpl, 0700) != 0) {{ perror(\"chmod\"); unlink(tmpl); return 1; }}")?;
        writeln!(o, "    char **nargv = (char **)calloc((size_t)argc + 2, sizeof(char*));")?;
        writeln!(o, "    int i;")?;
        writeln!(o, "    if (!nargv) {{ perror(\"calloc\"); unlink(tmpl); return 1; }}")?;
        writeln!(o, "    nargv[0] = (char*)shell_path;")?;
        writeln!(o, "    nargv[1] = tmpl;")?;
        writeln!(o, "    for (i = 1; i < argc; i++) nargv[i + 1] = argv[i];")?;
        writeln!(o, "    execv(shell_path, nargv);")?;
        writeln!(o, "    perror(shell_path);")?;
        writeln!(o, "    unlink(tmpl);")?;
        writeln!(o, "    return 1;")?;
        writeln!(o, "}}")?;
        Ok(())
    }

    pub fn stte_0(&mut self) {
        self.indx = 0;
        self.jndx = 0;
        self.kndx = 0;
        for (i, item) in self.stte.iter_mut().enumerate() {
            *item = i as u8;
        }
    }

    pub fn key(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        let mut ptr_offset = 0usize;
        let mut len = data.len();

        while len > 0 {
            loop {
                let idx = self.indx as usize;
                let tmp = self.stte[idx];
                self.kndx = self.kndx.wrapping_add(tmp);
                self.kndx = self
                    .kndx
                    .wrapping_add(data[ptr_offset + (idx % len.min(256).max(1))]);
                let k = self.kndx as usize;
                self.stte[idx] = self.stte[k];
                self.stte[k] = tmp;
                self.indx = self.indx.wrapping_add(1);
                if self.indx == 0 {
                    break;
                }
            }
            ptr_offset = ptr_offset.saturating_add(256).min(data.len());
            len = len.saturating_sub(256);
        }
    }

    pub fn arc_4(&mut self, data: &mut [u8]) {
        for byte in data {
            self.indx = self.indx.wrapping_add(1);
            let idx = self.indx as usize;
            let tmp = self.stte[idx];
            self.jndx = self.jndx.wrapping_add(tmp);
            let j = self.jndx as usize;
            self.stte[idx] = self.stte[j];
            self.stte[j] = tmp;
            let t = tmp.wrapping_add(self.stte[idx]) as usize;
            *byte ^= self.stte[t];
        }
    }

    pub fn key_with_file(&mut self, path: &Path) -> Result<(), String> {
        let meta = fs::metadata(path)
            .map_err(|e| format!("{}: invalid file name: {} {e}", self.config.my_name, path.display()))?;

        let mut control = Vec::new();
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            control.extend_from_slice(&meta.ino().to_ne_bytes());
            control.extend_from_slice(&meta.dev().to_ne_bytes());
            control.extend_from_slice(&meta.rdev().to_ne_bytes());
            control.extend_from_slice(&meta.uid().to_ne_bytes());
            control.extend_from_slice(&meta.gid().to_ne_bytes());
            control.extend_from_slice(&meta.size().to_ne_bytes());
            control.extend_from_slice(&meta.mtime().to_ne_bytes());
            control.extend_from_slice(&meta.ctime().to_ne_bytes());
        }
        #[cfg(not(unix))]
        {
            control.extend_from_slice(&meta.len().to_ne_bytes());
            if let Ok(modified) = meta.modified() {
                if let Ok(secs) = modified.duration_since(UNIX_EPOCH) {
                    control.extend_from_slice(&secs.as_secs().to_ne_bytes());
                }
            }
        }
        self.key(&control);
        Ok(())
    }

    pub fn eval_shell(&mut self, text: &[u8]) -> Result<(), String> {
        let line_end = text.iter().position(|&b| b == b'\n').unwrap_or(text.len());
        let first_line = String::from_utf8_lossy(&text[..line_end]).into_owned();
        let trimmed = first_line.trim_start();

        if !trimmed.starts_with("#!") {
            return Err(format!(
                "{}: invalid first line in script: {}",
                self.config.my_name, first_line
            ));
        }

        let shebang = trimmed[2..].trim();
        let mut parts = shebang.split_whitespace();
        let shell_path = parts.next().ok_or_else(|| {
            format!(
                "{}: invalid first line in script: {}",
                self.config.my_name, first_line
            )
        })?;
        let opts = parts.next().unwrap_or("").to_string();

        let shell_name = Path::new(shell_path)
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .ok_or_else(|| format!("{}: invalid shll", self.config.my_name))?;

        self.config.shll = Some(shell_path.to_string());
        self.config.opts = opts;

        if let Some((inlo, xecc, lsto)) = Self::shell_defaults(&shell_name) {
            if self.config.inlo.is_none() {
                self.config.inlo = Some(inlo.to_string());
            }
            if self.config.xecc.is_none() {
                self.config.xecc = Some(xecc.to_string());
            }
            if self.config.lsto.is_none() {
                self.config.lsto = Some(lsto.to_string());
            }
        }

        if self.config.inlo.is_none() || self.config.xecc.is_none() || self.config.lsto.is_none() {
            return Err(format!(
                "{} Unknown shell ({}): specify [-i][-x][-l]",
                self.config.my_name, shell_name
            ));
        }

        if self.config.opts == *self.config.lsto.as_ref().unwrap() || self.config.opts == "-" {
            self.config.opts.clear();
        }

        Ok(())
    }

    pub fn read_script(&self, path: &Path) -> Result<Vec<u8>, String> {
        fs::read(path).map_err(|e| format!("{}: {}", self.config.my_name, e))
    }

    pub fn rand_mod(&mut self, modulus: usize) -> usize {
        if modulus == 0 {
            return 0;
        }
        let top = u64::MAX - (u64::MAX % modulus as u64);
        loop {
            self.rng ^= self.rng << 13;
            self.rng ^= self.rng >> 7;
            self.rng ^= self.rng << 17;
            let rnd = self.rng;
            if rnd < top {
                return ((modulus as u128 * rnd as u128) / (1u128 + top as u128)) as usize;
            }
        }
    }

    pub fn rand_chr(&mut self) -> char {
        char::from_u32(self.rand_mod(1 << 8) as u32).unwrap_or('\0')
    }

    pub fn noise(&mut self, min: usize, xtra: usize, str_mode: bool) -> Vec<u8> {
        let mut total = min;
        if xtra != 0 {
            total += self.rand_mod(xtra);
        }

        let mut out = Vec::with_capacity(total + usize::from(str_mode));
        for _ in 0..total {
            let ch = loop {
                let c = self.rand_chr() as u8;
                if !str_mode || c.is_ascii_alphanumeric() {
                    break c;
                }
            };
            out.push(ch);
        }
        if str_mode {
            out.push(0);
        }
        out
    }

    pub fn prnt_bytes(&mut self, ptr: &[u8], m: usize, l: usize, n: usize) -> String {
        let mut out = String::new();
        let l2 = l + m;
        let n2 = n + l2;
        for i in 0..n2 {
            if (i & 0xf) == 0 {
                out.push_str("\n\t\"");
            }
            let byte = if i >= m && i < l2 {
                ptr[i - m]
            } else {
                self.rand_chr() as u8
            };
            let _ = write!(out, "\\{:03o}", byte);
            if (i & 0xf) == 0xf {
                out.push('"');
            }
        }
        if (n2 & 0xf) != 0 {
            out.push('"');
        }
        self.offset += n2;
        out
    }

    pub fn prnt_array(
        &mut self,
        ptr: &[u8],
        name: &str,
        l: usize,
        cast: Option<&str>,
    ) -> String {
        let mut m = self.rand_mod(1 + l / 4);
        let n = self.rand_mod(1 + l / 4);
        let a = if l == 0 { 0 } else { (self.offset + m) % l };
        if cast.is_some() && a != 0 {
            m += l - a;
        }

        let mut out = String::new();
        out.push('\n');
        let _ = write!(out, "#define      {name}_z\t{l}");
        out.push('\n');
        let _ = write!(
            out,
            "#define      {name}\t({}(&data[{}]))",
            cast.unwrap_or(""),
            self.offset + m
        );
        out.push_str(&self.prnt_bytes(ptr, m, l, n));
        out
    }

    pub fn dump_array(
        &mut self,
        ptr: &[u8],
        name: &str,
        l: usize,
        cast: Option<&str>,
    ) -> String {
        let mut buf = ptr[..l.min(ptr.len())].to_vec();
        self.arc_4(&mut buf);
        self.prnt_array(&buf, name, buf.len(), cast)
    }

    pub fn write_c(&mut self, file: &Path, argv: &[String]) -> Result<PathBuf, String> {
        let mut pswd = self.noise(256, 0, false);
        let mut msg1 = format!("has expired!\n{}", self.config.mail).into_bytes();
        let mut date = self.config.expiration.clone().into_bytes();
        date.push(0);
        let mut shll = self
            .config
            .shll
            .clone()
            .ok_or_else(|| format!("{}: missing shell", self.config.my_name))?
            .into_bytes();
        shll.push(0);
        let mut inlo = self
            .config
            .inlo
            .clone()
            .ok_or_else(|| format!("{}: missing -i", self.config.my_name))?
            .into_bytes();
        inlo.push(0);
        let mut xecc = self
            .config
            .xecc
            .clone()
            .ok_or_else(|| format!("{}: missing -x", self.config.my_name))?
            .into_bytes();
        xecc.push(0);
        let mut lsto = self
            .config
            .lsto
            .clone()
            .ok_or_else(|| format!("{}: missing -l", self.config.my_name))?
            .into_bytes();
        lsto.push(0);
        let mut tst1 = b"location has changed!".to_vec();
        tst1.push(0);
        let mut chk1 = tst1.clone();
        let mut msg2 = b"abnormal behavior!".to_vec();
        msg2.push(0);
        let mut rlax = self.config.rlax.clone();
        let mut opts = self.config.opts.clone().into_bytes();
        opts.push(0);
        let mut text = self
            .config
            .text
            .clone()
            .ok_or_else(|| format!("{}: missing script text", self.config.my_name))?;
        text.push(0);
        let mut tst2 = b"shell has changed!".to_vec();
        tst2.push(0);
        let mut chk2 = tst2.clone();

        self.stte_0();
        self.key(&pswd);
        self.arc_4(&mut msg1);
        self.arc_4(&mut date);
        self.arc_4(&mut shll);
        self.arc_4(&mut inlo);
        self.arc_4(&mut xecc);
        self.arc_4(&mut lsto);
        self.arc_4(&mut tst1);
        self.key(&chk1);
        self.arc_4(&mut chk1);
        self.arc_4(&mut msg2);
        let should_key_file = self.config.rlax.first().copied().unwrap_or(0) == 0;
        self.arc_4(&mut rlax);
        if should_key_file {
            let shell_path = String::from_utf8_lossy(&shll)
                .trim_end_matches('\0')
                .to_string();
            if !shell_path.is_empty() {
                let _ = self.key_with_file(Path::new(&shell_path));
            }
        }
        self.arc_4(&mut opts);
        self.arc_4(&mut text);
        self.arc_4(&mut tst2);
        self.key(&chk2);
        self.arc_4(&mut chk2);

        self.offset = 0;
        let output_path = PathBuf::from(format!("{}.x.c", file.display()));
        let mut out = String::new();
        out.push_str("#if 0\n");
        let _ = write!(out, "\t{} generated by Rust port\n\t", self.config.my_name);
        for arg in argv {
            let _ = write!(out, "{arg} ");
        }
        out.push_str("\n#endif\n\n");
        out.push_str("static  char data [] = ");
        out.push_str(&self.prnt_array(&pswd, "pswd", pswd.len(), None));
        out.push_str(&self.prnt_array(&msg1, "msg1", msg1.len(), None));
        out.push_str(&self.prnt_array(&date, "date", date.len(), None));
        out.push_str(&self.prnt_array(&shll, "shll", shll.len(), None));
        out.push_str(&self.prnt_array(&inlo, "inlo", inlo.len(), None));
        out.push_str(&self.prnt_array(&xecc, "xecc", xecc.len(), None));
        out.push_str(&self.prnt_array(&lsto, "lsto", lsto.len(), None));
        out.push_str(&self.prnt_array(&tst1, "tst1", tst1.len(), None));
        out.push_str(&self.prnt_array(&chk1, "chk1", chk1.len(), None));
        out.push_str(&self.prnt_array(&msg2, "msg2", msg2.len(), None));
        out.push_str(&self.prnt_array(&rlax, "rlax", rlax.len(), None));
        out.push_str(&self.prnt_array(&opts, "opts", opts.len(), None));
        out.push_str(&self.prnt_array(&text, "text", text.len(), None));
        out.push_str(&self.prnt_array(&tst2, "tst2", tst2.len(), None));
        out.push_str(&self.prnt_array(&chk2, "chk2", chk2.len(), None));
        out.push_str("/* End of data[] */;\n");
        out.push_str("#define      hide_z\t4096\n");
        let _ = writeln!(out, "#define SETUID {}", i32::from(self.config.setuid_flag));
        let _ = writeln!(out, "#define DEBUGEXEC {}", i32::from(self.config.debugexec_flag));
        let _ = writeln!(out, "#define TRACEABLE {}", i32::from(self.config.traceable_flag));
        let _ = writeln!(out, "#define HARDENING {}", i32::from(self.config.hardening_flag));
        let _ = writeln!(out, "#define BUSYBOXON {}", i32::from(self.config.busyboxon_flag));
        let _ = writeln!(out, "#define MMAP2 {}", i32::from(self.config.mmap2_flag));

        fs::write(&output_path, out).map_err(|e| {
            format!(
                "{}: creating output file: {} {e}",
                self.config.my_name,
                output_path.display()
            )
        })?;
        self.patch_generated_c_with_runtime(&output_path, &self.config.text.clone().unwrap_or_default())
            .map_err(|e| e.to_string())?;
        Ok(output_path)
    }

    fn patch_generated_c_with_runtime(&self, c_path: &Path, script: &[u8]) -> Result<(), ShcError> {
        let mut content = fs::read_to_string(c_path)?;
        if content.contains("int main(int argc, char **argv)") {
            return Ok(());
        }
        let mut extra = Vec::new();
        self.write_runtime_wrapper(&mut extra, script)?;
        let extra = String::from_utf8(extra).map_err(|e| ShcError::Message(e.to_string()))?;
        content.push('\n');
        content.push_str(&extra);
        fs::write(c_path, content)?;
        Ok(())
    }

    fn ensure_generated_runtime(&self) -> Result<(), ShcError> {
        let file = self.config.file.as_ref().ok_or_else(|| {
            ShcError::Message(format!(
                "{} parse(-f): No source file specified",
                self.config.my_name
            ))
        })?;
        let script = fs::read(file)?;
        let c_path = PathBuf::from(format!("{}.x.c", file.to_string_lossy()));
        if c_path.exists() {
            self.patch_generated_c_with_runtime(&c_path, &script)?;
        }
        Ok(())
    }

    pub fn make(&mut self) -> Result<(), String> {
        let file = self
            .config
            .file
            .clone()
            .ok_or_else(|| format!("{}: missing source file", self.config.my_name))?;
        let output = self
            .config
            .file2
            .clone()
            .unwrap_or_else(|| PathBuf::from(format!("{}.x", file.display())));
        self.config.file2 = Some(output.clone());

        let cc = env::var("CC").unwrap_or_else(|_| "cc".to_string());
        let cflags = env::var("CFLAGS").unwrap_or_default();
        let ldflags = env::var("LDFLAGS").unwrap_or_default();

        let source = format!("{}.x.c", file.display());
        if self.config.verbose {
            eprintln!(
                "{}: {} {} {} {} -o {}",
                self.config.my_name,
                cc,
                cflags,
                ldflags,
                source,
                output.display()
            );
        }

        let status = Command::new(&cc)
            .args(cflags.split_whitespace())
            .args(ldflags.split_whitespace())
            .arg(&source)
            .arg("-o")
            .arg(&output)
            .status()
            .map_err(|e| format!("{}: {e}", self.config.my_name))?;

        if !status.success() {
            return Err(format!("{}: compiler failed", self.config.my_name));
        }

        let strip = env::var("STRIP").unwrap_or_else(|_| "strip".to_string());
        let _ = Command::new(&strip).arg(&output).status();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&output, fs::Permissions::from_mode(0o755));
        }

        Ok(())
    }

    pub fn do_all(&mut self, args: &[String]) -> Result<(), String> {
        self.parse_args(args)?;
        let file = self
            .config
            .file
            .clone()
            .ok_or_else(|| format!("{} parse(-f): No source file specified", self.config.my_name))?;
        let text = self.read_script(&file)?;
        self.eval_shell(&text)?;
        self.config.text = Some(text);
        self.write_c(&file, args)?;
        self.make()?;
        Ok(())
    }

    pub fn main() -> i32 {
        let args: Vec<String> = env::args().collect();
        let mut shc = Shc::default();
        match shc.do_all(&args) {
            Ok(()) => 0,
            Err(err) => {
                let _ = writeln!(io::stderr(), "{err}");
                1
            }
        }
    }

    pub fn module_src() -> Self {
        Self::default()
    }

    fn parse_date(value: &str) -> Option<i64> {
        let mut parts = value.split('/');
        let day: i64 = parts.next()?.parse().ok()?;
        let month: i64 = parts.next()?.parse().ok()?;
        let year: i64 = parts.next()?.parse().ok()?;
        if parts.next().is_some() {
            return None;
        }
        if !(1..=31).contains(&day) || !(1..=12).contains(&month) || year < 1970 {
            return None;
        }
        Some(
            ((year - 1970) * 365 + ((month - 1) * 31) + (day - 1))
                .saturating_mul(24 * 60 * 60),
        )
    }

    fn usage_line(&self) -> &'static str {
        "Usage: shc -f script [-o outfile] [-i inline] [-x exec] [-l lastopt] [-e dd/mm/yyyy] [-m addr] [-r] [-v] [-D] [-S] [-U] [-H] [-B] [-2]"
    }

    fn usage_text(&self) -> String {
        format!("{} {}", self.config.my_name, self.usage_line())
    }

    fn shell_defaults(shell: &str) -> Option<(&'static str, &'static str, &'static str)> {
        match shell {
            "perl" => Some(("-e", "exec('%s',@ARGV);", "--")),
            "rc" => Some(("-c", "builtin exec %s $*", "")),
            "sh" | "dash" | "bash" | "zsh" | "bsh" | "Rsh" | "ksh" => {
                Some(("-c", "exec '%s' \"$@\"", ""))
            }
            "tsh" | "ash" => Some(("-c", "exec '%s' \"$@\"", "--")),
            "csh" | "tcsh" => Some(("-c", "exec '%s' $argv", "-b")),
            _ => None,
        }
    }
}
