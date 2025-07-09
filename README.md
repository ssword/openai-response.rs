# OpenAI CLI Tool

A command-line tool written in Rust for interacting with OpenAI's Responses API.

## Features

- 🤖 Send questions/prompts to OpenAI's latest models via the Responses API
- 💬 Support for both command-line arguments and interactive input
- 🔧 Configurable model selection (gpt-4.1, gpt-4.1-mini, etc.)
- 📊 Comprehensive response metadata and usage statistics
- 🛡️ Comprehensive error handling for API and network issues
- 🚀 Fast and lightweight Rust implementation
- 📋 Rich response format with detailed content structure

## Installation

### Prerequisites

- Rust (latest stable version)
- OpenAI API key

### Build from Source

```bash
git clone <repository-url>
cd OpenAI-API
cargo build --release
```

The compiled binary will be available at `target/release/OpenAI-API`.

## Setup

### API Key Configuration

Set your OpenAI API key as an environment variable:

```bash
export OPENAI_API_KEY="your-api-key-here"
```

For permanent setup, add this to your shell profile (`.bashrc`, `.zshrc`, etc.).

## Usage

### Command Line Arguments

```bash
# Basic usage with a prompt
./target/release/OpenAI-API "Tell me a three sentence bedtime story about a unicorn"

# Using a different model
./target/release/OpenAI-API --model gpt-3.5-turbo "What is the capital of France?"

# Show usage statistics
./target/release/OpenAI-API --verbose "Explain quantum computing in simple terms"
```

### Interactive Mode

Run without arguments to enter interactive mode:

```bash
./target/release/OpenAI-API
```

You'll be prompted to enter your question.

### Options

#### Core Options

- `--model <MODEL>`: Specify the OpenAI model to use (default: gpt-4o-mini)
- `--verbose`: Show detailed usage statistics and response metadata
- `--json`: Output the complete response in JSON format

#### Response Configuration

- `--temperature <TEMP>`: Set response randomness (0.0 to 2.0)
- `--max-output-tokens <TOKENS>`: Maximum number of output tokens
- `--instructions <TEXT>`: System instructions for the model
- `--top-p <VALUE>`: Nucleus sampling parameter (0.0 to 1.0)
- `--top-logprobs <COUNT>`: Number of top log probabilities to return (0-20)

#### Behavior Options

- `--stream`: Enable streaming response
- `--background`: Run in background
- `--store <BOOL>`: Whether to store the response
- `--parallel-tool-calls <BOOL>`: Allow parallel tool calls

#### Advanced Options

- `--service-tier <TIER>`: Service tier (auto, default, flex, priority)
- `--user <ID>`: User identifier for tracking
- `--previous-response-id <ID>`: Previous response ID for multi-turn conversations
- `--truncation <STRATEGY>`: Truncation strategy (auto, disabled)

#### Utility Options

- `--help`: Display help information
- `--version`: Show version information

## Examples

### Basic Question

```bash
$ ./target/release/OpenAI-API "What is Rust programming language?"
🤖 Sending request to OpenAI...

📝 Response:
Rust is a systems programming language that focuses on safety, speed, and concurrency...
```

### With Verbose Output

```bash
$ ./target/release/OpenAI-API --verbose "Explain machine learning"
🤖 Sending request to OpenAI...

📝 Response:
Machine learning is a subset of artificial intelligence...

📊 Response Metadata:
  Response ID: resp_67ccd2bed1ec8190b14f964abc0542670bb6a6b452d3795b
  Model: gpt-4.1-2025-04-14
  Created At: 1741476542
  Status: completed

📊 Token Usage:
  Input tokens: 15
  Output tokens: 150
  Total tokens: 165
  Cached tokens: 0
  Reasoning tokens: 0
```

### Using Different Model with Temperature

```bash
$ ./target/release/OpenAI-API --model gpt-4o --temperature 0.8 "Write a haiku about programming"
```

### Advanced Parameters

