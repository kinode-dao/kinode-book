# `kit run-tests`

short: `kit t`

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
[`tests.toml`](https://github.com/kinode-dao/core_tests/blob/master/tests.toml), from [core tests](https://github.com/kinode-dao/core_tests), will be used as an example:
```toml
{{#webinclude https://raw.githubusercontent.com/kinode-dao/core_tests/master/tests.toml}}
```

The top-level of `tests.toml` consists of four fields:

Key                                               | Value Type
------------------------------------------------- | ----------
[`runtime`](#runtime)                             | `{ FetchVersion = "<version>" }` or `{ RepoPath = "~/path/to/repo" }`
[`runtime_build_release`](#runtime_build_release) | Boolean
[`persist_home`](#persist_home)                   | Boolean
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
{{#webinclude https://raw.githubusercontent.com/kinode-dao/core_tests/master/tests.toml 1}}
```

### `runtime_build_release`

If given `runtime = RepoPath`, `runtime_build_release` decides whether to build the runtime as `--release` or not.

For example:

```toml
{{#webinclude https://raw.githubusercontent.com/kinode-dao/core_tests/master/tests.toml 3}}
```

### `persist_home`

Whether or not to persist the node home directories after tests have been run.
It is recommended to have this set to `false` except when debugging a test.

### `tests`

An Array of Tables.
Each Table specifies one test to run.
That test consists of:

Key                     | Value Type               | Value Description
----------------------- | ------------------------ | -----------------
`setup_packages`        | Array of Tables [(`SetupPackage`s)](https://github.com/kinode-dao/kit/blob/10e2bd5d44cf44690c2360e60523ac5b06d1d5f0/src/run_tests/types.rs#L37-L40) | Each Table in the Array contains `path` (to the package) and `run` (whether or not to run the package or merely load it in)
`setup_scripts`         | Array of Tables [(`Script`s)](https://github.com/kinode-dao/kit/blob/10e2bd5d44cf44690c2360e60523ac5b06d1d5f0/src/run_tests/types.rs#L43-L46)       | Each Table in the Array contains `path` (to the script) and `args` (to be passed to the script); these scripts will run alongside the test nodes
`test_package_paths`    | Array of Strings (`PathBuf`s)                                                                                                                       | Paths to test packages to run
`test_scripts`          | Array of [`Script`s](https://github.com/kinode-dao/kit/blob/10e2bd5d44cf44690c2360e60523ac5b06d1d5f0/src/run_tests/types.rs#L43-L46)                | Each Table in the Array contains `path` (to the script) and `args` (to be passed to the script); these scripts will be run as tests and must return a `0` on success
`timeout_secs`          | Integer > 0                                                                                                                                         | Timeout for this entire series of test packages
`network_router`        | Table                                                                                                                                               | Table containing `port` (of network router server) and `defects` (to simulate network weather/defects; currently only `"None"` accepted)
[`nodes`](#nodes)       | Array of Tables                                                                                                                                     | Each Table specifies configuration of one node to spin up for test

Each test package is [a single-process package that accepts and responds with certain messages](#test-package-interface).


For example:
```toml
{{#webinclude https://raw.githubusercontent.com/kinode-dao/core_tests/master/tests.toml 6:15}}
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
{{#webinclude https://raw.githubusercontent.com/kinode-dao/core_tests/master/tests.toml 15:25}}
```

## Test Package Interface

A test package is a single-process package that accepts and responds with certain messages.
The interface is defined as:


```wit
{{#webinclude https://raw.githubusercontent.com/kinode-dao/kinode/main/kinode/packages/tester/api/tester%3Asys-v0.wit}}
```

A `run` `request` starts the test.
A `run` `response` marks the end of a test, and is either an `Ok` Result, indicating success, or a `Err` Result with information as to where the error occurred.

In the Rust language, a helper macro for failures can be found in [`tester_types.rs`](https://github.com/kinode-dao/kinode/blob/main/kinode/packages/tester/tester_types.rs).
The macro is `fail!()`: it automatically sends the Response as specified above, filing out the fields, and exits.
