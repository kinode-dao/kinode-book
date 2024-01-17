# `kit run-tests`

`kit run-tests` runs the tests specified by the given `.toml` file, or `tests.toml`, e.g.,

```bash
kit run-tests my_tests.toml
```

or

```bash
kit run-tests
```

to run the current working directory's `tests.toml`.

## Discussion

`kit run-tests` runs a series of tests specified  by [a `.toml` file](#teststoml).
Each test is run in a fresh environment of one or more fake nodes.
A test can setup one or more packages before running a series of test packages.
Each test package is [a single-process package that accepts and responds with certain messages](#test-package-format).

Tests are orchestrated from the outside of the node by `kit run-tests` and run on the inside of the node by the `tester` core package.
For a given test, the `tester` package runs the specified test packages in order.
Each test package must respond to the `tester` package with a `Pass` or `Fail`.
The `tester` package stops on the first `Fail`, or responds with a `Pass` if all tests `Pass`.
If a given test `Pass`es, the next test in the series is run.

## Arguments

```bash
$ kit t --help
Run Nectar tests

Usage: kit run-tests [PATH]

Arguments:
  [PATH]  Path to tests configuration file [default: tests.toml]

Options:
  -h, --help  Print help
```

### Optional positional arg: `PATH`

Path to [`.toml`](https://toml.io/en/) file specifying tests to run; defaults to `tests.toml` in current working directory.

## `tests.toml`

The testing protocol is specified by a `.toml` file.
Consider the following example, from [core tests]():

```toml
runtime = { FetchVersion = "0.5.0" }
runtime_build_verbose = false


[[tests]]

setup_package_paths = ["chat"]
test_packages = [
    { path = "chat_test", "grant_capabilities" = ["chat:chat:nectar"] }
    { path = "key_value_test", grant_capabilities = [] },
    { path = "sqlite_test", grant_capabilities = [] },
]
package_build_verbose = false
timeout_secs = 5
network_router = { port = 9001, defects = "None" }

[[tests.nodes]]

port = 8080
home = "home/first"
fake_node_name = "first.nec"
runtime_verbose = false

[[tests.nodes]]

port = 8081
home = "home/second"
fake_node_name = "second.nec"
runtime_verbose = false


[[tests]]

setup_package_paths = []
test_packages = [
    { path = "key_value_test", grant_capabilities = [] }
]
package_build_verbose = false
timeout_secs = 5
network_router = { port = 9001, defects = "None" }

[[tests.nodes]]

port = 8080
home = "home/first"
fake_node_name = "first.nec"
runtime_verbose = false
```

which has the directory structure

```bash
core_tests
├── chat
│   ├── chat
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   └── pkg
│       ├── manifest.json
│       └── metadata.json
├── chat_test
│   ├── chat_test
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── lib.rs
│   │       └── tester_types.rs
│   └── pkg
│       ├── manifest.json
│       └── metadata.json
├── key_value_test
│   ├── key_value_test
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── key_value_types.rs
│   │       ├── lib.rs
│   │       └── tester_types.rs
│   └── pkg
│       ├── key_value_test.wasm
│       ├── manifest.json
│       └── metadata.json
├── sqlite_test
│   ├── pkg
│   │   ├── manifest.json
│   │   └── metadata.json
│   └── sqlite_test
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           ├── lib.rs
│           ├── sqlite_types.rs
│           └── tester_types.rs
└── tests.toml

```

The top-level consists of three fields:

Key                                               | Value Type
------------------------------------------------- | ----------
[`runtime`](#runtime)                             | `{ FetchVersion = "<version>" }` or `{ RepoPath = "~/path/to/repo" }`
[`runtime_build_verbose`](#runtime_build_verbose) | Boolean
[`tests`](#tests)                                 | Array of Tables

### `runtime`

Specify the runtime to use for the tests.
Two option variants are supported.
An option variant is specified with the key of an Table.

The first, and recommended is `FetchVersion`.
The value of the `FetchVersion` Table is the version number to fetch and use (or `"latest"`).
That version of the runtime binary will be fetched from remote if not found locally.

The second is `RepoPath`.
The value of the `RepoPath` Table is the path to a local copy of the runtime repo.
Given a valid path, that repo will be compiled and used.

### `runtime_build_verbose`

Whether to print `stdout`/`stderr` from building the given repo, if given `RepoPath` `runtime`.

### `tests`

An Array of Tables.
Each Table specifies one test to run.
That test consists of:

Key                     | Value Type      | Value Description
----------------------- | --------------- | -----------------
`setup_package_paths`   | Array of Paths  | Paths to packages to load into all nodes before running test
`test_packages`         | Array of Tables | Table containing `path` (to test package) and `grant_capabilities` (which will be granted by test package)
`package_build_verbose` | Boolean         | Whether to print `stdout`/`stderr` from building the setup & test packages
`timeout_secs`          | Integer > 0     | Timeout for this entire series of test packages
`network_router`        | Table           | Table containing `port` (of network router server) and `defects` (to simulate network weather/defects; currently only `"None"` accepted)
[`nodes`](#nodes)       | Array of Tables | Each Table specifies configuration of one node to spin up for test

Each test package is [a single-process package that accepts and responds with certain messages](#test-package-format).

#### `nodes`

Each test specifies one or more nodes: fake nodes that the tests will be run on.
The first node is the "master" node that will orchestrate the test.
Each node is specified by a Table.
That Table consists of:

Key               | Value Type     | Value Description
----------------- | -------------- | -----------------
`port`            | Integer > 0    | Port to run node on (must not be already bound)
`home`            | Path           | Where to place node's home directory
`fake_node_name`  | String         | Name of fake node
`password`        | String or Null | Password of fake node (default: `"secret"`)
`rpc`             | String or Null | [`wss://` URI of Ethereum RPC](../login.md#starting-the-nectar-node)
`runtime_verbose` | Boolean        | Whether to print `stdout`/`stderr` from the node

## Test package format

A test package is a single-process package that accepts and responds with certain messages.
Those certain messages are:

```json
{
    "Run": {
        "input_node_names": [
            "<master_node_name>",
            "<second_node_name>",
            ...
        ],
        test_timeout: <number>
    }
}
```

which starts the test,

```json
"Pass"
```

which should be sent as a Response after the test has completed successfully, and

```json
{
    "Fail": {
        "test": "test_where_error_occurred",
        "file": "file_where_error_occurred",
        "line": <line_number_where_failure_occurred>,
        "column": <column_number_where_failure_occurred>
    }
}
```

which should be sent as a Response if the test fails.

In the Rust language, there is a helper macro for failutres in `tester_types.rs`.
That file can be found in the core `modules/tester/tester_types.rs`.
The macro is `fail!()`: it automatically sends the Response, filing out the fields, and exits.
