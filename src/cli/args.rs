use anyhow::{anyhow, Result};
use clap::Parser;
use std::io::{self, Write};

/// OpenAI CLI tool for interacting with the Responses API
#[derive(Parser)]
#[command(name = "openai-cli")]
#[command(about = "A command-line tool to interact with OpenAI's Responses API")]
#[command(version = "1.0")]
pub struct Args {
    /// The question or prompt to send to OpenAI
    #[arg(help = "The question or prompt to send to OpenAI")]
    pub prompt: Option<String>,

    /// OpenAI model to use
    #[arg(short, long, default_value = "gpt-4o-mini")]
    pub model: String,

    /// Show usage statistics and response metadata
    #[arg(short, long)]
    pub verbose: bool,

    /// Temperature for response randomness (0.0 to 2.0)
    #[arg(short, long)]
    pub temperature: Option<f32>,

    /// Output response in JSON format
    #[arg(short, long)]
    pub json: bool,

    /// Maximum number of output tokens
    #[arg(long)]
    pub max_output_tokens: Option<u32>,

    /// System instructions for the model
    #[arg(long)]
    pub instructions: Option<String>,

    /// Enable streaming response
    #[arg(long)]
    pub stream: bool,

    /// Run in background
    #[arg(long)]
    pub background: bool,

    /// Service tier (auto, default, flex, priority)
    #[arg(long)]
    pub service_tier: Option<String>,

    /// User identifier for tracking
    #[arg(long)]
    pub user: Option<String>,

    /// Previous response ID for multi-turn conversations
    #[arg(long)]
    pub previous_response_id: Option<String>,

    /// Top-p nucleus sampling parameter
    #[arg(long)]
    pub top_p: Option<f32>,

    /// Number of top log probabilities to return (0-20)
    #[arg(long)]
    pub top_logprobs: Option<u32>,

    /// Truncation strategy (auto, disabled)
    #[arg(long, default_value = "disabled")]
    pub truncation: String,

    /// Whether to store the response
    #[arg(long)]
    pub store: Option<bool>,

    /// Whether to allow parallel tool calls
    #[arg(long)]
    pub parallel_tool_calls: Option<bool>,
}

impl Args {
    pub fn get_prompt(&self) -> Result<String> {
        match &self.prompt {
            Some(p) => Ok(p.clone()),
            None => self.get_prompt_from_user(),
        }
    }

    fn get_prompt_from_user(&self) -> Result<String> {
        print!("Enter your question: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(anyhow!("No prompt provided"));
        }

        Ok(trimmed.to_string())
    }

    pub fn validate_temperature(&self) -> Result<()> {
        if let Some(temp) = self.temperature {
            if temp < 0.0 || temp > 2.0 {
                return Err(anyhow!("Temperature must be between 0.0 and 2.0"));
            }
        }
        Ok(())
    }

    pub fn validate_top_p(&self) -> Result<()> {
        if let Some(top_p) = self.top_p {
            if top_p < 0.0 || top_p > 1.0 {
                return Err(anyhow!("Top-p must be between 0.0 and 1.0"));
            }
        }
        Ok(())
    }

    pub fn validate_top_logprobs(&self) -> Result<()> {
        if let Some(logprobs) = self.top_logprobs {
            if logprobs > 20 {
                return Err(anyhow!("Top logprobs must be between 0 and 20"));
            }
        }
        Ok(())
    }

    pub fn validate_service_tier(&self) -> Result<()> {
        if let Some(tier) = &self.service_tier {
            match tier.as_str() {
                "auto" | "default" | "flex" | "priority" => Ok(()),
                _ => Err(anyhow!(
                    "Service tier must be one of: auto, default, flex, priority"
                )),
            }
        } else {
            Ok(())
        }
    }

    pub fn validate_truncation(&self) -> Result<()> {
        match self.truncation.as_str() {
            "auto" | "disabled" => Ok(()),
            _ => Err(anyhow!("Truncation must be either 'auto' or 'disabled'")),
        }
    }

    pub fn validate_all(&self) -> Result<()> {
        self.validate_temperature()?;
        self.validate_top_p()?;
        self.validate_top_logprobs()?;
        self.validate_service_tier()?;
        self.validate_truncation()?;
        Ok(())
    }
}
