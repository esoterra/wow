use std::path::PathBuf;

use crate::{config::Config, registry::Registry, shims::Shims};
use anyhow::Result;
use std::env;

pub struct Workspace {
    pub path: PathBuf,
    pub config: Config,
    pub registry: Registry,
    pub shims: Shims,
}

impl Workspace {
    pub fn try_new() -> Result<Option<Self>> {
        let Some((path, config_path)) = find_paths()? else {
            return Ok(None);
        };
        let config = Config::parse_file(config_path)?;
        let registry = Registry::new(config.registry.as_deref())?;
        let shims = Shims::new()?;
        Ok(Some(Self {
            path,
            config,
            registry,
            shims,
        }))
    }
}

/// Returns (workspace, config) paths
fn find_paths() -> Result<Option<(PathBuf, PathBuf)>> {
    let cwd = env::current_dir()?;
    let mut current = Some(cwd.as_path());

    while let Some(dir) = current {
        let config = dir.join("wow.toml");
        if config.is_file() {
            return Ok(Some((dir.to_owned(), config)));
        }

        current = dir.parent();
    }

    Ok(None)
}
