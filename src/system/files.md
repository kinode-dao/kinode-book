# Files

## Virtual File System (VFS)

The primary way to access files within your node is through the [VFS API](../apis/vfs.md).
The VFS API follows [`std::fs`](https://doc.rust-lang.org/std/fs/index.html) closely, while also adding some capabilities checks on paths.

VFS files exist in the "/vfs" folder within your home node, and files are grouped by [`package_id`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/kinode/process/standard/struct.PackageId.html).
For example, part of the VFS might look like:

```text
node-home/vfs
├── app_store:sys
│   ├── pkg
│   │   ├── api
│   │   │   └── app_store:sys-v0.wit
│   │   ├── app_store.wasm
│   │   ├── manifest.json
│   │   ...
│   └── tmp
├── chess:sys
│   ├── pkg
│   │   ├── api
│   │   │   └── chess:sys-v0.wit
│   │   ├── chess.wasm
│   │   ├── manifest.json
│   │   └── ui
│   │       │
│   │       ...
│   └── tmp
├── homepage:sys
│   ├── pkg
│   │   ├── api
│   │   │   └── homepage:sys-v0.wit
│   │   ├── homepage.wasm
│   │   ├── manifest.json
│   │   └── ui
│   │       │
│   │       ...
│   └── tmp
...
```

## Drives

A drive is a directory within a package's VFS directory, e.g., `app_store:sys/pkg/` or `your_package:publisher.os/my_drive/`.
Drives are owned by processes.
Processes can share access to drives they own via [capabilities](process/capabilities.md).
Each package is spawned with two drives: [`pkg/`](#pkg-drive) and [`tmp/`](#tmp-drive).
All processes in a package have caps to these default drives.
Processes can also create additional drives.
These new drives are permissioned at the process-level: other processes will need to be granted capabilities to read or write these drives.

### `pkg/` drive

The `pkg/` drive contains metadata about the package that Kinode requires to run that package, `.wasm` binaries, and optionally the API of the package and the UI.
When creating packages, the `pkg/` drive is populated by [`kit build`](../kit/build.md) and loaded into the Kinode using [`kit start-package`](../kit/start-package.md).

### `tmp/` drive

The `tmp/` drive can be written to directly by the owning package using standard filesystem functionality (i.e. `std::fs` in Rust) via WASI in addition to the Kinode VFS.

## Usage

For usage examples, see the [VFS API](../apis/vfs.md).
