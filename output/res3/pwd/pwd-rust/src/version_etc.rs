use std::io::{self, Write};

pub struct VersionEtc;

impl VersionEtc {
    pub fn emit<W: Write>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        version: &str,
        authors: &[&str],
    ) -> io::Result<()> {
        Self::emit_with_count(stream, command_name, package, version, authors, authors.len())
    }

    pub fn emit_from_array<W: Write>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        version: &str,
        authors: &[&str],
    ) -> io::Result<()> {
        Self::emit(stream, command_name, package, version, authors)
    }

    pub fn emit_variadic<W: Write, I, S>(
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
        let collected: Vec<String> = authors
            .into_iter()
            .take(10)
            .map(|author| author.as_ref().to_owned())
            .collect();
        let refs: Vec<&str> = collected.iter().map(String::as_str).collect();
        Self::emit(stream, command_name, package, version, &refs)
    }

    fn emit_with_count<W: Write>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        version: &str,
        authors: &[&str],
        n_authors: usize,
    ) -> io::Result<()> {
        match command_name {
            Some(name) => writeln!(stream, "{name} ({package}) {version}")?,
            None => writeln!(stream, "{package} {version}")?,
        }

        write!(stream, "Copyright (C) {}\n", env!("CARGO_PKG_VERSION"))?;
        writeln!(stream)?;
        write!(
            stream,
            "License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.\n\
This is free software: you are free to change and redistribute it.\n\
There is NO WARRANTY, to the extent permitted by law.\n"
        )?;
        writeln!(stream)?;

        match n_authors {
            0 => {}
            1 => writeln!(stream, "Written by {}.", authors[0])?,
            2 => writeln!(stream, "Written by {} and {}.", authors[0], authors[1])?,
            3 => writeln!(
                stream,
                "Written by {}, {}, and {}.",
                authors[0], authors[1], authors[2]
            )?,
            4 => writeln!(
                stream,
                "Written by {}, {}, {},\nand {}.",
                authors[0], authors[1], authors[2], authors[3]
            )?,
            5 => writeln!(
                stream,
                "Written by {}, {}, {},\n{}, and {}.",
                authors[0], authors[1], authors[2], authors[3], authors[4]
            )?,
            6 => writeln!(
                stream,
                "Written by {}, {}, {},\n{}, {}, and {}.",
                authors[0], authors[1], authors[2], authors[3], authors[4], authors[5]
            )?,
            7 => writeln!(
                stream,
                "Written by {}, {}, {},\n{}, {}, {}, and {}.",
                authors[0], authors[1], authors[2], authors[3], authors[4], authors[5], authors[6]
            )?,
            8 => writeln!(
                stream,
                "Written by {}, {}, {},\n{}, {}, {}, {},\nand {}.",
                authors[0],
                authors[1],
                authors[2],
                authors[3],
                authors[4],
                authors[5],
                authors[6],
                authors[7]
            )?,
            9 => writeln!(
                stream,
                "Written by {}, {}, {},\n{}, {}, {}, {},\n{}, and {}.",
                authors[0],
                authors[1],
                authors[2],
                authors[3],
                authors[4],
                authors[5],
                authors[6],
                authors[7],
                authors[8]
            )?,
            _ => writeln!(
                stream,
                "Written by {}, {}, {},\n{}, {}, {}, {},\n{}, {}, and others.",
                authors[0],
                authors[1],
                authors[2],
                authors[3],
                authors[4],
                authors[5],
                authors[6],
                authors[7],
                authors[8]
            )?,
        }

        Ok(())
    }

    pub fn emit_bug_reporting_address<W: Write>(stream: &mut W) -> io::Result<()> {
        let package_name = env!("CARGO_PKG_NAME");
        let homepage = env!("CARGO_PKG_HOMEPAGE");
        let bug_report = option_env!("CARGO_PKG_REPOSITORY")
            .or(option_env!("CARGO_PKG_HOMEPAGE"))
            .unwrap_or("");

        writeln!(stream)?;
        if !bug_report.is_empty() {
            writeln!(stream, "Report bugs to: {bug_report}")?;
        }
        if !homepage.is_empty() {
            writeln!(stream, "{package_name} home page: <{homepage}>")?;
        } else {
            writeln!(stream, "{package_name} home page: <>")?;
        }
        writeln!(
            stream,
            "General help using GNU software: <https://www.gnu.org/gethelp/>"
        )?;
        Ok(())
    }
}
