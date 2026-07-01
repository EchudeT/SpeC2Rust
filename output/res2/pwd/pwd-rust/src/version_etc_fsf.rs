use std::io::{self, Write};

use crate::version_etc::VersionEtc;

pub struct VersionEtcFsf;

impl VersionEtcFsf {
    pub fn print<W: Write>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        version: &str,
        authors: &[&str],
    ) -> io::Result<()> {
        VersionEtc::print_with_slice(stream, command_name, package, version, authors)
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
            .map(|author| author.as_ref().to_owned())
            .collect();
        let borrowed: Vec<&str> = owned.iter().map(String::as_str).collect();
        VersionEtc::print_with_slice(stream, command_name, package, version, &borrowed)
    }

    pub fn print_with_optional_authors<W, I, S>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        version: &str,
        authors: I,
    ) -> io::Result<()>
    where
        W: Write,
        I: IntoIterator<Item = Option<S>>,
        S: AsRef<str>,
    {
        let owned: Vec<String> = authors
            .into_iter()
            .take_while(|author| author.is_some())
            .flatten()
            .map(|author| author.as_ref().to_owned())
            .collect();
        let borrowed: Vec<&str> = owned.iter().map(String::as_str).collect();
        VersionEtc::print_with_slice(stream, command_name, package, version, &borrowed)
    }

    pub fn emit_bug_reporting_address<W: Write>(
        stream: &mut W,
        package_name: &str,
        package_url: Option<&str>,
        bug_report: &str,
        packager: Option<&str>,
        packager_bug_reports: Option<&str>,
    ) -> io::Result<()> {
        VersionEtc::emit_bug_reporting_address(
            stream,
            package_name,
            package_url,
            bug_report,
            packager,
            packager_bug_reports,
        )
    }
}
