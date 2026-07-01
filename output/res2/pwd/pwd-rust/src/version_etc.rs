use std::io::{self, Write};

pub struct VersionEtc;

impl VersionEtc {
    pub fn print<W: Write>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        version: &str,
        authors: &[&str],
    ) -> io::Result<()> {
        Self::print_with_slice(stream, command_name, package, version, authors)
    }

    pub fn print_with_slice<W: Write>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        version: &str,
        authors: &[&str],
    ) -> io::Result<()> {
        Self::print_core(stream, command_name, package, version, authors)
    }

    pub fn print_with_iter<W, I, S>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        version: &str,
        authors: I,
    ) -> io::Result<()>
    where
        W: Write,
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let owned: Vec<String> = authors
            .into_iter()
            .take(10)
            .map(|author| author.as_ref().to_owned())
            .collect();
        let borrowed: Vec<&str> = owned.iter().map(String::as_str).collect();
        Self::print_core(stream, command_name, package, version, &borrowed)
    }

    pub fn emit_bug_reporting_address<W: Write>(
        stream: &mut W,
        package_name: &str,
        package_url: Option<&str>,
        bug_report: &str,
        packager: Option<&str>,
        packager_bug_reports: Option<&str>,
    ) -> io::Result<()> {
        writeln!(stream)?;
        writeln!(stream, "Report bugs to: {bug_report}")?;
        if let (Some(packager_name), Some(packager_reports)) = (packager, packager_bug_reports) {
            writeln!(stream, "Report {packager_name} bugs to: {packager_reports}")?;
        }
        let home_page = match package_url {
            Some(url) => url.to_owned(),
            None => format!("https://www.gnu.org/software/{package_name}/"),
        };
        writeln!(stream, "{package_name} home page: <{home_page}>")?;
        writeln!(
            stream,
            "General help using GNU software: <https://www.gnu.org/gethelp/>"
        )
    }

    fn print_core<W: Write>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        version: &str,
        authors: &[&str],
    ) -> io::Result<()> {
        if let Some(command_name) = command_name {
            writeln!(stream, "{command_name} ({package}) {version}")?;
        } else {
            writeln!(stream, "{package} {version}")?;
        }

        write!(
            stream,
            "Copyright (C) {}\n",
            env!("CARGO_PKG_VERSION_MAJOR")
        )?;
        writeln!(stream)?;
        write!(
            stream,
            "License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.\n\
This is free software: you are free to change and redistribute it.\n\
There is NO WARRANTY, to the extent permitted by law.\n"
        )?;
        writeln!(stream)?;

        match authors.len() {
            0 => {}
            1 => {
                writeln!(stream, "Written by {}.", authors[0])?;
            }
            2 => {
                writeln!(stream, "Written by {} and {}.", authors[0], authors[1])?;
            }
            3 => {
                writeln!(
                    stream,
                    "Written by {}, {}, and {}.",
                    authors[0], authors[1], authors[2]
                )?;
            }
            4 => {
                write!(
                    stream,
                    "Written by {}, {}, {},\nand {}.\n",
                    authors[0], authors[1], authors[2], authors[3]
                )?;
            }
            5 => {
                write!(
                    stream,
                    "Written by {}, {}, {},\n{}, and {}.\n",
                    authors[0], authors[1], authors[2], authors[3], authors[4]
                )?;
            }
            6 => {
                write!(
                    stream,
                    "Written by {}, {}, {},\n{}, {}, and {}.\n",
                    authors[0], authors[1], authors[2], authors[3], authors[4], authors[5]
                )?;
            }
            7 => {
                write!(
                    stream,
                    "Written by {}, {}, {},\n{}, {}, {}, and {}.\n",
                    authors[0],
                    authors[1],
                    authors[2],
                    authors[3],
                    authors[4],
                    authors[5],
                    authors[6]
                )?;
            }
            8 => {
                write!(
                    stream,
                    "Written by {}, {}, {},\n{}, {}, {}, {},\nand {}.\n",
                    authors[0],
                    authors[1],
                    authors[2],
                    authors[3],
                    authors[4],
                    authors[5],
                    authors[6],
                    authors[7]
                )?;
            }
            9 => {
                write!(
                    stream,
                    "Written by {}, {}, {},\n{}, {}, {}, {},\n{}, and {}.\n",
                    authors[0],
                    authors[1],
                    authors[2],
                    authors[3],
                    authors[4],
                    authors[5],
                    authors[6],
                    authors[7],
                    authors[8]
                )?;
            }
            _ => {
                write!(
                    stream,
                    "Written by {}, {}, {},\n{}, {}, {}, {},\n{}, {}, and others.\n",
                    authors[0],
                    authors[1],
                    authors[2],
                    authors[3],
                    authors[4],
                    authors[5],
                    authors[6],
                    authors[7],
                    authors[8]
                )?;
            }
        }

        Ok(())
    }
}
