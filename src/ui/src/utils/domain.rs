use anyhow::{Error, Result};
use gloo::utils::window;

pub fn host() -> Result<String> {
    let location = window().location();

    let protocol = location
        .protocol()
        .map_err(|e| Error::msg(format!("Failed to get current domain: {:?}", e)))?;
    let host = location
        .host()
        .map_err(|e| Error::msg(format!("Failed to get current domain: {:?}", e)))?;
    let url = format!("{}//{}", protocol, host);

    Ok(url)
}
