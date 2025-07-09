use openai_api::cli::args::{Args, DisplayMode};
use openai_api::cli::markdown::MarkdownRenderer;
use clap::Parser;

#[test]
fn test_markdown_flag_parsing() {
    let args = Args::parse_from(&["test", "--markdown", "Hello world"]);
    
    assert_eq!(args.markdown, true);
    assert_eq!(args.json, false);
    assert_eq!(args.plain, false);
    assert_eq!(args.get_display_mode(), DisplayMode::Markdown);
    assert_eq!(args.prompt, Some("Hello world".to_string()));
}

#[test]
fn test_markdown_is_default() {
    let args = Args::parse_from(&["test", "Hello world"]);
    
    assert_eq!(args.markdown, true);
    assert_eq!(args.json, false);
    assert_eq!(args.plain, false);
    assert_eq!(args.get_display_mode(), DisplayMode::Markdown);
    assert_eq!(args.prompt, Some("Hello world".to_string()));
}

#[test]
fn test_plain_flag_parsing() {
    let args = Args::parse_from(&["test", "--plain", "Hello world"]);
    
    assert_eq!(args.markdown, true); // still true as default, but overridden by display mode
    assert_eq!(args.json, false);
    assert_eq!(args.plain, true);
    assert_eq!(args.get_display_mode(), DisplayMode::Plain);
    assert_eq!(args.prompt, Some("Hello world".to_string()));
}

#[test]
fn test_json_flag_parsing() {
    let args = Args::parse_from(&["test", "--json", "Hello world"]);
    
    assert_eq!(args.markdown, true); // still true as default, but overridden by display mode
    assert_eq!(args.json, true);
    assert_eq!(args.plain, false);
    assert_eq!(args.get_display_mode(), DisplayMode::Json);
    assert_eq!(args.prompt, Some("Hello world".to_string()));
}

#[test]
fn test_markdown_alias_parsing() {
    let args = Args::parse_from(&["test", "--md", "Hello world"]);
    
    assert_eq!(args.markdown, true);
    assert_eq!(args.json, false);
    assert_eq!(args.plain, false);
    assert_eq!(args.get_display_mode(), DisplayMode::Markdown);
    assert_eq!(args.prompt, Some("Hello world".to_string()));
}

#[test]
fn test_output_format_mutual_exclusion() {
    let args = Args {
        prompt: Some("test".to_string()),
        model: "gpt-4o-mini".to_string(),
        verbose: false,
        temperature: None,
        json: true,
        markdown: true,
        plain: true,
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

    assert!(args.validate_output_format().is_err());
}

#[test]
fn test_plain_only_valid() {
    let args = Args {
        prompt: Some("test".to_string()),
        model: "gpt-4o-mini".to_string(),
        verbose: false,
        temperature: None,
        json: false,
        markdown: true,
        plain: true,
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

    assert!(args.validate_output_format().is_ok());
    assert_eq!(args.get_display_mode(), DisplayMode::Plain);
}

#[test]
fn test_markdown_renderer_creation() {
    let _renderer = MarkdownRenderer::new();
    // Test that the renderer can be created without panicking
    // This is a basic smoke test
    assert!(true);
}

#[test]
fn test_markdown_renderer_default() {
    let _renderer = MarkdownRenderer::default();
    // Test that the default renderer can be created
    assert!(true);
}

#[test]
fn test_markdown_detection() {
    let renderer = MarkdownRenderer::new();
    
    // Test various markdown patterns
    assert!(renderer.looks_like_markdown("**bold text**"));
    assert!(renderer.looks_like_markdown("*italic text*"));
    assert!(renderer.looks_like_markdown("`code`"));
    assert!(renderer.looks_like_markdown("# Header"));
    assert!(renderer.looks_like_markdown("## Header 2"));
    assert!(renderer.looks_like_markdown("- List item"));
    assert!(renderer.looks_like_markdown("1. Numbered item"));
    assert!(renderer.looks_like_markdown("```code block```"));
    assert!(renderer.looks_like_markdown("> Quote"));
    
    // Test plain text
    assert!(!renderer.looks_like_markdown("Just plain text"));
    assert!(!renderer.looks_like_markdown("No markdown here"));
}