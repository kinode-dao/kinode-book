# Files

## VFS

The primary way to access files within your node is through the VFS [API](./apis/vfs.md).
It tries to map over the functions of std::fs as clearly as possible, adding some capabilities checks on paths and some combinatory actions.

They exist within the "/vfs" folder within your home node, and files are grouped by `package_id`.
An example part of your VFS might look like this:

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
