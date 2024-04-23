use std::fs;

use anyhow::{Context, Result};
use cap_std::fs::Dir;
use wasmtime::{
    component::{Component, Linker}, Config, Engine, Store
};
use wasmtime_wasi::{DirPerms, FilePerms, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi::command::{Command, add_to_linker as add_command_to_linker};

use crate::{shims::Shims, workspace::Workspace};

pub async fn run(tool_name: String, args: Vec<String>) -> Result<()> {
    match Workspace::try_new()? {
        Some(workspace) => run_tool(workspace, tool_name, args).await,
        None => Shims::new()?.execute_fallback(tool_name, args),
    }
}

async fn run_tool(mut workspace: Workspace, tool_name: String, args: Vec<String>) -> Result<()> {
    let tool = workspace
        .config
        .tools
        .get(&tool_name)
        .context("No such tool")?;
    let component_path = workspace.registry.component_path(tool).await?;
    let component_bytes = fs::read(component_path)?;

    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let args = [vec![tool_name.clone()], args].concat();

    let wow_store = WowStore {
        table: ResourceTable::new(),
        ctx: build_wasi_ctx(&workspace, args.as_slice())?
    };

    let engine = Engine::new(&config).unwrap();
    let mut linker: Linker<WowStore> = Linker::new(&engine);
    add_command_to_linker(&mut linker)?;
    let mut store: Store<WowStore> = Store::new(&engine, wow_store);

    let component = Component::new(&engine, &component_bytes).unwrap();
    let (command, _instance) = Command::instantiate_async(&mut store, &component, &linker).await?;

    command.wasi_cli_run().call_run(&mut store).await?.ok().context("Failed to execute tool")?;

    Ok(())
}

struct WowStore {
    table: ResourceTable,
    ctx: WasiCtx,
}

impl WasiView for WowStore {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

fn build_wasi_ctx(workspace: &Workspace, args: &[String]) -> Result<WasiCtx> {
    let root = std::fs::File::open(&workspace.path)?;

    let mut builder = WasiCtxBuilder::new();

    builder.inherit_stdin().inherit_stdout().inherit_stderr();
    builder.inherit_env();
    builder.args(args);
    builder.preopened_dir(
        Dir::from_std_file(root),
        DirPerms::all(),
        FilePerms::all(),
        "/",
    );

    Ok(builder.build())
}
