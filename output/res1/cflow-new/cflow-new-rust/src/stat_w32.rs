use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct StatW32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WindowsFileTime {
    pub low_date_time: u32,
    pub high_date_time: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i32,
}

impl StatW32 {
    const WINDOWS_TICKS_PER_SECOND: u64 = 10_000_000;
    const NANOS_PER_TICK: u32 = 100;
    const DAYS_BETWEEN_1601_AND_1970: u64 = 134_774;
    const SECONDS_PER_DAY: u64 = 86_400;
    const EPOCH_OFFSET_TICKS: u64 = Self::DAYS_BETWEEN_1601_AND_1970
        * Self::SECONDS_PER_DAY
        * Self::WINDOWS_TICKS_PER_SECOND;

    pub fn filetime_to_unix_seconds(filetime: WindowsFileTime) -> i64 {
        let since_1601 = Self::filetime_ticks(filetime);
        if since_1601 == 0 {
            0
        } else {
            let since_1970 = since_1601.saturating_sub(Self::EPOCH_OFFSET_TICKS);
            (since_1970 / Self::WINDOWS_TICKS_PER_SECOND) as i64
        }
    }

    pub fn filetime_to_timespec(filetime: WindowsFileTime) -> Timespec {
        let since_1601 = Self::filetime_ticks(filetime);
        if since_1601 == 0 {
            Timespec {
                tv_sec: 0,
                tv_nsec: 0,
            }
        } else {
            let since_1970 = since_1601.saturating_sub(Self::EPOCH_OFFSET_TICKS);
            Timespec {
                tv_sec: (since_1970 / Self::WINDOWS_TICKS_PER_SECOND) as i64,
                tv_nsec: ((since_1970 % Self::WINDOWS_TICKS_PER_SECOND) as u32
                    * Self::NANOS_PER_TICK) as i32,
            }
        }
    }

    pub fn filetime_to_system_time(filetime: WindowsFileTime) -> Option<SystemTime> {
        let ticks = Self::filetime_ticks(filetime);
        if ticks == 0 {
            return None;
        }

        let ts = Self::filetime_to_timespec(filetime);
        Some(UNIX_EPOCH + Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32))
    }

    fn filetime_ticks(filetime: WindowsFileTime) -> u64 {
        ((filetime.high_date_time as u64) << 32) | filetime.low_date_time as u64
    }
}
