use base64::DecodeError;

#[derive(Responder)]
pub enum GeonftError {
    #[response(status = 500)]
    AnyhowError(String),
    IoError(std::io::Error),
    #[response(status = 500)]
    SerdeError(String),
    #[response(status = 500)]
    DecodeError(String),
}

pub type Result<T> = std::result::Result<T, GeonftError>;

impl From<anyhow::Error> for GeonftError {
    fn from(e: anyhow::Error) -> Self {
        GeonftError::AnyhowError(format!("{}", e))
    }
}

impl From<std::io::Error> for GeonftError {
    fn from(e: std::io::Error) -> Self {
        GeonftError::IoError(e)
    }
}

impl From<serde_json::Error> for GeonftError {
    fn from(e: serde_json::Error) -> Self {
        GeonftError::SerdeError(format!("{}", e))
    }
}

impl From<DecodeError> for GeonftError {
    fn from(e: base64::DecodeError) -> Self {
        GeonftError::DecodeError(format!("{}", e))
    }
}
