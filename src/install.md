# Installation

Let's get the Uqbar core software to run a live node.

If you just want to get developing as fast as possible, [My First Uqbar Application](./my_first_app/chapter_1.md) is a better place to start.

## Download Binary

The easiest way to get the software is to download a [precompiled release binary](https://github.com/uqbar-dao/uqbar/releases).
There are different binaries depending on architecture and OS.
Extract the `.zip` file and the binary is inside.

### Apple

To run the binary, go to `System Settings > Privacy and Security`, and click to `Allow Anyway` the `uqbar` binary:

![Apple unknown developer](./assets/apple-unknown-developer.png)

## Build From Source

### Acquire Rust and various tools

First, we need to install Rust and some `cargo` tools.
In your terminal, run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-tools
rustup install nightly
rustup target add wasm32-wasi
rustup target add wasm32-wasi --toolchain nightly
cargo install cargo-wasi
```

For more information, or debugging, see the [Rust lang install page](https://www.rust-lang.org/tools/install).

### Acquire Uqbar core

Clone and set up the repository:

```bash
git clone git@github.com:uqbar-dao/uqbar.git

cd uqbar
mkdir .cargo
echo "net.git-fetch-with-cli = true" > .cargo/config
```

Build the binary:

```bash
cargo +nightly build --release
```

The resulting binary will be at path `target/release/uqbar`.

You can also build the binary without the `--release` flag, which will build significantly faster since it does not perform any optimizations during compilation.
The non-release binary will be at path `target/debug/uqbar`.
The downside of eliding the `--release` flag is that the binary will run much more slowly since it is unoptimized.
