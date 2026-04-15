use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;

use crate::config::Config;
use crate::fetch::ai::{post_chat, ChatOpts};
use crate::models::{ClassifiedReading, RawReading};

#[derive(Deserialize)]
struct StepResult {
    step: u8,
    reason: String,
}

const SYSTEM_PROMPT: &str =
    "You are an AA step classifier. Given a daily reading excerpt, return the \
     single most relevant AA step number (1-12) and a one-sentence reason. \
     Respond with JSON only: {\"step\": 3, \"reason\": \"...\"}";

pub async fn classify(
    client: &Client,
    config: &Config,
    reading: RawReading,
) -> Result<ClassifiedReading> {
    let opts = ChatOpts {
        max_tokens: Some(256),
        temperature: Some(0.3),
        json_mode: true,
    };
    let content = post_chat(client, config, SYSTEM_PROMPT, &reading.text, opts).await?;
    let result: StepResult =
        serde_json::from_str(&content).context("parsing step JSON from AI response")?;

    let step = result.step.clamp(1, 12);

    Ok(ClassifiedReading {
        step,
        reason: result.reason,
        source: reading.source,
        title: reading.title,
        text: reading.text,
        url: reading.url,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn classify_parses_step_from_gateway_response() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "choices": [{
                    "message": {
                        "content": "{\"step\": 3, \"reason\": \"Surrender to a Higher Power\"}"
                    }
                }]
            }"#,
            )
            .create_async()
            .await;

        std::env::set_var("VERCEL_AI_GATEWAY_TOKEN", "test-token");

        let client = Client::new();
        let config = crate::config::Config {
            ai: crate::config::AiConfig {
                model: "anthropic/claude-haiku-4-5".to_string(),
                gateway_url: server.url(),
                api_version: None,
            },
        };

        let raw = RawReading {
            source: "AA.org".to_string(),
            title: "Daily Reflections".to_string(),
            text: "Made a decision to turn our will...".to_string(),
            url: "https://www.aa.org/daily-reflections".to_string(),
        };

        let result = classify(&client, &config, raw).await.unwrap();
        assert_eq!(result.step, 3);
        assert_eq!(result.reason, "Surrender to a Higher Power");
        assert_eq!(result.source, "AA.org");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn classify_clamps_step_to_valid_range() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"choices":[{"message":{"content":"{\"step\":99,\"reason\":\"out of range\"}"}}]}"#)
            .create_async()
            .await;

        std::env::set_var("VERCEL_AI_GATEWAY_TOKEN", "test-token");
        let client = Client::new();
        let config = crate::config::Config {
            ai: crate::config::AiConfig {
                model: "test".to_string(),
                gateway_url: server.url(),
                api_version: None,
            },
        };
        let raw = RawReading {
            source: "Test".to_string(),
            title: "Test".to_string(),
            text: "text".to_string(),
            url: "http://test".to_string(),
        };
        let result = classify(&client, &config, raw).await.unwrap();
        assert_eq!(result.step, 12);
    }

    #[tokio::test]
    async fn classify_clamps_step_below_minimum() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"choices":[{"message":{"content":"{\"step\":0,\"reason\":\"invalid\"}"}}]}"#,
            )
            .create_async()
            .await;

        std::env::set_var("VERCEL_AI_GATEWAY_TOKEN", "test-token");
        let client = Client::new();
        let config = crate::config::Config {
            ai: crate::config::AiConfig {
                model: "test".to_string(),
                gateway_url: server.url(),
                api_version: None,
            },
        };
        let raw = RawReading {
            source: "Test".to_string(),
            title: "Test".to_string(),
            text: "text".to_string(),
            url: "http://test".to_string(),
        };
        let result = classify(&client, &config, raw).await.unwrap();
        assert_eq!(result.step, 1);
    }

    #[tokio::test]
    async fn classify_invalid_step_number_non_numeric() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"choices":[{"message":{"content":"{\"step\":\"twelve\",\"reason\":\"text instead of number\"}"}}]}"#)
            .create_async()
            .await;

        std::env::set_var("VERCEL_AI_GATEWAY_TOKEN", "test-token");
        let client = Client::new();
        let config = crate::config::Config {
            ai: crate::config::AiConfig {
                model: "test".to_string(),
                gateway_url: server.url(),
                api_version: None,
            },
        };
        let raw = RawReading {
            source: "Test".to_string(),
            title: "Test".to_string(),
            text: "text".to_string(),
            url: "http://test".to_string(),
        };

        let result = classify(&client, &config, raw).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn classify_missing_step_field_returns_error() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"choices":[{"message":{"content":"{\"reason\":\"missing step field\"}"}}]}"#,
            )
            .create_async()
            .await;

        std::env::set_var("VERCEL_AI_GATEWAY_TOKEN", "test-token");
        let client = Client::new();
        let config = crate::config::Config {
            ai: crate::config::AiConfig {
                model: "test".to_string(),
                gateway_url: server.url(),
                api_version: None,
            },
        };
        let raw = RawReading {
            source: "Test".to_string(),
            title: "Test".to_string(),
            text: "text".to_string(),
            url: "http://test".to_string(),
        };

        let result = classify(&client, &config, raw).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn classify_missing_token_returns_error() {
        std::env::remove_var("VERCEL_AI_GATEWAY_TOKEN");

        let client = Client::new();
        let config = crate::config::Config {
            ai: crate::config::AiConfig {
                model: "test".to_string(),
                gateway_url: "https://example.com".to_string(),
                api_version: None,
            },
        };
        let raw = RawReading {
            source: "Test".to_string(),
            title: "Test".to_string(),
            text: "text".to_string(),
            url: "http://test".to_string(),
        };

        let result = classify(&client, &config, raw).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn classify_preserves_reading_metadata() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"choices":[{"message":{"content":"{\"step\":5,\"reason\":\"Test reason\"}"}}]}"#)
            .create_async()
            .await;

        std::env::set_var("VERCEL_AI_GATEWAY_TOKEN", "test-token");
        let client = Client::new();
        let config = crate::config::Config {
            ai: crate::config::AiConfig {
                model: "test".to_string(),
                gateway_url: server.url(),
                api_version: None,
            },
        };

        let raw = RawReading {
            source: "Special Source".to_string(),
            title: "Special Title".to_string(),
            text: "Special text".to_string(),
            url: "https://special.example.com".to_string(),
        };

        let result = classify(&client, &config, raw).await.unwrap();
        assert_eq!(result.source, "Special Source");
        assert_eq!(result.title, "Special Title");
        assert_eq!(result.text, "Special text");
        assert_eq!(result.url, "https://special.example.com");
    }

    #[tokio::test]
    async fn classify_invalid_json_response_returns_error() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"choices":[{"message":{"content":"not valid json"}}]}"#)
            .create_async()
            .await;

        std::env::set_var("VERCEL_AI_GATEWAY_TOKEN", "test-token");
        let client = Client::new();
        let config = crate::config::Config {
            ai: crate::config::AiConfig {
                model: "test".to_string(),
                gateway_url: server.url(),
                api_version: None,
            },
        };
        let raw = RawReading {
            source: "Test".to_string(),
            title: "Test".to_string(),
            text: "text".to_string(),
            url: "http://test".to_string(),
        };

        let result = classify(&client, &config, raw).await;
        assert!(result.is_err());
    }
}
