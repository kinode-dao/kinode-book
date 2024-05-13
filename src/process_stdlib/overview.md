# `process_lib` Overview

This page serves as an introduction to the [process standard library](https://github.com/kinode-dao/process_lib), which is the easiest way to write Rust apps on Kinode OS.
The full documentation can be found [here](https://docs.rs/kinode_process_lib), and the crate lives [here](https://crates.io/crates/kinode_process_lib).

Note that the process lib is under heavy ongoing development.
Therefore, the best way to use it is by selecting a tag or commit hash from the GitHub repo and using that as the version in your `Cargo.toml` file.
See [here](https://github.com/kinode-dao/process_lib/releases) for a list of published versions.

In your `Cargo.toml` file, use a version tag like this:
```toml
kinode_process_lib = { git = "https://github.com/kinode-dao/process_lib.git", tag = "v0.5.4-alpha" }
```

To use a specific commit hash, use this:
```toml
kinode_process_lib = { git = "https://github.com/kinode-dao/process_lib.git", rev = "5305453" }
```

Make sure to use a recent version of the process_lib while the system is in alpha and very active development.

The major version of the process_lib will always match the major version of Kinode OS.
Since the current major version of both is 0, breaking changes can occur at any time.
Once the major version reaches 1, breaking changes will only occur between major versions.
As is, developers may have to update their version of process_lib as they update Kinode OS.

Since Kinode apps use the [WebAssembly Component Model](https://component-model.bytecodealliance.org/), they are built on top of a [WIT](https://component-model.bytecodealliance.org/design/wit.html) (Wasm Interface Type) [package](https://github.com/kinode-dao/kinode-wit/blob/master/kinode.wit).
[`wit-bindgen`](https://github.com/bytecodealliance/wit-bindgen) is used to generate Rust code from a WIT file. 
The generated code then contains the core types and functions that are available to all Kinode apps.

However, the types themselves are unwieldy to use directly, and runtime modules present APIs that can be drastically simplified by using helper functions and types in the process standard library.

Almost all code examples in this book make use of the process lib. For specific examples of its usage, check out the [docs](https://docs.rs/kinode_process_lib) or just follow the tutorials later in this book.
