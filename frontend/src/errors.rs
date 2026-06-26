use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppError {
    message: String,
}

impl AppError {
    pub fn new (message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl From<reqwasm::Error> for AppError {
    fn from(err: reqwasm::Error) -> Self {
        AppError{
            message: format!("Request error: {}", err)
        }
    }
}