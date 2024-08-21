/// Simple example of saving and loading state.
/// Usage:
/// ```
/// # Start node.
/// kit f
///
/// # Start package from a new terminal.
/// kit bs save_state
///
/// # Watch as process continually restarts, incrementing the counter in state.
/// ```
use kinode_process_lib::{call_init, get_state, println, set_state, Address};

wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v0",
});

call_init!(init);
fn init(_our: Address) {
    println!("init");
    match get_state() {
        None => {
            println!("no state found");
            let counter: u64 = 0;
            set_state(&counter.to_le_bytes());
        }
        Some(state) => {
            let mut counter = u64::from_le_bytes(state.try_into().unwrap());
            println!("counter: {}", counter);
            counter += 1;
            set_state(&counter.to_le_bytes());
        }
    }
    std::thread::sleep(std::time::Duration::from_secs(2));
}
