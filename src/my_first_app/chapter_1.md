# My First Uqbar Application

Welcome!
In these tutorials, you'll setup your development environment and learn about the `uqdev` tools.
You'll learn about templates and also walk through writing an application from the group up, backend and frontend.
And finally, you'll learn how to deploy applications through the Uqbar app store.

For the purposes of this documentation, terminal commands are provided as-is for ease of copying EXCEPT when the output of the command is also shown.
In that case, the command is prepended with a `$ ` to distinguish the command from the output.
The `$ ` should not be copied into the terminal.

# Chapter 1: Setting Up the Development Environment

In this chapter, you'll walk through setting up an Uqbar development environment.
By the end, you will have created an Uqbar application, or package, composed of one or more processes that run on a live Uqbar node.
The application will be a simple chat interface: `my_chat_app`.

The following assumes a Unix environment — macOS or Linux.
If on Windows, [get WSL](https://learn.microsoft.com/en-us/windows/wsl/install) first.
In general, Uqbar does not support Windows.

## Acquiring Rust and the Uqbar Development Tools (`uqdev`)

Install Rust and the Uqbar Development Tools, or `uqdev`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --git https://github.com/uqbar-dao/uqdev
```

## Creating a New Uqbar Package Template

The `uqdev` toolkit has a [variety of features](https://github.com/uqbar-dao/uqdev).
One of those tools is `new`, which creates a template for an Uqbar package.
The `new` tool takes two arguments: a path to create the template directory and a name for the package:

```
$ uqdev new --help
Create an Uqbar template package

Usage: uqdev new [OPTIONS] <DIR>

Arguments:
  <DIR>  Path to create template directory at

Options:
  -a, --package <PACKAGE>      Name of the package [default: DIR]
  -u, --publisher <PUBLISHER>  Name of the publisher [default: template.uq]
  -l, --language <LANGUAGE>    Programming language of the template [default: rust] [possible values: rust, python, javascript]
  -t, --template <TEMPLATE>    Template to create [default: chat] [possible values: chat, fibonacci]
      --ui                     If set, use the template with UI
  -h, --help                   Print help
```

Create a package `my_chat_app`:

```bash
uqdev new my_chat_app
```

## Exploring the Package

Uqbar packages are sets of one or more Uqbar [processes](../processes.md).
An Uqbar package is represented in Unix as a directory that has a `pkg/` directory within.
Each process within the package is its own directory.
By default, the `uqdev new` command creates a simple, one-process package, a chat app.
Other templates, including a Python template and a UI-enabled template can be used by passing different flags to `uqdev new` (see `uqdev new --help`).
The default template looks like:

```bash
$ tree my_chat_app
my_chat_app
├── my_chat_app
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── pkg
    ├── manifest.json
    └── metadata.json

3 directories, 4 files
```

The `my_chat_app/` package here contains one process, also named `my_chat_app/`.
The process directory contains source files and other metadata for compiling that process.

Here, the `Cargo.toml` file is standard for Rust projects: it specifies dependencies.
It is exhaustively defined [here](https://doc.rust-lang.org/cargo/reference/manifest.html).
The `src/` directory is where the code for the process lives.

Also within the package directory is a `pkg/` directory.
The `pkg/` directory contains two files, `manifest.json` and `metadata.json`, that specify information the Uqbar node needs to run the package, which will be enumerated below.
The `pkg/` directory is also where `.wasm` binaries will be deposited by [`uqbar build`](#building-the-package).
The files in the `pkg/` directory contents are injected into the Uqbar node with [`uqbar start-package`](#starting-the-package).

Though not included here, packages with a frontend have a `ui/` directory as well.
For an example, look at the result of:
```bash
uqdev new my_chat_app_with_ui --ui
tree my_chat_app_with_ui
```
Note that not all templates have a UI-enabled version.
As of 230104, only the Rust chat template has a UI-enabled version.

### `pkg/manifest.json`

The `manifest.json` file contains information the Uqbar node needs in order to run the package:

```bash
$ cat my_chat_app/pkg/manifest.json
[
    {
        "process_name": "my_chat_app",
        "process_wasm_path": "/my_chat_app.wasm",
        "on_exit": "Restart",
        "request_networking": true,
        "request_messaging": [
            "net:sys:uqbar"
        ],
        "grant_messaging": [],
        "public": true
    }
]
```

This is a json array of json objects.
Each object represents one process that will be started when the package is installed.
A package with multiple processes need not start them all at install time.
A package may start more than one of the same process, as long as they each have a unique `process_name`.

Each object has the following fields:

Key                    | Required? | Value type
---------------------- | --------- | ----------
`"process_name"`       | Yes       | string
`"process_wasm_path"`  | Yes       | string (representing a path)
`"on_exit"`            | Yes       | string (`"None"` or `"Restart"`) or object (covered elsewhere)
`"request_networking"` | Yes       | bool
`"request_messaging"`  | No        | array of strings to note process names, or objects to note custom capabilities and from what process to request them
`"grant_messaging"`    | No        | array of strings to note process names, or objects to note custom capabilities to generate and send to a process
`"public"`             | Yes       | bool

### `pkg/metadata.json`

The `metadata.json` file contains information about the package and the publisher:

```bash
$ cat my_chat_app/pkg/metadata.json
{
    "package": "my_chat_app",
    "publisher": "template.uq",
    "version": [0, 1, 0]
}
```

Here, the `publisher` is some default value, but for a real package, this field should contain the QNS id of the publishing node.
The `publisher` can also be set with a `uqdev new --publisher` flag.

### `src/lib.rs`

TODO

## Building the Package

To build the package, use the `uqdev build` tool.

This tool accepts an optional directory path as the first argument, or, if none is provided, attempts to build the current working directory.
As such, either of the following will work:

```bash
uqdev build my_chat_app
```

or

```bash
cd my_chat_app
uqdev build
```

## Booting a Fake Uqbar Node

Often, it is optimal to develop on a fake node.
Fake nodes are simple to set up, easy to restart if broken, and mocked networking makes development testing very straightforward.
To boot a fake Uqbar node for development purposes, use the `uqdev boot-fake-node` tool.

`uqdev boot-fake-node` downloads the OS- and architecture-appropriate Uqbar core binary and runs it without connecting to the live network.
Instead, it connects to a mocked local network, allowing different fake nodes on the same machine to communicate with each other.
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

`uqdev boot-fake-node` also accepts a `--runtime-path` argument.
When supplied, if it is a path to the Uqbar core repo, it will compile and use that binary to start the node.
Or, if it is a path to an Uqbar binary, it will use that binary to start the node.
For example:

```bash
uqdev boot-fake-node --runtime-path ~/path/to/uqbar
```

where `~/path/to/uqbar` must be replaced with a path to the Uqbar core repo or an Uqbar binary.

## Option: Starting a Real Uqbar Node

Alternatively, development sometimes calls for a real node, which has access to the actual Uqbar network and its providers, such as integrated LLMs.

To develop on a real Uqbar node, connect to the network and follow the instructions to [setup an Uqbar node](../install.md).

## Starting the Package

Time to load and initiate the `my_chat_app` package. For this, you will use the `uqdev start-package` tool.
Like [`uqdev build`](#building-the-package), the `uqdev start-package` tool receives an optional directory containing the package or, if no directory is received, tries the current working directory.
It also requires a URL: the address of the node on which to initiate the package.
The node's URL can be input in one of two ways:
1. If running on localhost, the port can be supplied with `-p` or `--port`,
2. More generally, the node's entire URL can be supplied with a `-u` or `--url` flag.

You can start the package from either within or outside `my_chat_app` directory.
After completing the previous step, you should be one directory above the `my_chat_app` directory and can use the following:

```bash
uqdev start-package my_chat_app -p 8080
```

or, if you are already in the correct package directory:

```bash
uqdev start-package -p 8080
```

where here the port provided following `-p` must match the port bound by the node or fake node (see discussion [above](#booting-a-fake-uqbar-node)).

The node's terminal should display something like

```
Fri 12/8 15:54 my_chat_app: begin
```

Congratulations on completing the first steps towards developing applications on Uqbar!

## Using the Package

To test out the functionality of `my_chat_app`, spin up another fake node to chat with in a new terminal:

```bash
uqdev boot-fake-node -h /tmp/uqbar-fake-node-2 -p 8081 -f fake2.uq
```

The fake nodes communicate over a mocked local network.

To start the same `my_chat_app` on the second fake node, again note the port, and supply it with a `start-package`:

```bash
uqdev start-package my_chat_app -p 8081
```

or, if already in the `my_chat_app/` package directory:

```bash
uqdev start-package -p 8081
```

To send a chat message from the first node, run the following in its terminal:

```
/m our@my_chat_app:my_chat_app:template.uq {"Send": {"target": "fake2.uq", "message": "hello world"}}
```

and replying:

```
/m our@my_chat_app:my_chat_app:template.uq {"Send": {"target": "fake.uq", "message": "wow, it works!"}}
```
