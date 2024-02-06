# `kit dev-ui`

`kit dev-ui` starts a web development server with hot reloading for the indicated UI-enabled package (or the current working directory), e.g.,

```bash
kit dev-ui foo
```

or

```bash
kit dev-ui
```

## Arguments

```
$ kit d --help
Start the web UI development server with hot reloading (same as `cd ui && npm i && npm start`)

Usage: kit dev-ui [OPTIONS] [DIR]

Arguments:
  [DIR]  The package directory to build (must contain a `ui` directory) [default: /home/nick/git/kit]

Options:
  -p, --port <NODE_PORT>  Node port: for use on localhost (overridden by URL) [default: 8080]
  -u, --url <URL>         Node URL (overrides NODE_PORT)
  -s, --skip-deps-check   If set, do not check for dependencies
  -h, --help              Print help
```

### Optional positional arg: `DIR`

The UI-enabled package directory to serve; defaults to current working directory.

### `--port`

For nodes running on localhost, the port of the node; defaults to `8080`.
`--port` is overridden by `--url` if both are supplied.

### `--url`

The URL the node is hosted at.
Can be either localhost or remote.
`--url` overrides `--port` if both are supplied.

### `--skip-deps-check`

Don't check for dependencies.
