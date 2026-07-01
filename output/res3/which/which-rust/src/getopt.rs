#[derive(Clone, Debug, Default)]
pub struct Getopt {
    original_args: Vec<String>,
    original_argc: usize,
    optind: usize,
    optarg: Option<String>,
    optopt: Option<char>,
    opterr: bool,
    initialized: bool,
    nextchar: usize,
    first_nonopt: usize,
    last_nonopt: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Ordering {
    RequireOrder,
    Permute,
    ReturnInOrder,
}

impl Default for Ordering {
    fn default() -> Self {
        Self::Permute
    }
}

impl Getopt {
    pub fn store_args_and_env(&mut self, argv: &[String]) {
        self.original_argc = argv.len();
        self.original_args = argv.to_vec();
    }

    pub fn exchange(&mut self, argv: &mut [String]) {
        let bottom = self.first_nonopt;
        let middle = self.last_nonopt;
        let top = self.optind;

        if !(top > middle && middle > bottom) {
            self.first_nonopt += self.optind.saturating_sub(self.last_nonopt);
            self.last_nonopt = self.optind;
            return;
        }

        argv[bottom..top].rotate_left(middle - bottom);

        self.first_nonopt += self.optind.saturating_sub(self.last_nonopt);
        self.last_nonopt = self.optind;
    }

    fn attribute(&self) -> bool {
        true
    }

    fn ordering_from_optstring(optstring: &str) -> (&str, Ordering) {
        if let Some(rest) = optstring.strip_prefix('+') {
            (rest, Ordering::RequireOrder)
        } else if let Some(rest) = optstring.strip_prefix('-') {
            (rest, Ordering::ReturnInOrder)
        } else {
            (optstring, Ordering::Permute)
        }
    }

