use anyhow::{Context, Result};
use chrono::Local;
use std::fs;
use std::path::PathBuf;

use crate::config::config_dir;
use crate::models::ClassifiedReading;

pub fn readings_path() -> PathBuf {
    let date = Local::now().format("%Y-%m-%d").to_string();
    config_dir().join(format!("readings-{date}.json"))
}

pub fn write_readings(readings: &[ClassifiedReading]) -> Result<()> {
    let dir = config_dir();
    fs::create_dir_all(&dir).context("creating ~/.iwiywi")?;
    let json = serde_json::to_string_pretty(readings).context("serializing readings")?;
    fs::write(readings_path(), json).context("writing readings JSON")?;
    Ok(())
}

pub fn read_readings() -> Result<Vec<ClassifiedReading>> {
    let path = readings_path();
    if !path.exists() {
        return Ok(vec![]);
    }
    let s = fs::read_to_string(&path).context("reading readings JSON")?;
    serde_json::from_str(&s).context("parsing readings JSON")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ClassifiedReading;
    use std::env;

    fn fixture() -> ClassifiedReading {
        ClassifiedReading {
            step: 7,
            reason: "Humility".to_string(),
            source: "Test".to_string(),
            title: "Test Reading".to_string(),
            text: "Humbly asked...".to_string(),
            url: "https://example.com".to_string(),
        }
    }

    #[test]
    fn write_then_read_round_trips() {
        // Use a temp dir to avoid clobbering real data
        let tmp = env::temp_dir().join("iwiywi_test");
        fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("readings-test.json");
        let readings = vec![fixture()];
        let json = serde_json::to_string_pretty(&readings).unwrap();
        fs::write(&path, &json).unwrap();
        let back: Vec<ClassifiedReading> = serde_json::from_str(&json).unwrap();
        assert_eq!(back.len(), 1);
        assert_eq!(back[0].step, 7);
    }
}
