use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use warg_client::{FileSystemClient, RegistryUrl};
use warg_credentials::keyring::get_auth_token;

use crate::config::Tool;

pub struct Registry {
    client: FileSystemClient,
}

impl Registry {
    pub fn new(url: Option<&str>) -> Result<Self> {
        let client = if let Some(url) = url {
            let config = warg_client::Config {
                home_url: Some(url.into()),
                ..Default::default()
            };

            FileSystemClient::new_with_config(Some(url), &config, None)?
        } else if let Some(config) = warg_client::Config::from_default_file()? {
            let auth = if config.keyring_auth {
                if let Some(reg_url) = &config.home_url {
                    get_auth_token(&RegistryUrl::new(reg_url)?)?
                } else if let Some(url) = config.home_url.as_ref() {
                    get_auth_token(&RegistryUrl::new(url)?)?
                } else {
                    None
                }
            } else {
                None
            };
            FileSystemClient::new_with_config(None, &config, auth)?
        } else {
            bail!("registry configuration not found");
        };

        Ok(Self { client })
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
