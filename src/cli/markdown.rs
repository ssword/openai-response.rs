use crate::api::models::ResponseApiResponse;
use crate::utils::content::extract_response_content;
use anyhow::Result;
use termimad::MadSkin;
use crossterm::style::Color;

pub struct MarkdownRenderer {
    skin: MadSkin,
}

impl MarkdownRenderer {
    pub fn new() -> Self {
        let mut skin = MadSkin::default();
        
        // Customize the skin for better terminal rendering
        // Note: Using available API for termimad 0.28
        skin.set_headers_fg(Color::Cyan);
        skin.bold.set_fg(Color::Yellow);
        skin.italic.set_fg(Color::Magenta);
        skin.inline_code.set_fg(Color::Green);
        skin.code_block.set_fg(Color::Green);
        
        Self { skin }
    }

    pub fn render_response(&self, response: &ResponseApiResponse, verbose: bool) -> Result<()> {
        if response.output.is_empty() {
            return Err(anyhow::anyhow!("No output messages returned from OpenAI"));
        }

        println!("\n📝 Response:");
        let content = extract_response_content(response);
        
        // Check if content contains markdown-like patterns
        if self.looks_like_markdown(&content) {
            // Render as markdown
            self.print_markdown(&content)?;
        } else {
            // Fallback to plain text with basic styling
            self.print_as_text(&content)?;
        }

        if verbose {
            self.render_verbose_info(response)?;
        }

        Ok(())
    }

    pub fn looks_like_markdown(&self, text: &str) -> bool {
        // Simple heuristics to detect markdown content
        text.contains("**") || 
        text.contains("*") || 
        text.contains("`") || 
        text.contains("# ") || 
        text.contains("## ") || 
        text.contains("- ") || 
        text.contains("1. ") ||
        text.contains("```") ||
        text.contains("> ")
    }

    fn print_markdown(&self, content: &str) -> Result<()> {
        // Use termimad to render markdown
        let rendered = self.skin.term_text(content);
        println!("{}", rendered);
        Ok(())
    }

    fn print_as_text(&self, content: &str) -> Result<()> {
        // Print as plain text but still styled
        println!("{}", content);
        Ok(())
    }

    fn render_verbose_info(&self, response: &ResponseApiResponse) -> Result<()> {
        let verbose_content = self.format_verbose_info(response);
        let rendered = self.skin.term_text(&verbose_content);
        println!("{}", rendered);
        Ok(())
    }

    fn format_verbose_info(&self, response: &ResponseApiResponse) -> String {
        let mut info = String::new();
        
        info.push_str("\n## 📊 Response Metadata\n\n");
        info.push_str(&format!("- **Response ID**: `{}`\n", response.id));
        info.push_str(&format!("- **Model**: `{}`\n", response.model));
        info.push_str(&format!("- **Created At**: `{}`\n", response.created_at));
        info.push_str(&format!("- **Status**: `{}`\n", response.status));
        info.push_str(&format!("- **Object**: `{}`\n", response.object));

        if !response.output.is_empty() {
            let first_output = &response.output[0];
            info.push_str(&format!("- **Message ID**: `{}`\n", first_output.id));
            info.push_str(&format!("- **Message Status**: `{}`\n", first_output.status));
            info.push_str(&format!("- **Role**: `{}`\n", first_output.role));
        }

        info.push_str("\n## 📊 Configuration\n\n");
        info.push_str(&format!("- **Temperature**: `{}`\n", response.temperature));
        info.push_str(&format!("- **Top P**: `{}`\n", response.top_p));
        info.push_str(&format!("- **Tool Choice**: `{}`\n", response.tool_choice));
        info.push_str(&format!("- **Truncation**: `{}`\n", response.truncation));
        info.push_str(&format!("- **Store**: `{}`\n", response.store));
        info.push_str(&format!("- **Parallel Tool Calls**: `{}`\n", response.parallel_tool_calls));

        info.push_str("\n## 📊 Token Usage\n\n");
        info.push_str(&format!("- **Input tokens**: `{}`\n", response.usage.input_tokens));
        info.push_str(&format!("- **Output tokens**: `{}`\n", response.usage.output_tokens));
        info.push_str(&format!("- **Total tokens**: `{}`\n", response.usage.total_tokens));

        if let Some(input_details) = &response.usage.input_tokens_details {
            if let Some(cached) = input_details.cached_tokens {
                info.push_str(&format!("- **Cached tokens**: `{}`\n", cached));
            }
        }

        if let Some(output_details) = &response.usage.output_tokens_details {
            if let Some(reasoning) = output_details.reasoning_tokens {
                info.push_str(&format!("- **Reasoning tokens**: `{}`\n", reasoning));
            }
        }

        if let Some(reasoning) = &response.reasoning {
            info.push_str("\n## 📊 Reasoning\n\n");
            if let Some(effort) = &reasoning.effort {
                info.push_str(&format!("- **Effort**: `{}`\n", effort));
            }
            if let Some(summary) = &reasoning.summary {
                info.push_str(&format!("- **Summary**: {}\n", summary));
            }
        }

        if let Some(previous_id) = &response.previous_response_id {
            info.push_str(&format!("- **Previous Response ID**: `{}`\n", previous_id));
        }

        info
    }
}

impl Default for MarkdownRenderer {
    fn default() -> Self {
        Self::new()
    }
}