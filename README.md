<div align="center">
  <h1>Workspaces on Wasm (<code>wow</code>)</h1>

  <p>
    <strong>The sandboxed workspace manager for development environments</strong>
  </p>

  <p>
    <a href="https://techforpalestine.org/learn-more"><img src="https://badge.techforpalestine.org/default" alt="build status" /></a>
  </p>
</div>

## Introduction

`wow` is a WIP workspace manager for development environments.
It allows you to share every *tool* you need for a project with your coworkers.

Every *tool* is implemented in WebAssembly byte-code and `wow` comes with a runtime
allowing you to *sandbox* each tool, so they are only granted permissions they need, but no more!

## Installing `wow`

```bash
cargo install --git https://github.com/esoterra/wow wow

vim ~/.bashrc # add `$HOME/.wow/bin` to path
```

## Usage

### List your tools

`wow` use a super simple configuration language named [KDL][kdl].

Start by writing the following in a file named `wow.kdl` located at the root of
your workspace:

```kdl
// This is our registry where we publish tools contained in `examples/`
registry "wow.wa.dev"

// List of tools you need for your project, they will be available
// on the command line as "cowsay" and "cat2" respectively.
tool "cowsay" package="wow:cowsay"
tool "cat2" package="wow:kitty"
```

[kdl]: https://kdl.dev

### `init` your workspace

Run `wow init` to set up the current workspace.
It will download and cache every tool and make them available in your
shell.

Using the previously defined `wow.kdl` will result in:
* `wow` detects the `wow.kdl` config file
* Pulls `wow:cowsay` and `wow:kitty`
* Creates `~/.wow/shims/cowsay` and `~/.wow/shims/cat2` shims

## Using commands

Once you've installed `wow` and initialized your workspace,
you can call the defined tools using the names you specified in your `wow.kdl` file.

```bash
$ cat2 ./wow.kdl
```

## WIP tools

We're going to work on getting the following tools compiled to components,
available in registries, and usable in `wow` as soon as we can. :rocket:

- [ ] wasm-tools
- [ ] wac
- [ ] claw-cli
