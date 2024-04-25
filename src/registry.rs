use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use warg_client::{
    storage::{ContentStorage, RegistryStorage},
    FileSystemClient,
};

use crate::config::Tool;

pub struct Registry {
    client: FileSystemClient,
}

impl Registry {
    pub fn new(url: &str) -> Result<Self> {
        let config = warg_client::Config {
            home_url: Some(url.into()),
            ..Default::default()
        };

        let client = FileSystemClient::new_with_config(Some(url), &config, None)?;

        Ok(Self { client })
    }

    pub async fn ensure_downloaded(&mut self, tool: &Tool) -> Result<()> {
        let package = &tool.package;
        let requirement = tool.version_req()?;

        println!("Downloading package '{}' version {}", package, requirement);
        self.client.upsert([package]).await?;
        self.client.download(package, &requirement).await?;
        Ok(())
    }

    pub async fn component_path(&mut self, tool: &Tool) -> Result<PathBuf> {
        let package = &tool.package;
        let requirement = tool.version_req()?;

        let package_info = self
            .client
            .registry()
            .load_package(&None, package)
            .await
            .context("Package not found.")?
            .context("Package not found.")?;
        let release = package_info
            .state
            .find_latest_release(&requirement)
            .context("Version not found.")?;
        let content = match &release.state {
            warg_protocol::package::ReleaseState::Released { content } => content,
            warg_protocol::package::ReleaseState::Yanked { .. } => bail!("Package was yanked."),
        };

        let path = self
            .client
            .content()
            .content_location(content)
            .context("Tool binary not found.")?;
        Ok(path)
    }
}
