# `kit build-start-package`

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
  [DIR]  The package directory to build [default: /home/nick/git/kit]

Options:
  -p, --port <NODE_PORT>     Node port: for use on localhost (overridden by URL) [default: 8080]
  -u, --url <URL>            Node URL (overrides NODE_PORT)
      --no-ui                If set, do NOT build the web UI for the process; no-op if passed with UI_ONLY
      --ui-only              If set, build ONLY the web UI for the process
  -s, --skip-deps-check      If set, do not check for dependencies
      --features <FEATURES>  Pass these comma-delimited feature flags to Rust cargo builds
  -h, --help                 Print help```
```

### Optional positional arg: `DIR`

The package directory to build, install and start on the node; defaults to the current working directory.

### `--port`

short: `-p`

For nodes running on localhost, the port of the node; defaults to `8080`.
`--port` is overridden by `--url` if both are supplied.

### `--url`

short: `-u`

The URL the node is hosted at.
Can be either localhost or remote.
`--url` overrides `--port` if both are supplied.

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
