use std::io::{self, Write};

pub struct VersionEtc;

impl VersionEtc {
    const COPYRIGHT_YEAR: &'static str = "2025";
    const COPYRIGHT_FORMAT: &'static str = "Copyright %s %s\n";
    const GPL_URL: &'static str = "https://gnu.org/licenses/gpl.html";
    const DEFAULT_BUG_REPORT: &'static str = "bug-coreutils@gnu.org";
    const DEFAULT_PACKAGE_NAME: &'static str = "GNU";
    const DEFAULT_PACKAGE_URL: &'static str = "https://www.gnu.org/software/";

    pub fn write<W: Write>(
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

        let copyright = Self::COPYRIGHT_FORMAT
            .replacen("%s", "(C)", 1)
            .replacen("%s", Self::COPYRIGHT_YEAR, 1);
        write!(stream, "{copyright}")?;
        writeln!(stream)?;

        writeln!(
            stream,
            "License GPLv3+: GNU GPL version 3 or later <{}>.",
            Self::GPL_URL
        )?;
        writeln!(
            stream,
            "This is free software: you are free to change and redistribute it."
        )?;
        writeln!(
            stream,
            "There is NO WARRANTY, to the extent permitted by law."
        )?;
        writeln!(stream)?;

        let author_lines = Self::format_authors(authors);
        if !author_lines.is_empty() {
            write!(stream, "{author_lines}")?;
        }

        Ok(())
    }

    pub fn write_from_iter<W, I, S>(
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
        let collected: Vec<String> = authors.into_iter().map(|a| a.as_ref().to_owned()).collect();
        let refs: Vec<&str> = collected.iter().map(String::as_str).collect();
        Self::write(stream, command_name, package, version, &refs)
    }

    pub fn bug_reporting_text(
        package_name: Option<&str>,
        bug_report: Option<&str>,
        package_url: Option<&str>,
    ) -> String {
        let package_name = package_name.unwrap_or(Self::DEFAULT_PACKAGE_NAME);
        let bug_report = bug_report.unwrap_or(Self::DEFAULT_BUG_REPORT);
        let package_url = package_url.unwrap_or(Self::DEFAULT_PACKAGE_URL);

        let mut out = String::new();
        out.push('\n');
        out.push_str(&format!("Report bugs to: {bug_report}\n"));
        out.push_str(&format!("{package_name} home page: <{package_url}>\n"));
        out.push_str("General help using GNU software: <https://www.gnu.org/gethelp/>\n");
        out
    }

    fn format_authors(authors: &[&str]) -> String {
        match authors.len() {
            0 => String::new(),
            1 => format!("Written by {}.\n", authors[0]),
            2 => format!("Written by {} and {}.\n", authors[0], authors[1]),
            3 => format!(
                "Written by {}, {}, and {}.\n",
                authors[0], authors[1], authors[2]
            ),
            4 => format!(
                "Written by {}, {}, {},\nand {}.\n",
                authors[0], authors[1], authors[2], authors[3]
            ),
            5 => format!(
                "Written by {}, {}, {},\n{}, and {}.\n",
                authors[0], authors[1], authors[2], authors[3], authors[4]
            ),
            6 => format!(
                "Written by {}, {}, {},\n{}, {}, and {}.\n",
                authors[0], authors[1], authors[2], authors[3], authors[4], authors[5]
            ),
            7 => format!(
                "Written by {}, {}, {},\n{}, {}, {}, and {}.\n",
                authors[0],
                authors[1],
                authors[2],
                authors[3],
                authors[4],
                authors[5],
                authors[6]
            ),
            8 => format!(
                "Written by {}, {}, {},\n{}, {}, {}, {},\nand {}.\n",
                authors[0],
                authors[1],
                authors[2],
                authors[3],
                authors[4],
                authors[5],
                authors[6],
                authors[7]
            ),
            9 => format!(
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
            ),
            _ => format!(
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
            ),
        }
    }
}
