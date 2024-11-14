use crate::kinode::process::mfa_data_demo::{Request as MfaRequest, Response as MfaResponse};
use crate::kinode::process::tester::{
    FailResponse, Request as TesterRequest, Response as TesterResponse, RunRequest,
};
use kinode_process_lib::{await_message, call_init, println, Address, Request, Response};

mod tester_lib;

wit_bindgen::generate!({
    path: "target/wit",
    world: "mfa-data-demo-test-template-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn handle_message(our: &Address) -> anyhow::Result<()> {
    let message = await_message()?;

    if !message.is_request() {
        fail!("mfa-fe-demo-test");
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
        fail!("mfa-fe-demo-test");
    }

    let our_demo_address = format!("{}@mfa-fe-demo:mfa-fe-demo:template.os", our.node).parse()?;

    let response = Request::new()
        .target(&our_demo_address)
        .body(MfaRequest::Hello("hello from test".to_string()))
        .send_and_await_response(5)??;
    if response.is_request() {
        fail!("mfa-fe-demo-test");
    };
    let MfaResponse::Hello(ref text) = response.body().try_into()? else {
        fail!("mfa-fe-demo-test");
    };
    if text != "hello to you too!" {
        fail!("mfa-fe-demo-test");
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
                println!("mfa-fe-demo-test: error: {e:?}");
                fail!("mfa-fe-demo-test");
            }
        };
    }
}
