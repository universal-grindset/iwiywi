use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ai: AiConfig,
    #[serde(default)]
    pub mobile: MobileConfig,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MobileConfig {
    pub gist_id: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ai: AiConfig {
                model: "anthropic/claude-haiku-4-5".to_string(),
                gateway_url: "https://ai-gateway.vercel.sh/v1".to_string(),
                api_version: None,
            },
            mobile: MobileConfig::default(),
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

pub fn save_config(cfg: &Config) -> Result<()> {
    let dir = config_dir();
    fs::create_dir_all(&dir).context("creating config dir")?;
    let s = toml::to_string_pretty(cfg).context("serializing config")?;
    fs::write(config_path(), s).context("writing config.toml")
}

pub fn load_env() -> Result<()> {
    let env_path = config_dir().join(".env");
    if env_path.exists() {
        dotenvy::from_path(&env_path).context("loading .env")?;
    }
    Ok(())
}

/// Derive the QR target URL from the saved gist_id.
/// Returns an empty string when there's no gist yet (QR will render an empty-URL placeholder).
pub fn qr_url(cfg: &Config) -> String {
    match &cfg.mobile.gist_id {
        Some(id) => format!("https://gist.github.com/{id}"),
        None => String::new(),
    }
}

const DEFAULT_IDLE_SECS: u64 = 60;

/// Parse a raw `IWIYWI_IDLE_SECS` value into an idle threshold.
/// Returns `None` when the screensaver should be disabled ("0"), otherwise a
/// `Duration`. Unparseable values fall back to the default (60s).
pub fn parse_idle_secs(raw: Option<&str>) -> Option<std::time::Duration> {
    let secs: u64 = match raw {
        None => DEFAULT_IDLE_SECS,
        Some(s) => s.parse().unwrap_or(DEFAULT_IDLE_SECS),
    };
    if secs == 0 {
        None
    } else {
        Some(std::time::Duration::from_secs(secs))
    }
}

/// Read `IWIYWI_IDLE_SECS` from the environment and parse it. See
/// `parse_idle_secs` for the semantics.
pub fn idle_secs() -> Option<std::time::Duration> {
    parse_idle_secs(std::env::var("IWIYWI_IDLE_SECS").ok().as_deref())
}

const DEFAULT_PULSE_SECS: u64 = 20;

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
    fn config_with_gist_id_round_trips() {
        let mut c = Config::default();
        c.mobile.gist_id = Some("abc123def456".to_string());
        let s = toml::to_string(&c).unwrap();
        let back: Config = toml::from_str(&s).unwrap();
        assert_eq!(back.mobile.gist_id.as_deref(), Some("abc123def456"));
    }

    #[test]
    fn qr_url_formats_gist_url() {
        let mut c = Config::default();
        c.mobile.gist_id = Some("deadbeef".to_string());
        assert_eq!(qr_url(&c), "https://gist.github.com/deadbeef");
    }

    #[test]
    fn qr_url_empty_when_no_gist() {
        assert_eq!(qr_url(&Config::default()), "");
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
    fn legacy_toml_without_mobile_section_loads() {
        let toml_str = r#"
            [ai]
            model = "x"
            gateway_url = "https://example.com"
        "#;
        let c: Config = toml::from_str(toml_str).unwrap();
        assert!(c.mobile.gist_id.is_none());
    }

    #[test]
    fn parse_idle_secs_defaults_to_sixty_when_none() {
        assert_eq!(
            parse_idle_secs(None),
            Some(std::time::Duration::from_secs(60))
        );
    }

    #[test]
    fn parse_idle_secs_returns_none_for_zero() {
        assert_eq!(parse_idle_secs(Some("0")), None);
    }

    #[test]
    fn parse_idle_secs_parses_positive_value() {
        assert_eq!(
            parse_idle_secs(Some("15")),
            Some(std::time::Duration::from_secs(15))
        );
    }

    #[test]
    fn parse_idle_secs_falls_back_on_garbage() {
        assert_eq!(
            parse_idle_secs(Some("not-a-number")),
            Some(std::time::Duration::from_secs(60))
        );
    }

    #[test]
    fn parse_idle_secs_falls_back_on_negative() {
        assert_eq!(
            parse_idle_secs(Some("-5")),
            Some(std::time::Duration::from_secs(60))
        );
    }

    #[test]
    fn parse_pulse_secs_defaults_to_twenty_when_none() {
        assert_eq!(parse_pulse_secs(None), Some(std::time::Duration::from_secs(20)));
    }

    #[test]
    fn parse_pulse_secs_returns_none_for_zero() {
        assert_eq!(parse_pulse_secs(Some("0")), None);
    }

    #[test]
    fn parse_pulse_secs_parses_positive_value() {
        assert_eq!(parse_pulse_secs(Some("45")), Some(std::time::Duration::from_secs(45)));
    }

    #[test]
    fn parse_pulse_secs_falls_back_on_garbage() {
        assert_eq!(parse_pulse_secs(Some("xx")), Some(std::time::Duration::from_secs(20)));
    }
}
