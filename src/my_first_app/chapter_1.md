# My First Uqbar Application

Welcome!
In the following tutorial, we'll walk through setting up an Uqbar development environment.
The finished product at the end of this tutorial will be a package, containing an Uqbar application (or package), composed of one or more processes, and an Uqbar node with that package loaded in and running.

In the following, commands to input to the terminal are provided as-is for ease of copying EXCEPT when the output of the command is also shown.
In that case, the command is prepended with a `$ ` to distinguish the command from the output.
The `$ ` should not be copied into the terminal.

## Chapter 1: Setting up the development environment

The following assumes a Unix environment -- macOS or Linux.
If on Windows, [get WSL](https://learn.microsoft.com/en-us/windows/wsl/install) first.
In general, Uqbar does not support Windows.

### Quickstart

Setup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-tools
rustup install nightly
rustup target add wasm32-wasi
rustup target add wasm32-wasi --toolchain nightly
cargo install cargo-wasi
cargo install --git https://github.com/uqbar-dao/uqdev
```

After setup, in one terminal, start up a fake node:

```bash
uqdev boot-fake-node
```

In another terminal:

```bash
uqdev new my_chat_app -p my_chat_app
cd my_chat_app
uqdev build
uqdev start-package -u http://locahost:8080  # Or whatever port fake node bound
```

Congratulations on building and deploying a chat app to a fake Uqbar node!

### Acquiring Rust

To install Rust, run

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For more information, or debugging, see the [Rust lang install page](https://www.rust-lang.org/tools/install).

### Optional: Acquiring the Uqbar node runner

The following is optional, because development can also occur on a fake node, covered [below](#booting-a-fake-uqbar-node).
To develop on a real Uqbar node, connected to the real network, follow the instructions to [setup an Uqbar node](https://github.com/uqbar-dao/uqbar?tab=readme-ov-file#setup).

### Acquiring Uqbar Development Tools: `uqdev`

Install the Uqbar Development Tools, or `uqdev`, using `cargo`:

```bash
cargo install --git https://github.com/uqbar-dao/uqdev
```

### Creating a new Uqbar package template

The `uqdev` toolkit has a variety of features.
One of those features is `new`, which creates a template for an Uqbar package.
The `new` tool takes two arguments, a path to create the template directory at, and a name for the package:

```bash
$ uqdev new --help
Create an Uqbar template package

Usage: uqdev new --package <package-name> <directory>

Arguments:
  <directory>  Path to create template directory at

Options:
  -p, --package <package-name>  Name of the package
  -h, --help                    Print help
```

Create a package `my_chat_app`:

```bash
uqdev new my_chat_app -p my_chat_app
```

### Exploring the package

Uqbar packages come in one of two structures.
The `uqdev new` command creates the simpler of the two: a single process.
The template contains:

```bash
$ ls my_chat_app
Cargo.toml  pkg/  src/
```

The `Cargo.toml` file is standard for Rust projects: it specifies dependencies.
It is exhaustively defined [here](https://doc.rust-lang.org/cargo/reference/manifest.html).

The `src/` directory is where the code for the process lives.

The `pkg/` directory contains two files, `manifest.json` and `metadata.json`, that specify information the Uqbar node needs to run the package (more information below).
The `pkg/` directory is also where `.wasm` binaries will be deposited by [`uqbar build`](#building-the-package).
The files in the `pkg/` directory are finally injected into the Uqbar node with [`uqbar start-package`](#starting-the-package).

#### `pkg/manifest.json`

The `manifest.json` file contains information the Uqbar node needs in order to run the package:

```bash
$ cat my_chat_app/pkg/manifest.json 
[
    {
        "process_name": "my_chat_app",
        "process_wasm_path": "/my_chat_app.wasm",
        "on_panic": "Restart",
        "request_networking": true,
        "request_messaging": [
            "net:sys:uqbar"
        ],
        "grant_messaging": [],
        "public": true
    }
]
```

It is a json array of json objects.
Each object represents one process that will be started when the package is installed.
A package with multiple processes need not start them all at install time.
A package may start more than one of the same process, as long as they have unique `process_name`s.
Each object has the following fields:

Key                    | Required? | Value type
---------------------- | --------- | ----------
`"process_name"`       | Yes       | string
`"process_wasm_path"`  | Yes       | string (representing a path)
`"on_panic"`           | Yes       | string (`"None"` or `"Restart"`) or object (covered elsewhere)
`"request_networking"` | Yes       | bool
`"request_messaging"`  | No        | array of strings
`"grant_networking"`   | No        | array of strings
`"public"`             | Yes       | bool

#### `pkg/metadata.json`

The `metadata.json` file contains information about the package and the publisher:

```bash
$ cat my_chat_app/pkg/metadata.json 
{
    "package": "my_chat_app",
    "publisher": "template.uq",
    "version": [0, 1, 0]
}
```

Here, the `publisher` is some default value, but for a real package, this field should be filled out with the QNS id of the publishing node.

#### `src/lib.rs`

TODO: Leaving this blank for now because I'm not sure the chat app is going to remain as the default template. Happy to fill this in; just lmk.

### Building the package

To build the package, use the `uqdev build` tool.
It accepts an optional directory path as the first argument, or, if none is provided, attempts to build the current working directory. As such, either of the following will work:

```bash
uqdev build my_chat_app
```

or

```bash
cd my_chat_app
uqdev build
```

### Booting a fake Uqbar node

Boot a fake Uqbar node for developmenmt purposes using the `uqdev boot-fake-node` tool.
`uqdev boot-fake-node` downloads the OS- and architecture-appropriate Uqbar core binary and runs it without connecting to the live network.
Instead, it is connects to a mocked local network, so that different fake nodes on the same machine can communicate with each other.
`uqdev boot-fake-node` has many optional configuration flags, but the defaults should work fine:

```bash
uqdev boot-fake-node
```

The fake node, just like a real node, will accept inputs from the terminal.
To exit from the fake node, press `Ctrl + C`.

By default, the fake node will bind to port `8080`.
Note the port number in the output for [later](#starting-the-package); it will look something like:

```bash
Fri 12/8 15:43 http_server: running on port 8080
```

### Option: Starting a real Uqbar node

Alternatively, start a real Uqbar node.
Instructions can be found [here](https://github.com/uqbar-dao/uqbar?tab=readme-ov-file#boot).

### Starting the package

To load in the `my_chat_app` package and start it, use the `uqdev start-package` tool.
The `uqdev start-package` tool takes an optional directory containing the package (or tries the current working directory, as in [`uqdev build`, above](#building-the-package)) and a required url: the node to start the package on.
The node's url follows a `-u` or `--url` flag.

If not in the directory, use:

```bash
uqdev start-package my_chat_app -u http://localhost:8080
```

or, if in the package directory:

```bash
uqdev start-package -u http://localhost:8080
```

where here the port provided in the url following `-u` must match the port bound by the node or fake node (see discussion [above](#booting-a-fake-uqbar-node)).

The node's terminal should display something like

```bash
Fri 12/8 15:54 my_chat_app: begin
```

Congratulations on completing the first steps towards developing applications on Uqbar!

### Using the package

TODO: again dependent on whether we want to use `chat`; if yes, I can put here how to spin up a second node & chat between them
