# Spawning and Managing Child Processes

In Kinode OS, a "parent" process can create additional processes, known as "children" (also discussed [here](../processes.md#spawning-child-processes)).
These child processes are particularly useful for handling intensive tasks (referred to as "workers") that require long computation times without hindering the performance of the main application.
They are also beneficial for segregating distinct logical components.
Each process is its own subdirectory within the package.
E.g., for Kinode processes written in Rust, each is its own Rust project, complete with a separate Cargo.toml file.

Your package's file structure might resemble the following:

```
my-package/
├─ pkg/
│  ├─ metadata.json
│  ├─ manifest.json
├─ parent/
│  ├─ src/
│  ├─ Cargo.toml
│  ├─ Cargo.lock
├─ child/
│  ├─ src/
│  ├─ Cargo.toml
│  ├─ Cargo.lock
```
To initiate a child process, use the `spawn` function from `kinode_process_lib`.
The following example demonstrates a basic parent process whose sole function is to spawn a child process and grant it the ability to send messages using `http_client`:
```rust
// imports
use kinode_process_lib::{println, spawn, Address, Capability, OnExit};

// boilerplate to generate types
wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

struct Component;

// parent app component boilerplate
impl Guest for Component {
    fn init(our: String) {
        // unpack the address string and print it to the terminal
        let our = Address::from_str(&our).unwrap();
        println!("{our}: start");

        // this function actually spawns the child process
        let spawned_process_id: ProcessId = match spawn(
            // name of the child process
            Some("spawned_child_process".to_string()),
            // path to find the compiled Wasm file for the child process
            "/child.wasm",
            // what to do when this process crashes/panics/finishes
            OnExit::None,
            // capabilities to pass onto the child
            vec![
                // the parents app already has the capability to message http_client here
                // so we are just passing it onto the child
                Capability {
                    issuer: Address::new(&our.node, ProcessId::from_str("http_client:distro:sys").unwrap()),
                    params: "\"messaging\"".into(),
                }
            ]),
            vec![],
            // this process will not be public
            false,
        ) {
            Ok(spawned_process_id) => spawned_process_id,
            Err(e) => {
                panic!("couldn't spawn"); //  TODO
            }
        }
    }
}
```

The child process can be anything, for simplicity's sake let's make it a degenerate process that does nothing but print it's name and die:
```rust
// same boilerplate as above
use kinode_process_lib::{println, Address};

wit_bindgen::generate!({
    // note that the WIT file can be in any directory
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

struct Component;

// child app component boilerplate
impl Guest for Component {
    fn init(our: String) {
        // unpack the address string and print it to the terminal
        let our = Address::from_str(&our).unwrap();
        println!("{our}: start");

        // print something else out
        println!("this is the child process, wow!");
    }
}
```
The spawn function in Kinode comprises several parameters, each serving a specific purpose in the process creation:

- `name: Option<String>`: This parameter specifies the name of the process.
If set to None, the process is automatically assigned a numerical identifier, resulting in a ProcessId formatted like `123456789:my-package:john.os`.

- `wasm_path: String`: Indicates the location of the compiled WebAssembly (Wasm) bytecode for the process.
This path should be relative to the `/pkg` directory in your project.

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
