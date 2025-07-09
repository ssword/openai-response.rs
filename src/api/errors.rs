use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenAIError {
    #[error("API Error: {message} (Type: {error_type}, Code: {code})")]
    ApiError {
        message: String,
        error_type: String,
        code: String,
    },
    #[error("HTTP Error {status}: {message}")]
    HttpError {
        status: reqwest::StatusCode,
        message: String,
    },
    #[error("JSON Parse Error: {0}")]
    JsonParseError(String),
    #[error("Network Error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("No output messages returned from OpenAI")]
    NoOutputMessages,
    #[error("Empty response content")]
    EmptyResponse,
}

// Error handling structures
#[derive(Deserialize, Debug)]
pub struct ErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub error: ErrorDetail,
}

impl OpenAIError {
    pub fn parse_api_error(response_text: &str, status: reqwest::StatusCode) -> Self {
        // Try to parse as API error first
        if let Ok(api_error) = serde_json::from_str::<ApiError>(response_text) {
            OpenAIError::ApiError {
                message: api_error.error.message,
                error_type: api_error
                    .error
                    .error_type
                    .unwrap_or_else(|| "unknown".to_string()),
                code: api_error
                    .error
                    .code
                    .unwrap_or_else(|| "unknown".to_string()),
            }
        } else {
            OpenAIError::HttpError {
                status,
                message: response_text.to_string(),
            }
        }
    }
}
