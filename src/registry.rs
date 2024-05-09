use anyhow::{Context, Result};
use std::path::PathBuf;
use warg_client::FileSystemClient;

use crate::config::Tool;

pub struct Registry {
    client: FileSystemClient,
}

impl Registry {
    pub fn new(url: Option<&str>) -> Result<Self> {
        Ok(Self {
            client: FileSystemClient::new_with_default_config(url)?,
        })
    }

    pub async fn ensure_downloaded(&mut self, tool: &Tool) -> Result<()> {
        println!(
            "Downloading package `{package}` version requirement `{version}`",
            package = &tool.package,
            version = tool.version_req()?
        );
        self.component_path(tool).await?;
        Ok(())
    }

    pub async fn component_path(&mut self, tool: &Tool) -> Result<PathBuf> {
        Ok(self
            .client
            .download(&tool.package, &tool.version_req()?)
            .await
            .context("Failed to download.")?
            .context("Package or version was not found.")?
            .path)
    }
}
