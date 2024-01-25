# File Transfer

This entry will teach you to build a simple file transfer app, allowing nodes to download files from a public directory.
It will use the [VFS](../apis/vfs.md) to read and write files, and will spin up worker processes for the transfer.

This guide assumes a basic understanding of Kinode process building, some familiarity with [`kit`](../kit/kit.md), requests and responses, and some knowledge of rust syntax.

## Contents

- [Start](#start)
- [Transfer](#transfer)
- [Final Code](#final-code)
- [Conclusion](#conclusion)
- [VFS API](../apis/vfs.md)
- [Github Repo](https://github.com/bitful-pannul/file_transfer)

## Start

First, initialize a new project with

```
kit new file_transfer
cd file_transfer
```

Here's a clean template so you have a complete fresh start:

This guide will use the following `kinode_process_lib` version in `file_transfer/Cargo.toml`:

```
kinode_process_lib = { git = "ssh://git@github.com/kinode-dao/process_lib.git", tag = "v0.5.4-alpha" }
```

Replace the `file_transfer/src/lib.rs` with:

```rust
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use kinode_process_lib::{await_message, println, Address, Message, Response};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

fn handle_message(our: &Address) -> anyhow::Result<()> {
    let message = await_message()?;
    println!("file_transfer: got message!: {:?}", message);
    Ok(())
}

struct Component;
impl Guest for Component {
    fn init(our: String) {
        println!("file_transfer: begin");

        let our = Address::from_str(&our).unwrap();

        loop {
            match handle_message(&our) {
                Ok(()) => {}
                Err(e) => {
                    println!("file_transfer: error: {:?}", e);
                }
            };
        }
    }
}
```

Before delving into the code, you can handle the capabilities you need to request at spawn.
These will be messaging capabilities to `"net:distro:sys"` (as you'll want to talk to other nodes), and one to `"vfs:distro:sys"` as you'll want to talk to the filesystem.

`pkg/manifest.json`

```json
[
    {
        "process_name": "file_transfer",
        "process_wasm_path": "/file_transfer.wasm",
        "on_exit": "Restart",
        "request_networking": true,
        "request_capabilities": [
            "net:distro:sys",
            "vfs:distro:sys"
        ],
        "grant_capabilities": [],
        "public": true
    }
]
```

Now, look at `file_transfer/src/lib.rs`.
First, add an import of some VFS functions from the `process_lib`:

```rust
use kinode_process_lib::vfs::{create_drive, metadata, open_dir, Directory, FileType},
```

and, to `init()`, create a [drive](../apis/vfs.md#drives) in your VFS and open it.
This is where files will be downloaded by other nodes.
You can add a whitelist a bit later!

```rust
let drive_path = create_drive(our.package_id(), "files").unwrap();
```

At first, this will be an app without UI.
To upload files into your public directory, simply copy them into the "files" directory located in `your_node/vfs/file_transfer:template.os/files`

You now need to let other nodes know what files they can download from you, so add some message types.

```rust
#[derive(Serialize, Deserialize, Debug)]
pub enum TransferRequest {
    ListFiles,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransferResponse {
    ListFiles(Vec<FileInfo>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
}
```

You can handle these messages cleanly by modifying the `handle_message()` function slightly.
It will match on whether a message is a request or a response, the errors get thrown to the main loop automatically with the `?` after the `await_message()` function.
The skeleton of `file_transfer/src/lib.rs` ends up looking like:

```rust
use kinode_process_lib::{
    await_message, println,
    vfs::{create_drive, metadata, open_dir, Directory, FileType},
    Address, Message, Response,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

#[derive(Serialize, Deserialize, Debug)]
pub enum TransferRequest {
    ListFiles,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransferResponse {
    ListFiles(Vec<FileInfo>),
}

fn handle_message(our: &Address, file_dir: &Directory) -> anyhow::Result<()> {
    let message = await_message()?;

    match message {
        Message::Response { ref source, ref body, .. } =>
            handle_transfer_response(source, body)?,
        Message::Request { ref source, ref body, .. } =>
            handle_transfer_request(&our, source, body, file_dir)?,
    };

    Ok(())
}

struct Component;
impl Guest for Component {
    fn init(our: String) {
        println!("file_transfer: begin");

        let our = Address::from_str(&our).unwrap();

        let drive_path = create_drive(our.package_id(), "files").unwrap();
        let file_dir = open_dir(&drive_path, false).unwrap();

        loop {
            match handle_message(&our, &file_dir) {
                Ok(()) => {}
                Err(e) => {
                    println!("file_transfer: error: {:?}", e);
                }
            };
        }
    }
}
```

You can then add the `handle_transfer_request()` and `handle_transfer_response()` functions.

```rust
fn handle_transfer_request(
    _our: &Address,
    _source: &Address,
    body: &Vec<u8>,
    files_dir: &Directory,
) -> anyhow::Result<()> {
    let transfer_request = serde_json::from_slice::<TransferRequest>(body)?;

    match transfer_request {
        TransferRequest::ListFiles => {
            let entries = files_dir.read()?;
            let files: Vec<FileInfo> = entries
                .iter()
                .filter_map(|file| match file.file_type {
                    FileType::File => match metadata(&file.path) {
                        Ok(metadata) => Some(FileInfo {
                            name: file.path.clone(),
                            size: metadata.len,
                        }),
                        Err(_) => None,
                    },
                    _ => None,
                })
                .collect();

            Response::new()
                .body(serde_json::to_vec(&TransferResponse::ListFiles(files))?)
                .send()?;
        }
    }

    Ok(())
}

fn handle_transfer_response(
    source: &Address,
    body: &Vec<u8>,
) -> anyhow::Result<()> {
    let transfer_response = serde_json::from_slice::<TransferResponse>(body)?;

    match transfer_response {
        TransferResponse::ListFiles(files) => {
            println!("got files from node: {:?} ,files: {:?}", source, files);
        }
        _ => {}
    }

    Ok(())
}
```

Now try this out by [booting two nodes](../kit/boot-fake-node.md#example-usage), i.e.,

```
kit f

# In another terminal
kit f --home /tmp/kinode-fake-node-2 -p 8081 -f fake2.os
```

and [starting the package](../kit/start-package.md) on both nodes,

```
kit b
kit s
kit s -p 8081
```

and then placing files in the `/vfs/file_transfer:file_transfer/files/` directory of the second (the `--home` dir path is specified as an argument to `boot-fake-node`), and sending a request from the first:

```
/m fake.os@file_transfer:file_transfer:template.os "ListFiles"
```

You should see a printed response.

```md
Thu 1/11 13:14 response from fake2.os@file_transfer:file_transfer:template.os: {"ListFiles":[{"name":"file_transfer:template.os/files/barry-lyndon.mp4","size":8760244}, {"name":"file_transfer:template.os/files/blue-danube.mp3","size":9668359}]}
```

### Transfer

Now the fun part: downloading/sending files!

In the following, you'll create a child process to handle the downloading/sending and sends progress updates to the parent `file_transfer` process.
Why the complicated architecture?

The `file_transfer` application must be able to handle multiple up/downloads simultaneously.
There are two ways to accomplish this.
The first is to add `context` to Requests sent so that different up/downloads can be disambiguated as they come in.
The second is to spawn a child "worker" to handle each up/download.
Using a child process also allows Requests to await the corresponding Response.
For further reading, see discussion on [`contexts`](../processes.md#please-respond), [awaiting](../processes.md#awaiting-a-response), [spawning children](../processes.md#spawning-child-processes), and more on the [parent-child pattern](../cookbook/manage_child_processes.md).

#### The main process: `file_transfer`

Start by defining some types.
You'll need a request that tells our main process to spin up a worker, requesting the node you're downloading from to do the same.
Also, a progress report would be nice!

```rust
#[derive(Serialize, Deserialize, Debug)]
pub enum TransferRequest {
    ListFiles,
    Download { name: String, target: Address },
    Progress { name: String, progress: u64 },
}
```

Now, a request to downoad a file will result in a respose to the requesting process to download the file using a worker.

Add a simple `Start` and `Done` variant, so you'll know when the worker has successfully been spawned and initialized.

```rust
#[derive(Serialize, Deserialize, Debug)]
pub enum TransferResponse {
    ListFiles(Vec<FileInfo>),
    Download { name: String, worker: Address },
    Start,
    Done,
}
```

Now, add the intra worker communication types:

```rust
#[derive(Serialize, Deserialize, Debug)]
pub enum WorkerRequest {
    Initialize {
        name: String,
        target_worker: Option<Address>,
    },
    Chunk {
        name: String,
        offset: u64,
        length: u64,
    },
    Size(u64),
}
```

Some notes:

- Workers will receive an `Inititialize` request from their own node, which tells the worker it is either a receiver or a sender based on if it has a target worker `Option<Address>`.
- Progress reports are sent back to the main process; if adding a frontend, these could be sent to it via WebSocket updates.

The only additional part you need to handle in the transfer app is the Download request you've added.
`TransferRequest::Download` will handle 2 cases:

1. An incoming download request; spawn a worker, which sends chunks to the remote `target_worker` given in the request,
2. An outgoing download request: spawn a worker, which sends its address to the remote node hosting the file.

To enable spawning and other features, change `file_transfer/src/lib.rs`s imports to:

```rust
use kinode_process_lib::{
    await_message, our_capabilities, println, spawn,
    vfs::{create_drive, metadata, open_dir, Directory, FileType},
    Address, Message, OnExit, Request, Response,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
```

and change `handle_transfer_request()` to:

```rust
fn handle_transfer_request(
    our: &Address,
    source: &Address,
    body: &Vec<u8>,
    files_dir: &Directory,
) -> anyhow::Result<()> {
    let transfer_request = serde_json::from_slice::<TransferRequest>(body)?;

    match transfer_request {
        TransferRequest::ListFiles => {
            let entries = files_dir.read()?;
            let files: Vec<FileInfo> = entries
                .iter()
                .filter_map(|file| match file.file_type {
                    FileType::File => match metadata(&file.path) {
                        Ok(metadata) => Some(FileInfo {
                            name: file.path.clone(),
                            size: metadata.len,
                        }),
                        Err(_) => None,
                    },
                    _ => None,
                })
                .collect();

            Response::new()
                .body(serde_json::to_vec(&TransferResponse::ListFiles(files))?)
                .send()?;
        }
        TransferRequest::Progress { name, progress } => {
            // for now, progress reports are just printed
            println!("file: {} progress: {}%", name, progress);
        }
        TransferRequest::Download { name, target } => {
            // spin up a worker, initialize based on whether it's a downloader or a sender.
            let our_worker = spawn(
                None,
                &format!("{}/pkg/worker.wasm", our.package_id()),
                OnExit::None,
                our_capabilities(),
                vec![],
                false,
            )?;

            let our_worker_address = Address {
                node: our.node.clone(),
                process: our_worker,
            };

            match source.node == our.node {
                true => {
                    // we want to download a file
                    let _resp = Request::new()
                        .body(serde_json::to_vec(&WorkerRequest::Initialize {
                            name: name.clone(),
                            target_worker: None,
                        })?)
                        .target(&our_worker_address)
                        .send_and_await_response(5)??;

                    // send our initialized worker address to the other node
                    Request::new()
                        .body(serde_json::to_vec(&TransferRequest::Download {
                            name: name.clone(),
                            target: our_worker_address,
                        })?)
                        .target(&target)
                        .send()?;
                }
                false => {
                    // they want to download a file
                    Request::new()
                        .body(serde_json::to_vec(&WorkerRequest::Initialize {
                            name: name.clone(),
                            target_worker: Some(target),
                        })?)
                        .target(&our_worker_address)
                        .send()?;
                }
            }
        }
    }

    Ok(())
}
```

There you go.
As you can see, the main transfer doesn't actually do much — it only handles a handshake.
This makes adding more features later on very simple.

#### The `worker`

Now, the actual worker.
The worker is its own process, just like the `file_transfer` process.
Therefore, you need to create a new process directory, `worker`, next to the `file_transfer` process, inside the `file_transfer` package.
E.g.,

```bash
cp -r file_transfer worker
```

and change the `worker/Cargo.toml` `name` to `worker`.

First, its worth noting that because when you spawn `worker` you give it `our_capabilities()` (i.e. it has the same capabilities as the parent process), the worker will have the ability to message both `"net:distro:sys"` and `"vfs:distro:sys"`.
Since `worker` is in the same package as `file_transfer`, it has the capability to open the `files` directory, see discussion on [VFS drives](../apis/vfs.md) for more details.

Overwrite the copied in `worker/src/lib.rs` with the skeleton of `worker`, including imports and `init()`:

```rust
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use kinode_process_lib::{
    await_message, get_blob, println,
    vfs::{open_dir, open_file, Directory, File, SeekFrom},
    Address, Message, ProcessId, Request, Response,
};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

const CHUNK_SIZE: u64 = 1048576; // 1MB

#[derive(Serialize, Deserialize, Debug)]
pub enum WorkerRequest {
    Initialize {
        name: String,
        target_worker: Option<Address>,
    },
    Chunk {
        name: String,
        offset: u64,
        length: u64,
    },
    Size(u64),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransferRequest {
    ListFiles,
    Download { name: String, target: Address },
    Progress { name: String, progress: u64 },
}

struct Component;
impl Guest for Component {
    fn init(our: String) {
        println!("file_transfer worker: begin");

        let our = Address::from_str(&our).unwrap();

        let drive_path = format!("{}/files", our.package_id());
        let files_dir = open_dir(&drive_path, false).unwrap();

        loop {
            match handle_message(&our, &files_dir) {
                Ok(()) => {}
                Err(e) => {
                    println!("file_transfer: error: {:?}", e);
                }
            };
        }
    }
}
```

You'll also need a bit of state for the receiving worker.
This is not persisted (you'll add that soon!), but when different chunks arrive, you need to know what file to write to and how long that file should eventually become to generate progress updates.
This is not known at the point of spawning (`init()` takes just an `our: String`), but instead from `WorkerRequest::Initialize`.

The state you'll initialize at the start of the worker will look like this:

```rust
let mut file: Option<File> = None;
let mut size: Option<u64> = None;
```

And then in the main loop we pass it to `handle_message()`:

```rust
struct Component;
impl Guest for Component {
    fn init(our: String) {
        println!("file_transfer worker: begin");

        let our = Address::from_str(&our).unwrap();

        let drive_path = format!("{}/files", our.package_id());
        let files_dir = open_dir(&drive_path, false).unwrap();

        let mut file: Option<File> = None;
        let mut size: Option<u64> = None;
        loop {
            match handle_message(&our, &mut file, &files_dir, &mut size) {
                Ok(()) => {}
                Err(e) => {
                    println!("file_transfer: error: {:?}", e);
                }
            };
        }
    }
}
```

The `handle_message()` function will handle three `WorkerRequest` variants: the requests `Initialize`, `Chunk` and `Size`.

`WorkerRequest::Initialize` runs once, received from the spawner:

```rust
fn handle_message(
    our: &Address,
    file: &mut Option<File>,
    files_dir: &Directory,
    size: &mut Option<u64>,
) -> anyhow::Result<()> {
    let message = await_message()?;

    match message {
        Message::Request {
            ref source,
            ref body,
            ..
        } => {
            let request = serde_json::from_slice::<WorkerRequest>(body)?;

            match request {
                WorkerRequest::Initialize {
                    name,
                    target_worker,
                } => {
                    // initialize command from main process,
                    // sets up worker, matches on if it's a sender or receiver.
                    // target_worker = None, we are receiver, else sender.

                    // open/create empty file in both cases.
                    let mut active_file =
                        open_file(&format!("{}/{}", files_dir.path, &name), true)?;

                    match target_worker {
                        Some(target_worker) => {
                            // we have a target, chunk the data, and send it.
                            let size = active_file.metadata()?.len;
                            let num_chunks = (size as f64 / CHUNK_SIZE as f64).ceil() as u64;

                            // give the receiving worker a size request so it can track it's progress!
                            Request::new()
                                .body(serde_json::to_vec(&WorkerRequest::Size(size))?)
                                .target(target_worker.clone())
                                .send()?;

                            active_file.seek(SeekFrom::Start(0))?;

                            for i in 0..num_chunks {
                                let offset = i * CHUNK_SIZE;
                                let length = CHUNK_SIZE.min(size - offset);

                                let mut buffer = vec![0; length as usize];
                                active_file.read_at(&mut buffer)?;

                                Request::new()
                                    .body(serde_json::to_vec(&WorkerRequest::Chunk {
                                        name: name.clone(),
                                        offset,
                                        length,
                                    })?)
                                    .target(target_worker.clone())
                                    .blob_bytes(buffer)
                                    .send()?;
                            }
                            Response::new().body(serde_json::to_vec(&"Done")?).send()?;
                            return Ok(());
                        }
                        None => {
                            // waiting for response, store created empty file.
                            *file = Some(active_file);
                            Response::new()
                                .body(serde_json::to_vec(&"Started")?)
                                .send()?;
                        }
                    }
                }
            }
        }
        _ => {
            println!("file_transfer worker: got something else than request...");
        }
    }
    Ok(())
}
```

So upon `Initialize`, you open the existing file or create an empty one.
Then:

- if receiver, save the `File` to your state, and then send a Started response to parent.
- if sender, get the file's length, send it as `Size` to the `target_worker`, and then iteratively send chunks to `target_worker`.

The `WorkerRequest::Chunk` branch of the `handle_message()` `match` will look like this:

```rust
WorkerRequest::Chunk {
    name,
    offset,
    length,
} => {
    let file = match file {
        Some(file) => file,
        None => {
            return Err(anyhow::anyhow!(
                "file_transfer: receive error: no file initialized"
            ));
        }
    };

    let bytes = match get_blob() {
        Some(blob) => blob.bytes,
        None => {
            return Err(anyhow::anyhow!("file_transfer: receive error: no blob"));
        }
    };

    file.write_all(&bytes)?;

    // if sender has sent us a size, give a progress update to main transfer!
    if let Some(size) = size {
        let progress = ((offset + length) as f64 / *size as f64 * 100.0) as u64;

        // send update to main process
        let main_app = Address {
            node: our.node.clone(),
            process: ProcessId::from_str(
                "file_transfer:file_transfer:template.os",
            )?,
        };

        Request::new()
            .body(serde_json::to_vec(&TransferRequest::Progress {
                name,
                progress,
            })?)
            .target(&main_app)
            .send()?;

        if progress >= 100 {
            Response::new().body(serde_json::to_vec(&"Done")?).send()?;
            return Ok(());
        }
    }
}
```

And `WorkerRequest::Size` branch is easy:

```rust
WorkerRequest::Size(incoming_size) => {
    *size = Some(incoming_size);
}
```

One more thing: once you're done sending, you can exit the process; the worker is not needed anymore.
Change your `handle_message()` function to return a `Result<bool>` instead telling the main loop whether it should exit or not.
As a bonus, we can add a print when it exits of how long it took to send/receive!

```rust
fn handle_message(
    our: &Address,
    file: &mut Option<File>,
    files_dir: &Directory,
    size: &mut Option<u64>,
) -> anyhow::Result<bool> {
```

Change the return value of `handle_message()` return the Ok(exit)` as appropriate.
Finally, change the main loop to:

```rust
struct Component;
impl Guest for Component {
    fn init(our: String) {
        println!("file_transfer worker: begin");
        let start = std::time::Instant::now();

        let our = Address::from_str(&our).unwrap();

        let drive_path = format!("{}/files", our.package_id());
        let files_dir = open_dir(&drive_path, false).unwrap();

        let mut file: Option<File> = None;
        let mut size: Option<u64> = None;

        loop {
            match handle_message(&our, &mut file, &files_dir, &mut size) {
                Ok(exit) => {
                    if exit {
                        println!(
                            "file_transfer worker done: exiting, took {:?}",
                            start.elapsed()
                        );
                        break;
                    }
                }
                Err(e) => {
                    println!("file_transfer: worker error: {:?}", e);
                }
            };
        }
    }
}
```

### Final Code

And Voilà! The worker and then the main process in entirety:

```rust
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use kinode_process_lib::{
    await_message, get_blob, println,
    vfs::{open_dir, open_file, Directory, File, SeekFrom},
    Address, Message, ProcessId, Request, Response,
};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

const CHUNK_SIZE: u64 = 1048576; // 1MB

#[derive(Serialize, Deserialize, Debug)]
pub enum WorkerRequest {
    Initialize {
        name: String,
        target_worker: Option<Address>,
    },
    Chunk {
        name: String,
        offset: u64,
        length: u64,
    },
    Size(u64),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransferRequest {
    ListFiles,
    Download { name: String, target: Address },
    Progress { name: String, progress: u64 },
}

fn handle_message(
    our: &Address,
    file: &mut Option<File>,
    files_dir: &Directory,
    size: &mut Option<u64>,
) -> anyhow::Result<bool> {
    let message = await_message()?;

    match message {
        Message::Request {
            ref body,
            ..
        } => {
            let request = serde_json::from_slice::<WorkerRequest>(body)?;

            match request {
                WorkerRequest::Initialize {
                    name,
                    target_worker,
                } => {
                    // initialize command from main process,
                    // sets up worker, matches on if it's a sender or receiver.
                    // target_worker = None, we are receiver, else sender.

                    // open/create empty file in both cases.
                    let mut active_file =
                        open_file(&format!("{}/{}", files_dir.path, &name), true)?;

                    match target_worker {
                        Some(target_worker) => {
                            // we have a target, chunk the data, and send it.
                            let size = active_file.metadata()?.len;
                            let num_chunks = (size as f64 / CHUNK_SIZE as f64).ceil() as u64;

                            // give the receiving worker a size request so it can track it's progress!
                            Request::new()
                                .body(serde_json::to_vec(&WorkerRequest::Size(size))?)
                                .target(target_worker.clone())
                                .send()?;

                            active_file.seek(SeekFrom::Start(0))?;

                            for i in 0..num_chunks {
                                let offset = i * CHUNK_SIZE;
                                let length = CHUNK_SIZE.min(size - offset);

                                let mut buffer = vec![0; length as usize];
                                active_file.read_at(&mut buffer)?;

                                Request::new()
                                    .body(serde_json::to_vec(&WorkerRequest::Chunk {
                                        name: name.clone(),
                                        offset,
                                        length,
                                    })?)
                                    .target(target_worker.clone())
                                    .blob_bytes(buffer)
                                    .send()?;
                            }
                            Response::new().body(serde_json::to_vec(&"Done")?).send()?;
                            return Ok(true);
                        }
                        None => {
                            // waiting for response, store created empty file.
                            *file = Some(active_file);
                            Response::new()
                                .body(serde_json::to_vec(&"Started")?)
                                .send()?;
                        }
                    }
                }
                // someone sending a chunk to us!
                WorkerRequest::Chunk {
                    name,
                    offset,
                    length,
                } => {
                    let file = match file {
                        Some(file) => file,
                        None => {
                            return Err(anyhow::anyhow!(
                                "file_transfer: receive error: no file initialized"
                            ));
                        }
                    };

                    let bytes = match get_blob() {
                        Some(blob) => blob.bytes,
                        None => {
                            return Err(anyhow::anyhow!("file_transfer: receive error: no blob"));
                        }
                    };

                    file.write_all(&bytes)?;
                    // if sender has sent us a size, give a progress update to main transfer!
                    if let Some(size) = size {
                        let progress = ((offset + length) as f64 / *size as f64 * 100.0) as u64;

                        // send update to main process
                        let main_app = Address {
                            node: our.node.clone(),
                            process: ProcessId::from_str(
                                "file_transfer:file_transfer:template.os",
                            )?,
                        };

                        Request::new()
                            .body(serde_json::to_vec(&TransferRequest::Progress {
                                name,
                                progress,
                            })?)
                            .target(&main_app)
                            .send()?;

                        if progress >= 100 {
                            Response::new().body(serde_json::to_vec(&"Done")?).send()?;
                            return Ok(true);
                        }
                    }
                }
                WorkerRequest::Size(incoming_size) => {
                    *size = Some(incoming_size);
                }
            }
        }
        _ => {
            println!("file_transfer worker: got something else than request...");
        }
    }
    Ok(false)
}

struct Component;
impl Guest for Component {
    fn init(our: String) {
        println!("file_transfer worker: begin");
        let start = std::time::Instant::now();

        let our = Address::from_str(&our).unwrap();

        let drive_path = format!("{}/files", our.package_id());
        let files_dir = open_dir(&drive_path, false).unwrap();

        let mut file: Option<File> = None;
        let mut size: Option<u64> = None;

        loop {
            match handle_message(&our, &mut file, &files_dir, &mut size) {
                Ok(exit) => {
                    if exit {
                        println!(
                            "file_transfer worker done: exiting, took {:?}",
                            start.elapsed()
                        );
                        break;
                    }
                }
                Err(e) => {
                    println!("file_transfer: worker error: {:?}", e);
                }
            };
        }
    }
}
```

And the main process:

```rust
use kinode_process_lib::{
    await_message, our_capabilities, println, spawn,
    vfs::{create_drive, metadata, open_dir, Directory, FileType},
    Address, Message, OnExit, Request, Response,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

#[derive(Serialize, Deserialize, Debug)]
pub enum TransferRequest {
    ListFiles,
    Download { name: String, target: Address },
    Progress { name: String, progress: u64 },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransferResponse {
    ListFiles(Vec<FileInfo>),
    Download { name: String, worker: Address },
    Done,
    Started,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WorkerRequest {
    Initialize {
        name: String,
        target_worker: Option<Address>,
    },
}

fn handle_transfer_request(
    our: &Address,
    source: &Address,
    body: &Vec<u8>,
    files_dir: &Directory,
) -> anyhow::Result<()> {
    let transfer_request = serde_json::from_slice::<TransferRequest>(body)?;

    match transfer_request {
        TransferRequest::ListFiles => {
            let entries = files_dir.read()?;
            let files: Vec<FileInfo> = entries
                .iter()
                .filter_map(|file| match file.file_type {
                    FileType::File => match metadata(&file.path) {
                        Ok(metadata) => Some(FileInfo {
                            name: file.path.clone(),
                            size: metadata.len,
                        }),
                        Err(_) => None,
                    },
                    _ => None,
                })
                .collect();

            Response::new()
                .body(serde_json::to_vec(&TransferResponse::ListFiles(files))?)
                .send()?;
        }
        TransferRequest::Download { name, target } => {
            // spin up a worker, initialize based on whether it's a downloader or a sender.
            let our_worker = spawn(
                None,
                &format!("{}/pkg/worker.wasm", our.package_id()),
                OnExit::None,
                our_capabilities(),
                vec![],
                false,
            )?;

            let our_worker_address = Address {
                node: our.node.clone(),
                process: our_worker,
            };

            match source.node == our.node {
                true => {
                    // we want to download a file
                    let _resp = Request::new()
                        .body(serde_json::to_vec(&WorkerRequest::Initialize {
                            name: name.clone(),
                            target_worker: None,
                        })?)
                        .target(&our_worker_address)
                        .send_and_await_response(5)??;

                    // send our initialized worker address to the other node
                    Request::new()
                        .body(serde_json::to_vec(&TransferRequest::Download {
                            name: name.clone(),
                            target: our_worker_address,
                        })?)
                        .target(&target)
                        .send()?;
                }
                false => {
                    // they want to download a file
                    Request::new()
                        .body(serde_json::to_vec(&WorkerRequest::Initialize {
                            name: name.clone(),
                            target_worker: Some(target),
                        })?)
                        .target(&our_worker_address)
                        .send()?;
                }
            }
        }
        TransferRequest::Progress { name, progress } => {
            println!("file: {} progress: {}%", name, progress);
        }
    }

    Ok(())
}

fn handle_transfer_response(source: &Address, body: &Vec<u8>) -> anyhow::Result<()> {
    let transfer_response = serde_json::from_slice::<TransferResponse>(body)?;

    match transfer_response {
        TransferResponse::ListFiles(files) => {
            println!("got files from node: {:?} ,files: {:?}", source, files);
        }
        _ => {}
    }

    Ok(())
}

fn handle_message(our: &Address, file_dir: &Directory) -> anyhow::Result<()> {
    let message = await_message()?;

    match message {
        Message::Response { ref source, ref body, .. } =>
            handle_transfer_response(source, body)?,
        Message::Request { ref source, ref body, .. } =>
            handle_transfer_request(&our, source, body, file_dir)?,
    };

    Ok(())
}

struct Component;
impl Guest for Component {
    fn init(our: String) {
        println!("file_transfer: begin");

        let our = Address::from_str(&our).unwrap();

        let drive_path = create_drive(our.package_id(), "files").unwrap();
        let files_dir = open_dir(&drive_path, false).unwrap();

        loop {
            match handle_message(&our, &files_dir) {
                Ok(()) => {}
                Err(e) => {
                    println!("file_transfer: error: {:?}", e);
                }
            };
        }
    }
}
```

### Conclusion

There you have it!

Try and run it, you can download a file with the command

```
/m our@file_transfer:file_transfer:template.os {"Download": {"name": "dawg.jpeg", "target": "fake2.os@file_transfer:file_transfer:template.os"}}
```

replacing node name and file name!

Stay tuned for additions to this guide, including restarting transfers after rebooting your node or losing connections, and a simple UI!
