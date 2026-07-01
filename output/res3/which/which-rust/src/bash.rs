use std::env;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;
use std::sync::OnceLock;

#[derive(Clone, Debug, Default)]
struct UserInfo {
    uid: u32,
    gid: u32,
    euid: u32,
    egid: u32,
    groups: Vec<u32>,
    user_name: Option<String>,
    shell: Option<String>,
    home_dir: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileStatus {
    NotFound,
    Exists,
    Directory,
    Readable,
    Executable,
    ReadableExecutable,
}

pub struct Bash;

static CURRENT_USER: OnceLock<UserInfo> = OnceLock::new();

impl Bash {
    fn current_user() -> &'static UserInfo {
        CURRENT_USER.get_or_init(UserInfo::default)
    }

    #[cfg(unix)]
    fn read_proc_status_line(prefix: &str) -> Option<Vec<u32>> {
        let status = fs::read_to_string("/proc/self/status").ok()?;
        for line in status.lines() {
            if let Some(rest) = line.strip_prefix(prefix) {
                let values = rest
                    .split_whitespace()
                    .filter_map(|part| part.parse::<u32>().ok())
                    .collect::<Vec<_>>();
                if !values.is_empty() {
                    return Some(values);
                }
            }
        }
        None
    }

    pub fn update_user_ids() -> bool {
        let user = Self::current_user();

        #[cfg(unix)]
        {
            let uids = Self::read_proc_status_line("Uid:").unwrap_or_default();
            let gids = Self::read_proc_status_line("Gid:").unwrap_or_default();

            let uid = uids.first().copied().unwrap_or(user.uid);
            let euid = uids.get(1).copied().unwrap_or(uid);
            let gid = gids.first().copied().unwrap_or(user.gid);
            let egid = gids.get(1).copied().unwrap_or(gid);

            return uid != euid || gid != egid;
        }

        #[cfg(not(unix))]
        {
            false
        }
    }

    pub fn max_groups() -> usize {
        64
    }

    pub fn initialize_group_array() {
        let _ = Self::update_user_ids();
        let user = Self::current_user();

        #[cfg(unix)]
        {
            let mut groups = Self::read_proc_status_line("Groups:").unwrap_or_default();

            if groups.is_empty() {
                groups.push(user.gid);
            }

            if let Some(pos) = groups.iter().position(|&g| g == user.gid) {
                if pos != 0 {
                    groups.swap(0, pos);
                }
            } else if groups.len() < Self::max_groups() {
                groups.insert(0, user.gid);
            }

            let _ = groups;
        }

        #[cfg(not(unix))]
        {
            let _ = user.gid;
        }
    }

    pub fn group_member(gid: u32) -> bool {
        {
            let user = Self::current_user();
            if gid == user.gid || gid == user.egid {
                return true;
            }
            if !user.groups.is_empty() {
                return user.groups.contains(&gid);
            }
        }

        Self::initialize_group_array();

        let user = Self::current_user();
        user.groups.contains(&gid)
    }

    pub fn file_status<P: AsRef<Path>>(name: P) -> FileStatus {
        let metadata = match fs::metadata(name.as_ref()) {
            Ok(metadata) => metadata,
            Err(_) => return FileStatus::NotFound,
        };

        if metadata.is_dir() {
            return FileStatus::Directory;
        }

        let _ = Self::update_user_ids();

        #[cfg(unix)]
        {
            let mode = metadata.permissions().mode();
            let user = Self::current_user();

            let readable = if user.euid == 0 {
                true
            } else if user.euid == metadata.uid() {
                mode & 0o400 != 0
            } else if Self::group_member(metadata.gid()) {
                mode & 0o040 != 0
            } else {
                mode & 0o004 != 0
            };

            let executable = if user.euid == 0 {
                mode & 0o111 != 0
            } else if user.euid == metadata.uid() {
                mode & 0o100 != 0
            } else if Self::group_member(metadata.gid()) {
                mode & 0o010 != 0
            } else {
                mode & 0o001 != 0
            };

            return match (readable, executable) {
                (true, true) => FileStatus::ReadableExecutable,
                (false, true) => FileStatus::Executable,
                (true, false) => FileStatus::Readable,
                (false, false) => FileStatus::Exists,
            };
        }

        #[cfg(not(unix))]
        {
            if metadata.is_file() {
                FileStatus::ReadableExecutable
            } else {
                FileStatus::Exists
            }
        }
    }

    pub fn absolute_program(string: &str) -> bool {
        string.contains('/')
    }

    pub fn sub_string(string: &str, start: usize, end: usize) -> String {
        if start >= end || start >= string.len() {
            return String::new();
        }
        string.get(start..end.min(string.len())).unwrap_or("").to_owned()
    }

    pub fn extract_colon_unit(string: &str, p_index: &mut usize) -> Option<String> {
        if *p_index >= string.len() {
            return None;
        }

        let bytes = string.as_bytes();
        let mut i = *p_index;

        if i != 0 && bytes.get(i) == Some(&b':') {
            i += 1;
        }

        let start = i;
        while i < bytes.len() && bytes[i] != b':' {
            i += 1;
        }

        *p_index = i;

        if i == start {
            if i < bytes.len() {
                *p_index += 1;
            }
            Some(String::new())
        } else {
            Some(Self::sub_string(string, start, i))
        }
    }

    pub fn get_next_path_element(path_list: &str, path_index_pointer: &mut usize) -> Option<String> {
        let path = Self::extract_colon_unit(path_list, path_index_pointer)?;
        if path.is_empty() {
            Some(".".to_string())
        } else {
            Some(path)
        }
    }

    pub fn make_full_pathname(path: &str, name: &str) -> String {
        let mut full_path = String::with_capacity(path.len() + name.len() + 1);
        full_path.push_str(path);
        full_path.push('/');
        full_path.push_str(name);
        full_path
    }

    pub fn get_current_user_info() {
        let _ = Self::update_user_ids();
        let mut user = Self::current_user().clone();

        if user.user_name.is_some() {
            return;
        }

        let user_name = env::var("USER")
            .or_else(|_| env::var("LOGNAME"))
            .unwrap_or_else(|_| "I have no name!".to_string());
        let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        let home_dir = env::var("HOME").unwrap_or_else(|_| "/".to_string());

        user.user_name = Some(user_name);
        user.shell = Some(shell);
        user.home_dir = Some(home_dir);
    }

    pub fn sh_get_env_value(v: &str) -> Option<String> {
        env::var(v).ok()
    }

    pub fn sh_get_home_dir() -> Option<String> {
        {
            let user = Self::current_user();
            if let Some(home) = &user.home_dir {
                return Some(home.clone());
            }
        }

        Self::get_current_user_info();

        let user = Self::current_user();
        user.home_dir.clone()
    }
}

pub fn absolute_program(string: &str) -> bool {
    Bash::absolute_program(string)
}

pub fn get_next_path_element(path_list: &str, path_index_pointer: &mut usize) -> Option<String> {
    Bash::get_next_path_element(path_list, path_index_pointer)
}

pub fn make_full_pathname(path: &str, name: &str) -> String {
    Bash::make_full_pathname(path, name)
}

pub fn sh_get_home_dir() -> Option<String> {
    Bash::sh_get_home_dir()
}
