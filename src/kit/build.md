# `kit build`

`kit build` builds the indicated package directory, or the current working directory if none supplied, e.g.,

```bash
kit build foo
```

or

```bash
kit build
```

`kit build` builds each process in the package and places the `.wasm` binaries into the `pkg/` directory for installation.
It automatically detects what language each process is, and builds it appropriately (from amongst the supported `rust`, `python`, and `javascript`).

## Discussion

`kit build` builds a Kinode package directory.
Specifically, it iterates through all directories within the given package directory and looks for `src/lib.??`, where the `??` is the file extension.
Currently, `rs`, `py`, and `js` are supported, corresponding to processes written in `rust`, `python`, and `javascript`, respectively.
Note that a package may have more than one process and those processes need not be written in the same language.

After compiling each process, it places the output `.wasm` binaries within the `pkg/` directory at the top-level of the given package directory.
The `pkg/` directory is the one that is zipped and injected into the node by [`kit start-package`](./start-package.md).
Thus, after `build`ing, the package is ready for `start-package`.

`kit build` also builds the UI if found in `ui/`.
There must exist a `ui/package.json` file with `scripts` defined like:
```
"build": "tsc && vite build",
"copy": "mkdir -p ../pkg/ui && rm -rf ../pkg/ui/* && cp -r dist/* ../pkg/ui/",
"build:copy": "npm run build && npm run copy",
```

To both `build` and `start-package` in one command, use `kit build-start-package`.

## Arguments

```bash
$ kit b --help
Build a Kinode package

Usage: kit build [OPTIONS] [DIR]

Arguments:
  [DIR]  The package directory to build [default: /home/nick/git/kit]

Options:
      --ui-only          If set, build ONLY the web UI for the process
  -q, --quiet            If set, do not print build stdout/stderr
  -s, --skip-deps-check  If set, do not check for dependencies
  -h, --help             Print help
```

### Optional positional arg: `DIR`

The package directory to build; defaults to the current working directory.

### `--ui-only`

Build ONLY the UI for a package with a UI.
Otherwise, for a package with a UI, both the package and the UI will be built.

### `--quiet`

Don't print the build stdout/stderr.

### `--skip-deps-check`

Don't check for dependencies.
