# `necdev dev-ui`

`necdev dev-ui` starts a web development server with hot reloading for the indicated UI-enabled package (or the current working directory), e.g.,

```bash
necdev dev-ui foo
```

or

```bash
necdev dev-ui
```

## Arguments

```bash
$ necdev dev-ui --help
Start the web UI development server with hot reloading (same as `cd ui && npm i && npm start`)

Usage: necdev dev-ui [OPTIONS] [DIR]

Arguments:
  [DIR]  The package directory to build (must contain a `ui` directory) [default: /home/nick/git/necdev]

Options:
  -p, --port <NODE_PORT>  Node port: for use on localhost (overridden by URL) [default: 8080]
  -u, --url <URL>         Node URL (overrides NODE_PORT)
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
