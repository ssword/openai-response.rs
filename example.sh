#!/bin/bash

# Example usage script for OpenAI CLI Tool
# Make sure to set your OPENAI_API_KEY environment variable before running

echo "OpenAI CLI Tool Examples"
echo "========================"
echo ""

# Check if API key is set
if [ -z "$OPENAI_API_KEY" ]; then
    echo "❌ Error: OPENAI_API_KEY environment variable is not set"
    echo "Please set it with: export OPENAI_API_KEY='your-api-key-here'"
    exit 1
fi

echo "✅ API key is set"
echo ""

# Build the project
echo "🔨 Building the project..."
cargo build --release
echo ""

# Example 1: Basic usage
echo "📝 Example 1: Basic question"
echo "Command: ./target/release/OpenAI-API \"What is Rust programming language?\""
echo "---"
./target/release/OpenAI-API "What is Rust programming language?"
echo ""
echo "---"
echo ""

# Example 2: Using different model
echo "📝 Example 2: Using GPT-3.5-turbo"
echo "Command: ./target/release/OpenAI-API --model gpt-3.5-turbo \"Write a haiku about programming\""
echo "---"
./target/release/OpenAI-API --model gpt-3.5-turbo "Write a haiku about programming"
echo ""
echo "---"
echo ""

# Example 3: Verbose output
echo "📝 Example 3: With usage statistics"
echo "Command: ./target/release/OpenAI-API --verbose \"Explain quantum computing in one sentence\""
echo "---"
./target/release/OpenAI-API --verbose "Explain quantum computing in one sentence"
echo ""
echo "---"
echo ""

echo "🎉 Examples completed!"
echo ""
echo "💡 Tips:"
echo "  - Run without arguments for interactive mode"
echo "  - Use --help to see all available options"
echo "  - Set different models with --model flag"
echo "  - Use --verbose to see token usage statistics"
