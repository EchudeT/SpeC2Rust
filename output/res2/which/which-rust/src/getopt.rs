use std::env;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OrderingMode {
    RequireOrder,
    Permute,
    ReturnInOrder,
}

#[derive(Debug, Default, Clone)]
struct StoredContext {
    original_argc: usize,
    original_argv: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Getopt {
    pub optind: usize,
    pub opterr: bool,
    pub optopt: Option<char>,
    pub optarg: Option<String>,
    nextchar: Option<usize>,
    first_nonopt: usize,
    last_nonopt: usize,
    initialized: bool,
    ordering: OrderingMode,
    posixly_correct: bool,
    stored: StoredContext,
}

impl Default for Getopt {
    fn default() -> Self {
        Self {
            optind: 1,
            opterr: true,
            optopt: None,
            optarg: None,
            nextchar: None,
            first_nonopt: 1,
            last_nonopt: 1,
            initialized: false,
            ordering: OrderingMode::Permute,
            posixly_correct: false,
            stored: StoredContext::default(),
        }
    }
}

impl Getopt {
    pub fn store_args_and_env(&mut self, args: &[String]) {
        self.stored.original_argc = args.len();
        self.stored.original_argv = args.to_vec();
    }

    pub fn exchange(&mut self, argv: &mut [String]) {
        let mut bottom = self.first_nonopt;
        let middle = self.last_nonopt;
        let mut top = self.optind;

        while top > middle && middle > bottom {
            if top - middle > middle - bottom {
                let len = middle - bottom;
                for i in 0..len {
                    argv.swap(bottom + i, top - (middle - bottom) + i);
                }
                top -= len;
            } else {
                let len = top - middle;
                for i in 0..len {
                    argv.swap(bottom + i, middle + i);
                }
                bottom += len;
            }
        }

        self.first_nonopt += self.optind.saturating_sub(self.last_nonopt);
        self.last_nonopt = self.optind;
    }

    fn initialize<'a>(&mut self, optstring: &'a str) -> &'a str {
        self.first_nonopt = self.optind;
        self.last_nonopt = self.optind;
        self.nextchar = None;
        self.posixly_correct = env::var_os("POSIXLY_CORRECT").is_some();

        let mut rest = optstring;
        if let Some(stripped) = rest.strip_prefix('-') {
            self.ordering = OrderingMode::ReturnInOrder;
            rest = stripped;
        } else if let Some(stripped) = rest.strip_prefix('+') {
            self.ordering = OrderingMode::RequireOrder;
            rest = stripped;
        } else if self.posixly_correct {
            self.ordering = OrderingMode::RequireOrder;
        } else {
            self.ordering = OrderingMode::Permute;
        }

