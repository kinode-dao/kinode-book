# `necdev build`

`necdev build` builds the indicated package directory, or the current working directory if none supplied, e.g.,

```bash
necdev build foo
```

or

```bash
necdev build
```

`necdev build` builds each process in the package and places the `.wasm` binaries into the `pkg/` directory for installation.
It automatically detects what language each process is, and builds it appropriately (from amongst the supported `rust`, `python`, and `javascript`).

## Arguments

```bash
$ necdev build --help
Build a Nectar process

Usage: necdev build [OPTIONS] [DIR]

Arguments:
  [DIR]  The package directory to build [default: /home/nick/git/necdev]

Options:
      --ui-only  If set, build ONLY the web UI for the process
  -q, --quiet    If set, do not print build stdout/stderr
  -h, --help     Print help

```

### Optional positional arg: `DIR`

The package directory to build; defaults to the current working directory.

### `--ui-only`

Build ONLY the UI for a package with a UI.
Otherwise, for a package with a UI, both the package and the UI will be built.

### `--quiet`

Don't print the build stdout/stderr.
