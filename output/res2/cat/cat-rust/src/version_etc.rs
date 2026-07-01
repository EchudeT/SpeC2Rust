use std::io::{self, Write};

use crate::version::Version;

pub struct VersionEtc;

impl VersionEtc {
    pub fn write<W: Write>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        version: &str,
        authors: &[&str],
    ) -> io::Result<()> {
        if let Some(command_name) = command_name {
            writeln!(stream, "{} ({}) {}", command_name, package, version)?;
        } else {
            writeln!(stream, "{} {}", package, version)?;
        }

        write!(stream, "{}", Self::copyright_line())?;
        writeln!(stream)?;
        writeln!(stream)?;

        writeln!(
            stream,
            "License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>."
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

    pub fn write_from_array<W: Write>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        version: &str,
        authors: &[&str],
    ) -> io::Result<()> {
        Self::write(stream, command_name, package, version, authors)
    }

    pub fn emit_bug_reporting_address<W: Write>(
        stream: &mut W,
        package_name: &str,
        package_bugreport: &str,
        package_url: Option<&str>,
        packager: Option<&str>,
        packager_bug_reports: Option<&str>,
    ) -> io::Result<()> {
        writeln!(stream)?;
        writeln!(stream, "Report bugs to: {}", package_bugreport)?;

        if let (Some(packager), Some(packager_bug_reports)) = (packager, packager_bug_reports) {
            writeln!(stream, "Report {} bugs to: {}", packager, packager_bug_reports)?;
        }

        let home_page = package_url.unwrap_or("https://www.gnu.org/software/");
        if package_url.is_some() {
            writeln!(stream, "{} home page: <{}>", package_name, home_page)?;
        } else {
            writeln!(stream, "{} home page: <{}>", package_name, home_page)?;
        }

        writeln!(
            stream,
            "General help using GNU software: <https://www.gnu.org/gethelp/>"
        )?;
        Ok(())
    }

    fn copyright_line() -> String {
        format!("{}, {}", "(C)", Version)
    }
}
