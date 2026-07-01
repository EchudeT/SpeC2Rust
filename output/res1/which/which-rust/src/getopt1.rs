use crate::getopt::Getopt;
use std::env;

#[derive(Clone, Debug, Default)]
pub struct Getopt1 {
    parser: Getopt,
}

impl Getopt1 {
    pub fn getopt_long(
        &mut self,
        argv: &mut [String],
        optstring: &str,
        _long_options: &[&str],
        _opt_index: Option<&mut usize>,
    ) -> Option<char> {
        self.parser.internal(argv, optstring, false)
    }

    pub fn getopt_long_only(
        &mut self,
        argv: &mut [String],
        optstring: &str,
        _long_options: &[&str],
        _opt_index: Option<&mut usize>,
    ) -> Option<char> {
        self.parser.internal(argv, optstring, true)
    }

    pub fn main(&mut self) -> i32 {
        let mut args: Vec<String> = env::args().collect();
        self.parser.store_args_and_env(args.len(), &args);

        let long_options = ["add", "append", "delete", "verbose", "create", "file"];
        let mut digit_optind = 0usize;

        loop {
            let this_option_optind = if self.parser.optind != 0 {
                self.parser.optind
            } else {
                1
            };
            let mut option_index = 0usize;

            let c = self.getopt_long(
                &mut args,
                "abc:d:0123456789",
                &long_options,
                Some(&mut option_index),
            );

            let Some(c) = c else {
                break;
            };

            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    if digit_optind != 0 && digit_optind != this_option_optind {
                        println!("digits occur in two different argv-elements.");
                    }
                    digit_optind = this_option_optind;
                    println!("option {c}");
                }
                'a' => {
                    println!("option a");
                }
                'b' => {
                    println!("option b");
                }
                'c' => {
                    let value = self.parser.optarg.clone().unwrap_or_default();
                    println!("option c with value `{value}'");
                }
                'd' => {
                    let value = self.parser.optarg.clone().unwrap_or_default();
                    println!("option d with value `{value}'");
                }
                '?' => {}
                other => {
                    println!("?? getopt returned character code 0{:o} ??", other as u32);
                }
            }
        }

        if self.parser.optind < args.len() {
            print!("non-option ARGV-elements: ");
            while self.parser.optind < args.len() {
                print!("{} ", args[self.parser.optind]);
                self.parser.optind += 1;
            }
            println!();
        }

        0
    }
}
