use std::env;
use std::fmt::Write as _;
use std::fs;
use std::io::{self, Write};
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
    rng: SimpleRng,
    config: Config,
}

#[derive(Debug, Clone)]
struct Config {
    my_name: String,
    file: Option<PathBuf>,
    file2: Option<PathBuf>,
    mail: String,
    inlo: Option<String>,
    xecc: Option<String>,
    lsto: Option<String>,
    date: String,
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

impl Default for Config {
    fn default() -> Self {
        Self {
            my_name: "shc".to_string(),
            file: None,
            file2: None,
            mail: String::new(),
            inlo: None,
            xecc: None,
            lsto: None,
            date: "0".to_string(),
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
        }
    }
}

#[derive(Debug)]
enum ShcError {
    Io(io::Error),
    InvalidArgs(String),
    InvalidScript(String),
    BuildFailed(String),
}

impl From<io::Error> for ShcError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl std::fmt::Display for ShcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShcError::Io(e) => write!(f, "{e}"),
            ShcError::InvalidArgs(s) => write!(f, "{s}"),
            ShcError::InvalidScript(s) => write!(f, "{s}"),
            ShcError::BuildFailed(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for ShcError {}

#[derive(Debug, Clone, Copy)]
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn seeded() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x9E37_79B9_7F4A_7C15);
        let pid = u64::from(std::process::id());
        let seed = nanos ^ pid.rotate_left(13) ^ 0xA076_1D64_78BD_642F;
        Self { state: seed | 1 }
    }

    fn next_u32(&mut self) -> u32 {
        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;
        (self.state.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 32) as u32
    }
}

#[derive(Clone, Copy)]
struct ShellEntry {
    shll: &'static str,
    inlo: &'static str,
    xecc: &'static str,
    lsto: &'static str,
}

const SHELLS_DB: &[ShellEntry] = &[
    ShellEntry {
        shll: "perl",
        inlo: "-e",
        xecc: "exec('%s',@ARGV);",
        lsto: "--",
    },
    ShellEntry {
        shll: "rc",
        inlo: "-c",
        xecc: "builtin exec %s $*",
        lsto: "",
    },
    ShellEntry {
        shll: "sh",
        inlo: "-c",
        xecc: "exec '%s' \"$@\"",
        lsto: "",
    },
    ShellEntry {
        shll: "dash",
        inlo: "-c",
        xecc: "exec '%s' \"$@\"",
        lsto: "",
    },
    ShellEntry {
        shll: "bash",
        inlo: "-c",
        xecc: "exec '%s' \"$@\"",
        lsto: "",
    },
    ShellEntry {
        shll: "zsh",
        inlo: "-c",
        xecc: "exec '%s' \"$@\"",
        lsto: "",
    },
    ShellEntry {
        shll: "bsh",
        inlo: "-c",
        xecc: "exec '%s' \"$@\"",
        lsto: "",
    },
    ShellEntry {
        shll: "Rsh",
        inlo: "-c",
        xecc: "exec '%s' \"$@\"",
        lsto: "",
    },
    ShellEntry {
        shll: "ksh",
        inlo: "-c",
        xecc: "exec '%s' \"$@\"",
        lsto: "",
    },
    ShellEntry {
        shll: "tsh",
        inlo: "-c",
        xecc: "exec '%s' \"$@\"",
        lsto: "--",
    },
    ShellEntry {
        shll: "ash",
        inlo: "-c",
        xecc: "exec '%s' \"$@\"",
        lsto: "--",
    },
    ShellEntry {
        shll: "csh",
        inlo: "-c",
        xecc: "exec '%s' $argv",
        lsto: "-b",
    },
    ShellEntry {
        shll: "tcsh",
        inlo: "-c",
        xecc: "exec '%s' $argv",
        lsto: "-b",
    },
];

