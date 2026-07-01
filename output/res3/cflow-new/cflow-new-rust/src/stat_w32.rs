use std::fs::Metadata;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct StatW32;

impl StatW32 {
    pub fn access_time(metadata: &Metadata) -> Option<Duration> {
        metadata.accessed().ok().and_then(Self::system_time_to_duration)
    }

    pub fn modification_time(metadata: &Metadata) -> Option<Duration> {
        metadata.modified().ok().and_then(Self::system_time_to_duration)
    }

    pub fn creation_time(metadata: &Metadata) -> Option<Duration> {
        metadata.created().ok().and_then(Self::system_time_to_duration)
    }

    pub fn access_time_nanos(metadata: &Metadata) -> Option<i128> {
        Self::access_time(metadata).map(Self::duration_to_nanos)
    }

    pub fn modification_time_nanos(metadata: &Metadata) -> Option<i128> {
        Self::modification_time(metadata).map(Self::duration_to_nanos)
    }

    pub fn creation_time_nanos(metadata: &Metadata) -> Option<i128> {
        Self::creation_time(metadata).map(Self::duration_to_nanos)
    }

    fn system_time_to_duration(time: SystemTime) -> Option<Duration> {
        time.duration_since(UNIX_EPOCH).ok()
    }

    fn duration_to_nanos(duration: Duration) -> i128 {
        i128::from(duration.as_secs()) * 1_000_000_000i128 + i128::from(duration.subsec_nanos())
    }
}
