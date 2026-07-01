use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserInfo {
    pub uid: u32,
    pub gid: u32,
    pub euid: u32,
    pub egid: u32,
    pub user_name: Option<String>,
    pub shell: Option<String>,
    pub home_dir: Option<String>,
}

impl Default for UserInfo {
    fn default() -> Self {
        Self {
            uid: 0,
            gid: 0,
            euid: 0,
            egid: 0,
            user_name: None,
            shell: None,
            home_dir: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileStatus {
    pub exists: bool,
    pub is_directory: bool,
    pub executable: bool,
    pub readable: bool,
}

impl FileStatus {
    fn missing() -> Self {
        Self {
            exists: false,
            is_directory: false,
            executable: false,
            readable: false,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Bash {
    current_user: UserInfo,
    group_array: Vec<u32>,
    maxgroups_cache: Option<usize>,
}

impl Bash {
    pub fn new() -> Self {
        let mut bash = Self::default();
        let _ = bash.uidget();
        bash
    }

    pub fn user_info(&self) -> &UserInfo {
        &self.current_user
    }

    pub fn uidget(&mut self) -> bool {
        let uid = read_id_from_proc_status("Uid", 0).unwrap_or(0);
        let euid = read_id_from_proc_status("Uid", 1).unwrap_or(uid);
        let gid = read_id_from_proc_status("Gid", 0).unwrap_or(0);
        let egid = read_id_from_proc_status("Gid", 1).unwrap_or(gid);

        if self.current_user.uid != uid {
            self.current_user.user_name = None;
            self.current_user.shell = None;
            self.current_user.home_dir = None;
        }

        self.current_user.uid = uid;
        self.current_user.gid = gid;
        self.current_user.euid = euid;
        self.current_user.egid = egid;

        self.current_user.uid != self.current_user.euid || self.current_user.gid != self.current_user.egid
    }

    pub fn getmaxgroups(&mut self) -> usize {
        if let Some(v) = self.maxgroups_cache {
            return v;
        }

        let computed = env::var("NGROUPS_MAX")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(1024);

        self.maxgroups_cache = Some(computed);
        computed
    }

    pub fn initialize_group_array(&mut self) {
        let maxgroups = self.getmaxgroups();

        let mut groups = read_supplementary_groups_from_proc_status().unwrap_or_default();

        if groups.is_empty() {
            groups.push(self.current_user.gid);
        }

        if !groups.contains(&self.current_user.gid) && groups.len() < maxgroups {
            groups.insert(0, self.current_user.gid);
        }

        if let Some(pos) = groups.iter().position(|g| *g == self.current_user.gid) {
            if pos != 0 {
                groups.swap(0, pos);
            }
        }

        self.group_array = groups;
    }

    pub fn group_member(&mut self, gid: u32) -> bool {
        if gid == self.current_user.gid || gid == self.current_user.egid {
            return true;
        }

        if self.group_array.is_empty() {
            self.initialize_group_array();
        }

        if self.group_array.is_empty() {
            return false;
        }

        self.group_array.contains(&gid)
    }

    pub fn file_status(&mut self, name: &str) -> FileStatus {
        let metadata = match fs::metadata(name) {
            Ok(metadata) => metadata,
            Err(_) => return FileStatus::missing(),
        };

        let file_type = metadata.file_type();
        if file_type.is_dir() {
            return FileStatus {
                exists: true,
                is_directory: true,
                executable: false,
                readable: false,
            };
        }

        let mut result = FileStatus {
            exists: true,
            is_directory: false,
            executable: false,
            readable: false,
        };

        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;

            let mode = metadata.mode();
            let st_uid = metadata.uid();
            let st_gid = metadata.gid();

            if self.current_user.euid == 0 {
                result.readable = true;
                result.executable = (mode & 0o111) != 0;
                return result;
            }

            if self.current_user.euid == st_uid {
                result.executable = (mode & 0o100) != 0;
                result.readable = (mode & 0o400) != 0;
            } else if self.group_member(st_gid) {
                result.executable = (mode & 0o010) != 0;
                result.readable = (mode & 0o040) != 0;
            } else {
                result.executable = (mode & 0o001) != 0;
                result.readable = (mode & 0o004) != 0;
            }
        }

        #[cfg(not(unix))]
        {
            result.readable = true;
            result.executable = metadata.is_file();
        }

        result
    }

    pub fn absolute_program(&self, string: &str) -> bool {
        string.contains('/')
    }

    pub fn substring(&self, string: &str, start: usize, end: usize) -> String {
        let start = start.min(string.len());
        let end = end.min(string.len());
        if start >= end {
            String::new()
        } else {
            string[start..end].to_string()
        }
    }

    pub fn extract_colon_unit(&self, string: Option<&str>, p_index: &mut usize) -> Option<String> {
        let string = string?;
        let len = string.len();

        if *p_index >= len {
            return None;
        }

        let mut i = *p_index;

        if i != 0 && string.as_bytes().get(i) == Some(&b':') {
            i += 1;
        }

        let start = i;
        while i < len && string.as_bytes()[i] != b':' {
            i += 1;
        }

        *p_index = i;

        if i == start {
            if i < len {
                *p_index += 1;
            }
            Some(String::new())
        } else {
            Some(self.substring(string, start, i))
        }
    }

    pub fn get_next_path_element(&self, path_list: Option<&str>, path_index_pointer: &mut usize) -> Option<String> {
        let path = self.extract_colon_unit(path_list, path_index_pointer)?;
        if path.is_empty() {
            Some(".".to_string())
        } else {
            Some(path)
        }
    }

    pub fn make_full_pathname(&self, path: &str, name: &str, _name_len: usize) -> String {
        let mut full = String::with_capacity(path.len() + name.len() + 1);
        full.push_str(path);
        full.push('/');
        full.push_str(name);
        full
    }

    pub fn get_current_user_info(&mut self) {
        if self.current_user.user_name.is_some() {
            return;
        }

        let home = env::var("HOME").ok().or_else(|| Some("/".to_string()));
        let shell = env::var("SHELL").ok().filter(|s| !s.is_empty()).or_else(|| Some("/bin/sh".to_string()));
        let user_name = env::var("USER")
            .ok()
            .or_else(|| env::var("LOGNAME").ok())
            .or_else(|| Some("I have no name!".to_string()));

        self.current_user.user_name = user_name;
        self.current_user.shell = shell;
        self.current_user.home_dir = home;
    }

    pub fn sh_get_env_value(&self, v: &str) -> Option<String> {
        env::var(v).ok()
    }

    pub fn sh_get_home_dir(&mut self) -> Option<String> {
        if self.current_user.home_dir.is_none() {
            self.get_current_user_info();
        }
        self.current_user.home_dir.clone()
    }
}

fn read_id_from_proc_status(field: &str, index: usize) -> Option<u32> {
    let contents = fs::read_to_string("/proc/self/status").ok()?;
    for line in contents.lines() {
        if let Some(rest) = line.strip_prefix(field) {
            let values: Vec<&str> = rest.split_whitespace().collect();
            if let Some(raw) = values.get(index) {
                if let Ok(parsed) = raw.parse::<u32>() {
                    return Some(parsed);
                }
            }
        }
    }
    None
}

fn read_supplementary_groups_from_proc_status() -> Option<Vec<u32>> {
    let contents = fs::read_to_string("/proc/self/status").ok()?;
    for line in contents.lines() {
        if let Some(rest) = line.strip_prefix("Groups:") {
            let groups = rest
                .split_whitespace()
                .filter_map(|part| part.parse::<u32>().ok())
                .collect::<Vec<_>>();
            return Some(groups);
        }
    }
    None
}

#[allow(dead_code)]
fn _normalize_join(path: &str, name: &str) -> PathBuf {
    Path::new(path).join(name)
}
