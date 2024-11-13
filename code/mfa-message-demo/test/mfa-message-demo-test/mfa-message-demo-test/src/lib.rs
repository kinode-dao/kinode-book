use crate::kinode::process::tester::{
    FailResponse, Request as TesterRequest, Response as TesterResponse, RunRequest,
};
use kinode_process_lib::{await_message, call_init, println, Address, Request, Response};

mod tester_lib;

wit_bindgen::generate!({
    path: "target/wit",
    world: "mfa-message-demo-test-template-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [PartialEq, serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn handle_message(our: &Address) -> anyhow::Result<()> {
    let message = await_message()?;

    if !message.is_request() {
        fail!("mfa-message-demo-test");
    }
    let source = message.source();
    if our.node != source.node {
        return Err(anyhow::anyhow!(
            "rejecting foreign Message from {:?}",
            source,
        ));
    }
    let TesterRequest::Run(RunRequest {
        input_node_names: node_names,
        ..
    }) = message.body().try_into()?;
    if node_names.len() != 1 {
        fail!("mfa-message-demo-test");
    }

    let our_demo_address =
        format!("{}@mfa-message-demo:mfa-message-demo:template.os", our.node).parse()?;

    let response = Request::new()
        .target(&our_demo_address)
        .body(b"hello from test")
        .send_and_await_response(5)??;
    if response.is_request() {
        fail!("mfa-message-demo-test");
    };
    let body = String::from_utf8_lossy(response.body());

    let expected = "hello world to you too!".to_string();
    if body != expected {
        println!("{body} != {expected} (expected)");
        fail!("mfa-message-demo-test");
    }

    Response::new().body(TesterResponse::Run(Ok(()))).send()?;

    Ok(())
}

call_init!(init);
fn init(our: Address) {
    println!("begin");

    loop {
        match handle_message(&our) {
            Ok(()) => {}
            Err(e) => {
                println!("mfa-message-demo-test: error: {e:?}");
                fail!("mfa-message-demo-test");
            }
        };
    }
}
