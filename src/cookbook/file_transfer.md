# File Transfer

In this entry we're going to be building a file transfer app, letting nodes download files from a public directory. It will use the vfs to read and write files, and will spin up worker processes for the transfer.

This guide assumes a basic understanding of nectar process building, some familiarity with necdev, requests and responses, and some knowledge of rust syntax.

## Start

First let's initialize a new project with `necdev new file_transfer`

I cleaned out the template code so we have a complete fresh start:

We're using the following nectar_process_lib version in Cargo.toml for this app:
`nectar_process_lib = { git = "ssh://git@github.com/uqbar-dao/process_lib.git", rev = "412fbfe" }`

```rust
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use nectar_process_lib::{await_message, println, Address, Message, ProcessId, Request, Response};

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

Before delving into the code, let's also handle the capabilities we need to request at spawn, these will be messaging capabilities to "net:sys:nectar" (as we want to talk to other nodes), and one to "vfs:sys:nectar" as we want to talk to the filesystem.

`pkg/manifest.json`

```json
[
    {
        "process_name": "file_transfer",
        "process_wasm_path": "/file_transfer.wasm",
        "on_exit": "Restart",
        "request_networking": true,
        "request_capabilities": [
            "net:sys:nectar",
            "vfs:sys:nectar"
        ],
        "grant_capabilities": [],
        "public": true
    }
]
```

Now, let's start by creating a drive (folder) in our vfs and opening it, where files will be downloaded by other nodes.
We'll add a whitelist a bit later!

We'll import a bunch of the vfs functions from the process_lib, and specifically the `create_drive` and `open_dir` functions.

```rust
use nectar_process_lib::vfs::{create_drive, metadata, open_dir, Directory, FileType},

