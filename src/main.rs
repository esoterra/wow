mod config;
mod registry;
mod run;
mod shims;
mod workspace;

use anyhow::{Context, Result};
use clap::Parser;
use workspace::Workspace;

/// The workspace manager for development environments.
#[derive(Debug, Parser)]
#[clap(name = "wow", version)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    Init,
    Run(Run),
}

#[derive(Debug, Parser)]
struct Run {
    tool_name: String,

    #[arg(allow_hyphen_values = true, last = true, hide = true)]
    args: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Init => exec_init().await.context("Initializing workspace")?,
        Command::Run(run) => run::run(&run.tool_name, run.args)
            .await
            .with_context(|| format!("Running tool '{}'", &run.tool_name))?,
    }

    Ok(())
}

async fn exec_init() -> Result<()> {
    let mut workspace = Workspace::try_new()?.context("Couldn't find workspace.")?;

    // workspace.registry.update().await?;

    for (_, tool) in workspace.config.tools.iter() {
        workspace
            .registry
            .ensure_downloaded(tool)
            .await
            .context("Ensuring tool component is downloaded")?;
    }

    for (tool_name, _) in workspace.config.tools.iter() {
        workspace
            .shims
            .ensure_shimmed(tool_name)
            .context("Shimming tool")?;
    }

    workspace
        .shims
        .check_path_configured()
        .context("Checking if PATH contains shim dir")?;

    Ok(())
}
