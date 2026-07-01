use std::fs::Metadata;
use std::time::Duration;

/// Utilities for extracting normalized filesystem timestamps from `Metadata`.
pub struct StatTime;

impl StatTime {
    fn clamp_nanos(nanos: i64) -> u32 {
        nanos.clamp(0, 999_999_999) as u32
    }

    #[cfg(unix)]
    fn unix_atime_parts(metadata: &Metadata) -> (i64, i64) {
        use std::os::unix::fs::MetadataExt;
        (metadata.atime(), metadata.atime_nsec())
    }

    #[cfg(unix)]
    fn unix_mtime_parts(metadata: &Metadata) -> (i64, i64) {
        use std::os::unix::fs::MetadataExt;
        (metadata.mtime(), metadata.mtime_nsec())
    }

    #[cfg(unix)]
    fn unix_ctime_parts(metadata: &Metadata) -> (i64, i64) {
        use std::os::unix::fs::MetadataExt;
        (metadata.ctime(), metadata.ctime_nsec())
    }

    #[cfg(windows)]
    fn windows_ticks_to_duration(ticks: u64) -> Duration {
        const WINDOWS_TO_UNIX_EPOCH_100NS: u64 = 116_444_736_000_000_000;
        if ticks <= WINDOWS_TO_UNIX_EPOCH_100NS {
            return Duration::ZERO;
        }
        let unix_100ns = ticks - WINDOWS_TO_UNIX_EPOCH_100NS;
        Duration::new(
            unix_100ns / 10_000_000,
            ((unix_100ns % 10_000_000) * 100) as u32,
        )
    }

    #[cfg(windows)]
    fn windows_atime(metadata: &Metadata) -> Duration {
        use std::os::windows::fs::MetadataExt;
        Self::windows_ticks_to_duration(metadata.last_access_time())
    }

    #[cfg(windows)]
    fn windows_mtime(metadata: &Metadata) -> Duration {
        use std::os::windows::fs::MetadataExt;
        Self::windows_ticks_to_duration(metadata.last_write_time())
    }

    #[cfg(windows)]
    fn windows_ctime(metadata: &Metadata) -> Duration {
        use std::os::windows::fs::MetadataExt;
        Self::windows_ticks_to_duration(metadata.creation_time())
    }

    #[cfg(any(unix, windows))]
    pub fn access_time(metadata: &Metadata) -> Duration {
        #[cfg(unix)]
        {
            let (secs, nanos) = Self::unix_atime_parts(metadata);
            return Duration::new(secs.max(0) as u64, Self::clamp_nanos(nanos));
        }

        #[cfg(windows)]
        {
            Self::windows_atime(metadata)
        }
    }

    #[cfg(not(any(unix, windows)))]
    pub fn access_time(_metadata: &Metadata) -> Duration {
        Duration::ZERO
    }

    #[cfg(any(unix, windows))]
    pub fn modification_time(metadata: &Metadata) -> Duration {
        #[cfg(unix)]
        {
            let (secs, nanos) = Self::unix_mtime_parts(metadata);
            return Duration::new(secs.max(0) as u64, Self::clamp_nanos(nanos));
        }

        #[cfg(windows)]
        {
            Self::windows_mtime(metadata)
        }
    }

    #[cfg(not(any(unix, windows)))]
    pub fn modification_time(_metadata: &Metadata) -> Duration {
        Duration::ZERO
    }

    #[cfg(any(unix, windows))]
    pub fn status_change_time(metadata: &Metadata) -> Duration {
        #[cfg(unix)]
        {
            let (secs, nanos) = Self::unix_ctime_parts(metadata);
            return Duration::new(secs.max(0) as u64, Self::clamp_nanos(nanos));
        }

        #[cfg(windows)]
        {
            Self::windows_ctime(metadata)
        }
    }

    #[cfg(not(any(unix, windows)))]
    pub fn status_change_time(_metadata: &Metadata) -> Duration {
        Duration::ZERO
    }

    #[cfg(unix)]
    pub fn birth_time(metadata: &Metadata) -> Option<Duration> {
        use std::os::unix::fs::MetadataExt;

        #[cfg(any(
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd",
            target_os = "dragonfly",
            target_os = "macos",
            target_os = "ios"
        ))]
        {
            let secs = metadata.birthtime();
            if secs < 0 {
                None
            } else {
                Some(Duration::new(secs as u64, 0))
            }
        }

        #[cfg(not(any(
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd",
            target_os = "dragonfly",
            target_os = "macos",
            target_os = "ios"
        )))]
        {
            let _ = metadata;
            None
        }
    }

    #[cfg(windows)]
    pub fn birth_time(metadata: &Metadata) -> Option<Duration> {
        Some(Self::windows_ctime(metadata))
    }

    #[cfg(not(any(unix, windows)))]
    pub fn birth_time(_metadata: &Metadata) -> Option<Duration> {
        None
    }

    pub fn access_time_nanos(metadata: &Metadata) -> i128 {
        let d = Self::access_time(metadata);
        d.as_secs() as i128 * 1_000_000_000i128 + d.subsec_nanos() as i128
    }

    pub fn modification_time_nanos(metadata: &Metadata) -> i128 {
        let d = Self::modification_time(metadata);
        d.as_secs() as i128 * 1_000_000_000i128 + d.subsec_nanos() as i128
    }

    pub fn status_change_time_nanos(metadata: &Metadata) -> i128 {
        let d = Self::status_change_time(metadata);
        d.as_secs() as i128 * 1_000_000_000i128 + d.subsec_nanos() as i128
    }

    pub fn birth_time_nanos(metadata: &Metadata) -> Option<i128> {
        Self::birth_time(metadata)
            .map(|d| d.as_secs() as i128 * 1_000_000_000i128 + d.subsec_nanos() as i128)
    }
}
