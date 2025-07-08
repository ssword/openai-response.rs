use openai_api::api::models::*;
use openai_api::utils::content::extract_response_content;
use serde_json::json;

#[test]
fn test_response_api_deserialization() {
    let response_json = json!({
        "id": "resp_67ccd2bed1ec8190b14f964abc0542670bb6a6b452d3795b",
        "object": "response",
        "created_at": 1741476542,
        "status": "completed",
        "error": null,
        "incomplete_details": null,
        "instructions": null,
        "max_output_tokens": null,
        "model": "gpt-4.1-2025-04-14",
        "output": [
            {
                "type": "message",
                "id": "msg_67ccd2bf17f0819081ff3bb2cf6508e60bb6a6b452d3795b",
                "status": "completed",
                "role": "assistant",
                "content": [
                    {
                        "type": "output_text",
                        "text": "Hello! How can I assist you today?",
                        "annotations": []
                    }
                ]
            }
        ],
        "parallel_tool_calls": true,
        "previous_response_id": null,
        "reasoning": {
            "effort": null,
            "summary": null
        },
        "store": true,
        "temperature": 1.0,
        "text": {
            "format": {
                "type": "text"
            }
        },
        "tool_choice": "auto",
        "tools": [],
        "top_p": 1.0,
        "truncation": "disabled",
        "usage": {
            "input_tokens": 36,
            "output_tokens": 87,
            "total_tokens": 123,
            "input_tokens_details": {
                "cached_tokens": 0
            },
            "output_tokens_details": {
                "reasoning_tokens": 0
            }
        },
        "user": null,
        "metadata": {}
    });

    let response: ResponseApiResponse = serde_json::from_value(response_json).unwrap();

    assert_eq!(response.id, "resp_67ccd2bed1ec8190b14f964abc0542670bb6a6b452d3795b");
    assert_eq!(response.object, "response");
    assert_eq!(response.model, "gpt-4.1-2025-04-14");
    assert_eq!(response.status, "completed");
    assert_eq!(response.output.len(), 1);
    assert_eq!(response.output[0].role, "assistant");

    assert_eq!(response.usage.total_tokens, 123);
    assert_eq!(response.usage.input_tokens, 36);
    assert_eq!(response.usage.output_tokens, 87);
}

#[test]
fn test_extract_response_content() {
    // Create a test response with output content
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
            content: vec![
                ContentType::Text { text: "Hello".to_string() },
                ContentType::OutputText {
                    text: "World".to_string(),
                    annotations: vec![]
                },
            ],
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

    assert_eq!(extract_response_content(&response), "Hello World");
}

#[test]
fn test_response_request_serialization() {
    let request = ResponseRequest::builder()
        .model("gpt-4.1".to_string())
        .input_text("Test prompt".to_string())
        .temperature(0.7)
        .store(true)
        .build();

    let json_str = serde_json::to_string(&request).unwrap();
    assert!(json_str.contains("gpt-4.1"));
    assert!(json_str.contains("Test prompt"));
    assert!(json_str.contains("0.7"));
}

#[test]
fn test_response_request_builder() {
    let request = ResponseRequest::builder()
        .model("gpt-4.1".to_string())
        .input_text("Hello, world!".to_string())
        .temperature(0.8)
        .max_output_tokens(100)
        .instructions("You are a helpful assistant".to_string())
        .stream(true)
        .background(false)
        .service_tier("priority".to_string())
        .user("user123".to_string())
        .top_p(0.9)
        .top_logprobs(5)
        .truncation("auto".to_string())
        .store(true)
        .parallel_tool_calls(true)
        .build();

    assert_eq!(request.model, Some("gpt-4.1".to_string()));
    assert_eq!(request.temperature, Some(0.8));
    assert_eq!(request.max_output_tokens, Some(100));
    assert_eq!(request.instructions, Some("You are a helpful assistant".to_string()));
    assert_eq!(request.stream, Some(true));
    assert_eq!(request.background, Some(false));
    assert_eq!(request.service_tier, Some("priority".to_string()));
    assert_eq!(request.user, Some("user123".to_string()));
    assert_eq!(request.top_p, Some(0.9));
    assert_eq!(request.top_logprobs, Some(5));
    assert_eq!(request.truncation, Some("auto".to_string()));
    assert_eq!(request.store, Some(true));
    assert_eq!(request.parallel_tool_calls, Some(true));
}

#[test]
fn test_simple_request() {
    let request = ResponseRequest::simple("gpt-4.1".to_string(), "Hello".to_string());

    assert_eq!(request.model, Some("gpt-4.1".to_string()));
    assert_eq!(request.store, Some(true));
    assert!(matches!(request.input, Some(InputType::Text(_))));
}