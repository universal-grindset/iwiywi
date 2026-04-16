//! Journal browser: press `J` (shift+j) to open a scrollable list of
//! past journal entries in an overlay. Select one to read in the AI
//! overlay panel. Entries are Markdown files in `~/.iwiywi/journal/`.

use std::path::Path;

/// One journal entry: filename + first non-header line as preview.
#[derive(Debug, Clone)]
pub struct JournalEntry {
    pub filename: String,
    pub preview: String,
}

/// Scan the journal directory and return entries sorted newest-first.
pub fn list_entries(journal_dir: &Path) -> Vec<JournalEntry> {
    let Ok(rd) = std::fs::read_dir(journal_dir) else {
        return Vec::new();
    };
    let mut entries: Vec<JournalEntry> = rd
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        .filter_map(|e| {
            let filename = e.file_name().to_string_lossy().to_string();
            let content = std::fs::read_to_string(e.path()).ok()?;
            let preview = content
                .lines()
                .find(|l| !l.starts_with('#') && !l.trim().is_empty())
                .unwrap_or("")
                .chars()
                .take(60)
                .collect::<String>();
            Some(JournalEntry { filename, preview })
        })
        .collect();
    entries.sort_by(|a, b| b.filename.cmp(&a.filename));
    entries
}

/// Read the full content of a journal entry.
#[allow(dead_code, reason = "available for future drill-into-entry feature")]
pub fn read_entry(journal_dir: &Path, filename: &str) -> Option<String> {
    std::fs::read_to_string(journal_dir.join(filename)).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn list_entries_returns_empty_for_missing_dir() {
        let entries = list_entries(Path::new("/nonexistent"));
        assert!(entries.is_empty());
    }

    #[test]
    fn list_entries_finds_md_files_newest_first() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("2026-01-01.md"), "# Jan\n\nOld entry").unwrap();
        std::fs::write(dir.path().join("2026-04-15.md"), "# Apr\n\nNew entry").unwrap();
        std::fs::write(dir.path().join("notes.txt"), "not markdown").unwrap();
        let entries = list_entries(dir.path());
        assert_eq!(entries.len(), 2);
        assert!(entries[0].filename.contains("2026-04-15"));
        assert!(entries[1].filename.contains("2026-01-01"));
    }

    #[test]
    fn preview_extracts_first_content_line() {
        let dir = tempdir().unwrap();
        std::fs::write(
            dir.path().join("2026-04-15.md"),
            "# Title\n\n_Reflection prompt_\n\nMy notes here.",
        )
        .unwrap();
        let entries = list_entries(dir.path());
        assert!(entries[0].preview.contains("Reflection prompt"));
    }

    #[test]
    fn read_entry_returns_content() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("test.md"), "hello").unwrap();
        assert_eq!(read_entry(dir.path(), "test.md").as_deref(), Some("hello"));
    }
}
