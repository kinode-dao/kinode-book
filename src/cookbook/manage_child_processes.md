# Spawning and Managing Child Processes
In Uqbar, a "parent" process can create additional processes, known as "children". These child processes are particularly useful for handling intensive tasks (referred to as "workers") that require long computation times without hindering the performance of the main application. They are also beneficial for segregating distinct logical components. Each child process operates within its own Rust project, complete with a separate Cargo.toml file, ensuring modular and organized code management.

Your project's file structure might resemble the following:

```
my-package/
├─ pkg/
│  ├─ metadata.json
│  ├─ manifest.json
│  ├─ parent.wasm
│  ├─ child.wasm
├─ parent/
│  ├─ src/
│  ├─ target/
│  ├─ Cargo.toml
│  ├─ Cargo.lock
├─ child/
│  ├─ src/
│  ├─ target/
│  ├─ Cargo.toml
│  ├─ Cargo.lock
```
To initiate a child process, use the `spawn` function from `uqbar_process_lib`. The following example demonstrates a basic parent process whose sole function is to spawn a child process and grant it the ability to send messages using `http_client`:
```rust
// imports
use uqbar_process_lib::{println, spawn, get_capability, Address, Capabilities, OnExit};

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

        // the parents app already has the capability to message http_client here we
        // are fetching that capability so that we can pass it to the child in `spawn`
        let Some(http_client_cap) = get_capability(
            &Address::new(&our.node, ProcessId::from_str("http_client:sys:uqbar").unwrap()),
            &"\"messaging\"".into(),
        ) else { todo!()};

        // this function actually spawns the child process
        let spawned_process_id: ProcessId = match spawn(
            // name of the child process
            Some("spawned_child_process".to_string()),
            // path to find the compiled wasm file for the child process
            "/child.wasm",
            // what to do when this process crashes/panics/finishes
            OnExit::None,
            // capabilities to pass onto the child
            &Capabilities::Some(vec![http_client_cap]),
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
use uqbar_process_lib::{println, Address};

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
The spawn function in Uqbar comprises several parameters, each serving a specific purpose in the process creation:

- `name: Option<String>`: This parameter specifies the name of the process. If set to None, the process is automatically assigned a numerical identifier, resulting in a ProcessId formatted like `123456789:my-package:john.uq`.

- `wasm_path: String`: Indicates the location of the compiled WebAssembly (wasm) bytecode for the process. This path should be relative to the `/pkg` directory in your project.

- `on_exit: OnExit`: Determines the behavior of the process upon termination, whether due to completion, a crash, or a panic. OnExit is an enum with three potential values:

  - `None`: The process will take no action upon exiting.
  - `Restart`: The process will automatically restart after termination.
  - `Requests: Vec<(Address, Request, Option<Payload>)>`: Upon process termination, a series of predefined requests will be dispatched. This feature is particularly useful for notifying other processes about the termination of this child process.
- `capabilities: Vec<SignedCapability>`: This argument is for passing immediate capabilities to the child process. As illustrated in the provided example, the parent's http_client messaging capability was shared with the child.
- `public: bool`: This boolean value determines whether the process can receive messages from other processes by default.

The fields within the spawn function closely mirror those found in the pkg/manifest.json file of your project, providing a consistent and intuitive setup for process management.