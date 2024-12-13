use kinode_process_lib::{await_message, call_init, println, Address, Request, Response};

wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v1",
});

call_init!(init);
fn init(our: Address) {
    println!("begin");

    Request::to(&our)
        .body(b"hello world")
        .expects_response(5)
        .send()
        .unwrap();

    loop {
        match await_message() {
            Err(send_error) => println!("got SendError: {send_error}"),
            Ok(message) => {
                let body = String::from_utf8_lossy(message.body());
                if message.is_request() {
                    println!("got a request: {body}");
                    Response::new()
                        .body(b"hello world to you too!")
                        .send()
                        .unwrap();
                } else {
                    println!("got a response: {body}");
                }
            }
        }
    }
}
