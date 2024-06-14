# `kit run-tests`

`kit run-tests` runs the tests specified by the given `.toml` file, or `tests.toml`, e.g.,

```
kit run-tests my_tests.toml
```

or

```
kit run-tests
```

to run the current working directory's `tests.toml`.

## Discussion

`kit run-tests` runs a series of tests specified  by [a `.toml` file](#teststoml).
Each test is run in a fresh environment of one or more fake nodes.
A test can setup one or more packages before running a series of test packages.
Each test package is [a single-process package that accepts and responds with certain messages](#test-package-interface).

Tests are orchestrated from the outside of the node by `kit run-tests` and run on the inside of the node by the [`tester`](https://github.com/kinode-dao/kinode/tree/main/kinode/packages/tester) core package.
For a given test, the `tester` package runs the specified test packages in order.
Each test package must respond to the `tester` package with a `Pass` or `Fail`.
The `tester` package stops on the first `Fail`, or responds with a `Pass` if all tests `Pass`.
If a given test `Pass`es, the next test in the series is run.

## Arguments

```
$ kit run-tests --help
Run Kinode tests

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
[`tests.toml`](https://github.com/kinode-dao/core_tests/blob/master/tests.toml), from [core tests](https://github.com/kinode-dao/core_tests), will be used as an example.

The top-level of `tests.toml` consists of three fields:

Key                                               | Value Type
------------------------------------------------- | ----------
[`runtime`](#runtime)                             | `{ FetchVersion = "<version>" }` or `{ RepoPath = "~/path/to/repo" }`
[`runtime_build_release`](#runtime_build_release) | Boolean
[`tests`](#tests)                                 | [Array of Tables](https://toml.io/en/v1.0.0#array-of-tables)

### `runtime`

Specify the runtime to use for the tests.
Two option variants are supported.
An option variant is specified with the key (e.g. `FetchVersion`) of a `toml` [Table](https://toml.io/en/v1.0.0#table) (e.g. `{FetchVersion = "0.7.2"}`).

The first, and recommended is `FetchVersion`.
The value of the `FetchVersion` Table is the version number to fetch and use (or `"latest"`).
That version of the runtime binary will be fetched from remote if not found locally.

The second is `RepoPath`.
The value of the `RepoPath` Table is the path to a local copy of the runtime repo.
Given a valid path, that repo will be compiled and used.

For example:

```toml
runtime = { FetchVersion = "latest" }
```


### `runtime_build_release`

If given `runtime = RepoPath`, `runtime_build_release` decides whether to build the runtime as `--release` or not.

For example:

```toml
runtime_build_release = true
```


### `tests`

An Array of Tables.
Each Table specifies one test to run.
That test consists of:

Key                     | Value Type      | Value Description
----------------------- | --------------- | -----------------
`setup_package_paths`   | Array of Paths  | Paths to packages to load into all nodes before running test
`test_packages`         | Array of Tables | Each Table in the Array contains `path` (to test package) and `grant_capabilities` (which will be granted by test package)
`timeout_secs`          | Integer > 0     | Timeout for this entire series of test packages
`network_router`        | Table           | Table containing `port` (of network router server) and `defects` (to simulate network weather/defects; currently only `"None"` accepted)
[`nodes`](#nodes)       | Array of Tables | Each Table specifies configuration of one node to spin up for test

Each test package is [a single-process package that accepts and responds with certain messages](#test-package-interface).

For example:
```toml
[[tests]]
setup_package_paths = ["chat"]
test_packages = [
    { path = "chat_test", grant_capabilities = ["chat:chat:template.os"] },
    { path = "key_value_test", grant_capabilities = ["kv:distro:sys"] },
    { path = "sqlite_test", grant_capabilities = ["sqlite:distro:sys"] },
]
timeout_secs = 5
# Plan to include defects = Latency, Dropping, ..., All
network_router = { port = 9001, defects = "None" }

[[tests.nodes]]
...

[[tests.nodes]]
...
```


#### `nodes`

Each test specifies one or more nodes: fake nodes that the tests will be run on.
The first node is the "master" node that will orchestrate the test.
Each node is specified by a Table.
That Table consists of:

Key                 | Value Type     | Value Description
------------------- | -------------- | -----------------
`port`              | Integer > 0    | Port to run node on (must not be already bound)
`home`              | Path           | Where to place node's home directory
`fake_node_name`    | String         | Name of fake node
`password`          | String or Null | Password of fake node (default: `"secret"`)
`rpc`               | String or Null | [`wss://` URI of Ethereum RPC](../login.md#starting-the-kinode-node)
`runtime_verbosity` | Integer >= 0   | The verbosity level to start the runtime with; higher is more verbose (default: `0`)

For example:

```toml
[[tests.nodes]]
port = 8080
home = "home/first"
fake_node_name = "first.os"
runtime_verbosity = 0

[[tests.nodes]]
port = 8081
home = "home/second"
fake_node_name = "second.os"
runtime_verbosity = 0
```


## Test Package Interface

A test package is a single-process package that accepts and responds with certain messages.
The interface is defined as:

```wit
interface tester {
    variant request {
        run(run-request),
    }

    variant response {
        run(result<_, fail-response>)
    }

    record run-request {
        input-node-names: list<string>,
        test-names: list<string>,
        test-timeout: u64,
    }

    record fail-response {
        test: string,
        file: string,
        line: u32,
        column: u32,
    }
}

world tester-sys-v0 {
    import tester;
    include process-v0;
}
```

A `run` `request` starts the test.
A `run` `response` marks the end of a test, and is either an `Ok` Result, indicating success, or a `Err` Result with information as to where the error occurred.

In the Rust language, a helper macro for failures can be found in [`tester_types.rs`](https://github.com/kinode-dao/kinode/blob/main/kinode/packages/tester/tester_types.rs).
The macro is `fail!()`: it automatically sends the Response as specified above, filing out the fields, and exits.
