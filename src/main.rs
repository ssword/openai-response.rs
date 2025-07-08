use anyhow::Result;
use clap::Parser;
use openai_api::{OpenAIClient, Config, Args};
use openai_api::cli::output::OutputFormatter;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Load configuration
    let config = Config::from_env()?;

    // Validate all parameters
    args.validate_all()?;

    // Get the prompt
    let prompt = args.get_prompt()?;

    // Create OpenAI client
    let client = OpenAIClient::new(config.api_key().to_string());

    println!("🤖 Sending request to OpenAI...");

    // Build the request with all provided parameters
    let mut request_builder = client.request_builder()
        .model(args.model.clone())
        .input_text(prompt);

    // Add optional parameters if provided
    if let Some(temp) = args.temperature {
        request_builder = request_builder.temperature(temp);
    }
    if let Some(max_tokens) = args.max_output_tokens {
        request_builder = request_builder.max_output_tokens(max_tokens);
    }
    if let Some(instructions) = &args.instructions {
        request_builder = request_builder.instructions(instructions.clone());
    }
    if args.stream {
        request_builder = request_builder.stream(true);
    }
    if args.background {
        request_builder = request_builder.background(true);
    }
    if let Some(service_tier) = &args.service_tier {
        request_builder = request_builder.service_tier(service_tier.clone());
    }
    if let Some(user) = &args.user {
        request_builder = request_builder.user(user.clone());
    }
    if let Some(prev_id) = &args.previous_response_id {
        request_builder = request_builder.previous_response_id(prev_id.clone());
    }
    if let Some(top_p) = args.top_p {
        request_builder = request_builder.top_p(top_p);
    }
    if let Some(logprobs) = args.top_logprobs {
        request_builder = request_builder.top_logprobs(logprobs);
    }
    request_builder = request_builder.truncation(args.truncation.clone());
    if let Some(store) = args.store {
        request_builder = request_builder.store(store);
    }
    if let Some(parallel) = args.parallel_tool_calls {
        request_builder = request_builder.parallel_tool_calls(parallel);
    }

    // Make the API request
    match client.get_response_with_builder(request_builder).await {
        Ok(response) => {
            if args.json {
                OutputFormatter::format_json(&response)?;
            } else {
                OutputFormatter::format_response(&response, args.verbose)?;
            }
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
