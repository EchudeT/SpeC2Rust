pub struct Shell;

impl Shell {
    pub fn get_home_dir() -> Option<String> {
        std::env::var("HOME")
            .ok()
            .filter(|value| !value.is_empty())
            .or_else(|| {
                crate::bash::Bash::sh_get_home_dir()
                    .filter(|value| !value.is_empty())
            })
    }

    pub fn module_tilde() -> Self {
        Self
    }
}
