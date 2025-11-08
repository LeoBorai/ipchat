use std::env::home_dir as std_home_dir;
use std::path::PathBuf;

use anyhow::{Context, Result};

const IPCHAT_HOME_DIR: &str = ".ipchat";

/// Retrieves the home directory path for the IPChat application.
pub fn home_dir() -> Result<PathBuf> {
    let home_dir = std_home_dir().context("Failed to get home directory")?;
    Ok(home_dir.join(IPCHAT_HOME_DIR))
}
