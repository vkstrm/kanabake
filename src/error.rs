#[derive(Debug)]
pub struct Error {
    message: String
}

impl std::error::Error for Error {}
unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl Error {
    pub fn new(message: &str) -> Error {
        Error {
            message: String::from(message),
        }
    }

    pub fn from_remain(remain: &str) -> Error {
        Error {
            message: format_error_message(remain)
        }
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

fn format_error_message(remain: &str) -> String {
    let invalid: &str;
    if remain.len() >= 3 {
        invalid = &remain[..3];
    } else {
        invalid = &remain[..remain.len()];
    }
    return format!("invalid sequence: \"{}\"", invalid).to_lowercase();
}