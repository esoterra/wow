use std::path::PathBuf;

use crate::{config::Config, registry::Registry, shims::Shims};
use anyhow::{bail, Result};
use std::env;

pub struct Workspace {
    pub path: PathBuf,
    pub config: Config,
    pub registry: Registry,
    pub shims: Shims,
}

impl Workspace {
    pub fn new() -> Result<Self> {
        let (path, config_path) = find_paths()?;
        let config = Config::parse_file(config_path)?;
        let registry = Registry::new(&config.registry)?;
        let shims = Shims::new()?;
        Ok(Self {
            path,
            config,
            registry,
            shims,
        })
    }
}

/// Returns (workspace, config) paths
fn find_paths() -> Result<(PathBuf, PathBuf)> {
    let cwd = env::current_dir()?;
    let mut next = cwd.as_path();

    let config_path = next.join("wow.kdl");
    if config_path.exists() {
        return Ok((next.to_owned(), config_path));
    }

    while let Some(parent) = next.parent() {
        next = parent;

        let config_path = next.join("wow.kdl");
        if config_path.exists() {
            return Ok((next.to_owned(), config_path));
        }
    }

    bail!("Could not find `wow.kdl` config in this directory or a parent.")
}
