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
Currently, `rs` is supported, corresponding to processes written in `rust`.
Note that a package may have more than one process and those processes need not be written in the same language.

After compiling each process, it places the output `.wasm` binaries within the `pkg/` directory at the top-level of the given package directory.
Here is an example of what a package directory will look like after using `kit build`:

```
my-rust-chat
├── Cargo.lock
├── Cargo.toml
├── metadata.json
├── pkg
│   ├── manifest.json
│   ├── my-rust-chat.wasm
│   ├── scripts.json
│   └── send.wasm
├── my-rust-chat
│   └── ...
└── send
    └── ...
```

The `pkg/` directory is then zipped and can be injected into the node with [`kit start-package`](./start-package.md).

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
$ kit build --help
Build a Kinode package

Usage: kit build [OPTIONS] [DIR]

Arguments:
  [DIR]  The package directory to build [default: /home/nick/git/kinode-book/src]

Options:
      --no-ui
          If set, do NOT build the web UI for the process; no-op if passed with UI_ONLY
      --ui-only
          If set, build ONLY the web UI for the process; no-op if passed with NO_UI
  -i, --include <INCLUDE>
          Build only these processes/UIs (can specify multiple times) [default: build all]
  -e, --exclude <EXCLUDE>
          Build all but these processes/UIs (can specify multiple times) [default: build all]
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
  -r, --reproducible
          Make a reproducible build using Docker
  -f, --force
          Force a rebuild
  -v, --verbose
          If set, output stdout and stderr
  -h, --help
          Print help
```
