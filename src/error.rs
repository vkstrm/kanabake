//! Custom error typ

/// Custom error type.
///
/// Contents of message contains specific invalid input
/// if it occurs during parsing of input.
#[derive(Debug)]
pub struct Error {
    message: String,
}

impl std::error::Error for Error {}
unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl Error {
    pub(crate) fn new(message: &str) -> Error {
        Error {
            message: String::from(message),
        }
    }

    pub(crate) fn from_remain(remain: &str) -> Error {
        Error {
            message: format_error_message(remain),
        }
    }

    /// Returns the error message
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
    let invalid = if remain.len() >= 3 {
        &remain[..3]
    } else {
        &remain[..remain.len()]
    };
    format!("invalid sequence: \"{}\"", invalid).to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_format() {
        assert_eq!(format_error_message("abcdefg"), "invalid sequence: \"abc\"");
        assert_eq!(format_error_message("abc"), "invalid sequence: \"abc\"");
        assert_eq!(format_error_message("ab"), "invalid sequence: \"ab\"");
        assert_eq!(format_error_message("a"), "invalid sequence: \"a\"");
    }
}
