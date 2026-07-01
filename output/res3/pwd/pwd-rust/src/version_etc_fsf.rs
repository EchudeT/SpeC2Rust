use std::io::{self, Write};

use crate::version_etc::VersionEtc;

pub struct VersionEtcFsf;

impl VersionEtcFsf {
    pub fn emit<W: Write>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        version: &str,
        authors: &[&str],
    ) -> io::Result<()> {
        VersionEtc::emit(stream, command_name, package, version, authors)
    }

    pub fn emit_from_array<W: Write>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        version: &str,
        authors: &[&str],
    ) -> io::Result<()> {
        VersionEtc::emit_from_array(stream, command_name, package, version, authors)
    }

    pub fn emit_variadic<W, I>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        version: &str,
        authors: I,
    ) -> io::Result<()>
    where
        W: Write,
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let owned: Vec<String> = authors
            .into_iter()
            .map(|author| author.as_ref().to_owned())
            .collect();
        let borrowed: Vec<&str> = owned.iter().map(String::as_str).collect();
        VersionEtc::emit(stream, command_name, package, version, &borrowed)
    }

    pub fn emit_bug_reporting_address<W: Write>(stream: &mut W) -> io::Result<()> {
        VersionEtc::emit_bug_reporting_address(stream)
    }
}
