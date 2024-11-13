use crate::kinode::process::chat_with_file_transfer::{
    ChatMessage, Request as ChatRequest, Response as ChatResponse, SendRequest,
};
use crate::kinode::process::file_transfer_worker::{DownloadRequest, Request as WorkerRequest};
use crate::kinode::process::standard::{Address as WitAddress, ProcessId as WitProcessId};
use crate::kinode::process::tester::{
    FailResponse, Request as TesterRequest, Response as TesterResponse, RunRequest,
};

use kinode_process_lib::{
    await_message, call_init, our_capabilities, println, save_capabilities, vfs::File, Address,
    ProcessId, Request, Response,
};

mod tester_lib;

wit_bindgen::generate!({
    path: "target/wit",
    world: "chat-with-file-transfer-test-template-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [PartialEq, serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

const FILE_NAME: &str = "my_file.txt";
const FILE_CONTENTS: &str = "hi";
const DRIVE_PATH: &str = "chat-with-file-transfer:template.os";

impl From<Address> for WitAddress {
    fn from(address: Address) -> Self {
        WitAddress {
            node: address.node,
            process: address.process.into(),
        }
    }
}

impl From<ProcessId> for WitProcessId {
    fn from(process: ProcessId) -> Self {
        WitProcessId {
            process_name: process.process_name,
            package_name: process.package_name,
            publisher_node: process.publisher_node,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto)]
enum Setup {
    Caps,
    WriteFile { name: String, contents: Vec<u8> },
}

fn make_chat_address(node: &str) -> Address {
    Address {
        node: node.to_string(),
        process: ProcessId::new(
            Some("chat-with-file-transfer"),
            "chat-with-file-transfer",
            "template.os",
        ),
    }
}

fn setup(our: &Address, their: &str) -> anyhow::Result<()> {
    let our_chat_address = make_chat_address(&our.node);
    let their_chat_address = make_chat_address(their);

    // write file on their
    Request::new()
        .target(their_chat_address.clone())
        .body(Setup::WriteFile {
            name: FILE_NAME.to_string(),
            contents: FILE_CONTENTS.as_bytes().to_vec(),
        })
        .send()?;

    // caps on our
    println!("chat-with-file-transfer-test: started caps handshake...");

    let response = Request::new()
        .target(our_chat_address.clone())
        .body(Setup::Caps)
        .send_and_await_response(5)??;

    save_capabilities(response.capabilities());
    println!(
        "chat-with-file-transfer-test: got caps {:#?}",
        our_capabilities()
    );

    Ok(())
}

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
    println!("chat-with-file-transfer-test: a");
    assert!(node_names.len() >= 2);
    // we are master node
    assert!(our.node == node_names[0]);

    if setup(&our, &node_names[1]).is_err() {
        fail!("chat-with-file-transfer-test");
    }

    let our_chat_address = make_chat_address(&our.node);
    let their_chat_address = make_chat_address(&node_names[1]);

    // Send
    println!("chat-with-file-transfer-test: b");
    let message: String = "hello".into();
    let _ = Request::new()
        .target(our_chat_address.clone())
        .body(ChatRequest::Send(SendRequest {
            target: node_names[1].clone(),
            message: message.clone(),
        }))
        .send_and_await_response(15)?
        .unwrap();

    // Get history from receiver & test
    println!("chat-with-file-transfer-test: c");
    let response = Request::new()
        .target(their_chat_address.clone())
        .body(ChatRequest::History(our.node.clone()))
        .send_and_await_response(15)?
        .unwrap();
    if response.is_request() {
        fail!("chat-with-file-transfer-test");
    };
    let ChatResponse::History(messages) = response.body().try_into()? else {
        fail!("chat-with-file-transfer-test");
    };
    let expected_messages = vec![ChatMessage {
        author: our.node.clone(),
        content: message,
    }];

    if messages != expected_messages {
        println!("{messages:?} != {expected_messages:?}");
        fail!("chat-with-file-transfer-test");
    }

    // Test file_transfer_worker
    println!("chat-with-file-transfer-test: d");
    let response = Request::new()
        .target(our_chat_address.clone())
        .body(WorkerRequest::Download(DownloadRequest {
            name: FILE_NAME.to_string(),
            target: their_chat_address.into(),
            is_requestor: true,
        }))
        .send_and_await_response(15)?
        .unwrap();
    if response.is_request() {
        fail!("chat-with-file-transfer-test");
    };
    std::thread::sleep(std::time::Duration::from_secs(3));

    let file = File {
        path: format!("{DRIVE_PATH}/files/{FILE_NAME}"),
        timeout: 5,
    };
    let file_contents = file.read()?;
    if file_contents != FILE_CONTENTS.as_bytes() {
        fail!("chat-with-file-transfer-test");
    }

    Response::new()
        .body(TesterResponse::Run(Ok(())))
        .send()
        .unwrap();

    Ok(())
}

call_init!(init);
fn init(our: Address) {
    println!("begin");

    loop {
        match handle_message(&our) {
            Ok(()) => {}
            Err(e) => {
                println!("chat-with-file-transfer-test: error: {e:?}");

                fail!("chat-with-file-transfer-test");
            }
        };
    }
}
