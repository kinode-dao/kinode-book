# `kit build-start-package`

short: `kit bs`

`kit build-start-package` builds, installs and starts the indicated package directory, or the current working directory if none supplied, e.g.,

```
kit build-start-package foo
```

or

```
kit build-start-package
```

## Discussion

`kit build-start-package` runs [`kit build`](./build.md) followed by [`kit start-package`](./start-package.md).

## Arguments

```
$ kit build-start-package --help
Build and start a Kinode package

Usage: kit build-start-package [OPTIONS] [DIR]

Arguments:
  [DIR]  The package directory to build [default: CWD]

Options:
  -p, --port <NODE_PORT>
          localhost node port; for remote see https://book.kinode.org/hosted-nodes.html#using-kit-with-your-hosted-node [default: 8080]
  -d, --download-from <NODE>
          Download API from this node if not found
  -w, --world <WORLD>
          Fallback WIT world name
  -l, --local-dependency <DEPENDENCY_PACKAGE_PATH>
          Path to local dependency package (can specify multiple times)
  -a, --add-to-api <PATH>
          Path to file to add to api.zip (can specify multiple times)
      --no-ui
          If set, do NOT build the web UI for the process; no-op if passed with UI_ONLY
      --ui-only
          If set, build ONLY the web UI for the process
  -s, --skip-deps-check
          If set, do not check for dependencies
      --features <FEATURES>
          Pass these comma-delimited feature flags to Rust cargo builds
  -f, --force
          Force a rebuild
  -v, --verbose
          If set, output stdout and stderr
  -h, --help
          Print help
```

### Optional positional arg: `DIR`

The package directory to build, install and start on the node; defaults to the current working directory.

### `--port`

short: `-p`

The localhost port of the node; defaults to `8080`.
To interact with a remote node, see [here](../hosted-nodes.md#using-kit-with-your-hosted-node).

### `--download-from`

short: `-d`

The mirror to download dependencies from (default: package `publisher`).

### `--world`

short: `-w`

[WIT `world`](../system/process/wit_apis.md) to use.
Not required for Rust processes; use for py or js.

### `--local-dependency`

short: `-l`

A path to a package that satisfies a build dependency.
Can be specified multiple times.

### `--add-to-api`

short: `-a`

A path to a file to include in the API published alongside the package.
Can be specified multiple times.

### `--no-ui`

Do not build the web UI for the process.
Does nothing if passed with `--ui-only`.

### `--ui-only`

Build ONLY the UI for a package with a UI.
Otherwise, for a package with a UI, both the package and the UI will be built.

### `--skip-deps-check`

short: `-s`

Don't check for dependencies.

### `--features`

Build the package with the given [cargo features](https://doc.rust-lang.org/cargo/reference/features.html).

Features can be used like shown [here](https://doc.rust-lang.org/cargo/reference/features.html#command-line-feature-options).
Currently the only feature supported system-wide is `simulation-mode`.

### `--force`

short: `-f`

Don't check if package doesn't need to be rebuilt: just build it.

### `--verbose`

short: `-v`

Always output stdout and stderr if set.
