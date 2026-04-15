use anyhow::{bail, Context, Result};
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

const FILENAME: &str = "iwiywi.md";
const DESCRIPTION: &str = "iwiywi — today's AA readings";

/// Create a new gist or update the existing one, return its ID.
///
/// If `existing_id` is `Some`, updates that gist in place. Otherwise creates a new
/// one and returns its ID so the caller can persist it.
pub fn publish(markdown: &str, existing_id: Option<&str>) -> Result<String> {
    match existing_id {
        Some(id) => {
            update(markdown, id)?;
            Ok(id.to_string())
        }
        None => create(markdown),
    }
}

fn create(markdown: &str) -> Result<String> {
    let mut child = Command::new("gh")
        .args([
            "gist",
            "create",
            "--public",
            "--desc",
            DESCRIPTION,
            "--filename",
            FILENAME,
            "-",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("spawning `gh gist create` — is the GitHub CLI installed?")?;

    child
        .stdin
        .as_mut()
        .expect("stdin piped")
        .write_all(markdown.as_bytes())
        .context("writing markdown to gh stdin")?;

    let output = child.wait_with_output().context("waiting for gh")?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("`gh gist create` failed ({}): {}", output.status, stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_gist_id(stdout.trim())
        .with_context(|| format!("parsing gist id from gh output: {stdout}"))
}

fn update(markdown: &str, id: &str) -> Result<()> {
    // `gh gist edit <id> --filename foo -` reads new contents from stdin.
    // We write to a temp file and pass it positionally to avoid any piping quirks.
    let tmp = std::env::temp_dir().join(format!("iwiywi-{id}.md"));
    fs::write(&tmp, markdown).context("writing temp markdown file")?;

    let status = Command::new("gh")
        .args(["gist", "edit", id, "--filename", FILENAME])
        .arg(&tmp)
        .status()
        .context("running `gh gist edit`")?;

    let _ = fs::remove_file(&tmp);

    if !status.success() {
        bail!("`gh gist edit` exited with {status}");
    }
    Ok(())
}

fn parse_gist_id(url_or_id: &str) -> Result<String> {
    // gh prints either a full URL (https://gist.github.com/<user>/<id>) or the ID.
    let trimmed = url_or_id.trim();
    if let Some(id) = trimmed.rsplit('/').next() {
        if !id.is_empty() {
            return Ok(id.to_string());
        }
    }
    bail!("empty gist id in output: {trimmed:?}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_id_from_full_url() {
        let id = parse_gist_id("https://gist.github.com/jdfowler/abcdef123456").unwrap();
        assert_eq!(id, "abcdef123456");
    }

    #[test]
    fn parses_bare_id() {
        let id = parse_gist_id("abcdef123456").unwrap();
        assert_eq!(id, "abcdef123456");
    }

    #[test]
    fn parses_url_with_trailing_whitespace() {
        let id = parse_gist_id("https://gist.github.com/user/deadbeef\n").unwrap();
        assert_eq!(id, "deadbeef");
    }

    #[test]
    fn empty_output_errors() {
        assert!(parse_gist_id("").is_err());
    }
}
