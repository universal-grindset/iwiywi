//! Journal: suspends the TUI and shells out to `$EDITOR` on
//! `~/.iwiywi/journal/YYYY-MM-DD.md`. Creates the file with a reflection
//! prompt on first use.

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_PROMPT: &str = "One thing to notice today.";

const TEMPLATE: &str = "\
# iwiywi journal — {date}

_{prompt}_

";

fn editor_command() -> Option<String> {
    for var in ["VISUAL", "EDITOR"] {
        if let Ok(v) = std::env::var(var) {
            if !v.trim().is_empty() { return Some(v); }
        }
    }
    Some("vi".to_string())
}

fn ensure_entry(path: &Path, prompt: Option<&str>) -> Result<()> {
    if path.exists() { return Ok(()); }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let date = chrono::Local::now().format("%A, %B %-d, %Y").to_string();
    let prompt = prompt.unwrap_or(DEFAULT_PROMPT);
    let body = TEMPLATE.replace("{date}", &date).replace("{prompt}", prompt);
    std::fs::write(path, body)?;
    Ok(())
}

/// Suspend the TUI, open today's journal entry in `$EDITOR`, then resume.
/// `seed` (optional) replaces the default `One thing to notice today.` line
/// when the entry file doesn't already exist.
pub fn open_today(journal_dir: PathBuf, seed: Option<String>) -> Result<PathBuf> {
    let filename = format!("{}.md", chrono::Local::now().format("%Y-%m-%d"));
    let path = journal_dir.join(filename);
    ensure_entry(&path, seed.as_deref())?;

    let cmd = editor_command().unwrap_or_else(|| "vi".to_string());

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    // Simple shell split — $EDITOR may include flags ("nvim -p").
    let mut parts = cmd.split_whitespace();
    let program = parts.next().unwrap_or("vi");
    let args: Vec<&str> = parts.collect();
    let status = Command::new(program).args(&args).arg(&path).status();

    execute!(io::stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    let _ = status;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn editor_command_falls_back_to_vi() {
        std::env::remove_var("VISUAL");
        std::env::remove_var("EDITOR");
        assert_eq!(editor_command().as_deref(), Some("vi"));
    }

    #[test]
    fn ensure_entry_creates_with_template_when_missing() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("2026-04-15.md");
        ensure_entry(&path, None).unwrap();
        let body = std::fs::read_to_string(&path).unwrap();
        assert!(body.starts_with("# iwiywi journal"));
        assert!(body.contains("One thing to notice today"));
    }

    #[test]
    fn ensure_entry_uses_seed_prompt_when_provided() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("2026-04-15.md");
        ensure_entry(&path, Some("Where did willingness show up today?")).unwrap();
        let body = std::fs::read_to_string(&path).unwrap();
        assert!(body.contains("Where did willingness show up today?"));
        assert!(!body.contains("One thing to notice today"));
    }

    #[test]
    fn ensure_entry_does_not_clobber_existing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("2026-04-15.md");
        std::fs::write(&path, "my notes").unwrap();
        ensure_entry(&path, None).unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "my notes");
    }
}
