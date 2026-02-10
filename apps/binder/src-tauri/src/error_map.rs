#[derive(Debug, Clone)]
pub struct AppErrorDto {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
    pub retryable: bool,
    pub user_action: Option<String>,
}

impl AppErrorDto {
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            details: None,
            retryable: false,
            user_action: None,
        }
    }
}

pub fn map_core_error(err: core::domain::errors::CoreError) -> AppErrorDto {
    AppErrorDto::new(err.code.as_str(), &err.message)
}