impl Default for Shc {
    fn default() -> Self {
        let mut s = Self {
            stte: [0; 256],
            indx: 0,
            jndx: 0,
            kndx: 0,
            offset: 0,
            rng: SimpleRng::seeded(),
            config: Config::default(),
        };
        s.stte_0();
        s
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

        let need_value = |idx: usize, args: &[String], name: &str, my_name: &str| {
            args.get(idx + 1)
                .cloned()
                .ok_or_else(|| format!("{my_name} parse({name}): Missing parameter"))
        };

        match arg.as_str() {
            "-e" => {
                let value = need_value(*index, args, "-e", &self.config.my_name)?;
                let ts = Self::parse_date_to_unix(&value).ok_or_else(|| {
                    format!("{} parse(-e {}): Not a valid value", self.config.my_name, value)
                })?;
                self.config.date = ts.to_string();
                *index += 2;
                Ok(true)
            }
            "-m" => {
                self.config.mail = need_value(*index, args, "-m", &self.config.my_name)?;
                *index += 2;
                Ok(true)
            }
            "-f" => {
                if self.config.file.is_some() {
                    return Err(format!(
                        "{} parse(-f): Specified more than once",
                        self.config.my_name
                    ));
                }
                self.config.file = Some(PathBuf::from(need_value(
                    *index,
                    args,
                    "-f",
                    &self.config.my_name,
                )?));
                *index += 2;
                Ok(true)
            }
            "-i" => {
                self.config.inlo = Some(need_value(*index, args, "-i", &self.config.my_name)?);
                *index += 2;
                Ok(true)
            }
            "-x" => {
                self.config.xecc = Some(need_value(*index, args, "-x", &self.config.my_name)?);
                *index += 2;
                Ok(true)
            }
            "-l" => {
                self.config.lsto = Some(need_value(*index, args, "-l", &self.config.my_name)?);
                *index += 2;
                Ok(true)
            }
            "-o" => {
                self.config.file2 = Some(PathBuf::from(need_value(
                    *index,
                    args,
                    "-o",
                    &self.config.my_name,
                )?));
                *index += 2;
                Ok(true)
            }
            "-r" => {
                self.config.rlax[0] = self.config.rlax[0].wrapping_add(1);
                *index += 1;
                Ok(true)
            }
            "-v" => {
                self.config.verbose = true;
                *index += 1;
                Ok(true)
            }
            "-S" => {
                self.config.setuid_flag = true;
                *index += 1;
                Ok(true)
            }
            "-D" => {
                self.config.debugexec_flag = true;
                *index += 1;
                Ok(true)
            }
            "-U" => {
                self.config.traceable_flag = false;
                *index += 1;
                Ok(true)
            }
            "-H" => {
                self.config.hardening_flag = true;
                *index += 1;
                Ok(true)
            }
            "-B" => {
                self.config.busyboxon_flag = true;
                *index += 1;
                Ok(true)
            }
            "-2" => {
                self.config.mmap2_flag = true;
                *index += 1;
                Ok(true)
            }
            "-C" | "-A" | "-h" => {
                *index += 1;
                Ok(false)
            }
            _ => Err(format!("{} parse: Unknown option", self.config.my_name)),
        }
    }

    pub fn parse_args<I>(&mut self, args: I) -> Result<(), String>
    where
        I: IntoIterator<Item = String>,
    {
        let collected: Vec<String> = args.into_iter().collect();
        if let Some(name) = collected.first() {
            self.config.my_name = Path::new(name)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or(name)
                .to_string();
        }

        let mut err = 0usize;
        let mut index = 1usize;

        loop {
            match self.parse_an_arg(&collected, &mut index) {
                Ok(true) => {}
                Ok(false) => break,
                Err(_) => {
                    err += 1;
                    if index >= collected.len() {
                        break;
                    }
                    index += 1;
                }
            }
            if index >= collected.len() {
                match self.parse_an_arg(&collected, &mut index) {
                    Ok(false) => break,
                    Ok(true) => {}
                    Err(_) => err += 1,
                }
                break;
            }
        }

        if err > 0 {
            return Err(format!(
                "\n{} Usage: -f <script> [-e date] [-m mail] [-i inlo] [-x xecc] [-l lsto] [-o out] [-rvDSUHCB2h]\n",
                self.config.my_name
            ));
        }
        Ok(())
    }

