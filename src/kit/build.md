# `kit build`

short: `kit b`

`kit build` builds the indicated package directory, or the current working directory if none supplied, e.g.,

```
kit build foo
```

or

```
kit build
```

`kit build` builds each process in the package and places the `.wasm` binaries into the `pkg/` directory for installation with [`kit start-package`](./start-package.md).
It automatically detects what language each process is, and builds it appropriately (from amongst the supported `rust`, `python`, and `javascript`).

## Discussion

`kit build` builds a Kinode package directory.
Specifically, it iterates through all directories within the given package directory and looks for `src/lib.??`, where the `??` is the file extension.
Currently, `rs`, `py`, and `js` are supported, corresponding to processes written in `rust`, `python`, and `javascript`, respectively.
Note that a package may have more than one process and those processes need not be written in the same language.

After compiling each process, it places the output `.wasm` binaries within the `pkg/` directory at the top-level of the given package directory.
Here is an example of what a package directory will look like after using `kit build`:

```
rustchat
├── Cargo.lock
├── Cargo.toml
├── metadata.json
├── pkg
│   ├── manifest.json
│   ├── rustchat.wasm
│   ├── scripts.json
│   └── send.wasm
├── rustchat
│   └── ...
└── send
    └── ...
```

The `pkg/` directory can then be zipped and injected into the node with [`kit start-package`](./start-package.md).

`kit build` also builds the UI if it is found in `pkg/ui/`.
There must exist a `ui/package.json` file with a `scripts` object containing the following arguments:
```json
"scripts": {
  "build": "tsc && vite build",
  "copy": "mkdir -p ../pkg/ui && rm -rf ../pkg/ui/* && cp -r dist/* ../pkg/ui/",
  "build:copy": "npm run build && npm run copy",
}
```

Additional UI dev info can be found [here](../apis/frontend_development.md).
To both `build` and `start-package` in one command, use `kit build-start-package`.

## Arguments

```
kit build --help
Build a Kinode package

Usage: kit build [OPTIONS] [DIR]

Arguments:
  [DIR]  The package directory to build [default: /home/nick/git/kit]

Options:
      --no-ui
          If set, do NOT build the web UI for the process; no-op if passed with UI_ONLY
      --ui-only
          If set, build ONLY the web UI for the process; no-op if passed with NO_UI
  -s, --skip-deps-check
          If set, do not check for dependencies
      --features <FEATURES>
          Pass these comma-delimited feature flags to Rust cargo builds
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
  -f, --force
          Force a rebuild
  -v, --verbose
          If set, output stdout and stderr
  -h, --help
          Print help

```

### Optional positional arg: `DIR`

The package directory to build; defaults to the current working directory.

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

### `--port`

short: `-p`

Node to pull dependencies from.
A package's dependencies can be satisfied by either:
1. A live node, the one running at the port given here, or
2. By local dependencies (specified using [`--local-dependency`](#--local-dependency), below).

### `--download-from`

short: `-d`

The mirror to download dependencies from (default: package `publisher`).

### `--world`

short: `-w`

[WIT `world`](../process/wit-apis.md) to use.
Not required for Rust processes; use for py or js.

### `--local-dependency`

short: `-l`

A path to a package that satisfies a build dependency.
Can be specified multiple times.

### `--add-to-api`

short: `-a`

A path to a file to include in the API published alongside the package.
Can be specified multiple times.

### `--force`

short: `-f`

Don't check if package doesn't need to be rebuilt: just build it.

### `--verbose`

short: `-v`

Always output stdout and stderr if set.
