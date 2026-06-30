use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AppError {
    pub message: String,
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

impl From<serde_wasm_bindgen::Error> for AppError {
    fn from(err: serde_wasm_bindgen::Error) -> Self {
        AppError{
            message: format!("Serialization error: {}", err)
        }
    }
}