    pub fn stte_0(&mut self) {
        self.indx = 0;
        self.jndx = 0;
        self.kndx = 0;
        for i in 0..256 {
            self.stte[i] = i as u8;
        }
    }

    pub fn key(&mut self, data: &[u8]) {
        let mut len = data.len();
        let mut start = 0usize;
        while len > 0 {
            loop {
                let idx = self.indx as usize;
                let tmp = self.stte[idx];
                self.kndx = self.kndx.wrapping_add(tmp);
                self.kndx = self.kndx.wrapping_add(data[start + (idx % len)]);
                let k = self.kndx as usize;
                self.stte[idx] = self.stte[k];
                self.stte[k] = tmp;
                self.indx = self.indx.wrapping_add(1);
                if self.indx == 0 {
                    break;
                }
            }
            start = start.saturating_add(256);
            len = len.saturating_sub(256);
        }
    }

    pub fn arc_4(&mut self, data: &mut [u8]) {
        for byte in data {
            self.indx = self.indx.wrapping_add(1);
            let idx = self.indx as usize;
            let mut tmp = self.stte[idx];
            self.jndx = self.jndx.wrapping_add(tmp);
            let j = self.jndx as usize;
            self.stte[idx] = self.stte[j];
            self.stte[j] = tmp;
            tmp = tmp.wrapping_add(self.stte[idx]);
            *byte ^= self.stte[tmp as usize];
        }
    }

