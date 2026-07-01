pub struct Shell;

impl Shell {
    pub fn get_home_dir() -> Option<String> {
        std::env::var("HOME").ok().filter(|value| !value.is_empty())
    }
}
