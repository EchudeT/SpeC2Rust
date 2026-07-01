use std::fs::Metadata;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Utilities for extracting and normalizing file timestamp information.
pub struct StatTime;

impl StatTime {
    /// Returns the file access time, if the platform exposes it.
    pub fn access_time(metadata: &Metadata) -> Option<SystemTime> {
        metadata.accessed().ok()
    }

    /// Returns the file modification time, if the platform exposes it.
    pub fn modification_time(metadata: &Metadata) -> Option<SystemTime> {
        metadata.modified().ok()
    }

    /// Returns the file creation/birth time, if the platform exposes it.
    pub fn birth_time(metadata: &Metadata) -> Option<SystemTime> {
        metadata.created().ok()
    }

    /// Returns the access time as `(seconds, nanoseconds)` since the Unix epoch.
    pub fn access_time_parts(metadata: &Metadata) -> Option<(i64, i32)> {
        Self::access_time(metadata).map(Self::system_time_parts)
    }

    /// Returns the modification time as `(seconds, nanoseconds)` since the Unix epoch.
    pub fn modification_time_parts(metadata: &Metadata) -> Option<(i64, i32)> {
        Self::modification_time(metadata).map(Self::system_time_parts)
    }

    /// Returns the birth time as `(seconds, nanoseconds)` since the Unix epoch.
    pub fn birth_time_parts(metadata: &Metadata) -> Option<(i64, i32)> {
        Self::birth_time(metadata).map(Self::system_time_parts)
    }

    /// Normalizes a `(seconds, nanoseconds)` pair so that nanoseconds are in
    /// the range `0..1_000_000_000`.
    pub fn normalize_parts(seconds: i64, nanoseconds: i32) -> (i64, i32) {
        let billion = 1_000_000_000i64;
        let total_nanos = seconds as i128 * billion as i128 + nanoseconds as i128;
        let mut secs = (total_nanos / billion as i128) as i64;
        let mut nanos = (total_nanos % billion as i128) as i32;

        if nanos < 0 {
            secs -= 1;
            nanos += billion as i32;
        }

        (secs, nanos)
    }

    fn system_time_parts(time: SystemTime) -> (i64, i32) {
        match time.duration_since(UNIX_EPOCH) {
            Ok(duration) => (
                duration.as_secs() as i64,
                duration.subsec_nanos() as i32,
            ),
            Err(err) => {
                let duration: Duration = err.duration();
                Self::normalize_parts(
                    -(duration.as_secs() as i64),
                    -(duration.subsec_nanos() as i32),
                )
            }
        }
    }
}
