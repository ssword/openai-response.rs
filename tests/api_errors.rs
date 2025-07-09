use openai_api::api::errors::*;
use reqwest::StatusCode;

#[test]
fn test_api_error_parsing() {
    let error_json = r#"
    {
        "error": {
            "message": "Invalid API key",
            "type": "invalid_request_error",
            "code": "invalid_api_key"
        }
    }
    "#;

    let error = OpenAIError::parse_api_error(error_json, StatusCode::UNAUTHORIZED);

    match error {
        OpenAIError::ApiError {
            message,
            error_type,
            code,
        } => {
            assert_eq!(message, "Invalid API key");
            assert_eq!(error_type, "invalid_request_error");
            assert_eq!(code, "invalid_api_key");
        }
        _ => panic!("Expected ApiError variant"),
    }
}

#[test]
fn test_http_error_parsing() {
    let error_text = "Server Error";
    let error = OpenAIError::parse_api_error(error_text, StatusCode::INTERNAL_SERVER_ERROR);

    match error {
        OpenAIError::HttpError { status, message } => {
            assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
            assert_eq!(message, "Server Error");
        }
        _ => panic!("Expected HttpError variant"),
    }
}
