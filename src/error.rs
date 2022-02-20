use thiserror::Error;

#[derive(Error, Debug)]
pub enum MiniFtpError {
    #[error("invalid command: {0}")]
    CommandParseError(String),
}