```bash
# With system instructions and token limits
$ ./target/release/OpenAI-API --instructions "Be concise and helpful" --max-output-tokens 100 "Explain quantum computing"

# With service tier and user tracking
$ ./target/release/OpenAI-API --service-tier priority --user user123 "Important question"

# Multi-turn conversation
$ ./target/release/OpenAI-API --previous-response-id resp_abc123 "Continue our discussion"

# With nucleus sampling and log probabilities
$ ./target/release/OpenAI-API --top-p 0.9 --top-logprobs 5 "Generate creative text"

# Background processing
$ ./target/release/OpenAI-API --background "Long analysis task"

# Streaming response
$ ./target/release/OpenAI-API --stream "Tell me a story"
```

### JSON Output

```bash
$ ./target/release/OpenAI-API --json "Hello, world!"
{
  "id": "resp_67ccd2bed1ec8190b14f964abc0542670bb6a6b452d3795b",
  "object": "response",
  "created_at": 1741476542,
  "status": "completed",
  "model": "gpt-4.1-2025-04-14",
  "output": [
    {
      "type": "message",
      "id": "msg_67ccd2bf17f0819081ff3bb2cf6508e60bb6a6b452d3795b",
      "status": "completed",
      "role": "assistant",
      "content": [
        {
          "type": "output_text",
          "text": "Hello! How can I help you today?",
          "annotations": []
        }
      ]
    }
  ],
  "usage": {
    "input_tokens": 10,
    "output_tokens": 12,
    "total_tokens": 22
  },
  "temperature": 1.0,
  "top_p": 1.0
}
```

## API Integration Details

This tool uses the OpenAI Responses API with comprehensive data structures that match the actual API response format:

- **Endpoint**: `https://api.openai.com/v1/responses`
- **Authentication**: Bearer token via `Authorization` header
- **Request Format**: JSON with `model`, `input`, and optional parameters like `temperature`
- **Response Parsing**: Full response structure including metadata, usage statistics, and rich content

### Data Structures

The tool implements detailed data structures that capture the complete OpenAI Responses API format:

- **ResponseApiResponse**: Complete response with ID, status, model, creation timestamp, output array, and usage
- **OutputMessage**: Individual output message with type, ID, status, role, and content array
- **Usage**: Detailed token usage including input/output tokens and breakdown details
- **ContentType**: Support for different content types (text, output_text with annotations)
- **Reasoning**: Optional reasoning information with effort and summary details

### Advanced Features

- **Temperature Control**: Adjust response creativity (0.0 = deterministic, 2.0 = very creative)
- **JSON Output**: Complete API response in JSON format for integration with other tools
- **Detailed Metadata**: Response IDs, timestamps, status information, and reasoning details
- **Token Usage Breakdown**: Input/output tokens with caching and reasoning token details

### Supported Models

- `gpt-4o-mini` (default)
- `gpt-4o`
- Any other OpenAI model supported by the Responses API

## Error Handling

The tool provides clear error messages for common issues:

- Missing or empty `OPENAI_API_KEY` environment variable
- Network connectivity problems
- OpenAI API errors (rate limits, invalid requests, etc.)
- Invalid JSON responses
- Empty responses

## Development

### Dependencies

- `reqwest` - HTTP client with JSON support
- `tokio` - Async runtime
- `serde` & `serde_json` - JSON serialization/deserialization
- `clap` - Command-line argument parsing
- `anyhow` - Error handling

### Building for Development

```bash
cargo build
cargo run -- "test prompt"
```

### Running Tests

```bash
cargo test
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license information here]

## Troubleshooting

### Common Issues

1. **"OPENAI_API_KEY environment variable not set"**
   - Ensure you've set the environment variable correctly
   - Check that the variable is exported in your current shell session

2. **Network/API errors**
   - Verify your internet connection
   - Check if your API key is valid and has sufficient credits
   - Ensure you're not hitting rate limits

3. **Build errors**
   - Make sure you have the latest stable Rust version
   - Run `cargo clean` and try building again

For more help, please open an issue in the repository.
