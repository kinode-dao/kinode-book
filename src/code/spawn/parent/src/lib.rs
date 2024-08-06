// imports
use kinode_process_lib::{call_init, println, spawn, Address, Capability, OnExit, ProcessId};

// boilerplate to generate types
wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v0",
});

// parent app component boilerplate
call_init!(init);
fn init(our: Address) {
    println!("{our}: start");

    // this function actually spawns the child process
    let _spawned_process_id = match spawn(
        // name of the child process (None -> random number)
        Some("spawned_child_process"),
        // path to find the compiled Wasm file for the child process
        &format!("{}/pkg/child.wasm", our.package_id()),
        // what to do when child crashes/panics/finishes
        OnExit::None,
        // capabilities to pass onto the child
        vec![
            // the parent process already has the capability to message
            // http_client here so we are just passing it onto the child
            Capability {
                issuer: Address::new(
                    &our.node,
                    "http_client:distro:sys".parse::<ProcessId>().unwrap(),
                ),
                params: "\"messaging\"".into(),
            },
        ],
        vec![],
        // this process will not be public
        false,
    ) {
        Ok(spawned_process_id) => spawned_process_id,
        Err(e) => {
            panic!("couldn't spawn: {e:?}");
        }
    };
}
