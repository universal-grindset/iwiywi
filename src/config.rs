use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ai: AiConfig,
    pub vercel: VercelConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub model: String,
    pub gateway_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VercelConfig {
    pub project_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ai: AiConfig {
                model: "anthropic/claude-haiku-4-5".to_string(),
                gateway_url: "https://ai-gateway.vercel.sh/v1".to_string(),
            },
            vercel: VercelConfig {
                project_url: String::new(),
            },
        }
    }
}

pub fn config_dir() -> PathBuf {
    dirs::home_dir()
        .expect("could not find home directory")
        .join(".iwiywi")
}

pub fn load_config() -> Result<Config> {
    let path = config_dir().join("config.toml");
    if !path.exists() {
        return Ok(Config::default());
    }
    let s = fs::read_to_string(&path).context("reading config.toml")?;
    toml::from_str(&s).context("parsing config.toml")
}

pub fn load_env() -> Result<()> {
    let env_path = config_dir().join(".env");
    if env_path.exists() {
        dotenvy::from_path(&env_path).context("loading .env")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_expected_model() {
        let c = Config::default();
        assert_eq!(c.ai.model, "anthropic/claude-haiku-4-5");
    }

    #[test]
    fn config_round_trips_toml() {
        let c = Config::default();
        let s = toml::to_string(&c).unwrap();
        let back: Config = toml::from_str(&s).unwrap();
        assert_eq!(back.ai.model, c.ai.model);
        assert_eq!(back.ai.gateway_url, c.ai.gateway_url);
    }

    #[test]
    fn config_with_custom_values() {
        let c = Config {
            ai: AiConfig {
                model: "custom-model".to_string(),
                gateway_url: "https://custom.example.com/v1".to_string(),
            },
            vercel: VercelConfig {
                project_url: "https://custom.vercel.app".to_string(),
            },
        };

        let s = toml::to_string(&c).unwrap();
        let back: Config = toml::from_str(&s).unwrap();
        assert_eq!(back.ai.model, "custom-model");
        assert_eq!(back.ai.gateway_url, "https://custom.example.com/v1");
        assert_eq!(back.vercel.project_url, "https://custom.vercel.app");
    }

    #[test]
    fn malformed_toml_returns_error() {
        let bad_toml = r#"
            [ai
            model = "missing closing bracket
        "#;

        let result: Result<Config, _> = toml::from_str(bad_toml);
        assert!(result.is_err());
    }

    #[test]
    fn config_preserves_special_characters_in_urls() {
        let c = Config {
            ai: AiConfig {
                model: "test".to_string(),
                gateway_url: "https://api.example.com/v1?key=abc&id=123".to_string(),
            },
            vercel: VercelConfig {
                project_url: "https://project-name-with-dashes.vercel.app".to_string(),
            },
        };

        let s = toml::to_string(&c).unwrap();
        let back: Config = toml::from_str(&s).unwrap();
        assert!(back.ai.gateway_url.contains("?key=abc&id=123"));
        assert!(back.vercel.project_url.contains("-with-dashes"));
    }

    #[test]
    fn default_config_has_sensible_defaults() {
        let c = Config::default();
        assert!(!c.ai.model.is_empty());
        assert!(!c.ai.gateway_url.is_empty());
        assert!(c.ai.gateway_url.starts_with("https://"));
    }

    #[test]
    fn empty_string_fields_deserialize() {
        let toml_str = r#"
            [ai]
            model = ""
            gateway_url = ""
            [vercel]
            project_url = ""
        "#;

        let c: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(c.ai.model, "");
        assert_eq!(c.ai.gateway_url, "");
        assert_eq!(c.vercel.project_url, "");
    }
}
