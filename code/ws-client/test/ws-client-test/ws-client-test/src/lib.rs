use crate::kinode::process::tester::{
    FailResponse, Request as TesterRequest, Response as TesterResponse, RunRequest,
};

use kinode_process_lib::{
    await_message, call_init, our_capabilities, println, spawn, Address, OnExit, Request, Response,
};

mod tester_lib;

wit_bindgen::generate!({
    path: "target/wit",
    world: "ws-client-test-template-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [PartialEq, serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

call_init!(init);
fn init(our: Address) {
    println!("{}: begin", our.process());

    println!("a");
    // get Run from tester
    let message = await_message().unwrap();
    if !message.is_request() {
        fail!("ws-client-test");
    }
    let source = message.source();
    if our.node != source.node {
        fail!("ws-client-test");
    }
    let TesterRequest::Run(RunRequest { .. }) = message.body().try_into().unwrap();

    println!("b");
    // run client
    let child = spawn(
        None,
        &format!("{}/setup/ws-client.wasm", our.package_id()),
        OnExit::Requests(vec![
            Request::to(&our).body(serde_json::to_vec(&Err::<(), ()>(())).unwrap())
        ]),
        our_capabilities(),
        vec!["http_client:distro:sys".parse().unwrap()],
        false,
    );
    if child.is_err() {
        fail!("ws-client-test");
    }

    // give child our process id
    let child = child.unwrap();
    let address: Address = format!("our@{child}").parse().unwrap();
    Request::to(address)
        .body(vec![])
        .expects_response(5)
        .send()
        .unwrap();

    println!("c");
    // listen for result from client
    let message = await_message().unwrap();
    println!("d");
    if !message.is_request() {
        fail!("ws-client-test");
    }
    println!("e");
    let source = message.source();
    if our.node != source.node {
        fail!("ws-client-test");
    }
    println!("f");
    match serde_json::from_slice(message.body()).unwrap() {
        Err(()) => {
            println!("g");
            fail!("ws-client-test");
        }
        Ok(()) => {
            println!("h");
            Response::new()
                .body(TesterResponse::Run(Ok(())))
                .send()
                .unwrap();
        }
    }
    println!("i");
}
