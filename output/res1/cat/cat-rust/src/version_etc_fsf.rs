use std::io::{self, Write};

use crate::version_etc::VersionEtc;

pub struct VersionEtcFsf;

impl VersionEtcFsf {
    pub fn write<W: Write>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        version: &str,
        authors: &[&str],
    ) -> io::Result<()> {
        VersionEtc::write(stream, command_name, package, version, authors)
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
        VersionEtc::write_from_iter(stream, command_name, package, version, authors)
    }

    pub fn bug_reporting_text(
        package_name: Option<&str>,
        bug_report: Option<&str>,
        package_url: Option<&str>,
    ) -> String {
        VersionEtc::bug_reporting_text(package_name, bug_report, package_url)
    }
}