let drive_path = create_drive(our.package_id(), "files").unwrap();
```

To start, this will be an app without UI, so the way to get files in, you simply copy them into the "files" folder located in `your_node/vfs/file_transfer:file_transfer:template.uq/files`

We now need some way for other nodes to know what files they can download from us, so let's add some message types!

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

We can start with this, a node can request a list of files, and we give them a list of file names and their sizes in bytes.

Adding some matching for requests and responses, and deserializing into our TransferRequest type.

```rust
use nectar_process_lib::{
    await_message, println,
    vfs::{create_drive, metadata, open_dir, Directory, FileType},
    Address, Message, ProcessId, Request, Response,
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

fn handle_transfer_request(
    our: &Address,
    source: &Address,
    body: &Vec<u8>,
    file_dir: &Directory,
) -> anyhow::Result<()> {
    let transfer_request = serde_json::from_slice::<TransferRequest>(body)?;

    match transfer_request {
        TransferRequest::ListFiles => {
            println!("hellö");
        }
    }

    Ok(())
}

fn handle_message(our: &Address, file_dir: &Directory) -> anyhow::Result<()> {
    let message = await_message()?;

    match message {
        Message::Response { .. } => {
            return Ok(());
        }
        Message::Request {
            ref source,
            ref body,
            ..
        } => {
            handle_transfer_request(&our, source, body, file_dir)?;
        }
    };

    println!("file_transfer: got message!: {:?}", message);
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

Now, we can fill in the ListFiles request and response behaviour, which is just a readDir action to the vfs.

```rust

match transfer_request {
    TransferRequest::ListFiles => {
        let entries = file_dir.read()?;
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
```

And add the corresponding handle_transfer_response too!

```rust

match message {
    Message::Response {
        ref source,
        ref body,
        ..
    } => {
        handle_transfer_response(our, source, body, file_dir)?;
    }
    Message::Request {
        ref source,
        ref body,
        ..
    } => {
        handle_transfer_request(&our, source, body, file_dir)?;
    }
};

// ...

fn handle_transfer_response(
    our: &Address,
    source: &Address,
    body: &Vec<u8>,
    file_dir: &Directory,
) -> anyhow::Result<()> {
    let transfer_response = serde_json::from_slice::<TransferResponse>(body)?;

    match transfer_response {
        TransferResponse::ListFiles(files) => {
            println!("got files from node: {:?} ,files: {:?}", source, files);
        }
    }

    Ok(())
}
```

You can now try this out by booting two nodes (fake or real), putting some files in the /files folder of one of them, and sending a request!

`/m node2.nec@file_transfer:file_transfer:template.uq "ListFiles"`

And you'll see a response printed!

### Transfer

Now, let's get to the fun part, downloading/sending files!

We could handle all of this within our file_transfer process, but something we can easily do better is to spin up another process, a worker, that does the downloading/sending, and just sends progress updates back to the main file_transfer!

This way we can have several files downloading at the same time, not waiting for one to finish.

Let's start by defining some types.

We'll need a request that tells our main process to spin up a worker, requesting the node we're downloading from to do the same. Also, a progress report would be nice!

```rust
#[derive(Serialize, Deserialize, Debug)]
pub enum TransferRequest {
    ListFiles,
    Download { name: String, target: Address },
    Progress { name: String, progress: u64 },
}
```

This will give us a request to say "I want to download this file", and we'll get back, "all good, you can do it by calling this worker".

We'll also add a simple Start variant, so we'll know when our worker has successfully been spawned and initialized.

```rust
#[derive(Serialize, Deserialize, Debug)]
pub enum TransferResponse {
    ListFiles(Vec<FileInfo>),
    Download {
        name: String,
        worker: Address,
        size: u64,
    },
    Start,
}
```

Now let's add the intra worker communication types:

```rust
#[derive(Serialize, Deserialize, Debug)]
pub enum WorkerRequest {
    Init {
        name: String,
        target_worker: Address,
        is_requestor: bool,
        size: u64,
    },
    Chunk {
        name: String,
        offset: u64,
        length: u64,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WorkerResponse {
    Chunk {
        name: String,
        offset: u64,
        length: u64,
    },
}
```

Workers will take an init function from their own node, then take requests for specific chunks from other worker nodes.

Progress responses are sent back to the main process, which can then pipe them through as websocket updates to our frontend!

Let's code this out so it becomes clearer, we'll import the spawn function from the process_lib.

The only additional part we need to handle in the transfer app is the Download request we've added.

It will handle 2 cases, a node sent us a download request, we spawn a worker, and give it a response. The other case is if we want to download a file from another node, we send ourselves a download request, it forwards to the remote node, and with its `worker_address` we can spawn up our own worker to start sending chunk requests!

```rust
    match transfer_request {
        TransferRequest::ListFiles => {
            // like before
        }
        TransferRequest::Progress { name, progress } => {
            // for now, progress reports are just printed
            println!("file: {} progress: {}", name, progress);
        }
        TransferRequest::Download { name, target } => {
            // if source == our_node, we will send a download request to the target.
            // if not, it's a start downlaod request from another node.
            if source.node == our.node {
                let resp = Request::new()
                    .body(body.clone())
                    .target(target)
                    .send_and_await_response(5)??;

                let transfer_response = serde_json::from_slice::<TransferResponse>(&resp.body())?;

                match transfer_response {
                    TransferResponse::Download { name, worker, size } => {
                        // spin up a worker, and init it with the worker that it can use to download
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

                        Request::new()
                            .body(serde_json::to_vec(&WorkerRequest::Init {
                                name: name.clone(),
                                target_worker: worker,
                                is_requestor: true,
                                size,
                            })?)
                            .target(our_worker_address)
                            .send()?;
                    }
                    _ => {
                        println!(
                            "file_transfer: got something else as response to download request!"
                        );
                    }
                }
            } else {
                // download request from remote node.
                // spin up our worker, requestor = false
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

                let size = metadata(&format!("{}/{}", files_dir.path, &name))?.len;

                // initialize it
                let _resp = Request::new()
                    .body(serde_json::to_vec(&WorkerRequest::Init {
                        name: name.clone(),
                        target_worker: target,
                        is_requestor: false,
                        size,
                    })?)
                    .target(&our_worker_address)
                    .send()?;

                // now send response to source with our worker!
                Response::new()
                    .body(serde_json::to_vec(&TransferResponse::Download {
                        name,
                        worker: our_worker_address,
                        size,
                    })?)
                    .send()?;
            }
        }
    }
```

There we go. As we see the main transfer doesn't actually do much, all it handles is a handshake. This gives us the possibility to add more features easily later on.

Now, the actual worker. Let's add this bit by bit:

First, because when we spawn our worker, we give it `our_capabilities()`, it will have access to messaging and the vfs drive because we also do. So we can simply open the `files_dir` without issue.

```rust
truct Component;
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

We'll also need a bit of state. This is not persisted (we'll add that soon!), but when different chunks come in, we need to know what file to write to, how long that file is, and what our target is! This is not known at the point of spawning (init takes just a our: String), but we have an Init request for it. The state will look like this, and will be wrapped in an Option, so we can set it as None at start.

```rust
struct WorkerState {
    target: Address,
    is_requestor: bool,
    file: File,
    size: u64,
}
```

And then in the init function we pass it to handle_message:

```rust
struct Component;
impl Guest for Component {
    fn init(our: String) {
        println!("file_transfer worker: begin");

        let our = Address::from_str(&our).unwrap();

        let drive_path = format!("{}/files", our.package_id());
        let files_dir = open_dir(&drive_path, false).unwrap();

        let mut state: Option<WorkerState> = None;

        loop {
            match handle_message(&our, &mut state, &files_dir) {
                Ok(()) => {}
                Err(e) => {
                    println!("file_transfer: error: {:?}", e);
                }
            };
        }
    }
}
```

The handle_message function will handle 3 types. Requests Init and Chunk, and Response Chunk. Let's do the requests first.
Init runs once, received from the spawner:

```rust
fn handle_message(
    our: &Address,
    state: &mut Option<WorkerState>,
    files_dir: &Directory,
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
                // initialize command from main process,
                // sets up worker whether sender or receiver.
                WorkerRequest::Init {
                    name,
                    target_worker,
                    is_requestor,
                    size,
                } => {
                    //  open file within files directory, create if it doesn't exist.
                    let file = open_file(&format!("{}/{}", files_dir.path, &name), true)?;

                    let new_state = WorkerState {
                        target: target_worker.clone(),
                        is_requestor,
                        size,
                        file,
                    };

                    *state = Some(new_state);

                    // if we're the requestor, send requests to target to get chunks!
                    if is_requestor {
                        // round up, so if file is smaller than CHUNK_SIZE, it won't be 0.
                        let num_chunks = (size as f64 / CHUNK_SIZE as f64).ceil() as u64;
                        for i in 0..num_chunks {
                            let offset = i * CHUNK_SIZE;
                            let length = CHUNK_SIZE.min(size - offset);

                            Request::new()
                                .body(serde_json::to_vec(&WorkerRequest::Chunk {
                                    name: name.clone(),
                                    offset,
                                    length,
                                })?)
                                .target(target_worker.clone())
                                .send()?;
                        }
                    }
                }
            _ => {
                println!("chunk in next section!");
            }
        }
        _ => {
            println!("not handling responses quite yet ;)");
        }
    }
    Ok(())
}
```

So upon init, we message the other main app and get back it's worker address and file size, we initialize our state, and send out requests for all the chunks we'll need at once! Note that it's a Request::send(), not send_and_await(), so it won't block our main loop!

Then we need to handle the chunk requests and responses.

WorkerRequest::Chunk will look like this:

```rust
// someone requesting a chunk from us.
WorkerRequest::Chunk {
    name,
    offset,
    length,
} => {
    let state = match state {
        Some(state) => state,
        None => {
            println!("file_transfer: error: no state");
            return Ok(());
        }
    };

    // get exact requested chunk from file.
    let mut buffer = vec![0; length as usize];

    state.file.seek(SeekFrom::Start(offset))?;
    state.file.read_at(&mut buffer)?;

    // send response, but this time with the chunk in the lazy_load_blob!
    let response = WorkerResponse::Chunk {
        name,
        offset,
        length,
    };

    Response::new()
        .body(serde_json::to_vec(&response)?)
        .blob_bytes(buffer)
        .send()?;
}
```

We just get the part of the file the requestor wants.
Now the reverse, the response, if we get a response with the parts we want, we write it to the file!

```rust
Message::Response {
    ref source,
    ref body,
    ..
} => {
    let response = serde_json::from_slice::<WorkerResponse>(&body)?;

    match response {
        // response for a chunk we requested.
        WorkerResponse::Chunk {
            name,
            offset,
            length,
        } => {
            let state = match state {
                Some(state) => state,
                None => {
                    println!("file_transfer: error: no state");
                    return Ok(());
                }
            };

            let bytes = match get_blob() {
                Some(blob) => blob.bytes,
                None => {
                    println!("file_transfer: error: no blob");
                    return Ok(());
                }
            };

            state.file.seek(SeekFrom::Start(offset))?;
            state.file.write_at(&bytes)?;

            let progress = (offset + length) / state.size * 100;

            // send update to main process
            let main_app = Address {
                node: our.node.clone(),
                process: ProcessId::from_str("file_transfer:file_transfer:template.uq")?,
            };

            Request::new()
                .body(serde_json::to_vec(&TransferRequest::Progress {
                    name,
                    progress,
                })?)
                .target(&main_app)
                .send()?;
        }
    }
    return Ok(());
}
```

Bam! Here's the worker in it's entirety:

```rust
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use nectar_process_lib::{
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
    Init {
        name: String,
        target_worker: Address,
        is_requestor: bool,
        size: u64,
    },
    Chunk {
        name: String,
        offset: u64,
        length: u64,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WorkerResponse {
    Chunk {
        name: String,
        offset: u64,
        length: u64,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransferResponse {
    ListFiles(Vec<FileInfo>),
    Download { name: String, worker: Address },
    Start,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransferRequest {
    ListFiles,
    Download { name: String, target: Address },
    Progress { name: String, progress: u64 },
}

struct WorkerState {
    target: Address,
    is_requestor: bool,
    file: File,
    size: u64,
}

fn handle_message(
    our: &Address,
    state: &mut Option<WorkerState>,
    files_dir: &Directory,
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
                // initialize command from main process,
                // sets up worker whether sender or receiver.
                WorkerRequest::Init {
                    name,
                    target_worker,
                    is_requestor,
                    size,
                } => {
                    //  open file within files directory, create if it doesn't exist.
                    let file = open_file(&format!("{}/{}", files_dir.path, &name), true)?;

                    let new_state = WorkerState {
                        target: target_worker.clone(),
                        is_requestor,
                        size,
                        file,
                    };

                    *state = Some(new_state);

                    // if we're the requestor, send requests to target to get chunks!
                    if is_requestor {
                        // round up, so if file is smaller than CHUNK_SIZE, it won't be 0.
                        let num_chunks = (size as f64 / CHUNK_SIZE as f64).ceil() as u64;
                        for i in 0..num_chunks {
                            let offset = i * CHUNK_SIZE;
                            let length = CHUNK_SIZE.min(size - offset);

                            Request::new()
                                .body(serde_json::to_vec(&WorkerRequest::Chunk {
                                    name: name.clone(),
                                    offset,
                                    length,
                                })?)
                                .target(target_worker.clone())
                                .send()?;
                        }
                    }
                }
                // someone requesting a chunk from us.
                WorkerRequest::Chunk {
                    name,
                    offset,
                    length,
                } => {
                    let state = match state {
                        Some(state) => state,
                        None => {
                            println!("file_transfer: error: no state");
                            return Ok(());
                        }
                    };

                    // get exact requested chunk from file.
                    let mut buffer = vec![0; length as usize];

                    state.file.seek(SeekFrom::Start(offset))?;
                    state.file.read_at(&mut buffer)?;

                    // send response, but this time with the chunk in the lazy_load_blob!
                    let response = WorkerResponse::Chunk {
                        name,
                        offset,
                        length,
                    };

                    Response::new()
                        .body(serde_json::to_vec(&response)?)
                        .blob_bytes(buffer)
                        .send()?;
                }
            }
        }
        Message::Response {
            ref source,
            ref body,
            ..
        } => {
            let response = serde_json::from_slice::<WorkerResponse>(&body)?;

            match response {
                // response for a chunk we requested.
                WorkerResponse::Chunk {
                    name,
                    offset,
                    length,
                } => {
                    let state = match state {
                        Some(state) => state,
                        None => {
                            println!("file_transfer: error: no state");
                            return Ok(());
                        }
                    };

                    let bytes = match get_blob() {
                        Some(blob) => blob.bytes,
                        None => {
                            println!("file_transfer: error: no blob");
                            return Ok(());
                        }
                    };

                    state.file.seek(SeekFrom::Start(offset))?;
                    state.file.write_at(&bytes)?;

                    let progress = (offset + length) / state.size * 100;

                    // send update to main process
                    let main_app = Address {
                        node: our.node.clone(),
                        process: ProcessId::from_str("file_transfer:file_transfer:template.uq")?,
                    };

                    Request::new()
                        .body(serde_json::to_vec(&TransferRequest::Progress {
                            name,
                            progress,
                        })?)
                        .target(&main_app)
                        .send()?;
                }
            }
            return Ok(());
        }
    }
    Ok(())
}

struct Component;
impl Guest for Component {
    fn init(our: String) {
        println!("file_transfer worker: begin");

        let our = Address::from_str(&our).unwrap();

        let drive_path = format!("{}/files", our.package_id());
        let files_dir = open_dir(&drive_path, false).unwrap();

        let mut state: Option<WorkerState> = None;

        loop {
            match handle_message(&our, &mut state, &files_dir) {
                Ok(()) => {}
                Err(e) => {
                    println!("file_transfer: error: {:?}", e);
                }
            };
        }
    }
}
```

### Conclusion file_transfer (NO_UI)

And there you have it!

Try and run it, you can download a file with the command `/m our@file_transfer:file_transfer:template.nec {"Download": {"name": "dawg.jpeg", "target": "buenosaires.nec@file_transfer:file_transfer:template.nec"}}`, replacing node name and picture name!

Stay tuned for additions to this guide, including restarting transfers after rebooting your node or losing connections, and a simple UI!
