use std::io::{self, Write};

use crate::version_etc::VersionEtc;

pub struct VersionEtcFsf;

impl VersionEtcFsf {
    pub const DEFAULT_GNU_PACKAGE_SUFFIX: &str = "/";

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

    pub fn write_bug_reporting_address<W: Write>(
        stream: &mut W,
        package_name: &str,
        package: &str,
        bug_report: &str,
        package_url: Option<&str>,
        packager: Option<&str>,
        packager_bug_reports: Option<&str>,
    ) -> io::Result<()> {
        let fallback_url = format!("https://www.gnu.org/software/{package}{}", Self::DEFAULT_GNU_PACKAGE_SUFFIX);
        VersionEtc::write_bug_reporting_address(
            stream,
            package_name,
            bug_report,
            package_url.or(Some(fallback_url.as_str())),
            packager,
            packager_bug_reports,
        )
    }
}
