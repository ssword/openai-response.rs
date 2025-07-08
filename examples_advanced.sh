#!/bin/bash

# Advanced Examples for OpenAI CLI Tool
# Demonstrates all the new Responses API parameters

echo "OpenAI CLI Tool - Advanced Examples"
echo "===================================="
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

# Example 1: Basic usage with system instructions
echo "📝 Example 1: System Instructions and Token Limits"
echo "Command: ./target/release/openai-api --instructions 'Be concise and helpful' --max-output-tokens 50 'Explain quantum computing'"
echo "---"
./target/release/openai-api --instructions "Be concise and helpful" --max-output-tokens 50 "Explain quantum computing"
echo ""
echo "---"
echo ""

# Example 2: Advanced sampling parameters
echo "📝 Example 2: Advanced Sampling (Top-p and Log Probabilities)"
echo "Command: ./target/release/openai-api --top-p 0.9 --top-logprobs 3 --temperature 0.8 'Write a creative opening line for a story'"
echo "---"
./target/release/openai-api --top-p 0.9 --top-logprobs 3 --temperature 0.8 "Write a creative opening line for a story"
echo ""
echo "---"
echo ""

# Example 3: Service tier and user tracking
echo "📝 Example 3: Service Tier and User Tracking"
echo "Command: ./target/release/openai-api --service-tier priority --user demo_user_123 'Important business question'"
echo "---"
./target/release/openai-api --service-tier priority --user demo_user_123 "Important business question"
echo ""
echo "---"
echo ""

# Example 4: Background processing
echo "📝 Example 4: Background Processing"
echo "Command: ./target/release/openai-api --background 'Analyze this complex topic in detail'"
echo "---"
./target/release/openai-api --background "Analyze this complex topic in detail"
echo ""
echo "---"
echo ""

# Example 5: Streaming response
echo "📝 Example 5: Streaming Response"
echo "Command: ./target/release/openai-api --stream 'Tell me a short story'"
echo "---"
./target/release/openai-api --stream "Tell me a short story"
echo ""
echo "---"
echo ""

# Example 6: JSON output with all parameters
echo "📝 Example 6: JSON Output with Multiple Parameters"
echo "Command: ./target/release/openai-api --json --temperature 0.7 --max-output-tokens 100 --store true 'Hello, world!'"
echo "---"
./target/release/openai-api --json --temperature 0.7 --max-output-tokens 100 --store true "Hello, world!"
echo ""
echo "---"
echo ""

# Example 7: Truncation strategy
echo "📝 Example 7: Auto Truncation Strategy"
echo "Command: ./target/release/openai-api --truncation auto --max-output-tokens 30 'Very long prompt that might exceed context'"
echo "---"
./target/release/openai-api --truncation auto --max-output-tokens 30 "Very long prompt that might exceed context window limits"
echo ""
echo "---"
echo ""

echo "🎉 Advanced examples completed!"
echo ""
echo "💡 Advanced Tips:"
echo "  - Use --service-tier priority for time-sensitive requests"
echo "  - Combine --top-p and --temperature for fine-tuned creativity control"
echo "  - Use --user for tracking and improved cache hit rates"
echo "  - Use --background for long-running analysis tasks"
echo "  - Use --stream for real-time response generation"
echo "  - Use --max-output-tokens to control response length and costs"
echo "  - Use --instructions for consistent system-level guidance"
echo "  - Use --truncation auto for handling large inputs gracefully"
echo ""
echo "🔧 Parameter Validation:"
echo "  - Temperature: 0.0 to 2.0"
echo "  - Top-p: 0.0 to 1.0"
echo "  - Top-logprobs: 0 to 20"
echo "  - Service-tier: auto, default, flex, priority"
echo "  - Truncation: auto, disabled"
