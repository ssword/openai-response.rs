# OpenAI API: Create a Model Response — Request Body Reference

This document provides a comprehensive overview of all parameters available in the request body for the `POST https://api.openai.com/v1/responses` endpoint. Use this as a reference when constructing requests to generate model responses.

---

## Request Body Fields

| Field                | Type                  | Required | Default    | Description |
|----------------------|-----------------------|----------|------------|-------------|
| `background`         | boolean \| null       | Optional | `false`    | Whether to run the model response in the background. |
| `include`            | array \| null         | Optional |            | Additional output data to include in the response. Supported values:<br>- `code_interpreter_call.outputs`<br>- `computer_call_output.output.image_url`<br>- `file_search_call.results`<br>- `message.input_image.image_url`<br>- `message.output_text.logprobs`<br>- `reasoning.encrypted_content` |
| `input`              | string \| array       | Optional |            | Text, image, or file inputs to the model, used to generate a response. |
| `instructions`       | string \| null        | Optional |            | A system (or developer) message inserted into the model's context. Not inherited from previous responses. |
| `max_output_tokens`  | integer \| null       | Optional |            | Upper bound for the number of tokens that can be generated for a response (includes visible output and reasoning tokens). |
| `max_tool_calls`     | integer \| null       | Optional |            | Max total calls to built-in tools allowed in a response (across all tools). |
| `metadata`           | map                   | Optional |            | Up to 16 key-value pairs (keys: max 64 chars, values: max 512 chars) for attaching additional structured info. |
| `model`              | string                | Optional |            | Model ID (e.g., `gpt-4o`, `o3`) used to generate the response. |
| `parallel_tool_calls`| boolean \| null       | Optional | `true`     | Whether to allow the model to run tool calls in parallel. |
| `previous_response_id`| string \| null       | Optional |            | Unique ID of the previous response to create multi-turn conversations. |
| `prompt`             | object \| null        | Optional |            | Reference to a prompt template and its variables. |
| `reasoning`          | object \| null        | Optional |            | **o-series models only:** Configuration options for reasoning models. |
| `service_tier`       | string \| null        | Optional | `auto`     | Processing type: `auto`, `default`, `flex`, or `priority`. |
| `store`              | boolean \| null       | Optional | `true`     | Whether to store the generated model response for later retrieval. |
| `stream`             | boolean \| null       | Optional | `false`    | If `true`, streams model response data as it is generated (server-sent events). |
| `temperature`        | number \| null        | Optional | `1`        | Sampling temperature (0 to 2). Higher = more random, lower = more deterministic. |
| `text`               | object                | Optional |            | Config options for a text response (plain text or structured JSON). |
| `tool_choice`        | string \| object      | Optional |            | How the model should select which tool(s) to use. See `tools` parameter for options. |
| `tools`              | array                 | Optional |            | Array of tools (built-in or custom functions) the model may call. |
| `top_logprobs`       | integer \| null       | Optional |            | Integer (0–20): number of most likely tokens to return with log probabilities. |
| `top_p`              | number \| null        | Optional | `1`        | Nucleus sampling: considers tokens with top_p probability mass. |
| `truncation`         | string \| null        | Optional | `disabled` | Truncation strategy:<br>- `auto`: Truncate to fit context window by dropping input items mid-conversation<br>- `disabled`: (default) Request fails if context window exceeded. |
| `user`               | string                | Optional |            | Stable identifier for end-users (improves cache hit rates and abuse detection). |

---

## Parameter Details

### `background`
- **Type:** `boolean` or `null`
- **Description:** If set to `true`, the model response runs in the background.

### `include`
- **Type:** `array` or `null`
- **Supported values:**
    - `code_interpreter_call.outputs`
    - `computer_call_output.output.image_url`
    - `file_search_call.results`
    - `message.input_image.image_url`
    - `message.output_text.logprobs`
    - `reasoning.encrypted_content`
- **Description:** Includes additional data in the model response output.

### `input`
- **Type:** `string` or `array`
- **Description:** The main content (text, image, or file) for the model to process.

### `instructions`
- **Type:** `string` or `null`
- **Description:** System/developer message inserted into the model's context for this response.

### `max_output_tokens`
- **Type:** `integer` or `null`
- **Description:** Sets an upper limit on the number of output tokens (including visible and reasoning tokens).

### `max_tool_calls`
- **Type:** `integer` or `null`
- **Description:** Maximum number of tool calls allowed per response (total across all tools).

### `metadata`
- **Type:** `map`
- **Description:** Up to 16 key-value pairs for storing additional information about the object.

### `model`
- **Type:** `string`
- **Description:** The model to use for the response, e.g., `gpt-4o`.

### `parallel_tool_calls`
- **Type:** `boolean` or `null`
- **Default:** `true`
- **Description:** Allow tool calls to run in parallel.

### `previous_response_id`
- **Type:** `string` or `null`
- **Description:** Use for multi-turn conversations by referencing a previous response.

### `prompt`
- **Type:** `object` or `null`
- **Description:** Reference to a prompt template and its variables.

### `reasoning`
- **Type:** `object` or `null`
- **Description:** (o-series models only) Configuration for reasoning models.

### `service_tier`
- **Type:** `string` or `null`
- **Default:** `auto`
- **Options:** `auto`, `default`, `flex`, `priority`
- **Description:** Specifies the processing type for the request.

### `store`
- **Type:** `boolean` or `null`
- **Default:** `true`
- **Description:** Whether to store the response for later retrieval.

### `stream`
- **Type:** `boolean` or `null`
- **Default:** `false`
- **Description:** If `true`, enables server-sent event streaming of the response.

### `temperature`
- **Type:** `number` or `null`
- **Default:** `1`
- **Description:** Controls randomness in sampling. Use `temperature` or `top_p`, not both.

### `text`
- **Type:** `object`
- **Description:** Options for text output (plain or structured JSON).

### `tool_choice`
- **Type:** `string` or `object`
- **Description:** Controls which tool(s) the model may use. See `tools`.

### `tools`
- **Type:** `array`
- **Description:** List of built-in or custom tools/functions available to the model.

### `top_logprobs`
- **Type:** `integer` or `null`
- **Range:** 0–20
- **Description:** Number of top tokens to return with log probabilities.

### `top_p`
- **Type:** `number` or `null`
- **Default:** `1`
- **Description:** Nucleus sampling. Considers tokens with cumulative probability up to `top_p`.

### `truncation`
- **Type:** `string` or `null`
- **Default:** `disabled`
- **Options:** `auto`, `disabled`
- **Description:** Strategy for handling context window overflows.

### `user`
- **Type:** `string`
- **Description:** Stable identifier for the end-user.

---

## Example Request

```json
{
  "model": "gpt-4o",
  "input": "Tell me a three sentence bedtime story about a unicorn.",
  "temperature": 0.7,
  "max_output_tokens": 100,
  "stream": false,
  "metadata": {
    "purpose": "demo",
    "user_id": "user_123"
  }
}
```

---

## Notes

- **Not all parameters are required.** Only specify those you need to customize.
- **Avoid setting both `temperature` and `top_p` unless you understand sampling strategies.**
- **For advanced use-cases:** Refer to the OpenAI API documentation for details on prompt templates, tool calling, and structured outputs.