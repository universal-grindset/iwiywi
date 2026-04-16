use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ai: AiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub model: String,
    pub gateway_url: String,
    /// When set, request goes to Azure OpenAI: `api-key` header instead of
    /// `Authorization: Bearer`, and `?api-version=<this>` is appended to the
    /// chat-completions URL. Leave unset for OpenAI / Vercel-style gateways.
    #[serde(default)]
    pub api_version: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ai: AiConfig {
                model: "anthropic/claude-haiku-4-5".to_string(),
                gateway_url: "https://ai-gateway.vercel.sh/v1".to_string(),
                api_version: None,
            },
        }
    }
}

pub fn config_dir() -> PathBuf {
    // `dirs::home_dir` returns None only on platforms without a HOME concept
    // (or with a broken environment). iwiywi is a desktop CLI; running it
    // without a home directory is unrecoverable, so panicking with context
    // is the honest failure mode.
    dirs::home_dir()
        .expect("could not resolve $HOME — iwiywi requires a writable home directory")
        .join(".iwiywi")
}

pub fn config_path() -> PathBuf {
    config_dir().join("config.toml")
}

pub fn load_config() -> Result<Config> {
    let path = config_path();
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

const DEFAULT_PULSE_SECS: u64 = 45;

/// Parse `IWIYWI_PULSE_SECS`. Returns `Some(Duration)` for auto-advance,
/// `None` for manual-only. Default 20s when unset; 0 → None.
pub fn parse_pulse_secs(raw: Option<&str>) -> Option<std::time::Duration> {
    let secs: u64 = match raw {
        None => DEFAULT_PULSE_SECS,
        Some(s) => s.parse().unwrap_or(DEFAULT_PULSE_SECS),
    };
    if secs == 0 {
        None
    } else {
        Some(std::time::Duration::from_secs(secs))
    }
}

pub fn pulse_secs() -> Option<std::time::Duration> {
    parse_pulse_secs(std::env::var("IWIYWI_PULSE_SECS").ok().as_deref())
}

/// Parse `IWIYWI_SOBER_SINCE=YYYY-MM-DD` into the number of whole days
/// between that date and today (local time). Returns `None` when the env
/// var is unset or unparseable. Negative values — a date in the future —
/// are returned as-is and treated as "don't show" by the caller.
pub fn parse_sobriety_days(raw: Option<&str>, today: chrono::NaiveDate) -> Option<i64> {
    let s = raw?;
    let start = chrono::NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d").ok()?;
    Some((today - start).num_days())
}

pub fn sobriety_days() -> Option<i64> {
    let raw = std::env::var("IWIYWI_SOBER_SINCE").ok();
    parse_sobriety_days(raw.as_deref(), chrono::Local::now().date_naive())
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
    fn malformed_toml_returns_error() {
        let bad = r#"
            [ai
            model = "missing closing bracket
        "#;
        let result: Result<Config, _> = toml::from_str(bad);
        assert!(result.is_err());
    }

    #[test]
    fn legacy_toml_minimal_loads() {
        let toml_str = r#"
            [ai]
            model = "x"
            gateway_url = "https://example.com"
        "#;
        let c: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(c.ai.model, "x");
    }

    #[test]
    fn parse_pulse_secs_defaults_when_none() {
        assert_eq!(
            parse_pulse_secs(None),
            Some(std::time::Duration::from_secs(45))
        );
    }

    #[test]
    fn parse_pulse_secs_returns_none_for_zero() {
        assert_eq!(parse_pulse_secs(Some("0")), None);
    }

    #[test]
    fn parse_pulse_secs_parses_positive_value() {
        assert_eq!(
            parse_pulse_secs(Some("45")),
            Some(std::time::Duration::from_secs(45))
        );
    }

    #[test]
    fn parse_pulse_secs_falls_back_on_garbage() {
        assert_eq!(
            parse_pulse_secs(Some("xx")),
            Some(std::time::Duration::from_secs(45))
        );
    }
}
