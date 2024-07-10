use crate::exports::kinode::process::server::{ClientRequest, ClientResponse, Guest};
use kinode_process_lib::{vfs, Request, Response};

wit_bindgen::generate!({
    path: "target/wit",
    world: "server-template-dot-os-api-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

const READ_TIMEOUT_SECS: u64 = 5;
const PUT_TIMEOUT_SECS: u64 = 5;

fn make_put_file_error(message: &str) -> anyhow::Result<Result<(), String>> {
    Response::new()
        .body(ClientResponse::PutFile(Err(message.to_string())))
        .send()?;
    return Err(anyhow::anyhow!(message.to_string()));
}

fn make_get_file_error(message: &str) -> anyhow::Result<Result<(), String>> {
    Response::new()
        .body(ClientResponse::GetFile(Err(message.to_string())))
        .send()?;
    return Err(anyhow::anyhow!(message.to_string()));
}

fn make_list_files_error(message: &str) -> anyhow::Result<Result<Vec<String>, String>> {
    Response::new()
        .body(ClientResponse::GetFile(Err(message.to_string())))
        .send()?;
    return Err(anyhow::anyhow!(message.to_string()));
}

fn put_file(host: String, path: String, name: String) -> anyhow::Result<Result<(), String>> {
    // rather than using `vfs::open_file()?.read()?`, which reads
    // the file into process memory, send the Request to VFS ourselves,
    // `inherit`ing the file contents into the ClientRequest
    //
    // let contents = vfs::open_file(path, false, None)?.read()?;
    //
    let response = Request::new()
        .target(("our", "vfs", "distro", "sys"))
        .body(serde_json::to_vec(&vfs::VfsRequest {
            path: path.to_string(),
            action: vfs::VfsAction::Read,
        })?)
        .send_and_await_response(READ_TIMEOUT_SECS)??;
    let response = response.body();
    let Ok(vfs::VfsResponse::Read) = serde_json::from_slice(&response) else {
        return make_put_file_error(&format!("Could not find file at {path}."));
    };
    let ClientResponse::PutFile(result) = Request::new()
        .target((&host, "server", "server", "template.os"))
        .inherit(true)
        .body(ClientRequest::PutFile(name))
        .send_and_await_response(PUT_TIMEOUT_SECS)??
        .body()
        .try_into()?
    else {
        return make_put_file_error(&format!("Got unexpected Response from server."));
    };
    Ok(result)
}

fn get_file(host: String, name: String) -> anyhow::Result<Result<(), String>> {
    let ClientResponse::GetFile(result) = Request::new()
        .target((&host, "server", "server", "template.os"))
        .body(ClientRequest::GetFile(name))
        .send_and_await_response(PUT_TIMEOUT_SECS)??
        .body()
        .try_into()?
    else {
        return make_get_file_error(&format!("Got unexpected Response from server."));
    };
    Ok(result)
}

fn list_files(host: String) -> anyhow::Result<Result<Vec<String>, String>> {
    let ClientResponse::ListFiles(result) = Request::new()
        .target((&host, "server", "server", "template.os"))
        .inherit(true)
        .body(ClientRequest::ListFiles)
        .send_and_await_response(PUT_TIMEOUT_SECS)??
        .body()
        .try_into()?
    else {
        return make_list_files_error(&format!("Got unexpected Response from server."));
    };
    Ok(result)
}

struct Api;
impl Guest for Api {
    fn put_file(host: String, path: String, name: String) -> Result<(), String> {
        match put_file(host, path, name) {
            Ok(result) => result,
            Err(e) => Err(format!("{e:?}")),
        }
    }

    fn get_file(host: String, name: String) -> Result<(), String> {
        match get_file(host, name) {
            Ok(result) => result,
            Err(e) => Err(format!("{e:?}")),
        }
    }

    fn list_files(host: String) -> Result<Vec<String>, String> {
        match list_files(host) {
            Ok(result) => result,
            Err(ref e) => Err(format!("{e:?}")),
        }
    }
}
export!(Api);
