# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based CLI tool for interacting with OpenAI's Responses API. The tool provides both command-line and interactive modes for sending prompts to OpenAI models and receiving responses.

## Key Commands

### Build and Run
```bash
# Build for development
cargo build

# Build for release
cargo build --release

# Run in development mode
cargo run -- "your prompt here"

# Run release binary
./target/release/OpenAI-API "your prompt here"
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_response_api_deserialization
```

### Example Usage
```bash
# Run example script (requires OPENAI_API_KEY)
./example.sh

# Interactive mode
./target/release/OpenAI-API

# With options
./target/release/OpenAI-API --model gpt-4o --verbose --temperature 0.8 "your prompt"
```

## Architecture

### Core Components

- **main.rs**: Single-file application containing all functionality
- **CLI Parsing**: Uses `clap` with derive macros for argument parsing
- **HTTP Client**: `reqwest` for OpenAI API communication
- **Data Structures**: Comprehensive serde models matching OpenAI API format

### Key Data Structures

- **ResponseRequest/ResponseApiResponse**: Main API request/response structures
- **OutputMessage**: Supports rich content arrays with different content types
- **ContentType**: Enum for different message content types (text, output_text)
- **Usage**: Detailed token usage statistics with breakdown
- **ErrorDetail/ApiError**: Structured error handling for API errors

### API Integration

- **Endpoint**: `https://api.openai.com/v1/responses`
- **Authentication**: Bearer token via `OPENAI_API_KEY` environment variable
- **Models**: Supports gpt-4o-mini (default), gpt-4o, and other OpenAI models
- **Parameters**: Temperature, model selection, JSON output, verbose mode

### Error Handling

- Environment variable validation (OPENAI_API_KEY)
- HTTP status code checking
- Structured API error parsing
- JSON deserialization error handling
- User input validation

## Development Notes

### Dependencies
- `anyhow`: Error handling
- `clap`: CLI argument parsing with derive features
- `reqwest`: HTTP client with JSON support
- `serde`/`serde_json`: Serialization/deserialization
- `tokio`: Async runtime with full features

### Environment Setup
The tool requires `OPENAI_API_KEY` environment variable to be set for API authentication.

### CLI Options
- `--model`: Specify OpenAI model
- `--verbose`: Show response metadata and token usage
- `--temperature`: Control response randomness (0.0-2.0)
- `--json`: Output complete API response in JSON format