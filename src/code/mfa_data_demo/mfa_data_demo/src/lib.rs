use crate::kinode::process::mfa_data_demo::{Request as MfaRequest, Response as MfaResponse};
use kinode_process_lib::{await_message, call_init, println, Address, Message, Request, Response};

wit_bindgen::generate!({
    path: "target/wit",
    world: "mfa-data-demo-template-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn handle_message(message: &Message) -> anyhow::Result<bool> {
    if message.is_request() {
        match message.body().try_into()? {
            MfaRequest::Hello(text) => {
                println!("got a Hello: {text}");
                Response::new()
                    .body(MfaResponse::Hello("hello to you too!".to_string()))
                    .send()?
            }
            MfaRequest::Goodbye => {
                println!("goodbye!");
                Response::new().body(MfaResponse::Goodbye).send()?;
                return Ok(true);
            }
        }
    } else {
        match message.body().try_into()? {
            MfaResponse::Hello(text) => println!("got a Hello response: {text}"),
            MfaResponse::Goodbye => println!("got a Goodbye response"),
        }
    }
    Ok(false)
}

call_init!(init);
fn init(our: Address) {
    println!("begin");

    Request::to(&our)
        .body(MfaRequest::Hello("hello world".to_string()))
        .expects_response(5)
        .send()
        .unwrap();

    loop {
        match await_message() {
            Err(send_error) => println!("got SendError: {send_error}"),
            Ok(ref message) => match handle_message(message) {
                Err(e) => println!("got error while handling message: {e:?}"),
                Ok(should_exit) => {
                    if should_exit {
                        return;
                    }
                }
            },
        }
    }
}
