use std::fs::Metadata;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct StatTime;

impl StatTime {
    pub fn accessed(metadata: &Metadata) -> Option<SystemTime> {
        metadata.accessed().ok()
    }

    pub fn modified(metadata: &Metadata) -> Option<SystemTime> {
        metadata.modified().ok()
    }

    pub fn created(metadata: &Metadata) -> Option<SystemTime> {
        metadata.created().ok()
    }

    pub fn changed(metadata: &Metadata) -> Option<SystemTime> {
        Self::created(metadata)
            .or_else(|| Self::modified(metadata))
            .or_else(|| Self::accessed(metadata))
    }

    pub fn accessed_duration(metadata: &Metadata) -> Option<Duration> {
        Self::accessed(metadata)
            .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
    }

    pub fn modified_duration(metadata: &Metadata) -> Option<Duration> {
        Self::modified(metadata)
            .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
    }

    pub fn created_duration(metadata: &Metadata) -> Option<Duration> {
        Self::created(metadata)
            .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
    }

    pub fn changed_duration(metadata: &Metadata) -> Option<Duration> {
        Self::changed(metadata)
            .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
    }

    pub fn nanoseconds_part(time: SystemTime) -> Option<u32> {
        time.duration_since(UNIX_EPOCH).ok().map(|d| d.subsec_nanos())
    }

    pub fn accessed_nanoseconds(metadata: &Metadata) -> Option<u32> {
        Self::accessed(metadata).and_then(Self::nanoseconds_part)
    }

    pub fn modified_nanoseconds(metadata: &Metadata) -> Option<u32> {
        Self::modified(metadata).and_then(Self::nanoseconds_part)
    }

    pub fn created_nanoseconds(metadata: &Metadata) -> Option<u32> {
        Self::created(metadata).and_then(Self::nanoseconds_part)
    }

    pub fn changed_nanoseconds(metadata: &Metadata) -> Option<u32> {
        Self::changed(metadata).and_then(Self::nanoseconds_part)
    }
}
