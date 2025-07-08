use reqwest::Client;
use crate::api::errors::OpenAIError;
use crate::api::models::{ResponseRequest, ResponseApiResponse, ResponseRequestBuilder};

pub struct OpenAIClient {
    client: Client,
    api_key: String,
}

impl OpenAIClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    /// Simple method for basic text responses (backward compatibility)
    pub async fn get_response(&self, model: &str, prompt: &str, temperature: Option<f32>) -> Result<ResponseApiResponse, OpenAIError> {
        let request = ResponseRequest::builder()
            .model(model.to_string())
            .input_text(prompt.to_string())
            .temperature(temperature.unwrap_or(1.0))
            .store(true)
            .build();

        self.send_request(request).await
    }

    /// Advanced method using the full request builder
    pub async fn get_response_with_builder(&self, builder: ResponseRequestBuilder) -> Result<ResponseApiResponse, OpenAIError> {
        let request = builder.build();
        self.send_request(request).await
    }

    /// Send a complete request with all parameters
    pub async fn send_request(&self, request: ResponseRequest) -> Result<ResponseApiResponse, OpenAIError> {
        let response = self.client
            .post("https://api.openai.com/v1/responses")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;

        if !status.is_success() {
            return Err(OpenAIError::parse_api_error(&response_text, status));
        }

        let response_data: ResponseApiResponse = serde_json::from_str(&response_text)
            .map_err(|e| OpenAIError::JsonParseError(e.to_string()))?;

        Ok(response_data)
    }

    /// Create a request builder for advanced usage
    pub fn request_builder(&self) -> ResponseRequestBuilder {
        ResponseRequestBuilder::new()
    }

    /// Convenience method for multi-turn conversations
    pub async fn continue_conversation(
        &self,
        model: &str,
        prompt: &str,
        previous_response_id: &str,
        temperature: Option<f32>,
    ) -> Result<ResponseApiResponse, OpenAIError> {
        let request = ResponseRequest::builder()
            .model(model.to_string())
            .input_text(prompt.to_string())
            .previous_response_id(previous_response_id.to_string())
            .temperature(temperature.unwrap_or(1.0))
            .store(true)
            .build();

        self.send_request(request).await
    }

    /// Convenience method for streaming responses
    pub async fn get_streaming_response(
        &self,
        model: &str,
        prompt: &str,
        temperature: Option<f32>,
    ) -> Result<ResponseApiResponse, OpenAIError> {
        let request = ResponseRequest::builder()
            .model(model.to_string())
            .input_text(prompt.to_string())
            .temperature(temperature.unwrap_or(1.0))
            .stream(true)
            .store(true)
            .build();

        self.send_request(request).await
    }

    /// Convenience method for background processing
    pub async fn get_background_response(
        &self,
        model: &str,
        prompt: &str,
        temperature: Option<f32>,
    ) -> Result<ResponseApiResponse, OpenAIError> {
        let request = ResponseRequest::builder()
            .model(model.to_string())
            .input_text(prompt.to_string())
            .temperature(temperature.unwrap_or(1.0))
            .background(true)
            .store(true)
            .build();

        self.send_request(request).await
    }
}