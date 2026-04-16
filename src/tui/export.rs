//! Export: write the current mixer's items to a Markdown file in
//! `~/.iwiywi/exports/`. Filename is `YYYY-MM-DD-HHMMSS.md`.

use std::path::PathBuf;

use crate::pulse::PulseMixer;

pub fn write_current(mixer: &PulseMixer, exports_dir: PathBuf) -> Option<PathBuf> {
    std::fs::create_dir_all(&exports_dir).ok()?;
    let now = chrono::Local::now();
    let filename = format!("{}.md", now.format("%Y-%m-%d-%H%M%S"));
    let path = exports_dir.join(filename);

    let mut body = String::new();
    body.push_str(&format!(
        "# iwiywi export\n\n_{date}_\n\n",
        date = now.format("%A, %B %-d, %Y · %H:%M")
    ));
    body.push_str(&format!(
        "_{n} items in current focus_\n\n---\n\n",
        n = mixer.len()
    ));
    for item in mixer.all() {
        body.push_str(&format!("### {}\n\n", item.label));
        body.push_str(&format!("_{}_\n\n", item.kind.display_label()));
        body.push_str(&format!("{}\n\n---\n\n", item.body));
    }

    std::fs::write(&path, body).ok()?;
    Some(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pulse::Order;
    use tempfile::tempdir;

    #[test]
    fn write_current_produces_a_markdown_file() {
        let dir = tempdir().unwrap();
        let mixer = PulseMixer::from_sources(&[], None, Order::Random);
        let path = write_current(&mixer, dir.path().to_path_buf()).expect("write");
        assert!(path.exists());
        let body = std::fs::read_to_string(&path).unwrap();
        assert!(body.starts_with("# iwiywi export"));
    }
}
