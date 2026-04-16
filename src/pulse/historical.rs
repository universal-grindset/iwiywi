use crate::models::ClassifiedReading;
use crate::pulse::{PulseItem, PulseKind, PulseSource};
use std::path::Path;

pub struct HistoricalReadings {
    items: Vec<PulseItem>,
}

impl HistoricalReadings {
    /// Load all `readings-*.json` files from `dir`, skipping today's file.
    /// Errors on individual files are logged to stderr and that file skipped.
    pub fn load_from(dir: &Path, today_basename: &str) -> Self {
        let mut items = Vec::new();
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return Self { items },
        };
        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();
            if !name.starts_with("readings-") || !name.ends_with(".json") {
                continue;
            }
            if name == today_basename {
                continue;
            }
            let path = entry.path();
            let raw = match std::fs::read_to_string(&path) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("warn: read {path:?}: {e}");
                    continue;
                }
            };
            let readings: Vec<ClassifiedReading> = match serde_json::from_str(&raw) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("warn: parse {path:?}: {e}");
                    continue;
                }
            };
            for r in readings {
                items.push(PulseItem {
                    kind: PulseKind::HistoricalReading,
                    step: Some(r.step),
                    label: format!("Step {} · {} (archive)", r.step, r.source),
                    body: r.text,
                });
            }
        }
        Self { items }
    }
}

impl PulseSource for HistoricalReadings {
    fn name(&self) -> &'static str {
        "historical"
    }
    fn items(&self) -> &[PulseItem] {
        &self.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn historical_loads_from_multiple_files() {
        let dir = tempdir().unwrap();
        let a = r#"[{"step":1,"reason":"r","source":"A","title":"t","text":"old1","url":"u"}]"#;
        let b = r#"[{"step":3,"reason":"r","source":"B","title":"t","text":"old2","url":"u"}]"#;
        std::fs::write(dir.path().join("readings-2026-01-01.json"), a).unwrap();
        std::fs::write(dir.path().join("readings-2026-01-02.json"), b).unwrap();
        let h = HistoricalReadings::load_from(dir.path(), "readings-2026-04-15.json");
        assert_eq!(h.items().len(), 2);
        let bodies: Vec<&str> = h.items().iter().map(|i| i.body.as_str()).collect();
        assert!(bodies.contains(&"old1"));
        assert!(bodies.contains(&"old2"));
    }

    #[test]
    fn historical_skips_today_file() {
        let dir = tempdir().unwrap();
        let today =
            r#"[{"step":1,"reason":"r","source":"A","title":"t","text":"today","url":"u"}]"#;
        std::fs::write(dir.path().join("readings-2026-04-15.json"), today).unwrap();
        let h = HistoricalReadings::load_from(dir.path(), "readings-2026-04-15.json");
        assert!(h.items().is_empty());
    }

    #[test]
    fn historical_ignores_unrelated_files() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("notes.txt"), "hi").unwrap();
        std::fs::write(dir.path().join("config.toml"), "[ai]").unwrap();
        let h = HistoricalReadings::load_from(dir.path(), "readings-anything.json");
        assert!(h.items().is_empty());
    }

    #[test]
    fn historical_continues_past_malformed_file() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("readings-bad.json"), "{not valid json").unwrap();
        let good = r#"[{"step":2,"reason":"r","source":"X","title":"t","text":"good","url":"u"}]"#;
        std::fs::write(dir.path().join("readings-good.json"), good).unwrap();
        let h = HistoricalReadings::load_from(dir.path(), "readings-2026-04-15.json");
        assert_eq!(h.items().len(), 1);
        assert_eq!(h.items()[0].body, "good");
    }
}
