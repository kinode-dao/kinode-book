use std::collections::{HashMap, HashSet};

use crate::kinode::process::server::{ClientRequest, ClientResponse};
use kinode_process_lib::{
    await_message, call_init, get_blob, println, vfs, Address, Message, PackageId, Request,
    Response,
};

wit_bindgen::generate!({
    path: "target/wit",
    world: "server-template-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

type State = HashMap<String, HashSet<String>>;
const READ_TIMEOUT_SECS: u64 = 5;

fn make_drive_name(our: &PackageId, source: &str) -> String {
    format!("/{our}/{source}")
}

fn make_put_file_error(message: &str) -> anyhow::Result<()> {
    Response::new()
        .body(ClientResponse::PutFile(Err(message.to_string())))
        .send()?;
    return Err(anyhow::anyhow!(message.to_string()));
}

fn make_get_file_error(message: &str) -> anyhow::Result<()> {
    Response::new()
        .body(ClientResponse::GetFile(Err(message.to_string())))
        .send()?;
    return Err(anyhow::anyhow!(message.to_string()));
}

fn make_list_files_error(message: &str) -> anyhow::Result<()> {
    Response::new()
        .body(ClientResponse::ListFiles(Err(message.to_string())))
        .send()?;
    return Err(anyhow::anyhow!(message.to_string()));
}

fn handle_put_file(
    name: &str,
    our: &PackageId,
    source: &str,
    state: &mut State,
) -> anyhow::Result<()> {
    let Some(ref blob) = get_blob() else {
        return make_put_file_error("Must give a file in the blob.");
    };

    let drive = vfs::create_drive(our.clone(), source, None)?;
    vfs::create_file(&format!("{drive}/{name}"), None)?.write(blob.bytes())?;
    state
        .entry(source.to_string())
        .or_insert_with(HashSet::new)
        .insert(name.to_string());
    Response::new()
        .body(ClientResponse::PutFile(Ok(())))
        .send()?;
    Ok(())
}

fn handle_get_file(name: &str, our: &PackageId, source: &str, state: &State) -> anyhow::Result<()> {
    let Some(ref names) = state.get(source) else {
        return make_get_file_error(&format!("{source} has no files to Get."));
    };
    if !names.contains(name) {
        return make_get_file_error(&format!("{source} has no such file {name}."));
    }

    // rather than using `vfs::open_file()?.read()?`, which reads
    // the file into process memory, send the Request to VFS ourselves,
    // `inherit`ing the file contents into the ClientResponse
    //
    // let contents = vfs::open_file(path, false, None)?.read()?;
    //
    let path = format!("{}/{name}", make_drive_name(our, source));
    let response = Request::new()
        .target(("our", "vfs", "distro", "sys"))
        .body(serde_json::to_vec(&vfs::VfsRequest {
            path,
            action: vfs::VfsAction::Read,
        })?)
        .send_and_await_response(READ_TIMEOUT_SECS)??;
    let response = response.body();
    let Ok(vfs::VfsResponse::Read) = serde_json::from_slice(&response) else {
        return make_get_file_error(&format!("Could not find file at {name}."));
    };
    Response::new()
        .inherit(true)
        .body(ClientResponse::GetFile(Ok(())))
        .send()?;
    Ok(())
}

fn handle_list_files(source: &str, state: &State) -> anyhow::Result<()> {
    let Some(ref names) = state.get(source) else {
        return make_list_files_error(&format!("{source} has no files to List."));
    };
    let mut names: Vec<String> = names.iter().cloned().collect();
    names.sort();
    Response::new()
        .body(ClientResponse::ListFiles(Ok(names)))
        .send()?;
    Ok(())
}

fn handle_message(our: &Address, message: &Message, state: &mut State) -> anyhow::Result<()> {
    let source = message.source();
    if !message.is_request() {
        return Err(anyhow::anyhow!("unexpected Response from {source}"));
    }
    match message.body().try_into()? {
        ClientRequest::PutFile(ref name) => {
            handle_put_file(name, &our.package_id(), source.node(), state)?
        }
        ClientRequest::GetFile(ref name) => {
            handle_get_file(name, &our.package_id(), source.node(), state)?
        }
        ClientRequest::ListFiles => handle_list_files(source.node(), state)?,
    }
    Ok(())
}

call_init!(init);
fn init(our: Address) {
    println!("begin");

    let mut state: State = HashMap::new();

    loop {
        match await_message() {
            Err(send_error) => println!("got SendError: {send_error}"),
            Ok(ref message) => match handle_message(&our, message, &mut state) {
                Err(e) => println!("got error while handling message: {e:?}"),
                Ok(_) => {}
            },
        }
    }
}
