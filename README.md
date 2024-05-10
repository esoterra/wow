# Workspaces on Wasm (wow)

The workspace manager for development environments.

## Installing `wow`

```bash
cargo install --git https://github.com/esoterra/wow wow

vim ~/.bashrc # add `$HOME/.wow/bin` to path
```

## Initialized a workspace

Running `wow init` will set up the current workspace.
The current directory must contain a `wow.toml` file.

```bash
$ git clone https://github.com/foo/bar

$ cd bar

$ cat wow.kdl
registry "ba.wa.dev"

tool "wasm-tools" package="ba:wasm-tools" version="1.2"

$ wow init
```

In this scenario initializing
* Detects the `wow.kdl` config file
* Pulls `ba:wasm-tools@1.2`
* Creates `~/.wow/shims/wasm-tools` shim

## Using commands

Once you've installed wow and initialized your workspace, you can call the defined tools using the names you specified in your `wow.kdl` file.

```bash
$ wasm-tools component new my-core.wasm -o my-component.wasm
```

## WIP tools

We're going to work on getting the following tools compiled to components, available in registries, and usable in `wow` as soon as we can.

- [ ] wasm-tools
- [ ] wac
- [ ] claw-cli
