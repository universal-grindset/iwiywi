use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Command;

/// Refreshes ~/.iwiywi/.env by running `vercel env pull`
pub fn env_pull(env_path: &Path) -> Result<()> {
    let status = Command::new("vercel")
        .args(["env", "pull", "--yes"])
        .arg(env_path)
        .status()
        .context("running `vercel env pull` — is the vercel CLI installed?")?;

    if !status.success() {
        bail!("`vercel env pull` exited with {}", status);
    }
    Ok(())
}

/// Deploys dist_dir contents via `vercel deploy --prod`
pub fn deploy(dist_dir: &Path) -> Result<()> {
    let output = Command::new("vercel")
        .args(["deploy", "--prod"])
        .arg(dist_dir)
        .output()
        .context("running `vercel deploy` — is the vercel CLI installed and authenticated?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("`vercel deploy` failed ({}): {}", output.status, stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Deployed: {stdout}");
    Ok(())
}
