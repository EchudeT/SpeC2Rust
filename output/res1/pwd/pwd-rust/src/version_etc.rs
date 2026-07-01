use std::io::{self, Write};

pub struct VersionEtc;

impl VersionEtc {
    pub const COPYRIGHT_YEAR: &str = "2025";
    pub const GPL_URL: &str = "https://gnu.org/licenses/gpl.html";
    pub const DEFAULT_BUG_REPORT_URL: &str = "https://www.gnu.org/gethelp/";

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

        write!(
            stream,
            "Copyright {} {}\n",
            "(C)",
            Self::COPYRIGHT_YEAR
        )?;

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
                writeln!(
                    stream,
                    "Written by {}, {}, {},\nand {}.",
                    authors[0], authors[1], authors[2], authors[3]
                )?;
            }
            5 => {
                writeln!(
                    stream,
                    "Written by {}, {}, {},\n{}, and {}.",
                    authors[0], authors[1], authors[2], authors[3], authors[4]
                )?;
            }
            6 => {
                writeln!(
                    stream,
                    "Written by {}, {}, {},\n{}, {}, and {}.",
                    authors[0], authors[1], authors[2], authors[3], authors[4], authors[5]
                )?;
            }
            7 => {
                writeln!(
                    stream,
                    "Written by {}, {}, {},\n{}, {}, {}, and {}.",
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
                writeln!(
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
                )?;
            }
            9 => {
                writeln!(
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
                )?;
            }
            _ => {
                writeln!(
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
                )?;
            }
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
        let collected: Vec<String> = authors.into_iter().map(|author| author.as_ref().to_owned()).collect();
        let borrowed: Vec<&str> = collected.iter().map(String::as_str).collect();
        Self::write(stream, command_name, package, version, &borrowed)
    }

    pub fn write_bug_reporting_address<W: Write>(
        stream: &mut W,
        package_name: &str,
        bug_report: &str,
        package_url: Option<&str>,
        packager: Option<&str>,
        packager_bug_reports: Option<&str>,
    ) -> io::Result<()> {
        writeln!(stream)?;
        writeln!(stream, "Report bugs to: {bug_report}")?;

        if let (Some(packager), Some(packager_bug_reports)) = (packager, packager_bug_reports) {
            writeln!(stream, "Report {packager} bugs to: {packager_bug_reports}")?;
        }

        let home_page = package_url.unwrap_or("");
        if home_page.is_empty() {
            writeln!(stream, "{package_name} home page: <>")?;
        } else {
            writeln!(stream, "{package_name} home page: <{home_page}>")?;
        }

        writeln!(
            stream,
            "General help using GNU software: <{}>",
            Self::DEFAULT_BUG_REPORT_URL
        )?;

        Ok(())
    }
}
