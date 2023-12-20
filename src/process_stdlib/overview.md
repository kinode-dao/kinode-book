# Overview

-WIP-

[link to the crate]

[link to the crate-docs]

The process standard library is the easiest way to write Rust apps on Uqbar.

Since Uqbar apps use the [WebAssembly Component Model](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md), they are built on top of a WIT (wasm Interface Type) package. This package contains the core types and functions that are available to all Uqbar apps, and these are automatically generated in Rust when building a wasm app. However, the types themselves are unwieldly to use directly, and runtime modules present APIs that can be drastically simplified by using helper functions and types in the process standard library.
