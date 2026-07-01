use std::io::{self, Write};

use crate::version::Version;
use crate::version_etc::VersionEtc;

pub struct VersionEtcFsf;

impl VersionEtcFsf {
    pub fn write<W: Write>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        authors: &[&str],
    ) -> io::Result<()> {
        VersionEtc::write_default(stream, command_name, package, authors)
    }

    pub fn write_with_version<W: Write>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        version: &str,
        authors: &[&str],
    ) -> io::Result<()> {
        VersionEtc::write(stream, command_name, package, version, authors)
    }

    pub fn write_current_version<W: Write>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        authors: &[&str],
    ) -> io::Result<()> {
        VersionEtc::write(stream, command_name, package, Version::as_str(), authors)
    }

    pub fn emit_bug_reporting_address<W: Write>(stream: &mut W, package_name: &str) -> io::Result<()> {
        VersionEtc::emit_bug_reporting_address(
            stream,
            package_name,
            "bug-coreutils@gnu.org",
            Some("https://www.gnu.org/software/coreutils/"),
            package_name,
            None,
            None,
        )
    }
}
