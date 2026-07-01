use std::io::{self, Write};

use crate::version_etc::VersionEtc;

/// FSF-flavored helpers for emitting version information.
///
/// This module is intentionally small: it layers package defaults and
/// convenience wrappers on top of `VersionEtc`.
pub struct VersionEtcFsf;

impl VersionEtcFsf {
    /// Write standard version output using the shared `VersionEtc` formatter.
    ///
    /// This is the Rust-style entry point for callers that want the behavior
    /// traditionally associated with the FSF convenience wrapper.
    pub fn write<W: Write>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        version: &str,
        authors: &[&str],
    ) -> io::Result<()> {
        VersionEtc::write(stream, command_name, package, version, authors)
    }

    /// Write version output from an author array.
    ///
    /// This mirrors the array-oriented convenience shape while staying within
    /// the Rust API surface.
    pub fn write_from_array<W: Write>(
        stream: &mut W,
        command_name: Option<&str>,
        package: &str,
        version: &str,
        authors: &[&str],
    ) -> io::Result<()> {
        VersionEtc::write_from_array(stream, command_name, package, version, authors)
    }

    /// Emit bug-reporting information using the shared formatter.
    pub fn emit_bug_reporting_address<W: Write>(
        stream: &mut W,
        package_name: &str,
        package_bugreport: &str,
        package_url: Option<&str>,
        packager: Option<&str>,
        packager_bug_reports: Option<&str>,
    ) -> io::Result<()> {
        VersionEtc::emit_bug_reporting_address(
            stream,
            package_name,
            package_bugreport,
            package_url,
            packager,
            packager_bug_reports,
        )
    }
}
