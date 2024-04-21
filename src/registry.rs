use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use warg_client::{
    storage::{ContentStorage, RegistryStorage},
    FileSystemClient,
};
use warg_protocol::{registry::PackageName, Version, VersionReq};

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

        let client = match FileSystemClient::try_new_with_config(url.into(), &config, None)? {
            warg_client::StorageLockResult::Acquired(client) => Ok(client),
            warg_client::StorageLockResult::NotAcquired(path) => {
                println!(
                    "blocking on lock for directory `{path}`...",
                    path = path.display()
                );

                FileSystemClient::new_with_config(Some(url), &config, None)
            }
        }?;

        Ok(Self { client })
    }

    pub async fn ensure_downloaded(&mut self, tool: &Tool) -> Result<()> {
        let package = PackageName::new(tool.package.clone())?;
        let requirement = match &tool.version {
            Some(version) => VersionReq::parse(&format!("={}", version))?,
            None => VersionReq::STAR,
        };
        self.client.download(&package, &requirement).await?;
        Ok(())
    }

    pub async fn component_path(&mut self, tool: &Tool) -> Result<PathBuf> {
        let package = PackageName::new(tool.package.clone())?;
        match &tool.version {
            Some(version) => {
                let version = Version::parse(version)?;
                self.exact_component_path(package, version).await
            }
            None => self.latest_component_path(package).await,
        }
    }

    async fn exact_component_path(
        &mut self,
        package: PackageName,
        version: Version,
    ) -> Result<PathBuf> {
        let package_info = self
            .client
            .registry()
            .load_package(&None, &package)
            .await
            .context("Package not found.")?
            .context("Package not found.")?;
        let release = package_info
            .state
            .release(&version)
            .context("Version not found.")?;
        let content = match &release.state {
            warg_protocol::package::ReleaseState::Released { content } => content,
            warg_protocol::package::ReleaseState::Yanked { .. } => bail!("Package was yanked."),
        };

        let path = self
            .client
            .content()
            .content_location(&content)
            .context("Tool binary not found.")?;
        Ok(path)
    }

    async fn latest_component_path(&mut self, package: PackageName) -> Result<PathBuf> {
        _ = package;
        bail!("TODO: support tools without versions")
    }
}
