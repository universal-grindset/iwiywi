//! Shared gateway chat-completions helper. Every AI call in the app goes
//! through `post_chat`. Handles the Azure OpenAI vs Vercel AI Gateway auth
//! split (`api-key` header + `?api-version=...` vs bearer token).

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Clone, Copy, Default)]
pub struct ChatOpts {
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub json_mode: bool,
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ResponseFormat>,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Serialize)]
struct ResponseFormat {
    r#type: &'static str,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChoiceMessage,
}

#[derive(Deserialize)]
struct ChoiceMessage {
    content: String,
}

/// POST a chat-completions request to the configured gateway. Returns the
/// text content of `choices[0].message.content`. All auth/endpoint
/// assembly lives here; callers just pass system/user/opts.
pub async fn post_chat(
    client: &Client,
    config: &Config,
    system: &str,
    user: &str,
    opts: ChatOpts,
) -> Result<String> {
    let request = ChatRequest {
        model: &config.ai.model,
        messages: vec![
            Message { role: "system", content: system },
            Message { role: "user",   content: user },
        ],
        max_tokens: opts.max_tokens,
        temperature: opts.temperature,
        response_format: opts.json_mode.then_some(ResponseFormat { r#type: "json_object" }),
    };

    let endpoint = match &config.ai.api_version {
        Some(v) => format!("{}/chat/completions?api-version={v}", config.ai.gateway_url),
        None => format!("{}/chat/completions", config.ai.gateway_url),
    };
    let req = client.post(&endpoint).json(&request);
    let req = match &config.ai.api_version {
        Some(_) => req.header(
            "api-key",
            std::env::var("AZURE_OPENAI_API_KEY").context("AZURE_OPENAI_API_KEY not set")?,
        ),
        None => req.bearer_auth(
            std::env::var("VERCEL_AI_GATEWAY_TOKEN").context("VERCEL_AI_GATEWAY_TOKEN not set")?,
        ),
    };
    let resp = req.send().await.context("calling AI gateway")?;
    let chat: ChatResponse = resp.json().await.context("parsing AI response")?;
    let content = chat
        .choices
        .into_iter()
        .next()
        .map(|c| c.message.content)
        .context("AI response had no choices")?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn post_chat_returns_content_from_gateway() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"choices":[{"message":{"content":"hello"}}]}"#)
            .create_async()
            .await;
        std::env::set_var("VERCEL_AI_GATEWAY_TOKEN", "t");

        let client = Client::new();
        let config = Config {
            ai: crate::config::AiConfig {
                model: "m".to_string(),
                gateway_url: server.url(),
                api_version: None,
            },
        };
        let out = post_chat(&client, &config, "sys", "user", ChatOpts::default())
            .await
            .unwrap();
        assert_eq!(out, "hello");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn post_chat_errors_when_token_missing() {
        std::env::remove_var("VERCEL_AI_GATEWAY_TOKEN");
        let client = Client::new();
        let config = Config {
            ai: crate::config::AiConfig {
                model: "m".to_string(),
                gateway_url: "https://example.com".to_string(),
                api_version: None,
            },
        };
        let result = post_chat(&client, &config, "sys", "user", ChatOpts::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn post_chat_errors_on_empty_choices() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"choices":[]}"#)
            .create_async()
            .await;
        std::env::set_var("VERCEL_AI_GATEWAY_TOKEN", "t");
        let client = Client::new();
        let config = Config {
            ai: crate::config::AiConfig {
                model: "m".to_string(),
                gateway_url: server.url(),
                api_version: None,
            },
        };
        let result = post_chat(&client, &config, "sys", "user", ChatOpts::default()).await;
        assert!(result.is_err());
    }
}
