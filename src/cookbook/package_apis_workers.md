# Exporting Workers in Package APIs

Kinode packages can export workers and expose them in easy-to-use ways.
Exporting and importing functions is discussed in the [previous recipe](./package_apis.md).
This recipe focuses on:
1. A simple example of exporting a worker and exposing it in an ergonmoic API.
2. A simple example of importing a worker.
3. Demonstrations of `kit` tooling for the above.

## Exporting a Worker

Exporting or importing a worker is much the same as exporting or importing an API as usual as discussed in the [previous recipe](./package_apis.md).
The main difference, in general, is that the exporter must include the worker when `kit build`ing — see [below](#chat-with-file-transfer-usage-example).
In the specific example here, the exporter also exports a function that makes use of the worker ergonomic: that function, `start_download()`, [`spawn()`s](https://github.com/kinode-dao/process_lib/blob/9a53504693676094ba06f601312457675d10ca8a/src/lib.rs#L137) the worker.
In addition, in this specific example, the importer handles the message types of the worker.

### Example: File Transfer

#### WIT API

```rust
...
{{#webinclude https://raw.githubusercontent.com/kinode-dao/kit/9c19d378b8f9f94975c4b4790029b1363c26e0fc/src/new/templates/rust/no-ui/file_transfer/api/%7Bpackage_name%7D%3A%7Bpublisher%7D-v0.wit 16:73}}
...
```

#### API Function Definitions

The API here `spawn()`s a worker, and so the worker is part of the API.

##### API

```rust
{{#webinclude https://raw.githubusercontent.com/kinode-dao/kit/9c19d378b8f9f94975c4b4790029b1363c26e0fc/src/new/templates/rust/no-ui/file_transfer/file_transfer_worker_api/src/lib.rs}}
```

##### Worker

```rust
{{#webinclude https://raw.githubusercontent.com/kinode-dao/kit/76380492ac93f701a837763968fdff24aaef36c6/src/new/templates/rust/no-ui/file_transfer/file_transfer_worker/src/lib.rs}}
```

#### Process

The `file_transfer` process imports and uses the exported `start_download()`:

```rust
{{#webinclude https://raw.githubusercontent.com/kinode-dao/kit/76380492ac93f701a837763968fdff24aaef36c6/src/new/templates/rust/no-ui/file_transfer/%7Bpackage_name%7D/src/lib.rs}}
```

## Importing an API

### Dependencies

#### `metadata.json`

The [`metadata.json`](https://github.com/kinode-dao/kinode-book/blob/main/src/code/chat-with-file-transfer/metadata.json#L14-L16) file has a `properties.dependencies` field.
When the `dependencies` field is populated, [`kit build`](../kit/build.md) will fetch that dependency from a Kinode hosting it.

See [previous recipe](./package_apis.md#dependencies) for more discussion of dependencies.

### Example: Chat with File Transfer

The example here is the `kit n chat` chat template with the small addition of file transfer functionality.
The addition of file transfer requires changes to the WIT API (to import the `file-transfer-worker` `interface`, e.g.) as well as to the process itself to make use of the imported types and functions.
Compare the [process](#process-1) with the unmodified `kit n chat` process.

#### WIT API

```rust
{{#includehidetest ../../code/chat-with-file-transfer/api/chat-with-file-transfer:template.os-v0.wit}}
```

#### Process

```rust
{{#includehidetest ../../code/chat-with-file-transfer/chat-with-file-transfer/src/lib.rs}}
```

## Chat with File Transfer Usage Example

### Build

```
# Start fake nodes.
kit f
kit f -o /tmp/kinode-fake-node-2 -p 8081 -f fake2.dev

# Create & build file_transfer dependency.
## The `-a` adds the worker Wasm file to the API so it can be exported properly.
kit n file_transfer -t file_transfer
kit b file_transfer -a file_transfer/pkg/file_transfer_worker.wasm

# Build chat_with_file_transfer.
## The `-l` satisfies the dependency using a local path.
kit b src/../code/chat-with-file-transfer -l file-transfer

# Start chat_with_file_transfer on fake nodes.
kit s src/../code/chat-with-file-transfer
kit s src/../code/chat-with-file-transfer -p 8081
```

### Usage

```
# First, put a file into `/tmp/kinode-fake-node-2/vfs/chat-with-file-transfer:template.os/files/`, e.g.:
echo 'hello world' > /tmp/kinode-fake-node-2/vfs/chat-with-file-transfer:template.os/files/my_file.txt

# In fake.dev terminal, download the file.
download:chat-with-file-transfer:template.os my_file.txt fake2.dev

# Confirm file was downloaded:
cat /tmp/kinode-fake-node/vfs/chat-with-file-transfer:template.os/files/my_file.txt
```
