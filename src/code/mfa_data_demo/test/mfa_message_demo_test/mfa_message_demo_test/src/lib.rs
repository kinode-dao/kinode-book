use crate::kinode::process::tester::{
    FailResponse, Request as TesterRequest, Response as TesterResponse, RunRequest,
};

use kinode_process_lib::{
    await_message, call_init, print_to_terminal, println, Address, ProcessId, Request, Response,
};

mod tester_lib;

wit_bindgen::generate!({
    path: "target/wit",
    world: "tester-sys-v0",
    generate_unused_types: true,
    additional_derives: [PartialEq, serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn handle_message(our: &Address) -> anyhow::Result<()> {
    let message = await_message().unwrap();

    if !message.is_request() {
        unimplemented!();
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
    print_to_terminal(0, "mfa_message_demo_test: a");
    assert!(node_names.len() == 1);

    let our_demo_address = Address {
        node: our.node.clone(),
        process: ProcessId::new(Some("mfa_message_demo"), "mfa_message_demo", "template.os"),
    };

    let response = Request::new()
        .target(&our_demo_address)
        .body(b"hello from test")
        .send_and_await_response(5)?
        .unwrap();
    if response.is_request() {
        fail!("mfa_message_demo_test");
    };
    let body = String::from_utf8_lossy(response.body());

    let expected = "hello world to you too!".to_string();
    if body != expected {
        println!("{body} != {expected} (expected)");
        fail!("mfa_message_demo_test");
    }

    Response::new()
        .body(TesterResponse::Run(Ok(())))
        .send()
        .unwrap();

    Ok(())
}

call_init!(init);
fn init(our: Address) {
    print_to_terminal(0, "begin");

    loop {
        match handle_message(&our) {
            Ok(()) => {}
            Err(e) => {
                print_to_terminal(0, format!("mfa_message_demo_test: error: {e:?}").as_str());

                fail!("mfa_message_demo_test");
            }
        };
    }
}
