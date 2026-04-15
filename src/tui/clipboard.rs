//! Clipboard copy. Shells out to `pbcopy` on macOS, or `xclip` / `wl-copy`
//! on Linux. Silent failure when no clipboard tool is available.

use std::io::Write;
use std::process::{Command, Stdio};

pub fn copy(text: &str) -> bool {
    let candidates: &[(&str, &[&str])] = &[
        ("pbcopy", &[]),
        ("wl-copy", &[]),
        ("xclip", &["-selection", "clipboard"]),
        ("xsel", &["--clipboard", "--input"]),
    ];
    for (cmd, args) in candidates {
        if let Ok(mut child) = Command::new(cmd)
            .args(*args)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            if let Some(stdin) = child.stdin.as_mut() {
                let _ = stdin.write_all(text.as_bytes());
            }
            let _ = child.wait();
            return true;
        }
    }
    false
}
