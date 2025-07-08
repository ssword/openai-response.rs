use serde::{Deserialize, Serialize};

// Content types for response output
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ContentType {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "output_text")]
    OutputText {
        text: String,
        annotations: Vec<serde_json::Value>, // Flexible for various annotation types
    },
}

// Input types for the Responses API
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum InputType {
    Text(String),
    Array(Vec<serde_json::Value>), // For complex inputs like images, files, etc.
}

// Prompt template reference
#[derive(Serialize, Debug, Clone)]
pub struct PromptTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}

// Reasoning configuration for o-series models
#[derive(Serialize, Debug, Clone)]
pub struct ReasoningConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<bool>,
}

// Tool choice configuration
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ToolChoice {
    String(String), // "auto", "none", "required"
    Object(serde_json::Value), // Specific tool selection
}

// Complete request structure for the Responses API
#[derive(Serialize, Debug, Default)]
pub struct ResponseRequest {
    // Core parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<InputType>,

    // Response configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,

    // Behavior configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    // Advanced configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>, // "auto", "default", "flex", "priority"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<String>, // "auto", "disabled"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    // Multi-turn conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,

    // Tools and functions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tool_calls: Option<u32>,

    // Advanced features
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<PromptTemplate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>, // Additional output data to include
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>, // Up to 16 key-value pairs
}

impl ResponseRequest {
    /// Create a new request builder with default values
    pub fn builder() -> ResponseRequestBuilder {
        ResponseRequestBuilder::new()
    }

    /// Create a simple request with just model and input
    pub fn simple(model: String, input: String) -> Self {
        Self {
            model: Some(model),
            input: Some(InputType::Text(input)),
            instructions: None,
            max_output_tokens: None,
            temperature: None,
            top_p: None,
            top_logprobs: None,
            background: None,
            stream: None,
            store: Some(true), // Default to storing responses
            parallel_tool_calls: None,
            service_tier: None,
            truncation: None,
            user: None,
            previous_response_id: None,
            tools: None,
            tool_choice: None,
            max_tool_calls: None,
            prompt: None,
            reasoning: None,
            text: None,
            include: None,
            metadata: None,
        }
    }
}

/// Builder pattern for constructing ResponseRequest with fluent API
#[derive(Debug, Default)]
pub struct ResponseRequestBuilder {
    request: ResponseRequest,
}

impl ResponseRequestBuilder {
    pub fn new() -> Self {
        Self {
            request: ResponseRequest {
                model: None,
                input: None,
                instructions: None,
                max_output_tokens: None,
                temperature: None,
                top_p: None,
                top_logprobs: None,
                background: None,
                stream: None,
                store: Some(true), // Default to storing responses
                parallel_tool_calls: None,
                service_tier: None,
                truncation: None,
                user: None,
                previous_response_id: None,
                tools: None,
                tool_choice: None,
                max_tool_calls: None,
                prompt: None,
                reasoning: None,
                text: None,
                include: None,
                metadata: None,
            },
        }
    }

    // Core parameters
    pub fn model(mut self, model: String) -> Self {
        self.request.model = Some(model);
        self
    }

    pub fn input_text(mut self, input: String) -> Self {
        self.request.input = Some(InputType::Text(input));
        self
    }

    pub fn input_array(mut self, input: Vec<serde_json::Value>) -> Self {
        self.request.input = Some(InputType::Array(input));
        self
    }

    pub fn instructions(mut self, instructions: String) -> Self {
        self.request.instructions = Some(instructions);
        self
    }

    // Response configuration
    pub fn max_output_tokens(mut self, tokens: u32) -> Self {
        self.request.max_output_tokens = Some(tokens);
        self
    }

    pub fn temperature(mut self, temp: f32) -> Self {
        self.request.temperature = Some(temp);
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.request.top_p = Some(top_p);
        self
    }

    pub fn top_logprobs(mut self, logprobs: u32) -> Self {
        self.request.top_logprobs = Some(logprobs);
        self
    }

    // Behavior configuration
    pub fn background(mut self, background: bool) -> Self {
        self.request.background = Some(background);
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.request.stream = Some(stream);
        self
    }

    pub fn store(mut self, store: bool) -> Self {
        self.request.store = Some(store);
        self
    }

    pub fn parallel_tool_calls(mut self, parallel: bool) -> Self {
        self.request.parallel_tool_calls = Some(parallel);
        self
    }

    // Advanced configuration
    pub fn service_tier(mut self, tier: String) -> Self {
        self.request.service_tier = Some(tier);
        self
    }

    pub fn truncation(mut self, truncation: String) -> Self {
        self.request.truncation = Some(truncation);
        self
    }

    pub fn user(mut self, user: String) -> Self {
        self.request.user = Some(user);
        self
    }

    // Multi-turn conversation
    pub fn previous_response_id(mut self, id: String) -> Self {
        self.request.previous_response_id = Some(id);
        self
    }

    // Tools and functions
    pub fn tools(mut self, tools: Vec<serde_json::Value>) -> Self {
        self.request.tools = Some(tools);
        self
    }

    pub fn tool_choice_string(mut self, choice: String) -> Self {
        self.request.tool_choice = Some(ToolChoice::String(choice));
        self
    }

    pub fn tool_choice_object(mut self, choice: serde_json::Value) -> Self {
        self.request.tool_choice = Some(ToolChoice::Object(choice));
        self
    }

    pub fn max_tool_calls(mut self, max_calls: u32) -> Self {
        self.request.max_tool_calls = Some(max_calls);
        self
    }

    // Advanced features
    pub fn prompt_template(mut self, template: PromptTemplate) -> Self {
        self.request.prompt = Some(template);
        self
    }

    pub fn reasoning_config(mut self, reasoning: ReasoningConfig) -> Self {
        self.request.reasoning = Some(reasoning);
        self
    }

    pub fn text_config(mut self, text: TextConfig) -> Self {
        self.request.text = Some(text);
        self
    }

    pub fn include(mut self, include: Vec<String>) -> Self {
        self.request.include = Some(include);
        self
    }

    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.request.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> ResponseRequest {
        self.request
    }
}

// Text format configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextFormat {
    #[serde(rename = "type")]
    pub format_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextConfig {
    pub format: TextFormat,
}

// Reasoning information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reasoning {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

// Output message structure for the new API
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputMessage {
    #[serde(rename = "type")]
    pub message_type: String, // "message"
    pub id: String,
    pub status: String, // "completed"
    pub role: String,   // "assistant"
    pub content: Vec<ContentType>,
}

// Detailed usage statistics
#[derive(Serialize, Deserialize, Debug)]
pub struct InputTokensDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_tokens: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputTokensDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tokens_details: Option<InputTokensDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tokens_details: Option<OutputTokensDetails>,
}

// Complete response structure matching the new OpenAI Responses API format
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseApiResponse {
    pub id: String,
    pub object: String, // "response"
    pub created_at: u64,
    pub status: String, // "completed"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,
    pub model: String,
    pub output: Vec<OutputMessage>,
    pub parallel_tool_calls: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Reasoning>,
    pub store: bool,
    pub temperature: f32,
    pub text: TextConfig,
    pub tool_choice: String, // "auto"
    pub tools: Vec<serde_json::Value>,
    pub top_p: f32,
    pub truncation: String, // "disabled"
    pub usage: Usage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

