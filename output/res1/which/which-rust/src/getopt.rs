use std::env;

#[derive(Clone, Debug, Default)]
pub struct Getopt {
    pub optind: usize,
    pub opterr: bool,
    pub optopt: Option<char>,
    pub optarg: Option<String>,
    original_argc: usize,
    original_argv: Vec<String>,
    first_nonopt: usize,
    last_nonopt: usize,
    nextchar: usize,
    initialized: bool,
}

impl Getopt {
    pub fn store_args_and_env(&mut self, argc: usize, argv: &[String]) {
        self.original_argc = argc;
        self.original_argv = argv.to_vec();
    }

    pub fn exchange(&mut self, argv: &mut [String]) {
        let mut bottom = self.first_nonopt;
        let middle = self.last_nonopt;
        let mut top = self.optind;

        while top > middle && middle > bottom {
            if top - middle > middle - bottom {
                let len = middle - bottom;
                for i in 0..len {
                    let a = bottom + i;
                    let b = top - (middle - bottom) + i;
                    argv.swap(a, b);
                }
                top -= len;
            } else {
                let len = top - middle;
                for i in 0..len {
                    let a = bottom + i;
                    let b = middle + i;
                    argv.swap(a, b);
                }
                bottom += len;
            }
        }

        self.first_nonopt += self.optind.saturating_sub(self.last_nonopt);
        self.last_nonopt = self.optind;
    }

    pub fn internal(
        &mut self,
        argv: &mut [String],
        optstring: &str,
        _long_only: bool,
    ) -> Option<char> {
        if !self.initialized {
            self.initialized = true;
            if self.optind == 0 {
                self.optind = 1;
            }
            self.first_nonopt = self.optind;
            self.last_nonopt = self.optind;
            self.nextchar = 0;
        }

        self.optarg = None;
        self.optopt = None;

        loop {
            if self.nextchar == 0 {
                if self.last_nonopt > self.optind {
                    self.last_nonopt = self.optind;
                }
                if self.first_nonopt > self.optind {
                    self.first_nonopt = self.optind;
                }

                while self.optind < argv.len() {
                    let current = &argv[self.optind];

                    if current == "--" {
                        self.optind += 1;
                        if self.first_nonopt != self.last_nonopt && self.last_nonopt != self.optind
                        {
                            self.exchange(argv);
                        } else if self.first_nonopt == self.last_nonopt {
                            self.first_nonopt = self.optind;
                        }
                        self.last_nonopt = argv.len();
                        self.optind = argv.len();
                        return None;
                    }

                    if !current.starts_with('-') || current == "-" {
                        if self.first_nonopt == self.last_nonopt {
                            self.first_nonopt = self.optind;
                        }
                        self.optind += 1;
                        self.last_nonopt = self.optind;
                        continue;
                    }

                    break;
                }

                if self.optind >= argv.len() {
                    if self.first_nonopt != self.last_nonopt {
                        self.optind = self.first_nonopt;
                    }
                    return None;
                }

                self.nextchar = 1;
            }

            let current = &argv[self.optind];
            let chars: Vec<char> = current.chars().collect();

            if self.nextchar >= chars.len() {
                self.nextchar = 0;
                self.optind += 1;
                continue;
            }

            let c = chars[self.nextchar];
            self.nextchar += 1;
            self.optopt = Some(c);

            let mut spec_iter = optstring.chars().peekable();
            let mut found = false;
            let mut requires_arg = false;
            let mut optional_arg = false;

            while let Some(spec) = spec_iter.next() {
                if spec == c {
                    found = true;
                    if spec_iter.peek() == Some(&':') {
                        spec_iter.next();
                        requires_arg = true;
                        if spec_iter.peek() == Some(&':') {
                            spec_iter.next();
                            optional_arg = true;
                        }
                    }
                    break;
                }
            }

            if !found || c == ':' {
                if self.nextchar >= chars.len() {
                    self.nextchar = 0;
                    self.optind += 1;
                }
                return Some('?');
            }

            if requires_arg {
                if self.nextchar < chars.len() {
                    let rest: String = chars[self.nextchar..].iter().collect();
                    self.optarg = Some(rest);
                    self.optind += 1;
                    self.nextchar = 0;
                } else if optional_arg {
                    self.optarg = None;
                    self.optind += 1;
                    self.nextchar = 0;
                } else if self.optind + 1 < argv.len() {
                    self.optarg = Some(argv[self.optind + 1].clone());
                    self.optind += 2;
                    self.nextchar = 0;
                } else {
                    self.nextchar = 0;
                    self.optind += 1;
                    return Some(if optstring.starts_with(':') { ':' } else { '?' });
                }
            } else if self.nextchar >= chars.len() {
                self.nextchar = 0;
                self.optind += 1;
            }

            return Some(c);
        }
    }

    pub fn getopt(&mut self, argv: &mut [String], optstring: &str) -> Option<char> {
        self.internal(argv, optstring, false)
    }

    pub fn main(&mut self) -> i32 {
        let mut args: Vec<String> = env::args().collect();
        self.store_args_and_env(args.len(), &args);

        while let Some(option) = self.getopt(&mut args, "abc:d::") {
            match option {
                'a' => {
                    println!("option a");
                }
                'b' => {
                    println!("option b");
                }
                'c' => {
                    println!("option c {:?}", self.optarg);
                }
                'd' => {
                    println!("option d {:?}", self.optarg);
                }
                '?' => {
                    eprintln!("invalid option");
                    return 1;
                }
                ':' => {
                    eprintln!("missing option argument");
                    return 1;
                }
                _ => {}
            }
        }

        for arg in args.iter().skip(self.optind) {
            println!("{arg}");
        }

        0
    }

    pub fn attribute(&self) -> bool {
        true
    }
}
