pub struct Strerror;

impl Strerror {
    pub fn message(errnum: i32) -> String {
        if let Some(message) = crate::strerror_override::StrerrorOverride::message(errnum) {
            return message.to_owned();
        }

        let message = std::io::Error::from_raw_os_error(errnum).to_string();

        if message.is_empty() {
            return format!("Unknown error {}", errnum);
        }

        let lower = message.to_ascii_lowercase();
        if lower.contains("os error")
            || lower.contains("unknown error")
            || lower.contains("unrecognized error")
        {
            format!("Unknown error {}", errnum)
        } else {
            message
        }
    }
}
