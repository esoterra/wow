use std::env;
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::{fs, path::PathBuf};

use anyhow::{Context, Result};

pub struct Shims {
    shim_dir: PathBuf,
}

impl Shims {
    pub fn new() -> Result<Self> {
        let home_dir = env::var("HOME").context("Could not determine home directory")?;
        let home_dir: PathBuf = home_dir.into();
        let shim_dir = home_dir.join(".wow/bin");
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

        println!("Creating shim for '{}'", tool_name);
        let shim_contents = format!("#!/usr/bin/env bash\nexec wow run {} -- $@\n", tool_name);
        fs::write(&shim_path, shim_contents)?;

        let shim_path_str = shim_path.to_str().context("Convert shim path to string")?;

        Command::new("chmod")
            .args(["+x", shim_path_str])
            .output()
            .context("failed to assign execute permission to shim")?;

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
        println!("Add directory `$HOME/.wow/bin` to your path.");
        Ok(())
    }

    /// This function does not return if it succeeds
    /// Execution is handed off to the new process
    pub fn execute_fallback(&self, tool_name: &str, args: Vec<String>) -> Result<()> {
        let fallback = self.find_fallback(&tool_name)?;
        let error = Command::new(fallback)
            .args(args)
            .exec();
        Err(error.into())
    }

    fn find_fallback(&self, tool_name: &str) -> Result<PathBuf> {
        let which_output = Command::new("which")
            .args(["-a", tool_name])
            .output()
            .context("Attempting to determine shim fallback")?;

        let which_stdout = which_output.stdout.to_vec();
        let which_stdout =
            String::from_utf8(which_stdout).context("Decoding 'which` output as UTF-8")?;
        which_stdout
            .split("\n")
            .map(|p| PathBuf::from(p))
            .filter(|p| !p.starts_with(&self.shim_dir))
            .nth(0)
            .context("Could not find fallback")
    }
}
