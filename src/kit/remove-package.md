# `kit remove-package`

short: `kit r`

`kit remove-package` removes an installed package from the given node (defaults to `localhost:8080`).

For example,
```
kit remove-package foo
```

or

```
kit remove-package -package foo --publisher template.os
```

## Discussion

If passed an optional positional argument `DIR` (the path to a package directory), the `metadata.json` therein is parsed to get the `package_id` and that package is removed from the node.
If no arguments are provided, the same process happens for the current working directory.
Alternatively, a `--package` and `--publisher` can be provided as arguments, and that package will be removed.

## Arguments

```
$ kit remove-package --help
Remove a running package from a node

Usage: kit remove-package [OPTIONS] [DIR]

Arguments:
  [DIR]  The package directory to remove (Overridden by PACKAGE/PUBLISHER) [default: CWD]

Options:
  -a, --package <PACKAGE>      Name of the package (Overrides DIR)
  -u, --publisher <PUBLISHER>  Name of the publisher (Overrides DIR)
  -p, --port <NODE_PORT>       localhost node port; for remote see https://book.kinode.org/hosted-nodes.html#using-kit-with-your-hosted-node [default: 8080]
  -h, --help                   Print help
```

### Optional positional arg: `DIR`

The package directory to be removed from the node; defaults to current working directory.

### `--package`

short: `-a`

The package name of the package to be removed; default is derived from `metadata.json` in `DIR`.

### `--publisher`

short `-u`

The publisher of the package to be removed; default is derived from `metadata.json` in `DIR`.

### `--port`

short: `-p`

For nodes running on localhost, the port of the node; defaults to `8080`.
`--port` is overridden by `--url` if both are supplied.

### `--url`

short: `-u`

The URL the node is hosted at.
Can be either localhost or remote.
`--url` overrides `--port` if both are supplied.
