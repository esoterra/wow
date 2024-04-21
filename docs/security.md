# Security

It's complicated to talk about the security of `wow` because two seemingly-conflicting things are both true:

1. Wasm-based sandboxing is going to blow away `npm install` or `cargo install` and run in terms of security. For example, tools don't have the ability to invoke the shell or access files outside the workspace.
2. It still isn't going to be *that* secure because things like VS Code and rust-analyzer will execute code in files (e.g. build.rs) in an insecure way and these dev tools need to be able to write files.

So we need to balance being really excited about the security benefits while being realistic and defensive about the fact that there's still a lot of damage that can be done.

## Potential Strategies

The security risks should be managed and we should consider clever techniques that are enabled by our wasm sandbox to mitigate them.

These are roughly ordered by least to most sophisticated.

### File filters / lists

Allow users to configure either using the `wow.toml` or some other mechanism the files that each command is allowed to touch.

**Security concerns:**
* Users may make very coarse allow/deny lists for convenience that have limited value.

### Trust on first use per file

When a tool tries to touch a file or directory, prompt the user to allow it to do so. This could make it obvious when a tool is trying to access something it shouldn't.

Users may not like this due to the volume of prompts some tools will trigger.

**Security concerns:**
* Tools that should be accessing files that can do damage won't be detected.
* Tools may attack by touching files users think are innocuous.
* Tools may overwhelm users desire to manage access closely.

### Read-only commands

Make some commands unable to modify anything at all and present them a read-only view of the file-system

The main limitation of this option is that some commands need to produce modify files (e.g. formatters) they can't be run read-only.

### Separation of read and write locations

Create a world that separates the read area (e.g. source stuff) vs. target area (e.g. compilation output). As long as these written files aren't automatically executed by tools, this is much more secure.

This also enables some commands, in a build process for example, to be parallelized more easily by avoiding concurrent accesses. It could also be used to perform pipelining where one tool's output is another tool's input.

### Deferred deltas

When a tool modifies a file, it actually modifies a copy-on-write file-system local to this command execution. These changes are then diffed against the original state and presented to the user who accepts/rejects them at the end.

**Security Concerns:**
* Users may not review the diff closely, especially if it is large.
* Even users which do closely review the diff may not notice dangerous edits.
