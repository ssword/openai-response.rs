use openai_api::api::models::{
    ContentType, OutputMessage, ResponseApiResponse, TextConfig, TextFormat, Usage,
};
use openai_api::utils::content::{extract_content_from_array, extract_response_content};

#[test]
fn test_extract_response_content() {
    let response = ResponseApiResponse {
        id: "test-123".to_string(),
        object: "response".to_string(),
        created_at: 1741476542,
        status: "completed".to_string(),
        error: None,
        incomplete_details: None,
        instructions: None,
        max_output_tokens: None,
        model: "gpt-4.1".to_string(),
        output: vec![OutputMessage {
            message_type: "message".to_string(),
            id: "msg-123".to_string(),
            status: "completed".to_string(),
            role: "assistant".to_string(),
            content: vec![ContentType::Text {
                text: "Simple message".to_string(),
            }],
        }],
        parallel_tool_calls: true,
        previous_response_id: None,
        reasoning: None,
        store: true,
        temperature: 1.0,
        text: TextConfig {
            format: TextFormat {
                format_type: "text".to_string(),
            },
        },
        tool_choice: "auto".to_string(),
        tools: vec![],
        top_p: 1.0,
        truncation: "disabled".to_string(),
        usage: Usage {
            input_tokens: 10,
            output_tokens: 20,
            total_tokens: 30,
            input_tokens_details: None,
            output_tokens_details: None,
        },
        user: None,
        metadata: None,
    };

    assert_eq!(extract_response_content(&response), "Simple message");
}

#[test]
fn test_extract_complex_content() {
    let content_array = vec![
        ContentType::Text {
            text: "Hello".to_string(),
        },
        ContentType::OutputText {
            text: "World".to_string(),
            annotations: vec![],
        },
    ];

    assert_eq!(extract_content_from_array(&content_array), "Hello World");
}

#[test]
fn test_extract_no_content() {
    let response = ResponseApiResponse {
        id: "test-123".to_string(),
        object: "response".to_string(),
        created_at: 1741476542,
        status: "completed".to_string(),
        error: None,
        incomplete_details: None,
        instructions: None,
        max_output_tokens: None,
        model: "gpt-4.1".to_string(),
        output: vec![],
        parallel_tool_calls: true,
        previous_response_id: None,
        reasoning: None,
        store: true,
        temperature: 1.0,
        text: TextConfig {
            format: TextFormat {
                format_type: "text".to_string(),
            },
        },
        tool_choice: "auto".to_string(),
        tools: vec![],
        top_p: 1.0,
        truncation: "disabled".to_string(),
        usage: Usage {
            input_tokens: 10,
            output_tokens: 20,
            total_tokens: 30,
            input_tokens_details: None,
            output_tokens_details: None,
        },
        user: None,
        metadata: None,
    };

    assert_eq!(extract_response_content(&response), "No content available");
}

#[test]
fn test_extract_empty_content_array() {
    let content_array: Vec<ContentType> = vec![];
    assert_eq!(
        extract_content_from_array(&content_array),
        "No content available"
    );
}

#[test]
fn test_extract_mixed_content() {
    let content_array = vec![
        ContentType::Text {
            text: "First".to_string(),
        },
        ContentType::Text {
            text: "Second".to_string(),
        },
        ContentType::OutputText {
            text: "Third".to_string(),
            annotations: vec![],
        },
    ];

    assert_eq!(
        extract_content_from_array(&content_array),
        "First Second Third"
    );
}
