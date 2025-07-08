use crate::api::models::{ResponseApiResponse, ContentType};

// Helper function to extract text content from the new response format
pub fn extract_response_content(response: &ResponseApiResponse) -> String {
    if response.output.is_empty() {
        return "No content available".to_string();
    }

    let mut text_parts = Vec::new();

    // Extract content from all output messages
    for output_message in &response.output {
        for content in &output_message.content {
            match content {
                ContentType::Text { text } => text_parts.push(text.clone()),
                ContentType::OutputText { text, .. } => text_parts.push(text.clone()),
            }
        }
    }

    if text_parts.is_empty() {
        "No content available".to_string()
    } else {
        text_parts.join(" ")
    }
}

// Helper function to extract content from a specific content array (for backward compatibility)
pub fn extract_content_from_array(content_array: &[ContentType]) -> String {
    let mut text_parts = Vec::new();
    for content in content_array {
        match content {
            ContentType::Text { text } => text_parts.push(text.clone()),
            ContentType::OutputText { text, .. } => text_parts.push(text.clone()),
        }
    }

    if text_parts.is_empty() {
        "No content available".to_string()
    } else {
        text_parts.join(" ")
    }
}