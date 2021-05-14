#[derive(Debug)]
pub struct Error { // TODO unit struct?
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
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}