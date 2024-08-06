// same boilerplate as above
use kinode_process_lib::{call_init, println, Address};

wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v0",
});

call_init!(init);
fn init(our: Address) {
    println!("{our}: start");

    // print something else out
    println!("this is the child process, wow!");
}
