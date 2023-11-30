use std::fmt;

use strum_macros::Display;

#[derive(Display)]
pub enum Code {
    #[strum(serialize = "Bad request.")]
    BadRequest = 400,

    #[strum(serialize = "Resource not found.")]
    NotFound = 404,

    #[strum(serialize = "Internal server error.")]
    Unknown = 500,
}

pub struct Error {
    code: Code,
    message: String,
}

// Different error messages according to Error.code
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = &self.message;

        write!(f, "{}", err_msg)
    }
}

// A unique format for dubugging output
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error {{ code: {}, message: {} }}",
            self.code, self.message
        )
    }
}

// Helper methods
pub fn not_found() -> Error {
    let code = Code::NotFound;
    let message = code.to_string();
    Error { code, message }
}

impl From<std::fmt::Error> for Error {
    fn from(e: std::fmt::Error) -> Self {
        Self {
            code: Code::Unknown,
            message: format!("generic error: {:?}", e),
        }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        Self {
            code: Code::Unknown,
            message: format!("sqlite error: {:?}", e),
        }
    }
}
