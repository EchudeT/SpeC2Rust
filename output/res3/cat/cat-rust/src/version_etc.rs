use std::io::{self, Write};

use crate::version::Version;

pub struct VersionEtc;

impl VersionEtc {
    pub const DEFAULT_LICENSE_URL: &'static str = "https://gnu.org/licenses/gpl.html";
    pub const DEFAULT_HELP_URL: &'static str = "https://www.gnu.org/gethelp/";
    pub const DEFAULT_COPYRIGHT_MARK: &'static str = "(C)";
    pub const DEFAULT_COPYRIGHT_YEAR: &'static str = "2025";

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
            Self::DEFAULT_COPYRIGHT_MARK,
            Self::DEFAULT_COPYRIGHT_YEAR
        )?;
        writeln!(stream)?;
        write!(
            stream,
            "License GPLv3+: GNU GPL version 3 or later <{}>.\n\
This is free software: you are free to change and redistribute it.\n\
There is NO WARRANTY, to the extent permitted by law.\n",
            Self::DEFAULT_LICENSE_URL
        )?;
        writeln!(stream)?;

        match authors.len() {
            0 => {}
            1 => writeln!(stream, "Written by {}.", authors[0])?,
            2 => writeln!(stream, "Written by {} and {}.", authors[0], authors[1])?,
            3 => writeln!(
                stream,
                "Written by {}, {}, and {}.",
                authors[0],
                authors[1],
                authors[2]
            )?,
            4 => writeln!(
                stream,
                "Written by {}, {}, {},\nand {}.",
                authors[0],
                authors[1],
                authors[2],
                authors[3]
            )?,
            5 => writeln!(
                stream,
                "Written by {}, {}, {},\n{}, and {}.",
                authors[0],
                authors[1],
                authors[2],
                authors[3],
                authors[4]
            )?,
            6 => writeln!(
                stream,
                "Written by {}, {}, {},\n{}, {}, and {}.",
                authors[0],
                authors[1],
                authors[2],
                authors[3],
                authors[4],
                authors[5]
            )?,
            7 => writeln!(
                stream,
                "Written by {}, {}, {},\n{}, {}, {}, and {}.",
                authors[0],
                authors[1],
                authors[2],
                authors[3],
                authors[4],
                authors[5],
                authors[6]
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

    pub fn write_default<W: Write>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        authors: &[&str],
    ) -> io::Result<()> {
        Self::write(stream, command_name, package, Version::as_str(), authors)
    }

    pub fn emit_bug_reporting_address<W: Write>(
        stream: &mut W,
        package_name: &str,
        package_bugreport: &str,
        package_url: Option<&str>,
        package_id_for_gnu_url: &str,
        packager_name: Option<&str>,
        packager_bug_reports: Option<&str>,
    ) -> io::Result<()> {
        writeln!(stream)?;
        writeln!(stream, "Report bugs to: {package_bugreport}")?;

        if let (Some(name), Some(address)) = (packager_name, packager_bug_reports) {
            writeln!(stream, "Report {name} bugs to: {address}")?;
        }

        let home_page = package_url
            .map(str::to_owned)
            .unwrap_or_else(|| format!("https://www.gnu.org/software/{package_id_for_gnu_url}/"));

        writeln!(stream, "{package_name} home page: <{home_page}>")?;
        writeln!(
            stream,
            "General help using GNU software: <{}>",
            Self::DEFAULT_HELP_URL
        )?;
        Ok(())
    }
}
