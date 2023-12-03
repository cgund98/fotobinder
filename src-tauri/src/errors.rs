use std::path::StripPrefixError;

use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]

pub enum AppError {
    #[error("400 <|> Bad Request. Reason: '{0}'")]
    BadRequest(String),

    #[error("404 <|> Resource not found with {0}")]
    NotFound(String),

    #[error("500 <|> Unexpected error: {0}")]
    Unknown(#[from] std::fmt::Error),

    #[error("500 <|> Unexpected sql error: {0}")]
    SQL(#[from] rusqlite::Error),

    #[error("500 <|> String prefix strip error: {0}")]
    StripPrefix(#[from] StripPrefixError),

    #[error("500 <|> Unable to read file: {0}")]
    FileIoError(#[from] std::io::Error),
}

#[derive(Serialize)]
struct Output {
    code: u16,
    message: String,
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let err_str = self.to_string();
        let mut parts = err_str.split(" <|> ");

        // Read code
        let first_word = parts.next().unwrap_or("500");
        let code = first_word.parse::<u16>().unwrap_or(500);

        // Read error message
        let mut message: String = parts.collect();
        if message.is_empty() {
            message = String::from("unable to parse error string.");
        }

        let out = Output { code, message };

        out.serialize(serializer)
    }
}
