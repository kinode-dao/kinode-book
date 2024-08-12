// same boilerplate as above
#[cfg(feature = "test")]
use kinode_process_lib::{await_message, Response};
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
    #[cfg(feature = "test")]
    {
        println!("child awaiting message from test...");
        let _message = await_message().unwrap();
        Response::new()
            .body(serde_json::to_vec(&Ok::<(), ()>(())).unwrap())
            .send()
            .unwrap();
    }
}
