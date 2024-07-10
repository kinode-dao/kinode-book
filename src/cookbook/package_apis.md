# Exporting & Importing Package APIs

Kinode packages can export APIs, as discussed [here](../process/wit-apis.md).
Processes can also import APIs.
These APIs can consist of types as well as functions.
This document focuses on
1. Simple examples of exporting and importing APIs (find the full code [here](https://github.com/kinode-dao/kinode-book/tree/main/src/code/remote_file_storage)).
2. Demonstrations of `kit` tooling to help build and export or import APIs.

## Exporting an API

APIs are defined in a WIT file.
A brief summary of more [thorough discussion](../process/wit-apis.md#high-level-overview) is provided here:
1. [WIT (Wasm Interface Type)](https://component-model.bytecodealliance.org/design/wit.html) is a language to define APIs.
   Kinode packages may define a WIT API by placing a WIT file in the top-level `api/` directory.
2. Processes define a [WIT `interface`](https://component-model.bytecodealliance.org/design/wit.html#interfaces).
3. Packages define a [WIT `world`](https://component-model.bytecodealliance.org/design/wit.html#worlds).
4. APIs define their own WIT `world` that `export`s at least one processes WIT `interface`.

### Example: Remote File Storage Server

#### WIT API

```rust
{{#includehidetest ../code/remote_file_storage/server/api/server:template.os-v0.wit}}
```

As summarized [above](#exporting-an-api), the `server` process defines an `interface` of the same name, and the package defines the `world server-template-dot-os-v0`.
The API is defined by `server-template-dot-os-api-v0`: the functions in the `server` interface are defined [below](#api-function-definitions) by `wit_bindgen::generate!()`ing that `world`.

#### API Function Definitions

```rust
{{#includehidetest ../code/remote_file_storage/server/server_api/src/lib.rs}}
```

#### Process

```rust
{{#includehidetest ../code/remote_file_storage/server/server/src/lib.rs}}
```

## Importing an API

### Dependencies

#### `metadata.json`

#### Fetching Dependencies

### Example: Remote File Storage Client Script

#### WIT API

```rust
{{#includehidetest ../code/remote_file_storage/client/api/client:template.os-v0.wit}}
```

#### Process

```rust
{{#includehidetest ../code/remote_file_storage/client/client/src/lib.rs}}
```
