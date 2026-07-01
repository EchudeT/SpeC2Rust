use std::fs::Metadata;
use std::io;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct StatW32;

impl StatW32 {
    pub fn metadata(path: impl AsRef<Path>) -> io::Result<Metadata> {
        std::fs::metadata(path)
    }

    pub fn filetime_ticks_to_unix_seconds(ticks_100ns: u64) -> i64 {
        const WINDOWS_TO_UNIX_EPOCH_SECONDS: i64 = 11_644_473_600;
        let secs = (ticks_100ns / 10_000_000) as i64;
        secs - WINDOWS_TO_UNIX_EPOCH_SECONDS
    }

    pub fn filetime_ticks_to_system_time(ticks_100ns: u64) -> Option<SystemTime> {
        const WINDOWS_TO_UNIX_EPOCH_SECONDS: u64 = 11_644_473_600;
        const TICKS_PER_SECOND: u64 = 10_000_000;
        const NANOS_PER_TICK: u32 = 100;

        let total_secs = ticks_100ns / TICKS_PER_SECOND;
        let subsec_ticks = ticks_100ns % TICKS_PER_SECOND;
        let nanos = (subsec_ticks as u32) * NANOS_PER_TICK;

        if total_secs >= WINDOWS_TO_UNIX_EPOCH_SECONDS {
            let unix_secs = total_secs - WINDOWS_TO_UNIX_EPOCH_SECONDS;
            Some(UNIX_EPOCH + Duration::new(unix_secs, nanos))
        } else {
            let delta_secs = WINDOWS_TO_UNIX_EPOCH_SECONDS - total_secs;
            UNIX_EPOCH.checked_sub(Duration::new(delta_secs, nanos))
        }
    }
}
