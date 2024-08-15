# File Transfer

This recipe looks at the `file_transfer` package, a template included with `kit` and also copied [here](https://github.com/kinode-dao/kinode-book/tree/main/src/code/file_transfer).
To create the template use
```
kit n file_transfer -t file_transfer
```

The `file_transfer` package shows off a few parts of Kinode userspace:
1. It makes use of the [VFS](../apis/vfs.md) to store files on disk.
2. It uses a manager-worker pattern (see conceptual discussion [here](../system/process/processes.md#awaiting-a-response) and [here](../system/process/processes.md#spawning-child-processes)) to enable multiple concurrent uploads/downloads without sacrificing code readability.
3. It exports its [WIT API](../system/process/wit_apis.md) so that other packages can easily build in file transfer functionality in a library-like manner, as demonstrated in [another recipe](./package_apis_workers.md).

## Protocol

The main `file_transfer` process is a thin wrapper over the `file_transfer_worker_api`.
The main process manages transfers and exposes a `ListFiles` Request variant that, when requested, returns the files that are available for download.

The `file_transfer_worker_api` makes calling the `file_transfer_worker` ergonomic.
Specifically, it provides a function, `start_download()`, which spins up a worker to download a file from a given node.
When called on the node serving the file, it spins up a worker to upload the requested file to the requestor.

Downloading a file proceeds as follows:
1. Requestor [calls](https://github.com/kinode-dao/kinode-book/blob/main/src/code/file_transfer/file_transfer/src/lib.rs#L94) [`start_download()`](https://github.com/kinode-dao/kinode-book/blob/main/src/code/file_transfer/file_transfer_worker_api/src/lib.rs#L14-L55), which:
   1. `spawn()`s a `file_transfer_worker`.
   2. Passes `file_transfer_worker` a `Download` Request variant.
   3. `file_transfer_worker` [forwards a modified `Download` Request variant to the `target`](https://github.com/kinode-dao/kinode-book/blob/main/src/code/file_transfer/file_transfer_worker/src/lib.rs#L70-L79).
2. Provider receives `Download` Request variant, calls `start_download()`, which:
   1. `spawn()`s a `file_transfer_worker`.
   2. Passes `file_transfer_worker` the `Download` Request variant.
   3. [Sends chunks of file to the requestor's `file_transfer_worker`](https://github.com/kinode-dao/kinode-book/blob/main/src/code/file_transfer/file_transfer_worker/src/lib.rs#L81-L110).

Thus, a worker is responsible for downloading/uploading a single file, and then exits.
All longer-term state and functionality is the responsibility of the main process, here, `file_transfer`.

Files are transferred from and to the `file_transfer:template.os/files` drive.
If you use the `file_transfer_worker` or `file_transfer_worker_api` in your own package, replace that first part of the path with your package's package id.

## WIT API

```rust
{{#includehidetest ../../code/file_transfer/api/file_transfer:template.os-v0.wit}}
```

## Main Process

```rust
{{#includehidetest ../../code/file_transfer/file_transfer/src/lib.rs}}
```

## Worker

```rust
{{#includehidetest ../../code/file_transfer/file_transfer_worker/src/lib.rs}}
```

## API

```rust
{{#includehidetest ../../code/file_transfer/file_transfer_worker_api/src/lib.rs}}
```

## Example Usage

### Build

```
# Start fake nodes.
kit f
kit f -o /tmp/kinode-fake-node-2 -p 8081 -f fake2.dev

# Create & build file_transfer.
## The `-a` adds the worker Wasm file to the API so it can be exported properly.
kit n file_transfer -t file_transfer
kit b file_transfer -a file_transfer/pkg/file_transfer_worker.wasm

# Start file_transfer on fake nodes.
kit s file_transfer
kit s file_transfer -p 8081
```

### Usage

```
# First, put a file into `/tmp/kinode-fake-node-2/vfs/file_transfer:template.os/files/`, e.g.:
echo 'hello world' > /tmp/kinode-fake-node-2/vfs/file_transfer:template.os/files/my_file.txt

# In fake.dev terminal, check if file exists.
list_files:file_transfer:template.os fake2.dev

# In fake.dev terminal, download the file.
download:file_transfer:template.os my_file.txt fake2.dev

# Confirm file was downloaded:
cat /tmp/kinode-fake-node/vfs/file_transfer:template.os/files/my_file.txt
```
