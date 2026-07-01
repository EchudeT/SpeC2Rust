use crate::getopt::Getopt;

pub struct Getopt1;

impl Getopt1 {
    pub fn getopt_long(
        parser: &mut Getopt,
        argv: &mut [String],
        options: &str,
        opt_index: Option<&mut usize>,
    ) -> Option<char> {
        parser.internal(argv, options, None, opt_index, false)
    }

    pub fn getopt_long_only(
        parser: &mut Getopt,
        argv: &mut [String],
        options: &str,
        opt_index: Option<&mut usize>,
    ) -> Option<char> {
        parser.internal(argv, options, None, opt_index, true)
    }

    pub fn main(argv: &[String]) -> i32 {
        let mut parser = Getopt::default();
        let mut args = argv.to_vec();
        let mut digit_optind = 0usize;

        loop {
            let this_option_optind = if parser.optind != 0 { parser.optind } else { 1 };
            let mut option_index = 0usize;

            let c = Self::getopt_long(
                &mut parser,
                &mut args,
                "abc:d:0123456789",
                Some(&mut option_index),
            );

            match c {
                None => break,
                Some('0')
                | Some('1')
                | Some('2')
                | Some('3')
                | Some('4')
                | Some('5')
                | Some('6')
                | Some('7')
                | Some('8')
                | Some('9') => {
                    if digit_optind != 0 && digit_optind != this_option_optind {
                        println!("digits occur in two different argv-elements.");
                    }
                    digit_optind = this_option_optind;
                    println!("option {}", c.unwrap());
                }
                Some('a') => {
                    println!("option a");
                }
                Some('b') => {
                    println!("option b");
                }
                Some('c') => {
                    let value = parser.optarg.clone().unwrap_or_default();
                    println!("option c with value `{value}`");
                }
                Some('d') => {
                    let value = parser.optarg.clone().unwrap_or_default();
                    println!("option d with value `{value}`");
                }
                Some('?') => {}
                Some(other) => {
                    println!("?? getopt returned character code 0{:o} ??", other as u32);
                }
            }
        }

        if parser.optind < args.len() {
            print!("non-option ARGV-elements: ");
            while parser.optind < args.len() {
                print!("{} ", args[parser.optind]);
                parser.optind += 1;
            }
            println!();
        }

        0
    }
}
