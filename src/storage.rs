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
        let readings = vec![fixture()];
        // Call actual module functions
        write_readings(&readings).expect("write failed");
        let back = read_readings().expect("read failed");
        assert_eq!(back.len(), 1);
        assert_eq!(back[0].step, 7);
        assert_eq!(back[0].source, "Test");
    }
}
