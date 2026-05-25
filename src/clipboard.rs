use anyhow::{Context, Result};
use std::io::Write;
use std::process::{Command, Stdio};

pub fn copy(text: &str) -> Result<()> {
    let mut child = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .spawn()
        .context("Failed to run wl-copy — is it installed?")?;

    child
        .stdin
        .as_mut()
        .context("Failed to open wl-copy stdin")?
        .write_all(text.as_bytes())
        .context("Failed to write to wl-copy")?;

    child.wait().context("wl-copy exited with error")?;
    Ok(())
}
