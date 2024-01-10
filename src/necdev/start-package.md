# `necdev start-package`

`necdev start-package` installs and starts the indicated package directory (or current working directory) on the given Nectar node, e.g.,

```bash
necdev start-package foo
```

or

```bash
necdev start-package
```

## Arguments

```bash
$ necdev start-package --help
Start a built Nectar process

Usage: necdev start-package [OPTIONS] [DIR]

Arguments:
  [DIR]  The package directory to build [default: /home/nick/git/necdev]

Options:
  -p, --port <NODE_PORT>  Node port: for use on localhost (overridden by URL) [default: 8080]
  -u, --url <URL>         Node URL (overrides NODE_PORT)
  -h, --help              Print help
```

### Optional positional arg: `DIR`

The package directory to install and start on the node; defaults to current working directory.

### `--port`

For nodes running on localhost, the port of the node; defaults to `8080`.
`--port` is overridden by `--url` if both are supplied.

### `--url`

The URL the node is hosted at.
Can be either localhost or remote.
`--url` overrides `--port` if both are supplied.
