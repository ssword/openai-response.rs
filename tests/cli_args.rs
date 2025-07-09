use clap::Parser;
use openai_api::cli::args::Args;

#[test]
fn test_args_parsing() {
    let args = Args::parse_from(&[
        "test",
        "--model",
        "gpt-3.5-turbo",
        "--verbose",
        "Hello world",
    ]);

    assert_eq!(args.model, "gpt-3.5-turbo");
    assert_eq!(args.verbose, true);
    assert_eq!(args.prompt, Some("Hello world".to_string()));
}

#[test]
fn test_temperature_validation() {
    let args = Args {
        prompt: Some("test".to_string()),
        model: "gpt-4".to_string(),
        verbose: false,
        temperature: Some(0.5),
        json: false,
        markdown: false,
        plain: false,
        max_output_tokens: None,
        instructions: None,
        stream: false,
        background: false,
        service_tier: None,
        user: None,
        previous_response_id: None,
        top_p: None,
        top_logprobs: None,
        truncation: "disabled".to_string(),
        store: None,
        parallel_tool_calls: None,
    };

    assert!(args.validate_temperature().is_ok());

    let invalid_args = Args {
        prompt: Some("test".to_string()),
        model: "gpt-4".to_string(),
        verbose: false,
        temperature: Some(3.0), // Invalid temperature
        json: false,
        markdown: false,
        plain: false,
        max_output_tokens: None,
        instructions: None,
        stream: false,
        background: false,
        service_tier: None,
        user: None,
        previous_response_id: None,
        top_p: None,
        top_logprobs: None,
        truncation: "disabled".to_string(),
        store: None,
        parallel_tool_calls: None,
    };

    assert!(invalid_args.validate_temperature().is_err());
}

#[test]
fn test_service_tier_validation() {
    let valid_args = Args {
        prompt: Some("test".to_string()),
        model: "gpt-4".to_string(),
        verbose: false,
        temperature: None,
        json: false,
        markdown: false,
        plain: false,
        max_output_tokens: None,
        instructions: None,
        stream: false,
        background: false,
        service_tier: Some("priority".to_string()),
        user: None,
        previous_response_id: None,
        top_p: None,
        top_logprobs: None,
        truncation: "disabled".to_string(),
        store: None,
        parallel_tool_calls: None,
    };

    assert!(valid_args.validate_service_tier().is_ok());

    let invalid_args = Args {
        prompt: Some("test".to_string()),
        model: "gpt-4".to_string(),
        verbose: false,
        temperature: None,
        json: false,
        markdown: false,
        plain: false,
        max_output_tokens: None,
        instructions: None,
        stream: false,
        background: false,
        service_tier: Some("invalid".to_string()),
        user: None,
        previous_response_id: None,
        top_p: None,
        top_logprobs: None,
        truncation: "disabled".to_string(),
        store: None,
        parallel_tool_calls: None,
    };

    assert!(invalid_args.validate_service_tier().is_err());
}

#[test]
fn test_top_p_validation() {
    let valid_args = Args {
        prompt: Some("test".to_string()),
        model: "gpt-4".to_string(),
        verbose: false,
        temperature: None,
        json: false,
        markdown: false,
        plain: false,
        max_output_tokens: None,
        instructions: None,
        stream: false,
        background: false,
        service_tier: None,
        user: None,
        previous_response_id: None,
        top_p: Some(0.9),
        top_logprobs: None,
        truncation: "disabled".to_string(),
        store: None,
        parallel_tool_calls: None,
    };

    assert!(valid_args.validate_top_p().is_ok());

    let invalid_args = Args {
        prompt: Some("test".to_string()),
        model: "gpt-4".to_string(),
        verbose: false,
        temperature: None,
        json: false,
        markdown: false,
        plain: false,
        max_output_tokens: None,
        instructions: None,
        stream: false,
        background: false,
        service_tier: None,
        user: None,
        previous_response_id: None,
        top_p: Some(1.5), // Invalid top_p
        top_logprobs: None,
        truncation: "disabled".to_string(),
        store: None,
        parallel_tool_calls: None,
    };

    assert!(invalid_args.validate_top_p().is_err());
}
