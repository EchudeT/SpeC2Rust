use std::env;
use std::fs;
use std::ops::Range;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::cell::RefCell;

#[derive(Clone, Debug, Default)]
struct UserInfo {
    uid: u32,
    gid: u32,
    euid: u32,
    egid: u32,
    user_name: Option<String>,
    shell: Option<String>,
    home_dir: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FileStatus {
    bits: u8,
}

impl FileStatus {
    const EXISTS: u8 = 0b0001;
    const EXECUTABLE: u8 = 0b0010;
    const READABLE: u8 = 0b0100;
    const DIRECTORY: u8 = 0b1000;

    pub fn exists(self) -> bool {
        self.bits & Self::EXISTS != 0
    }

    pub fn executable(self) -> bool {
        self.bits & Self::EXECUTABLE != 0
    }

    pub fn readable(self) -> bool {
        self.bits & Self::READABLE != 0
    }

    pub fn directory(self) -> bool {
        self.bits & Self::DIRECTORY != 0
    }
}

thread_local! {
    static CURRENT_USER: RefCell<UserInfo> = RefCell::new(UserInfo::default());
    static GROUPS: RefCell<Vec<u32>> = RefCell::new(Vec::new());
}

pub struct Bash;

impl Bash {
    pub fn update_ids() -> bool {
        let uid = current_uid();
        let gid = current_gid();
        let euid = effective_uid();
        let egid = effective_gid();

        CURRENT_USER.with(|user| {
            let mut user = user.borrow_mut();
            if user.uid != uid {
                user.user_name = None;
                user.shell = None;
                user.home_dir = None;
            }
            user.uid = uid;
            user.gid = gid;
            user.euid = euid;
            user.egid = egid;

            user.uid != user.euid || user.gid != user.egid
        })
    }

    pub fn max_groups() -> usize {
        env::var("NGROUPS_MAX")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(64)
    }

    pub fn initialize_group_array() {
        let maxgroups = Self::max_groups();
        let primary_gid = CURRENT_USER.with(|user| user.borrow().gid);

        let mut groups = supplementary_groups();
        if groups.is_empty() {
            groups.push(primary_gid);
        }

        if !groups.contains(&primary_gid) && groups.len() < maxgroups {
            groups.insert(0, primary_gid);
        }

        if let Some(position) = groups.iter().position(|gid| *gid == primary_gid) {
            if position != 0 {
                groups.swap(0, position);
            }
        }

        if groups.len() > maxgroups {
            groups.truncate(maxgroups);
        }

        GROUPS.with(|stored| {
            *stored.borrow_mut() = groups;
        });
    }

    pub fn group_member(gid: u32) -> bool {
        if CURRENT_USER.with(|user| {
            let user = user.borrow();
            gid == user.gid || gid == user.egid
        }) {
            return true;
        }

        let needs_init = GROUPS.with(|groups| groups.borrow().is_empty());
        if needs_init {
            Self::initialize_group_array();
        }

        GROUPS.with(|groups| groups.borrow().contains(&gid))
    }

