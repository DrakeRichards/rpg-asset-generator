use crate::json::clean_schema;

use anyhow::{Error, Result};
use async_openai::{
    types::{
        ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
        CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema,
    },
    Client,
};
use serde_json::Value;

pub async fn request_structured_response(
    schema_name: &str,
    schema_description: Option<String>,
    schema_text: &str,
    initial_prompt: String,
    system_prompt: String,
) -> Result<String> {
    let client = Client::new();

    // Clean the schema for use with OpenAI's API.
    let cleaned_schema: String = clean_schema(schema_text)?;
    let schema: Option<Value> = serde_json::from_str(cleaned_schema.as_str())?;

    // Set the response format to JSON schema.
    let response_format = ResponseFormat::JsonSchema {
        json_schema: ResponseFormatJsonSchema {
            description: schema_description,
            name: schema_name.to_string(),
            schema,
            strict: Some(true),
        },
    };

    // Create the request.
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-2024-08-06")
        .messages([
            ChatCompletionRequestSystemMessage::from(system_prompt).into(),
            ChatCompletionRequestUserMessage::from(initial_prompt).into(),
        ])
        .response_format(response_format)
        .build()?;

    // Send the request to OpenAI's API.
    let response = client.chat().create(request).await?;

    // Get the response content.
    for choice in response.choices {
        if let Some(content) = choice.message.content {
            return Ok(content);
        }
    }

    Err(Error::msg("No response content found"))
}
