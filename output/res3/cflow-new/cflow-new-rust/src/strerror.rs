pub struct Strerror;

impl Strerror {
    pub fn error_string(errnum: i32) -> String {
        if let Some(message) = crate::strerror_override::StrerrorOverride::error_string(errnum) {
            return message.to_owned();
        }

        let message = std::io::Error::from_raw_os_error(errnum).to_string();
        if message.is_empty() || message == "os error 0" || message == format!("os error {errnum}") {
            return format!("Unknown error {errnum}");
        }

        message
    }
}
