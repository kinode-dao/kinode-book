# Overview

The [process standard library](https://github.com/uqbar-dao/process_lib) is the easiest way to write Rust apps on Kinode OS.
Documentation can be found [here](https://docs.rs/kinode_process_lib), and the crate lives [here](https://crates.io/crates/kinode_process_lib).

Since Kinode apps use the [WebAssembly Component Model](https://component-model.bytecodealliance.org/), they are built on top of a WIT (Wasm Interface Type) package.
This interface contains the core types and functions that are available to all Kinode apps, and these are automatically generated in Rust when building a Wasm app.
However, the types themselves are unwieldy to use directly, and runtime modules present APIs that can be drastically simplified by using helper functions and types in the process standard library.
