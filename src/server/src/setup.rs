use std::{net::SocketAddr, path::PathBuf};

use anyhow::Result;
use tokio::fs::create_dir_all;
use tracing::info;

use crate::util::path::home_dir;

#[derive(Clone, Debug)]
pub struct Config {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Setup {
    home_dir: PathBuf,
    config: Config,
    local_ip: SocketAddr,
}

impl Setup {
    pub async fn new(local_ip: SocketAddr) -> Result<Self> {
        let home_dir = Self::setup_home_dir().await?;
        let config = Config {
            name: "anonymous".to_string(),
        };
        Ok(Self {
            home_dir,
            config,
            local_ip,
        })
    }

    #[inline]
    pub const fn home_dir(&self) -> &PathBuf {
        &self.home_dir
    }

    #[inline]
    pub const fn config(&self) -> &Config {
        &self.config
    }

    #[inline]
    pub const fn local_ip(&self) -> &SocketAddr {
        &self.local_ip
    }

    async fn setup_home_dir() -> Result<PathBuf> {
        let home_dir = home_dir()?;

        if !home_dir.exists() {
            info!(?home_dir, "Creating IPChat Home Directory");
            create_dir_all(&home_dir).await?;
        } else {
            info!(?home_dir, "Home directory already exists");
        }

        Ok(home_dir)
    }
}
