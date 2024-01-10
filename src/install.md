# Installation

This section will teach you how to get the NectarOS core software, required to run a live node.
After acquiring the software, you can learn how to run it and [Join the Network](./login.md).
However, if you are just interested in starting development as fast as possible, start with [My First Nectar Application](./my_first_app/chapter_1.md).

## Download Binary

The recommended method for most users is to use a precompiled binary.
If you want to make edits to the Nectar core software, see [Build From Source](#build-from-source).

First, get the software itself by downloading a [precompiled release binary](https://github.com/uqbar-dao/nectar/releases).
Choose the correct binary for your particular computer architecture and OS.
Extract the `.zip` file and the binary is inside.

Note that some operating systems, particularly Apple, may flag the download as suspicious.
While the binary has not been tested exhaustively on all Linux distributions, it should *just work*.

### Apple

First, attempt to run the binary, which Apple will block.
Then, go to `System Settings > Privacy and Security` and click to `Allow Anyway` for the `nectar` binary:

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

### Acquire NectarOS core

Clone and set up the repository:

```bash
git clone git@github.com:uqbar-dao/nectar.git

cd nectar
mkdir .cargo
echo "net.git-fetch-with-cli = true" > .cargo/config
```

Build the binary:

```bash
cargo +nightly build --release
```

The resulting binary will be at path `target/release/nectar`.

You can also build the binary without the `--release` flag.
This command will build the binary significantly faster, as it does not perform any optimizations during compilation, but it will run much more slowly after compiling.
The non-release binary will be at path `target/debug/nectar`.
