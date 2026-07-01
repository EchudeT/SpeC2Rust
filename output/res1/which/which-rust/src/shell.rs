use std::env;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Copy)]
pub struct Shell;

impl Shell {
    pub fn get_home_dir(&self) -> Option<String> {
        env::var("HOME").ok().filter(|value| !value.is_empty())
    }

    pub fn module_tilde(&self, input: &str) -> Option<String> {
        if !input.starts_with('~') {
            return None;
        }

        let suffix_start = input.find('/').unwrap_or(input.len());
        let user_part = &input[1..suffix_start];
        let suffix = &input[suffix_start..];

        if !user_part.is_empty() {
            return None;
        }

        let home_dir = self.get_home_dir()?;
        let mut expanded = PathBuf::from(home_dir);
        if let Some(stripped) = suffix.strip_prefix('/') {
            if !stripped.is_empty() {
                expanded.push(stripped);
            }
        }

        Some(expanded.to_string_lossy().into_owned())
    }
}
