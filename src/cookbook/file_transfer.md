# File Transfer

This recipe looks at the `file-transfer` package, a template included with `kit` and also copied [here](https://github.com/kinode-dao/kinode-book/tree/main/code/file-transfer).
To create the template use
```
kit n file-transfer -t file-transfer
```

The `file-transfer` package shows off a few parts of Kinode userspace:
1. It makes use of the [VFS](../apis/vfs.md) to store files on disk.
2. It uses a manager-worker pattern (see conceptual discussion [here](../system/process/processes.md#awaiting-a-response) and [here](../system/process/processes.md#spawning-child-processes)) to enable multiple concurrent uploads/downloads without sacrificing code readability.
3. It exports its [WIT API](../system/process/wit_apis.md) so that other packages can easily build in file transfer functionality in a library-like manner, as demonstrated in [another recipe](./package_apis_workers.md).

## Protocol

The main `file-transfer` process is a thin wrapper over the `file-transfer-worker-api`.
The main process manages transfers and exposes a `ListFiles` Request variant that, when requested, returns the files that are available for download.

The `file-transfer-worker-api` makes calling the `file-transfer-worker` ergonomic.
Specifically, it provides a function, `start_download()`, which spins up a worker to download a file from a given node.
When called on the node serving the file, it spins up a worker to upload the requested file to the requestor.

Downloading a file proceeds as follows:
1. Requestor [calls](https://github.com/kinode-dao/kinode-book/blob/main/code/file-transfer/file-transfer/src/lib.rs#L94) [`start_download()`](https://github.com/kinode-dao/kinode-book/blob/main/src/code/file-transfer/file-transfer-worker-api/src/lib.rs#L14-L55), which:
   1. `spawn()`s a `file-transfer-worker`.
   2. Passes `file-transfer-worker` a `Download` Request variant.
   3. `file-transfer-worker` [forwards a modified `Download` Request variant to the `target`](https://github.com/kinode-dao/kinode-book/blob/main/src/code/file-transfer/file-transfer-worker/src/lib.rs#L70-L79).
2. Provider receives `Download` Request variant, calls `start_download()`, which:
   1. `spawn()`s a `file-transfer-worker`.
   2. Passes `file-transfer-worker` the `Download` Request variant.
   3. [Sends chunks of file to the requestor's `file-transfer-worker`](https://github.com/kinode-dao/kinode-book/blob/main/src/code/file-transfer/file-transfer-worker/src/lib.rs#L81-L110).

Thus, a worker is responsible for downloading/uploading a single file, and then exits.
All longer-term state and functionality is the responsibility of the main process, here, `file-transfer`.

Files are transferred from and to the `file-transfer:template.os/files` drive.
If you use the `file-transfer-worker` or `file-transfer-worker-api` in your own package, replace that first part of the path with your package's package id.

## WIT API

```rust
{{#includehidetest ../../code/file-transfer/api/file-transfer:template.os-v0.wit}}
```

## Main Process

```rust
{{#includehidetest ../../code/file-transfer/file-transfer/src/lib.rs}}
```

## Worker

```rust
{{#includehidetest ../../code/file-transfer/file-transfer-worker/src/lib.rs}}
```

## API

```rust
{{#includehidetest ../../code/file-transfer/file-transfer-worker-api/src/lib.rs}}
```

## Example Usage

### Build

```
# Start fake nodes.
kit f
kit f -o /tmp/kinode-fake-node-2 -p 8081 -f fake2.dev

# Create & build file-transfer.
## The `-a` adds the worker Wasm file to the API so it can be exported properly.
kit n file-transfer -t file-transfer
kit b file-transfer -a file-transfer/pkg/file-transfer-worker.wasm

# Start file-transfer on fake nodes.
kit s file-transfer
kit s file-transfer -p 8081
```

### Usage

```
# First, put a file into `/tmp/kinode-fake-node-2/vfs/file-transfer:template.os/files/`, e.g.:
echo 'hello world' > /tmp/kinode-fake-node-2/vfs/file-transfer:template.os/files/my_file.txt

# In fake.dev terminal, check if file exists.
list-files:file-transfer:template.os fake2.dev

# In fake.dev terminal, download the file.
download:file-transfer:template.os my_file.txt fake2.dev

# Confirm file was downloaded:
cat /tmp/kinode-fake-node/vfs/file-transfer:template.os/files/my_file.txt
```
