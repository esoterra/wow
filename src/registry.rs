use anyhow::{Context, Result};
use std::path::PathBuf;
use warg_client::{FileSystemClient, RegistryUrl};
use warg_credentials::keyring::get_auth_token;

use crate::config::Tool;

pub struct Registry {
    client: FileSystemClient,
}

impl Registry {
    pub fn new(url: Option<&str>) -> Result<Self> {
        let config = warg_client::Config::from_default_file()?.unwrap_or_default();
        let url = url.or(config.home_url.as_deref());
        let auth_token = if config.keyring_auth && url.is_some() {
            get_auth_token(&RegistryUrl::new(url.unwrap())?)?
        } else {
            None
        };
        Ok(Self {
            client: FileSystemClient::new_with_config(url, &config, auth_token)?,
        })
    }

    pub async fn ensure_downloaded(&mut self, tool: &Tool) -> Result<()> {
        let package = &tool.package;
        let requirement = tool.version_req()?;

        println!("Downloading package '{}' version {}", package, requirement);
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
