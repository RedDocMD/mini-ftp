use thiserror::Error;

#[derive(Error, Debug)]
pub enum MiniFtpError {
    #[error("invalid command: {0}")]
    CommandParseError(String),
    #[error("invalid message: {0}")]
    InvalidMessage(String),
    #[error("UTF-8 error")]
    Utf8Error(#[from] std::str::Utf8Error),
}
