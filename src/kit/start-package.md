# `kit start-package`

`kit start-package` installs and starts the indicated package directory (or current working directory) on the given Kinode, e.g.,

```
kit start-package foo
```

or

```
kit start-package
```

## Discussion

`kit start-package` injects a built package into the given node and starts it.
`start-package` is designed to be used after a package has been built with [`kit build`](./build.md).
Specifically, it first zips and injects the `pkg/` directory within the given package directory, which contains metadata about the package for the node as well as the `.wasm` binaries for each process.
Then it injects a message to the node to start the package.

To both `build` and `start-package` in one command, use `kit build-start-package`.

## Arguments

```
$ kit s --help
Start a built Kinode process

Usage: kit start-package [OPTIONS] [DIR]

Arguments:
  [DIR]  The package directory to build [default: /home/nick/git/kit]

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
