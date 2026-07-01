use crate::getopt::Getopt;

#[derive(Clone, Debug, Default)]
pub struct Getopt1 {
    parser: Getopt,
}

impl Getopt1 {
    pub fn getopt_long(
        &mut self,
        argv: &mut [String],
        options: &str,
    ) -> Option<Result<char, char>> {
        self.parser.internal(argv, options, false)
    }

    pub fn getopt_long_only(
        &mut self,
        argv: &mut [String],
        options: &str,
    ) -> Option<Result<char, char>> {
        self.parser.internal(argv, options, true)
    }

    pub fn main(&mut self, argv: &[String]) -> i32 {
        let mut args = argv.to_vec();
        self.parser.store_args_and_env(argv);

        let mut digit_optind = 0usize;

        loop {
            let this_option_optind = 1usize;
            let mut option_index = 0usize;

            let c = self.getopt_long(&mut args, "abc:d:0123456789");
            let Some(result) = c else {
                break;
            };

            match result {
                Ok('\0') => {
                    let long_name = match option_index {
                        0 => "add",
                        1 => "append",
                        2 => "delete",
                        3 => "verbose",
                        4 => "create",
                        5 => "file",
                        _ => "",
                    };
                    print!("option {}", long_name);
                    println!();
                }
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
                    println!("option c with value ``");
                }
                Ok('d') => {
                    println!("option d with value ``");
                }
                Err('?') => {}
                Ok(other) => {
                    println!("?? getopt returned character code 0{:o} ??", other as u32);
                }
                Err(other) => {
                    println!("?? getopt returned character code 0{:o} ??", other as u32);
                }
            }

            let _ = &mut option_index;
        }

        if !args.is_empty() {
            println!("non-option ARGV-elements: ");
        }

        0
    }
}
