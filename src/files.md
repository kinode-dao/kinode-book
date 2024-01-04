# Files

## VFS

The primary way to access files within your node is through the [VFS API](./apis/vfs.md).
The VFS API follows std::fs closely, adding some capabilities checks on paths and some combinatory actions.

VFS files exist in the "/vfs" folder within your home node, and files are grouped by `package_id`.
For example, part of the VFS might look like:

```
.
├── app_store:uqbar
│   └── pkg
│       ├── app_store.wasm
│       ├── ft_worker.wasm
│       ├── manifest.json
│       └── metadata.json
├── chess:uqbar
│   └── pkg
│       ├── chess.html
│       ├── chess.wasm
│       ├── index.css
│       ├── index.js
│       ├── manifest.json
│       └── metadata.json
├── homepage:uqbar
│   └── pkg
│       ├── homepage.wasm
│       ├── manifest.json
│       └── metadata.json
```

## Links

- [VFS API](./apis/vfs.md)
- [std::fs API](https://doc.rust-lang.org/std/fs/index.html)
