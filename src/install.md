# Installation

This section will teach you how to get the Kinode OS core software, required to run a live node.
After acquiring the software, you can learn how to run it and [Join the Network](./login.md).
However, if you are just interested in starting development as fast as possible, start with [My First Kinode Application](./my_first_app/chapter_1.md).

## Download Binary

The recommended method for most users is to use a precompiled binary.
If you want to make edits to the Kinode core software, see [Build From Source](#build-from-source).

First, get the software itself by downloading a [precompiled release binary](https://github.com/kinode-dao/kinode/releases).
Choose the correct binary for your particular computer architecture and OS.
There is no need to download the `simulation-mode` binary — it is used behind the scenes.
Extract the `.zip` file and the binary is inside.

Note that some operating systems, particularly Apple, may flag the download as suspicious.
While the binary has not been tested exhaustively on all Linux distributions, it should *just work*.

### Apple

First, attempt to run the binary, which Apple will block.
Then, go to `System Settings > Privacy and Security` and click to `Allow Anyway` for the `kinode` binary:

![Apple unknown developer](./assets/apple-unknown-developer.png)

## Build From Source

You can compile the binary from source using the following instructions.

### Acquire Rust and various tools

Install Rust and some `cargo` tools, by running the following in your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-tools
rustup install nightly
rustup target add wasm32-wasi
rustup target add wasm32-wasi --toolchain nightly
cargo install cargo-wasi
```

For more information, or debugging, see the [Rust lang install page](https://www.rust-lang.org/tools/install).

### Acquire Kinode OS core

Clone and set up the repository:

```bash
git clone git@github.com:kinode-dao/kinode.git
```

Build the binary:

```bash
# OPTIONAL: --release flag
cargo +nightly build -p kinode
```

The resulting binary will be at path `target/debug/kinode`.

You can also build the binary with the `--release` flag.
Building without `--release` will produce the binary significantly faster, as it does not perform any optimizations during compilation, but the node will run much more slowly after compiling.
The release binary will be at path `target/release/kinode`.