        rest
    }

    fn optstring_contains(optstring: &str, c: char) -> Option<usize> {
        optstring.chars().position(|ch| ch == c)
    }

    fn char_at(s: &str, idx: usize) -> Option<char> {
        s.as_bytes().get(idx).map(|b| *b as char)
    }

    fn is_nonoption(arg: &str) -> bool {
        !arg.starts_with('-') || arg == "-"
    }

    pub fn internal(
        &mut self,
        argv: &mut [String],
        optstring: &str,
        longopts: Option<&[()]>,
        longind: Option<&mut usize>,
        long_only: bool,
    ) -> Option<char> {
        let argc = argv.len();
        self.optarg = None;
        self.optopt = None;

        if self.optind == 0 || !self.initialized {
            if self.optind == 0 {
                self.optind = 1;
            }
            let _ = self.initialize(optstring);
            self.initialized = true;
        }

        let optstring = self.initialize(optstring);

        if self.nextchar.is_none()
            || self
                .optind
                .checked_sub(0)
                .and_then(|_| argv.get(self.optind))
                .map(|arg| self.nextchar.unwrap_or(0) >= arg.len())
                .unwrap_or(true)
        {
            if self.last_nonopt > self.optind {
                self.last_nonopt = self.optind;
            }
            if self.first_nonopt > self.optind {
                self.first_nonopt = self.optind;
            }

            if matches!(self.ordering, OrderingMode::Permute) {
                if self.first_nonopt != self.last_nonopt && self.last_nonopt != self.optind {
                    self.exchange(argv);
                } else if self.last_nonopt != self.optind {
                    self.first_nonopt = self.optind;
                }

                while self.optind < argc && Self::is_nonoption(&argv[self.optind]) {
                    self.optind += 1;
                }
                self.last_nonopt = self.optind;
            }

            if self.optind != argc && argv[self.optind] == "--" {
                self.optind += 1;

                if self.first_nonopt != self.last_nonopt && self.last_nonopt != self.optind {
                    self.exchange(argv);
                } else if self.first_nonopt == self.last_nonopt {
                    self.first_nonopt = self.optind;
                }
                self.last_nonopt = argc;
                self.optind = argc;
            }

            if self.optind == argc {
                if self.first_nonopt != self.last_nonopt {
                    self.optind = self.first_nonopt;
                }
                return None;
            }

            if Self::is_nonoption(&argv[self.optind]) {
                if matches!(self.ordering, OrderingMode::RequireOrder) {
                    return None;
                }
                self.optarg = Some(argv[self.optind].clone());
                self.optind += 1;
                return Some('\u{1}');
            }

            let current = &argv[self.optind];
            let start = 1 + usize::from(longopts.is_some() && current.as_bytes().get(1) == Some(&b'-'));
            self.nextchar = Some(start);
        }

        if longopts.is_some() && self.optind < argc {
            let current = &argv[self.optind];
            let treat_as_long = current.as_bytes().get(1) == Some(&b'-')
                || (long_only
                    && current.len() > 2
                    && Self::char_at(current, 1)
                        .and_then(|c| Self::optstring_contains(optstring, c))
                        .is_none());
            if treat_as_long {
                self.nextchar = Some(current.len());
                self.optind += 1;
                self.optopt = None;
                if let Some(index) = longind {
                    *index = 0;
                }
                return Some('?');
            }
        }

        let current = &argv[self.optind];
        let idx = self.nextchar.unwrap_or(1);
        let c = match Self::char_at(current, idx) {
            Some(ch) => ch,
            None => {
                self.nextchar = None;
                return None;
            }
        };
        self.nextchar = Some(idx + 1);

        if self.nextchar.unwrap_or(0) >= current.len() {
            self.optind += 1;
        }

        let temp_index = Self::optstring_contains(optstring, c);
        if temp_index.is_none() || c == ':' {
            self.optopt = Some(c);
            return Some('?');
        }
        let temp_index = temp_index.unwrap();

        let tail: Vec<char> = optstring.chars().skip(temp_index).take(3).collect();

        if tail.len() >= 2 && tail[0] == 'W' && tail[1] == ';' {
            if self.nextchar.unwrap_or(0) < current.len() {
                self.optarg = Some(current[self.nextchar.unwrap_or(0)..].to_string());
                self.optind += 1;
            } else if self.optind >= argc {
                self.optopt = Some(c);
                return Some(if optstring.starts_with(':') { ':' } else { '?' });
            } else if self.optind < argc {
                self.optarg = argv.get(self.optind).cloned();
                self.optind += 1;
            }
            self.nextchar = None;
            return Some('W');
        }

        if tail.get(1) == Some(&':') {
            if tail.get(2) == Some(&':') {
                if self.nextchar.unwrap_or(0) < current.len() {
                    self.optarg = Some(current[self.nextchar.unwrap_or(0)..].to_string());
                    self.optind += 1;
                } else {
                    self.optarg = None;
                }
                self.nextchar = None;
            } else {
                if self.nextchar.unwrap_or(0) < current.len() {
                    self.optarg = Some(current[self.nextchar.unwrap_or(0)..].to_string());
                    self.optind += 1;
                } else if self.optind == argc {
                    self.optopt = Some(c);
                    return Some(if optstring.starts_with(':') { ':' } else { '?' });
                } else {
                    self.optarg = argv.get(self.optind).cloned();
                    self.optind += 1;
                }
                self.nextchar = None;
            }
        }

        Some(c)
    }

    pub fn getopt(&mut self, argv: &mut [String], optstring: &str) -> Option<char> {
        self.internal(argv, optstring, None, None, false)
    }

    pub fn main(&mut self, argv: &[String]) -> i32 {
        let mut args = argv.to_vec();
        self.store_args_and_env(&args);

        let mut digit_optind = 0usize;

        loop {
            let this_option_optind = if self.optind != 0 { self.optind } else { 1 };
            let c = self.getopt(&mut args, "abc:d:0123456789");
            let Some(c) = c else {
                break;
            };

            match c {
                '0'..='9' => {
                    if digit_optind != 0 && digit_optind != this_option_optind {
                        println!("digits occur in two different argv-elements.");
                    }
                    digit_optind = this_option_optind;
                    println!("option {}", c);
                }
                'a' => println!("option a"),
                'b' => println!("option b"),
                'c' => {
                    println!(
                        "option c with value `{}`",
                        self.optarg.clone().unwrap_or_default()
                    );
                }
                '?' => {}
                other => {
                    println!("?? getopt returned character code 0{:o} ??", other as u32);
                }
            }
        }

        if self.optind < args.len() {
            print!("non-option ARGV-elements: ");
            while self.optind < args.len() {
                print!("{} ", args[self.optind]);
                self.optind += 1;
            }
            println!();
        }

        0
    }

    pub fn attribute(&self) -> bool {
        true
    }
}
