use std::collections::HashMap;

use crate::kinode::process::chat_with_file_transfer::{
    ChatMessage, Request as ChatRequest, Response as ChatResponse, SendRequest,
};
use crate::kinode::process::file_transfer_worker::{
    start_download, DownloadRequest, ProgressRequest, Request as WorkerRequest,
    Response as WorkerResponse,
};
use crate::kinode::process::standard::{Address as WitAddress, ProcessId as WitProcessId};
use kinode_process_lib::{
    await_message, call_init, get_capability, println,
    vfs::{create_drive, open_file},
    Address, Message, ProcessId, Request, Response,
};

wit_bindgen::generate!({
    path: "target/wit",
    world: "chat-with-file-transfer-template-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

#[derive(Debug, serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto)]
#[serde(untagged)] // untagged as a meta-type for all incoming messages
enum Msg {
    // requests
    ChatRequest(ChatRequest),
    WorkerRequest(WorkerRequest),

    // responses
    WorkerResponse(WorkerResponse),
}

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

type MessageArchive = HashMap<String, Vec<ChatMessage>>;

fn handle_chat_request(
    our: &Address,
    source: &Address,
    request: &ChatRequest,
    message_archive: &mut MessageArchive,
) -> anyhow::Result<()> {
    match request {
        ChatRequest::Send(SendRequest {
            ref target,
            ref message,
        }) => {
            if target == &our.node {
                println!("{}: {}", source.node, message);
                let message = ChatMessage {
                    author: source.node.clone(),
                    content: message.into(),
                };
                message_archive
                    .entry(source.node.clone())
                    .and_modify(|e| e.push(message.clone()))
                    .or_insert(vec![message]);
            } else {
                let _ = Request::new()
                    .target(Address {
                        node: target.clone(),
                        process: "chat_with_file_transfer:chat_with_file_transfer:template.os"
                            .parse()?,
                    })
                    .body(request)
                    .send_and_await_response(5)?
                    .unwrap();
                let message = ChatMessage {
                    author: our.node.clone(),
                    content: message.into(),
                };
                message_archive
                    .entry(target.clone())
                    .and_modify(|e| e.push(message.clone()))
                    .or_insert(vec![message]);
            }
            Response::new().body(ChatResponse::Send).send().unwrap();
        }
        ChatRequest::History(ref node) => {
            Response::new()
                .body(ChatResponse::History(
                    message_archive
                        .get(node)
                        .map(|msgs| msgs.clone())
                        .unwrap_or_default(),
                ))
                .send()
                .unwrap();
        }
    }
    Ok(())
}

fn handle_worker_request(
    our: &Address,
    source: &Address,
    request: &WorkerRequest,
) -> anyhow::Result<()> {
    match request {
        WorkerRequest::Download(DownloadRequest {
            ref name,
            ref target,
            is_requestor,
        }) => {
            match start_download(
                &our.clone().into(),
                &source.clone().into(),
                name,
                target,
                *is_requestor,
            ) {
                Ok(_) => {}
                Err(e) => return Err(anyhow::anyhow!("{e}")),
            }
        }
        WorkerRequest::Progress(ProgressRequest { name, progress }) => {
            println!("{} progress: {}%", name, progress);
            Response::new().body(WorkerResponse::Progress).send()?;
        }
    }
    Ok(())
}

fn handle_worker_response(response: &WorkerResponse) -> anyhow::Result<()> {
    match response {
        WorkerResponse::Download(ref result) => {
            if let Err(e) = result {
                return Err(anyhow::anyhow!("{e}"));
            }
        }
        WorkerResponse::Progress => {}
    }
    Ok(())
}

fn handle_message(
    our: &Address,
    message: &Message,
    message_archive: &mut MessageArchive,
) -> anyhow::Result<()> {
    match message.body().try_into()? {
        // requests
        Msg::ChatRequest(ref cr) => handle_chat_request(our, message.source(), cr, message_archive),
        Msg::WorkerRequest(ref wr) => handle_worker_request(our, message.source(), wr),

        // responses
        Msg::WorkerResponse(ref wr) => handle_worker_response(wr),
    }
}

#[cfg(feature = "test")]
#[derive(Debug, serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto)]
enum Setup {
    Caps,
    WriteFile { name: String, contents: Vec<u8> },
}

#[cfg(feature = "test")]
fn handle_tester_setup(our: &Address, drive_path: &str) -> anyhow::Result<()> {
    println!("awaiting setup...");

    let Ok(message) = await_message() else {
        return Err(anyhow::anyhow!("a"));
    };
    // TODO: confirm its from tester
    match message.body().try_into()? {
        Setup::Caps => {
            println!("got caps...");
            let vfs_read_cap = serde_json::json!({
                "kind": "read",
                "drive": drive_path,
            })
            .to_string();
            let vfs_address = Address {
                node: our.node.clone(),
                process: "vfs:distro:sys".parse()?,
            };

            let read_cap = get_capability(&vfs_address, &vfs_read_cap).unwrap();

            Response::new()
                .body(vec![])
                .capabilities(vec![read_cap])
                .send()
                .unwrap();
            println!("sent caps");
        }
        Setup::WriteFile {
            ref name,
            ref contents,
        } => {
            println!("got write file...");
            let file = open_file(&format!("{drive_path}/{name}"), true, None)?;
            file.write(contents)?;
        }
    }
    println!("setup done");
    Ok(())
}

call_init!(init);
fn init(our: Address) {
    println!("begin");

    let drive_path = create_drive(our.package_id(), "files", None).unwrap();
    let mut message_archive = HashMap::new();

    #[cfg(feature = "test")]
    handle_tester_setup(&our, &drive_path).unwrap();

    loop {
        match await_message() {
            Err(send_error) => println!("got SendError: {send_error}"),
            Ok(ref message) => match handle_message(&our, message, &mut message_archive) {
                Ok(_) => {}
                Err(e) => println!("got error while handling message: {e:?}"),
            },
        }
    }
}