    pub fn key_with_file(&mut self, path: &Path) -> Result<(), io::Error> {
        let meta = fs::metadata(path)?;
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
                if let Ok(d) = modified.duration_since(UNIX_EPOCH) {
                    control.extend_from_slice(&d.as_secs().to_ne_bytes());
                }
            }
            if let Ok(created) = meta.created() {
                if let Ok(d) = created.duration_since(UNIX_EPOCH) {
                    control.extend_from_slice(&d.as_secs().to_ne_bytes());
                }
            }
        }

        self.key(&control);
        Ok(())
    }

    pub fn eval_shell(&mut self, text: &[u8]) -> Result<(), String> {
        let first_line_end = text.iter().position(|b| *b == b'\n').unwrap_or(text.len());
        let first_line = String::from_utf8_lossy(&text[..first_line_end]).to_string();
        let trimmed = first_line.trim_start();

        if !trimmed.starts_with("#!") {
            return Err(format!(
                "{}: invalid first line in script: {}",
                self.config.my_name, first_line
            ));
        }

        let rest = trimmed[2..].trim();
        let mut parts = rest.split_whitespace();
        let shll_full = parts
            .next()
            .ok_or_else(|| format!("{}: invalid shll", self.config.my_name))?
            .to_string();
        let opts = parts.next().unwrap_or("").to_string();

        let ptr = Path::new(&shll_full)
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| format!("{}: invalid shll", self.config.my_name))?
            .to_string();

        self.config.shll = Some(shll_full);

        for entry in SHELLS_DB {
            if entry.shll == ptr {
                if self.config.inlo.is_none() {
                    self.config.inlo = Some(entry.inlo.to_string());
                }
                if self.config.xecc.is_none() {
                    self.config.xecc = Some(entry.xecc.to_string());
                }
                if self.config.lsto.is_none() {
                    self.config.lsto = Some(entry.lsto.to_string());
                }
            }
        }

        if self.config.inlo.is_none() || self.config.xecc.is_none() || self.config.lsto.is_none() {
            return Err(format!(
                "{} Unknown shell ({}): specify [-i][-x][-l]",
                self.config.my_name, ptr
            ));
        }

        let lsto = self.config.lsto.clone().unwrap_or_default();
        self.config.opts = if !opts.is_empty() && opts == lsto {
            String::new()
        } else if opts == "-" {
            String::new()
        } else {
            opts
        };

        Ok(())
    }

    pub fn read_script(&self, path: &Path) -> Result<Vec<u8>, io::Error> {
        fs::read(path)
    }

    pub fn rand_mod(&mut self, modulus: u32) -> u32 {
        if modulus == 0 {
            return 0;
        }
        let top = u32::MAX - (u32::MAX % modulus);
        loop {
            let rnd = self.rng.next_u32();
            if rnd < top {
                return ((modulus as u64 * rnd as u64) / (1u64 + top as u64)) as u32;
            }
        }
    }

    pub fn rand_chr(&mut self) -> char {
        char::from_u32(self.rand_mod(1 << 8)).unwrap_or('\0')
    }

    pub fn noise(&mut self, out: &mut Vec<u8>, min: usize, xtra: usize, alnum_only: bool) -> usize {
        let mut total = min;
        if xtra != 0 {
            total += self.rand_mod(xtra as u32) as usize;
        }

        out.clear();
        out.reserve(total + usize::from(alnum_only));

        for _ in 0..total {
            let b = loop {
                let c = self.rand_chr() as u8;
                if !alnum_only || (c as char).is_ascii_alphanumeric() {
                    break c;
                }
            };
            out.push(b);
        }

        if alnum_only {
            out.push(0);
        }

        total
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

    pub fn prnt_array(&mut self, ptr: &[u8], name: &str, l: usize, cast: Option<&str>) -> String {
        let mut m = self.rand_mod((1 + l / 4) as u32) as usize;
        let n = self.rand_mod((1 + l / 4) as u32) as usize;
        let a = (self.offset + m) % l.max(1);
        if cast.is_some() && a != 0 {
            m += l - a;
        }
        let cast_prefix = cast.unwrap_or("");
        let mut out = String::new();
        let _ = write!(
            out,
            "\n#define      {name}_z\t{l}\n#define      {name}\t({cast_prefix}(&data[{}]))",
            self.offset + m
        );
        out.push_str(&self.prnt_bytes(ptr, m, l, n));
        out
    }

    pub fn dump_array(&mut self, ptr: &mut [u8], name: &str, l: usize, cast: Option<&str>) -> String {
        self.arc_4(&mut ptr[..l]);
        self.prnt_array(&ptr[..l], name, l, cast)
    }

    pub fn write_c(&mut self, file: &Path, argv: &[String]) -> Result<PathBuf, ShcError> {
        let mut pswd = vec![0u8; 256];
        let pswd_z = self.noise(&mut pswd, 256, 0, false);

        let mut msg1 = format!("has expired!\n{}", self.config.mail).into_bytes();
        let mut date = self.config.date.clone().into_bytes();
        date.push(0);

        let shll_value = self
            .config
            .shll
            .clone()
            .ok_or_else(|| ShcError::InvalidScript("missing shell".to_string()))?;
        let mut shll = shll_value.into_bytes();
        shll.push(0);

        let mut inlo = self
            .config
            .inlo
            .clone()
            .ok_or_else(|| ShcError::InvalidScript("missing inlo".to_string()))?
            .into_bytes();
        inlo.push(0);

        let mut xecc = self
            .config
            .xecc
            .clone()
            .ok_or_else(|| ShcError::InvalidScript("missing xecc".to_string()))?
            .into_bytes();
        xecc.push(0);

        let mut lsto = self
            .config
            .lsto
            .clone()
            .ok_or_else(|| ShcError::InvalidScript("missing lsto".to_string()))?
            .into_bytes();
        lsto.push(0);

        let mut tst1 = b"location has changed!\0".to_vec();
        let mut chk1 = tst1.clone();
        let mut msg2 = b"abnormal behavior!\0".to_vec();
        let mut rlax = self.config.rlax.clone();
        let mut opts = self.config.opts.clone().into_bytes();
        opts.push(0);

        let mut text = self
            .config
            .text
            .clone()
            .ok_or_else(|| ShcError::InvalidScript("missing script text".to_string()))?;
        text.push(0);

        let mut tst2 = b"shell has changed!\0".to_vec();
        let mut chk2 = tst2.clone();
        let kwsh_plain = self
            .config
            .shll
            .clone()
            .ok_or_else(|| ShcError::InvalidScript("missing shell".to_string()))?;

        self.stte_0();
        self.key(&pswd[..pswd_z]);

        // The original C runtime decrypts the embedded arrays before using them.
        // This Rust port currently emits a much smaller wrapper runtime that reads
        // the generated arrays directly, so keep wrapper-consumed fields plaintext.
        // Still preserve the key schedule steps that depend on kwsh/rlax so option
        // handling stays aligned with the parsed C flow.
        let indx = self.config.rlax.first().copied().unwrap_or(0) == 0;
        if indx {
            self.key_with_file(Path::new(&kwsh_plain))
                .map_err(ShcError::Io)?;
        }

        let _ = (&mut msg1, &mut date, &mut shll, &mut inlo, &mut xecc, &mut lsto);
        let _ = (&mut tst1, &mut chk1, &mut msg2, &mut rlax, &mut opts, &mut tst2, &mut chk2);
        // text already includes the original script bytes; add the C-style trailing NUL
        // but do not encrypt, because the emitted runtime writes text directly.

        self.offset = 0;

        let out_path = PathBuf::from(format!("{}.x.c", file.display()));
        let mut o = fs::File::create(&out_path)?;

        writeln!(o, "#if 0")?;
        writeln!(o, "\t{} Rust port generated output", self.config.my_name)?;
        write!(o, "\t")?;
        for arg in argv {
            write!(o, "{arg} ")?;
        }
        writeln!(o, "\n#endif\n")?;
        write!(o, "static  char data [] = ")?;

        let arrays: [(&str, Vec<u8>); 15] = [
            ("pswd", pswd),
            ("msg1", msg1),
            ("date", date),
            ("shll", shll),
            ("inlo", inlo),
            ("xecc", xecc),
            ("lsto", lsto),
            ("tst1", tst1),
            ("chk1", chk1),
            ("msg2", msg2),
            ("rlax", rlax),
            ("opts", opts),
            ("text", text),
            ("tst2", tst2),
            ("chk2", chk2),
        ];

        for (name, data) in arrays {
            write!(o, "{}", self.prnt_array(&data, name, data.len(), None))?;
        }

        writeln!(o, "/* End of data[] */;")?;
        writeln!(o, "#define      hide_z\t{}", 1 << 12)?;
        writeln!(o, "#define SETUID {}", i32::from(self.config.setuid_flag))?;
        writeln!(o, "#define DEBUGEXEC {}", i32::from(self.config.debugexec_flag))?;
        writeln!(o, "#define TRACEABLE {}", i32::from(self.config.traceable_flag))?;
        writeln!(o, "#define HARDENING {}", i32::from(self.config.hardening_flag))?;
        writeln!(o, "#define BUSYBOXON {}", i32::from(self.config.busyboxon_flag))?;
        writeln!(o, "#define MMAP2 {}", i32::from(self.config.mmap2_flag))?;
        writeln!(o, "{}", self.module_src())?;
        o.flush()?;

        Ok(out_path)
    }

    pub fn make(&mut self) -> Result<(), ShcError> {
        let file = self
            .config
            .file
            .clone()
            .ok_or_else(|| ShcError::InvalidArgs("missing source file".to_string()))?;

        let file2 = self
            .config
            .file2
            .clone()
            .unwrap_or_else(|| PathBuf::from(format!("{}.x", file.display())));

        let cc = env::var("CC").unwrap_or_else(|_| "cc".to_string());
        let cflags = env::var("CFLAGS").unwrap_or_default();
        let ldflags = env::var("LDFLAGS").unwrap_or_default();

        let status = Command::new(&cc)
            .args(cflags.split_whitespace())
            .args(ldflags.split_whitespace())
            .arg(format!("{}.x.c", file.display()))
            .arg("-o")
            .arg(&file2)
            .status()?;

        if !status.success() {
            return Err(ShcError::BuildFailed("compiler command failed".to_string()));
        }

        let strip = env::var("STRIP").unwrap_or_else(|_| "strip".to_string());
        let _ = Command::new(strip).arg(&file2).status();
        let _ = Command::new("chmod").arg("ug=rwx,o=rx").arg(&file2).status();

        self.config.file2 = Some(file2);
        Ok(())
    }

    pub fn do_all(&mut self, args: &[String]) -> Result<(), ShcError> {
        self.parse_args(args.to_vec()).map_err(ShcError::InvalidArgs)?;

        let file = self
            .config
            .file
            .clone()
            .ok_or_else(|| ShcError::InvalidArgs("missing source file".to_string()))?;

        let text = self.read_script(&file)?;
        self.eval_shell(&text).map_err(ShcError::InvalidScript)?;
        self.config.text = Some(text);
        let _ = self.write_c(&file, args)?;
        self.make()?;
        Ok(())
    }

    pub fn main(args: &[String]) -> i32 {
        let mut shc = Shc::default();
        match shc.do_all(args) {
            Ok(()) => 0,
            Err(e) => {
                eprintln!("{e}");
                1
            }
        }
    }

    pub fn module_src(&self) -> String {
        r#"
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

static int write_all(int fd, const char *buf, size_t len) {
    while (len) {
        ssize_t wr = write(fd, buf, len);
        if (wr < 0) {
            if (errno == EINTR) continue;
            return -1;
        }
        buf += (size_t)wr;
        len -= (size_t)wr;
    }
    return 0;
}

int main(int argc, char **argv) {
    char tmpl[] = "/tmp/shcXXXXXX";
    int fd = mkstemp(tmpl);
    if (fd < 0) {
        perror("mkstemp");
        return 1;
    }
    if (fchmod(fd, S_IRUSR | S_IWUSR | S_IXUSR) != 0) {
        perror("fchmod");
        close(fd);
        unlink(tmpl);
        return 1;
    }
    if (write_all(fd, text, text_z - 1) != 0) {
        perror("write");
        close(fd);
        unlink(tmpl);
        return 1;
    }
    close(fd);

    int extra = (opts[0] != '\0') ? 1 : 0;
    char **nargv = calloc((size_t)argc + 3u, sizeof(char *));
    if (!nargv) {
        perror("calloc");
        unlink(tmpl);
        return 1;
    }

    int n = 0;
    nargv[n++] = shll;
    if (extra) nargv[n++] = opts;
    nargv[n++] = tmpl;
    for (int i = 1; i < argc; i++) nargv[n++] = argv[i];
    nargv[n] = NULL;

    execv(shll, nargv);
    perror(shll);
    unlink(tmpl);
    return 1;
}
"#
        .trim()
        .to_string()
    }

    fn parse_date_to_unix(value: &str) -> Option<i64> {
        let mut it = value.split('/');
        let day: i64 = it.next()?.parse().ok()?;
        let month: i64 = it.next()?.parse().ok()?;
        let year: i64 = it.next()?.parse().ok()?;
        if it.next().is_some() {
            return None;
        }
        if !(1..=31).contains(&day) || !(1..=12).contains(&month) || year < 1970 {
            return None;
        }

        let days = days_from_civil(year, month, day)?;
        Some(days * 86_400)
    }
}

fn days_from_civil(year: i64, month: i64, day: i64) -> Option<i64> {
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let y = year - if month <= 2 { 1 } else { 0 };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = month + if month > 2 { -3 } else { 9 };
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some(era * 146097 + doe - 719468)
}
