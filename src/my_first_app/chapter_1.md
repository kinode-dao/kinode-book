# Environment Setup

In this section, you'll walk through setting up a Kinode development environment.
By the end, you will have created a Kinode application, or package, composed of one or more processes that run on a live Kinode.
The application will be a simple chat interface: `my-chat-app`.

The following assumes a Unix environment — macOS or Linux.
If on Windows, [get WSL](https://learn.microsoft.com/en-us/windows/wsl/install) first.
In general, Kinode does not support Windows.

## Acquiring Rust and the Kinode Development Tools (`kit`)

Install Rust and the Kinode Development Tools, or `kit`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --git https://github.com/kinode-dao/kit --locked
```

You can find a video guide that walks through setting up `kit` [here](https://www.youtube.com/watch?v=N8B_s_cm61k).

## Creating a New Kinode Package Template

The `kit` toolkit has a [variety of features](../kit/kit-dev-toolkit.md).
One of those tools is `new`, which creates a template for a Kinode package.
The `new` tool takes two arguments: a path to create the template directory and a name for the package:

```
$ kit new --help
Create a Kinode template package

Usage: kit new [OPTIONS] <DIR>

Arguments:
  <DIR>  Path to create template directory at (must contain only a-z, 0-9, `-`)

Options:
  -a, --package <PACKAGE>      Name of the package (must contain only a-z, 0-9, `-`) [default: DIR]
  -u, --publisher <PUBLISHER>  Name of the publisher (must contain only a-z, 0-9, `-`, `.`) [default: template.os]
  -l, --language <LANGUAGE>    Programming language of the template [default: rust] [possible values: rust]
  -t, --template <TEMPLATE>    Template to create [default: chat] [possible values: blank, chat, echo, fibonacci, file-transfer]
      --ui                     If set, use the template with UI
  -h, --help                   Print help
```

Create a package `my-chat-app` (you can name it anything "Kimap-safe", i.e. containing only a-z, 0-9, `-`; but we'll assume you're working with `my-chat-app` in this document):

```bash
kit new my-chat-app
```

## Exploring the Package

Kinode packages are sets of one or more Kinode [processes](../system/process/processes.md).
A Kinode package is represented in Unix as a directory that has a `pkg/` directory within.
Each process within the package is its own directory.
By default, the `kit new` command creates a simple, one-process package, a chat app.
Other templates, including a Python template and a UI-enabled template can be used by passing [different flags to `kit new`](../kit/new.html#discussion).
The default template looks like:

```
$ tree my-chat-app
my-chat-app
├── api
│   └── my-chat-app:template.os-v0.wit
├── Cargo.toml
├── metadata.json
├── my-chat-app
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── pkg
│   ├── manifest.json
│   └── scripts.json
├── send
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── test
    ├── my-chat-app-test
    │   ├── api
    │   │   └── my-chat-app-test:template.os-v0.wit
    │   ├── Cargo.toml
    │   ├── metadata.json
    │   ├── my-chat-app-test
    │   │   ├── Cargo.toml
    │   │   └── src
    │   │       ├── lib.rs
    │   │       └── tester_lib.rs
    │   └── pkg
    │       └── manifest.json
    └── tests.toml
```

The `my-chat-app/` package here contains two processes:
- `my-chat-app/` — containing the main application code, and
- `send/` — containing a [script](../cookbook/writing_scripts.html).

Rust process directories, like the ones here, contain:
- `src/` — source files where the code for the process lives, and
- `Cargo.toml` — the standard Rust file specifying dependencies, etc., for that process.

Another standard Rust `Cargo.toml` file, a [virtual manifest](https://doc.rust-lang.org/cargo/reference/workspaces.html#virtual-workspace) is also included in `my-chat-app/` root.

Also within the package directory is a `pkg/` directory.
The `pkg/` dirctory contains two files:
- `manifest.json` — required: specifes information the Kinode needs to run the package, and
- `scripts.json` — optional: specifies details needed to run [scripts](../cookbook/writing_scripts.html).

The `pkg/` directory is also where `.wasm` binaries will be deposited by [`kit build`](#building-the-package).
The files in the `pkg/` directory are injected into the Kinode with [`kit start-package`](#starting-the-package).

The `metadata.json` is a required file that contains app metadata which is used in the Kinode [App Store](./chapter_5.html).

The `api/` directory contains the [WIT API](../system/process/wit_apis.md) for the `my-chat-app` package, see more discussion [below](#api).

Lastly, the `test/` directory contains tests for the `my-chat-app` package.
The `tests.toml` file specifies the configuration of the tests.
The `my-chat-app-test/` direcotry is itself a package: the test for `my-chat-app`.
For more discussion of tests see [`kit run-tests`](../kit/run-tests.md), or see usage, [below](#testing-the-package).

Though not included in this template, packages with a frontend have a `ui/` directory as well.
For an example, look at the result of:
```bash
kit new my-chat-app-with-ui --ui
tree my-chat-app-with-ui
```
Note that not all templates have a UI-enabled version.
More details about templates can be found [here](../kit/new.html#existshas-ui-enabled-version).

### `pkg/manifest.json`

The `manifest.json` file contains information the Kinode needs in order to run the package:

```bash
$ cat my-chat-app/pkg/manifest.json
[
    {
        "process_name": "my-chat-app",
        "process_wasm_path": "/my-chat-app.wasm",
        "on_exit": "Restart",
        "request_networking": true,
        "request_capabilities": [
            "http-server:distro:sys",
            "vfs:distro:sys"
        ],
        "grant_capabilities": [],
        "public": true
    }
]
```

This is a JSON array of JSON objects.
Each object represents one process that will be started when the package is installed.
A package with multiple processes need not start them all at install time.
A package may start more than one of the same process, as long as they each have a unique `process_name`.

Each object requires the following fields:

Key                      | Value Type                                                                                     | Description
------------------------ | ---------------------------------------------------------------------------------------------- | -----------
`"process_name"`         | String                                                                                         | The name of the process
`"process_wasm_path"`    | String                                                                                         | The path to the process
`"on_exit"`              | String (`"None"` or `"Restart"`) or Object (covered [elsewhere](./chapter_2.md#aside-on_exit)) | What to do in case the process exits
`"request_networking"`   | Boolean                                                                                        | Whether to ask for networking capabilities from kernel
`"request_capabilities"` | Array of Strings or Objects                                                                    | Strings are `ProcessId`s to request messaging capabilties from; Objects have a `"process"` field (`ProcessId` to request from) and a `"params"` field (capability to request)
`"grant_capabilities"`   | Array of Strings or Objects                                                                    | Strings are `ProcessId`s to grant messaging capabilties to; Objects have a `"process"` field (`ProcessId` to grant to) and a `"params"` field (capability to grant)
`"public"`               | Boolean                                                                                        | Whether to allow any process to message us

### `metadata.json`

The `metadata.json` file contains ERC721 compatible metadata about the package.
The only required fields are `package_name`, `current_version`, and `publisher`, which are filled in with default values:

```bash
$ cat my-chat-app/metadata.json
{
    "name": "my-chat-app",
    "description": "",
    "image": "",
    "properties": {
        "package_name": "my-chat-app",
        "current_version": "0.1.0",
        "publisher": "template.os",
        "mirrors": [],
        "code_hashes": {
            "0.1.0": ""
        },
        "wit_version": 1,
        "dependencies": []
    },
    "external_url": "",
    "animation_url": ""
}
```
Here, the `publisher` is the default value (`"template.os"`), but for a real package, this field should contain the KNS ID of the publishing node.
The `publisher` can also be set with a `kit new --publisher` flag.
The `wit_version` is an optional field:

`wit_version` value | Resulting `kinode.wit` version
------------------- | ------------------------------
elided              | [`kinode.wit` `0.7.0`](https://github.com/kinode-dao/kinode-wit/blob/aa2c8b11c9171b949d1991c32f58591c0e881f85/kinode.wit)
`0`                 | [`kinode.wit` `0.8.0`](https://github.com/kinode-dao/kinode-wit/blob/758fac1fb144f89c2a486778c62cbea2fb5840ac/kinode.wit)
`1`                 | [`kinode.wit` `1.0.0`](https://github.com/kinode-dao/kinode-wit/blob/v1.0.0/kinode.wit)

The `dependencies` field is also optional; see discussion in [WIT APIs](../system/process/wit_apis.md).
The rest of these fields are not required for development, but become important when publishing a package with the [`app-store`](https://github.com/kinode-dao/kinode/tree/main/kinode/packages/app-store).

As an aside: each process has a unique `ProcessId`, used to address messages to that process, that looks like

```
<process-name>:<package-name>:<publisher-node>
```

Each field separated by `:`s must be "Kimap safe", i.e. can only contain a-z, 0-9, `-` (and, for publisher node, `.`).

You can read more about `ProcessId`s [here](../system/process/processes.md#overview).

### `api/`

The `api/` directory is an optional directory where packages can declare their public API.
Other packages can then mark a package as a dependency in their `metadata.json` to include those types and functions defined therein.
The API is useful for composability and for LLM agents as definitions of "tools" for programatic access.

For further reading, see discussion in [WIT APIs](../system/process/wit_apis.md), [the package APIs recipe](../cookbook/package_apis.md), [the package APIs (with workers) recipe](../cookbook/package_apis_workers.md), and [`kit view-api`](../kit/view-api.md).

## Building the Package

To build the package, use the [`kit build`](../kit/build.md#) tool.

This tool accepts an optional directory path as the first argument, or, if none is provided, attempts to build the current working directory.
As such, either of the following will work:

```bash
kit build my-chat-app
```

or

```bash
cd my-chat-app
kit build
```

## Booting a Fake Kinode

Often, it is optimal to develop on a fake node.
Fake nodes are simple to set up, easy to restart if broken, and mocked networking makes development testing very straightforward.
To boot a fake Kinode for development purposes, use the [`kit boot-fake-node` tool](../kit/boot-fake-node.md).

`kit boot-fake-node` downloads the OS- and architecture-appropriate Kinode core binary and runs it without connecting to the live network.
Instead, it connects to a mocked local network, allowing different fake nodes on the same machine to communicate with each other.
`kit boot-fake-node` has many optional configuration flags, but the defaults should work fine:

```bash
kit boot-fake-node
```

The fake node, just like a real node, will accept inputs from the terminal.
To exit from the fake node, press `Ctrl + C`.

By default, the fake node will bind to port `8080`.
Note the port number in the output for [later](#starting-the-package); it will look something like:

```bash
Thu 22:50 http-server: running on port 8080
```

`kit boot-fake-node` also accepts a `--runtime-path` argument.
When supplied, if it is a path to the Kinode core repo, it will compile and use that binary to start the node.
Or, if it is a path to a Kinode binary, it will use that binary to start the node.
For example:

```bash
kit boot-fake-node --runtime-path ~/path/to/kinode
```

where `~/path/to/kinode` must be replaced with a path to the Kinode core repo.

Note that your node will be named `fake.dev`, as opposed to `fake.os`.
The `.dev` suffix is used for development nodes.

## Optional: Starting a Real Kinode

Alternatively, development sometimes calls for a real node, which has access to the actual Kinode network and its providers.

To develop on a real Kinode, connect to the network and follow the instructions to [setup a Kinode](../getting_started/install.md).

## Starting the Package

Now it is time to load and initiate the `my-chat-app` package. For this, you will use the [`kit start-package`](../kit/start-package.md) tool.
Like [`kit build`](#building-the-package), the `kit start-package` tool takes an optional directory argument — the package — defaulting to the current working directory.
It also accepts a URL: the address of the node on which to initiate the package.
The node's URL can be input in one of two ways:

1. If running on localhost, the port can be supplied with `-p` or `--port`,
2. More generally, the node's entire URL can be supplied with a `-u` or `--url` flag.

If neither the `--port` nor the `--url` is given, `kit start-package` defaults to `http://localhost:8080`.

You can start the package from either within or outside `my-chat-app` directory.
After completing the previous step, you should be one directory above the `my-chat-app` directory and can use the following:

```bash
kit start-package my-chat-app -p 8080
```

or, if you are already in the correct package directory:

```bash
kit start-package -p 8080
```

where here the port provided following `-p` must match the port bound by the node or fake node (see discussion [above](#booting-a-fake-kinode)).

The node's terminal should display something like

```
Thu 22:51 app-store:sys: successfully installed my-chat-app:template.os
```

Congratulations: you've now built and installed your first application on Kinode!

## Using the Package

To test out the functionality of `my-chat-app`, spin up another fake node to chat with in a new terminal:

```bash
kit boot-fake-node -o /tmp/kinode-fake-node-2 -p 8081 -f fake2.dev
```

The fake nodes communicate over a mocked local network.

To start the same `my-chat-app` on the second fake node, again note the port, and supply it with a `start-package`:

```bash
kit start-package my-chat-app -p 8081
```

or, if already in the `my-chat-app/` package directory:

```bash
kit start-package -p 8081
```

To send a chat message from the first node, run the following in its terminal:

```
m our@my-chat-app:my-chat-app:template.os '{"Send": {"target": "fake2.dev", "message": "hello world"}}'
```

and replying, from the other terminal:

```
m our@my-chat-app:my-chat-app:template.os '{"Send": {"target": "fake.dev", "message": "wow, it works!"}}'
```

Messages can also be injected from the outside.
From a bash terminal, use `kit inject-message`, like so:

```bash
kit inject-message my-chat-app:my-chat-app:template.os '{"Send": {"target": "fake2.dev", "message": "hello from the outside world"}}'
kit inject-message my-chat-app:my-chat-app:template.os '{"Send": {"target": "fake.dev", "message": "replying from fake2.dev using first method..."}}' --node fake2.dev
kit inject-message my-chat-app:my-chat-app:template.os '{"Send": {"target": "fake.dev", "message": "and second!"}}' -p 8081
```

## Testing the Package

To run the `my-chat-app/` tests, *first close all fake nodes* and then run

```bash
kit run-tests my-chat-app
```

or, if already in the `my-chat-app/` package directory:

```bash
kit run-tests
```

For more details, see [`kit run-tests`](../kit/run-tests.md).
