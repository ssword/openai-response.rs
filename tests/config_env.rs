use openai_api::config::env::Config;
use std::env;

#[test]
fn test_config_valid_key() {
    // Save original value if it exists
    let original = env::var("OPENAI_API_KEY").ok();

    env::set_var("OPENAI_API_KEY", "test-key-123");

    let config = Config::from_env().unwrap();
    assert_eq!(config.api_key(), "test-key-123");
    assert_eq!(config.default_model(), "gpt-4o-mini");

    // Restore original value or remove
    match original {
        Some(val) => env::set_var("OPENAI_API_KEY", val),
        None => env::remove_var("OPENAI_API_KEY"),
    }
}

#[test]
fn test_config_accessors() {
    // Test that the accessor methods work correctly
    let config = Config {
        api_key: "test-key".to_string(),
        default_model: "gpt-4o-mini".to_string(),
    };

    assert_eq!(config.api_key(), "test-key");
    assert_eq!(config.default_model(), "gpt-4o-mini");
}