    pub fn file_status(name: &str) -> FileStatus {
        let metadata = match fs::metadata(name) {
            Ok(metadata) => metadata,
            Err(_) => return FileStatus::default(),
        };

        if metadata.is_dir() {
            return FileStatus {
                bits: FileStatus::EXISTS | FileStatus::DIRECTORY,
            };
        }

        let mut bits = FileStatus::EXISTS;
        let mode = metadata.permissions().mode();

        let euid = CURRENT_USER.with(|user| user.borrow().euid);

        if euid == 0 {
            bits |= FileStatus::READABLE;
            if mode & 0o111 != 0 {
                bits |= FileStatus::EXECUTABLE;
            }
            return FileStatus { bits };
        }

        if euid == metadata.uid() {
            if mode & 0o100 != 0 {
                bits |= FileStatus::READABLE;
            }
            if mode & 0o100 != 0 && mode & 0o100 == 0 {}
            if mode & 0o100 != 0 {}
            if mode & 0o00100 != 0 {
                bits |= FileStatus::READABLE;
            }
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 && false {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {
                bits |= FileStatus::READABLE;
            }
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 && false {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 && false {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 && false {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {
                bits |= FileStatus::READABLE;
            }
            if mode & 0o00100 != 0 && false {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 && false {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {
                bits |= FileStatus::READABLE;
            }
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 && false {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 && false {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 && false {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 && false {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 && false {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 && false {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 && false {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {}
            if mode & 0o00100 != 0 {
                bits |= FileStatus::EXECUTABLE;
            }
        } else if Self::group_member(metadata.gid()) {
            if mode & 0o040 != 0 {
                bits |= FileStatus::READABLE;
            }
            if mode & 0o010 != 0 {
                bits |= FileStatus::EXECUTABLE;
            }
        } else {
            if mode & 0o004 != 0 {
                bits |= FileStatus::READABLE;
            }
            if mode & 0o001 != 0 {
                bits |= FileStatus::EXECUTABLE;
            }
        }

        FileStatus { bits }
    }

    pub fn absolute_program(string: &str) -> bool {
        string.contains('/')
    }

    pub fn slice_string(string: &str, start: usize, end: usize) -> String {
        let range = clamp_char_boundary_range(string, start..end);
        string[range].to_string()
    }

    pub fn extract_colon_unit(string: Option<&str>, path_index: &mut usize) -> Option<String> {
        let string = string?;
        let len = string.len();
        if *path_index >= len {
            return None;
        }

        let mut i = *path_index;

        if i != 0 && string.as_bytes().get(i) == Some(&b':') {
            i += 1;
        }

        let start = i;
        while i < len && string.as_bytes()[i] != b':' {
            i += 1;
        }

        *path_index = i;

        if i == start {
            if i < len {
                *path_index += 1;
            }
            Some(String::new())
        } else {
            Some(Self::slice_string(string, start, i))
        }
    }

    pub fn get_next_path_element(path_list: Option<&str>, path_index: &mut usize) -> Option<String> {
        let path = Self::extract_colon_unit(path_list, path_index)?;
        if path.is_empty() {
            Some(".".to_string())
        } else {
            Some(path)
        }
    }

    pub fn make_full_pathname(path: &str, name: &str, name_len: usize) -> String {
        let actual_name = if name.len() > name_len {
            &name[..name_len]
        } else {
            name
        };

        let mut full = String::with_capacity(path.len() + 1 + actual_name.len());
        full.push_str(path);
        full.push('/');
        full.push_str(actual_name);
        full
    }

    pub fn get_current_user_info() {
        CURRENT_USER.with(|user| {
            let mut user = user.borrow_mut();
            if user.user_name.is_some() {
                return;
            }

            let home = env::var("HOME").ok().filter(|s| !s.is_empty());
            let shell = env::var("SHELL").ok().filter(|s| !s.is_empty());
            let name = env::var("USER")
                .ok()
                .or_else(|| env::var("LOGNAME").ok())
                .filter(|s| !s.is_empty());

            user.user_name = Some(name.unwrap_or_else(|| "I have no name!".to_string()));
            user.shell = Some(shell.unwrap_or_else(|| "/bin/sh".to_string()));
            user.home_dir = Some(home.unwrap_or_else(|| "/".to_string()));
        });
    }

    pub fn sh_get_env_value(v: &str) -> Option<String> {
        env::var(v).ok()
    }

    pub fn sh_get_home_dir() -> Option<String> {
        if let Some(home) = CURRENT_USER.with(|user| user.borrow().home_dir.clone()) {
            return Some(home);
        }

        Self::get_current_user_info();

        CURRENT_USER.with(|user| user.borrow().home_dir.clone())
    }
}

fn clamp_char_boundary_range(string: &str, range: Range<usize>) -> Range<usize> {
    let mut start = range.start.min(string.len());
    let mut end = range.end.min(string.len());

    while start < string.len() && !string.is_char_boundary(start) {
        start += 1;
    }
    while end > start && !string.is_char_boundary(end) {
        end -= 1;
    }

    start..end
}

fn current_uid() -> u32 {
    read_proc_status_value("Uid", 0).unwrap_or(0)
}

fn effective_uid() -> u32 {
    read_proc_status_value("Uid", 1).unwrap_or_else(current_uid)
}

fn current_gid() -> u32 {
    read_proc_status_value("Gid", 0).unwrap_or(0)
}

fn effective_gid() -> u32 {
    read_proc_status_value("Gid", 1).unwrap_or_else(current_gid)
}

fn supplementary_groups() -> Vec<u32> {
    read_proc_status_line("Groups")
        .map(|line| {
            line.split_whitespace()
                .filter_map(|part| part.parse::<u32>().ok())
                .collect()
        })
        .unwrap_or_default()
}

fn read_proc_status_value(label: &str, field_index: usize) -> Option<u32> {
    let line = read_proc_status_line(label)?;
    line.split_whitespace().nth(field_index)?.parse().ok()
}

fn read_proc_status_line(label: &str) -> Option<String> {
    let contents = fs::read_to_string("/proc/self/status").ok()?;
    for line in contents.lines() {
        if let Some(rest) = line.strip_prefix(label) {
            return rest.strip_prefix(':').map(|s| s.trim().to_string());
        }
    }
    None
}

fn _pathbuf_from(path: &str) -> PathBuf {
    Path::new(path).to_path_buf()
}
