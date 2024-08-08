# Spawning and Managing Child Processes

In Kinode OS, a "parent" process can create additional processes, known as "children" (also discussed [here](../process/processes.md#spawning-child-processes)).
These child processes are particularly useful for handling intensive tasks (referred to as "workers") that require long computation times without hindering the performance of the main application.
They are also beneficial for segregating distinct logical components.
Each process is its own subdirectory within the package.
E.g., for Kinode processes written in Rust, each is its own Rust project, complete with a separate Cargo.toml file.

Your package's file structure might resemble the following:

```
spawn
├── Cargo.toml
├── metadata.json
├── child
│   ├── Cargo.toml
│   └── src
├── parent
│   ├── Cargo.toml
│   └── src
├── pkg
...
```
To start a child process, use the [`spawn()`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/fn.spawn.html) function from [`kinode_process_lib`](https://github.com/kinode-dao/process_lib).
The following example demonstrates a basic parent process whose sole function is to spawn a child process and grant it the ability to send messages using `http_client`:
```rust
{{#includehidetest ../code/spawn/parent/src/lib.rs}}
```

The child process can be anything, for simplicity's sake, here is a degenerate process that does nothing but print it's name and die:
```rust
{{#includehidetest ../code/spawn/child/src/lib.rs}}
```
The spawn function in Kinode comprises several parameters, each serving a specific purpose in the process creation:

- `name: Option<String>`: This parameter specifies the name of the process.
If set to None, the process is automatically assigned a numerical identifier, resulting in a ProcessId formatted like `123456789:my-package:john.os`.

- `wasm_path: String`: Indicates the location of the compiled WebAssembly (Wasm) bytecode for the process.

- `on_exit: OnExit`: Determines the behavior of the process upon termination, whether due to completion, a crash, or a panic.
OnExit is an enum with three potential values:

  - `None`: The process will take no action upon exiting.
  - `Restart`: The process will automatically restart after termination.
  - `Requests: Vec<(Address, Request, Option<LazyLoadBlob>)>`: Upon process termination, a series of predefined requests will be dispatched.
- `request_capabilities: Vec<Capability>`: This argument is for passing immediate capabilities to the child process.
   As illustrated in the provided example, the parent's `http_client` messaging capability was shared with the child.

- `grant_capabilities: Vec<ProcessId>`: This argument is for granting capabilities to other processes on start.
  However, for security reasons, you limit it just to the `"messaging"` cap for messaging this process back, hence why it is a `Vec<ProcessId>` instead of vector of arbitrary capabilities.

- `public: bool`: This boolean value determines whether the process can receive messages from other processes by default.

The fields within the spawn function closely mirror those found in the pkg/manifest.json file of your project, providing a consistent and intuitive setup for process management.