    fn parse_short_spec<'a>(optstring: &'a str, c: char) -> Option<&'a str> {
        let mut iter = optstring.char_indices().peekable();
        while let Some((idx, ch)) = iter.next() {
            if ch == c {
                let end = iter.peek().map(|(i, _)| *i).unwrap_or(optstring.len());
                return Some(&optstring[idx..end]);
            }
        }
        None
    }

    pub fn internal(
        &mut self,
        argv: &mut [String],
        optstring: &str,
        _long_only: bool,
    ) -> Option<Result<char, char>> {
        self.optarg = None;

        if self.optind == 0 || !self.initialized {
            if self.optind == 0 {
                self.optind = 1;
            }
            self.initialized = true;
        }

        let (optstring, ordering) = Self::ordering_from_optstring(optstring);

        loop {
            if self.nextchar == 0 {
                if self.last_nonopt > self.optind {
                    self.last_nonopt = self.optind;
                }
                if self.first_nonopt > self.optind {
                    self.first_nonopt = self.optind;
                }

                if ordering == Ordering::Permute {
                    if self.first_nonopt != self.last_nonopt && self.last_nonopt != self.optind {
                        self.exchange(argv);
                    } else if self.last_nonopt != self.optind {
                        self.first_nonopt = self.optind;
                    }

                    while self.optind < argv.len() {
                        let current = &argv[self.optind];
                        if current == "-" || !current.starts_with('-') {
                            self.optind += 1;
                        } else {
                            break;
                        }
                    }
                    self.last_nonopt = self.optind;
                }

                if self.optind < argv.len() && argv[self.optind] == "--" {
                    self.optind += 1;

                    if self.first_nonopt != self.last_nonopt && self.last_nonopt != self.optind {
                        self.exchange(argv);
                    } else if self.first_nonopt == self.last_nonopt {
                        self.first_nonopt = self.optind;
                    }
                    self.last_nonopt = argv.len();
                    self.optind = argv.len();
                }

                if self.optind >= argv.len() {
                    if self.first_nonopt != self.last_nonopt {
                        self.optind = self.first_nonopt;
                    }
                    return None;
                }

                let current = &argv[self.optind];
                let is_nonoption = current == "-" || !current.starts_with('-');
                if is_nonoption {
                    match ordering {
                        Ordering::RequireOrder => return None,
                        Ordering::ReturnInOrder => {
                            self.optarg = Some(current.clone());
                            self.optind += 1;
                            return Some(Ok('\u{1}'));
                        }
                        Ordering::Permute => {}
                    }
                }

                let skip = if argv[self.optind].starts_with("--") { 2 } else { 1 };
                self.nextchar = skip;
            }

            let current = &argv[self.optind];
            if self.nextchar >= current.len() {
                self.nextchar = 0;
                continue;
            }

            let remainder = &current[self.nextchar..];
            let mut chars = remainder.chars();
            let c = match chars.next() {
                Some(c) => c,
                None => {
                    self.nextchar = 0;
                    continue;
                }
            };
            self.nextchar += c.len_utf8();

            if self.nextchar >= current.len() {
                self.optind += 1;
                self.nextchar = 0;
            }

            let temp = Self::parse_short_spec(optstring, c);
            if temp.is_none() || c == ':' {
                self.optopt = Some(c);
                return Some(Err('?'));
            }

            let spec = temp.unwrap_or_default();
            let mut spec_chars = spec.chars();
            let _ = spec_chars.next();
            let first_colon = spec_chars.next() == Some(':');
            let second_colon = spec_chars.next() == Some(':');

            if first_colon {
                if second_colon {
                    if self.nextchar != 0 && self.optind <= argv.len() {
                        let arg_index = if self.nextchar == 0 {
                            self.optind.saturating_sub(1)
                        } else {
                            self.optind
                        };
                        if arg_index < argv.len() {
                            let source = &argv[arg_index];
                            if self.nextchar <= source.len() {
                                self.optarg = Some(source[self.nextchar..].to_string());
                            }
                        }
                        if self.nextchar != 0 {
                            self.optind = self.optind.max(arg_index + 1);
                        }
                    } else {
                        self.optarg = None;
                    }
                    self.nextchar = 0;
                } else if self.nextchar != 0 {
                    let arg_index = self.optind.saturating_sub(1);
                    if arg_index < argv.len() {
                        let source = &argv[arg_index];
                        if self.nextchar <= source.len() {
                            self.optarg = Some(source[self.nextchar..].to_string());
                        }
                    }
                    self.nextchar = 0;
                } else if self.optind >= argv.len() {
                    self.optopt = Some(c);
                    return Some(Err(if optstring.starts_with(':') { ':' } else { '?' }));
                } else {
                    self.optarg = Some(argv[self.optind].clone());
                    self.optind += 1;
                    self.nextchar = 0;
                }
            }

            return Some(Ok(c));
        }
    }

    pub fn getopt(
        &mut self,
        argv: &mut [String],
        optstring: &str,
    ) -> Option<Result<char, char>> {
        self.internal(argv, optstring, false)
    }

    pub fn main(&mut self, argv: &[String]) -> i32 {
        let mut args = argv.to_vec();
        self.store_args_and_env(argv);

        let mut digit_optind = 0usize;

        loop {
            let this_option_optind = if self.optind != 0 { self.optind } else { 1 };

            let c = self.getopt(&mut args, "abc:d:0123456789");
            let Some(result) = c else {
                break;
            };

            match result {
                Ok(ch @ '0'..='9') => {
                    if digit_optind != 0 && digit_optind != this_option_optind {
                        println!("digits occur in two different argv-elements.");
                    }
                    digit_optind = this_option_optind;
                    println!("option {}", ch);
                }
                Ok('a') => {
                    println!("option a");
                }
                Ok('b') => {
                    println!("option b");
                }
                Ok('c') => {
                    println!(
                        "option c with value `{}`",
                        self.optarg.as_deref().unwrap_or_default()
                    );
                }
                Ok('d') => {
                    println!(
                        "option d with value `{}`",
                        self.optarg.as_deref().unwrap_or_default()
                    );
                }
                Ok('\u{1}') => {}
                Err('?') => {}
                Err(':') => {}
                Ok(other) => {
                    println!("?? getopt returned character code 0{:o} ??", other as u32);
                }
                Err(other) => {
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

        let _ = self.attribute();
        0
    }
}
