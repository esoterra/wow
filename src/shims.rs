use std::env;
use std::{fs, path::PathBuf};

use anyhow::{Context, Result};

pub struct Shims {
    shim_dir: PathBuf,
}

impl Shims {
    pub fn new() -> Result<Self> {
        #[allow(deprecated)]
        let home_dir = env::home_dir().context("Could not determine home directory")?;
        let shim_dir = home_dir.join(".wow/shims");
        Ok(Shims { shim_dir })
    }

    pub fn ensure_shimmed(&self, tool_name: &str) -> Result<()> {
        if !self.shim_dir.exists() {
            println!("Creating shim directory");
            fs::create_dir_all(&self.shim_dir).context("Creating shim directory")?;
        }

        let shim_path = self.shim_dir.join(tool_name);

        if shim_path.exists() {
            // shim already exists
            // assume it is configured correctly
            return Ok(());
        }

        let shim_contents = format!("#!/usr/bin/env bash\nexec wow run {} -- $@\n", tool_name);
        fs::write(&shim_path, shim_contents)?;

        let shim_path_str = shim_path.to_str().context("Convert shim path to string")?;

        use std::process::Command;
        Command::new("chmod")
            .args(["+x", shim_path_str])
            .output()
            .expect("failed to assign execute permission to shim");

        Ok(())
    }

    pub fn check_path_configured(&self) -> Result<()> {
        let Some(path) = env::var_os("PATH") else {
            println!("PATH variable not set");
            return Ok(());
        };
        let path_dirs = env::split_paths(&path);
        for path_dir in path_dirs {
            if path_dir == self.shim_dir {
                return Ok(());
            }
        }
        println!("Add directory `~/.wow/shims` to your path.");
        Ok(())
    }
}
