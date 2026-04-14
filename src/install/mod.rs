use anyhow::{bail, Context, Result};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn plist_path() -> PathBuf {
    dirs::home_dir()
        .expect("no home dir")
        .join("Library/LaunchAgents/com.iwiywi.fetch.plist")
}

fn binary_path() -> Result<PathBuf> {
    std::env::current_exe().context("getting current exe path")
}

pub fn plist_content(binary: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>com.iwiywi.fetch</string>
  <key>ProgramArguments</key>
  <array>
    <string>{binary}</string>
    <string>fetch</string>
  </array>
  <key>StartCalendarInterval</key>
  <dict>
    <key>Hour</key>
    <integer>6</integer>
    <key>Minute</key>
    <integer>0</integer>
  </dict>
  <key>StandardOutPath</key>
  <string>{home}/.iwiywi/fetch.log</string>
  <key>StandardErrorPath</key>
  <string>{home}/.iwiywi/fetch.log</string>
  <key>RunAtLoad</key>
  <false/>
</dict>
</plist>"#,
        binary = binary,
        home = dirs::home_dir().unwrap().display(),
    )
}

pub fn run() -> Result<()> {
    let binary = binary_path()?;
    let binary_str = binary.to_str().context("binary path is not valid UTF-8")?;
    let content = plist_content(binary_str);
    let path = plist_path();

    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(&path, &content).context("writing plist")?;

    let status = Command::new("launchctl")
        .args(["load", "-w"])
        .arg(&path)
        .status()
        .context("running launchctl load")?;

    if !status.success() {
        bail!("launchctl load failed with {}", status);
    }

    println!("Installed: {}", path.display());
    println!("iwiywi fetch will run daily at 6:00am local time.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plist_content_contains_binary_and_label() {
        let content = plist_content("/usr/local/bin/iwiywi");
        assert!(content.contains("<string>com.iwiywi.fetch</string>"));
        assert!(content.contains("<string>/usr/local/bin/iwiywi</string>"));
        assert!(content.contains("<string>fetch</string>"));
    }

    #[test]
    fn plist_content_schedules_at_6am() {
        let content = plist_content("/usr/local/bin/iwiywi");
        assert!(content.contains("<key>Hour</key>"));
        assert!(content.contains("<integer>6</integer>"));
        assert!(content.contains("<key>Minute</key>"));
        assert!(content.contains("<integer>0</integer>"));
    }

    #[test]
    fn plist_content_includes_log_paths() {
        let content = plist_content("/usr/local/bin/iwiywi");
        assert!(content.contains("fetch.log"));
        assert!(content.contains("StandardOutPath"));
        assert!(content.contains("StandardErrorPath"));
    }
}
