use crate::api::models::ResponseApiResponse;
use crate::utils::content::extract_response_content;
use anyhow::Result;

pub struct OutputFormatter;

impl OutputFormatter {
    pub fn format_response(response: &ResponseApiResponse, verbose: bool) -> Result<()> {
        if response.output.is_empty() {
            return Err(anyhow::anyhow!("No output messages returned from OpenAI"));
        }

        println!("\n📝 Response:");
        let content = extract_response_content(response);
        println!("{}", content);

        if verbose {
            Self::format_verbose_info(response);
        }

        Ok(())
    }

    pub fn format_json(response: &ResponseApiResponse) -> Result<()> {
        println!("{}", serde_json::to_string_pretty(response)?);
        Ok(())
    }

    fn format_verbose_info(response: &ResponseApiResponse) {
        println!("\n📊 Response Metadata:");
        println!("  Response ID: {}", response.id);
        println!("  Model: {}", response.model);
        println!("  Created At: {}", response.created_at);
        println!("  Status: {}", response.status);
        println!("  Object: {}", response.object);

        if !response.output.is_empty() {
            let first_output = &response.output[0];
            println!("  Message ID: {}", first_output.id);
            println!("  Message Status: {}", first_output.status);
            println!("  Role: {}", first_output.role);
        }

        println!("\n📊 Configuration:");
        println!("  Temperature: {}", response.temperature);
        println!("  Top P: {}", response.top_p);
        println!("  Tool Choice: {}", response.tool_choice);
        println!("  Truncation: {}", response.truncation);
        println!("  Store: {}", response.store);
        println!("  Parallel Tool Calls: {}", response.parallel_tool_calls);

        println!("\n📊 Token Usage:");
        println!("  Input tokens: {}", response.usage.input_tokens);
        println!("  Output tokens: {}", response.usage.output_tokens);
        println!("  Total tokens: {}", response.usage.total_tokens);

        if let Some(input_details) = &response.usage.input_tokens_details {
            if let Some(cached) = input_details.cached_tokens {
                println!("  Cached tokens: {}", cached);
            }
        }

        if let Some(output_details) = &response.usage.output_tokens_details {
            if let Some(reasoning) = output_details.reasoning_tokens {
                println!("  Reasoning tokens: {}", reasoning);
            }
        }

        if let Some(reasoning) = &response.reasoning {
            println!("\n📊 Reasoning:");
            if let Some(effort) = &reasoning.effort {
                println!("  Effort: {}", effort);
            }
            if let Some(summary) = &reasoning.summary {
                println!("  Summary: {}", summary);
            }
        }

        if let Some(previous_id) = &response.previous_response_id {
            println!("  Previous Response ID: {}", previous_id);
        }
    }
}
