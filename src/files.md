# Files

## Virtual File System (VFS)

The primary way to access files within your node is through the [VFS API](./apis/vfs.md).
The VFS API follows std::fs closely, adding some capabilities checks on paths and some combinatory actions.

VFS files exist in the "/vfs" folder within your home node, and files are grouped by `package_id`.
For example, part of the VFS might look like:

```text
.
├── app_store:sys
│   ├── metadata.json
│   └── pkg
│       ├── app_store.wasm
│       ├── ft_worker.wasm
│       └── manifest.json
├── chess:sys
│   ├── metadata.json
│   └── pkg
│       ├── chess.html
│       ├── chess.wasm
│       ├── index.css
│       ├── index.js
│       └── manifest.json
├── homepage:sys
│   ├── metadata.json
│   └── pkg
│       ├── homepage.wasm
│       └── manifest.json
```

## Usage

To access files in the VFS, you need to create or open a [drive](./apis/vfs.md#drives), this can be done with the function `create_drive` from the [standard library](./process_stdlib/overview.md):

```rust
let drive_path: String = create_drive(our.package_id(), "drive_name")?;

let test_file = create_file(&format("{}/test.txt", &drive_path))?;

let text = b"hello world!"
file.write(&text)?;
```

## References

- [VFS API](./apis/vfs.md)
- [std::fs API](https://doc.rust-lang.org/std/fs/index.html)
