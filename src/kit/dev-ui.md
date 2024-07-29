# `kit dev-ui`

short: `kit d`

`kit dev-ui` starts a web development server with hot reloading for the indicated UI-enabled package (or the current working directory), e.g.,

```
kit dev-ui foo
```

or

```
kit dev-ui
```

## Arguments

```
$ kit dev-ui --help
Start the web UI development server with hot reloading (same as `cd ui && npm i && npm run dev`)

Usage: kit dev-ui [OPTIONS] [DIR]

Arguments:
  [DIR]  The package directory to build (must contain a `ui` directory) [default: /home/nick/git/kinode-book/src]

Options:
  -p, --port <NODE_PORT>  localhost node port; for remote see https://book.kinode.org/hosted-nodes.html#using-kit-with-your-hosted-node [default: 8080]
      --release           If set, create a production build
  -s, --skip-deps-check   If set, do not check for dependencies
  -h, --help              Print help
```

### Optional positional arg: `DIR`

The UI-enabled package directory to serve; defaults to current working directory.

### `--port`

short: `-p`

For nodes running on localhost, the port of the node; defaults to `8080`.
`--port` is overridden by `--url` if both are supplied.

### `--release`

Create a production build.
Defaults to dev build.

### `--skip-deps-check`

short: `-s`

Don't check for dependencies.
