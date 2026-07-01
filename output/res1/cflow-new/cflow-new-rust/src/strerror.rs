pub struct Strerror;

impl Strerror {
    pub fn message(errnum: i32) -> String {
        if let Some(message) = crate::strerror_override::StrerrorOverride::strerror(errnum) {
            return message.to_owned();
        }

        let io_error = std::io::Error::from_raw_os_error(errnum);
        let message = io_error.to_string();

        if message.is_empty() || message == format!("os error {errnum}") {
            return format!("Unknown error {errnum}");
        }

        message
    }
}
