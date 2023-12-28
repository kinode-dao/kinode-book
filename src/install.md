# Installation

This section will teach you how to run the Uqbar core software on a live node. However, if you are just interested in starting development as fast as possible, start with [My First Uqbar Application](./my_first_app/chapter_1.md).

## Download Binary

First, get the software itself by downloading a [precompiled release binary](https://github.com/uqbar-dao/uqbar/releases).
Choose the correct binary for your particular computer architecture and OS. 
Extract the `.zip` file and the binary is inside.

Note that some operating systems, particularly Apple, may flag the download as suspicious. While the binary has not been tested exhaustively on all Linux distributions, it should *just work*. 

### Apple

First, attempt to run the binary, which Apple will block. Then, go to `System Settings > Privacy and Security` and click to `Allow Anyway` for the `uqbar` binary:

![Apple unknown developer](./assets/apple-unknown-developer.png)

## Build From Source

### Acquire Rust and various tools

Now, we need to install Rust and some `cargo` tools.
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

You can also build the binary without the `--release` flag. This command will build the binary significantly faster, as it does not perform any optimizations during compilation, but it will run much more slowly after compiling. 
The non-release binary will be at path `target/debug/uqbar`.


