# Workspaces on Wasm (wow)

The workspace manager for development environments.

## Installing `wow`

```bash
cargo install --git https://github.com/esoterra/wow wow

```

## Initialized a workspace

Running `wow init` will set up the current workspace.
The current directory must contain a `wow.toml` file.

```bash
$ git clone https://github.com/foo/bar

$ cd bar

$ cat wow.toml
registry = "ba.wa.dev"

[tools]
wasm-tools = { package = "ba:wasm-tools", version = "1.2" }

$ wow init

$ vim ~/.bashrc # add `$HOME/.wow/shims` to path
```

In this scenario initializing
* Detects the `wow.toml` config file
* Pulls `ba:wasm-tools@1.2`
* Creates `~/.wow/shims/wasm-tools` shim

## Using commands

Once you've installed wow and initialized your workspace, you can call the defined tools using the names you specified in your `wow.toml` file.

```bash
$ wasm-tools component new my-core.wasm -o my-component.wasm
```

## WIP tools

We're going to work on getting the following tools compiled to components, available in registries, and usable in `wow` as soon as we can.

- [ ] wasm-tools
- [ ] wac
- [ ] claw-cli
