# `kit view-api`

short: `kit v`

`kit view-api` fetches the list of APIs or a specific API for the given package.
`view-api` relies on a node to do so, e.g.

```
kit view-api --port 8080
```

lists all the APIs of packages downloaded by the Kinode running at port 8080.

## Example Usage

```bash
# Fetch and display the API for the given package
kit view-api app-store:sys
```

## Discussion

Packages have the option to [expose their API using a WIT file](../system/process/wit_apis.md).
When a package is distributed, its API is posted by the distributor along with the package itself.
Downloading the package also downloads the API.

## Arguments

```
$ kit view-api --help
Fetch the list of APIs or a specific API

Usage: kit view-api [OPTIONS] [PACKAGE_ID]

Arguments:
  [PACKAGE_ID]  Get API of this package (default: list all APIs)

Options:
  -p, --port <NODE_PORT>      localhost node port; for remote see https://book.kinode.org/hosted-nodes.html#using-kit-with-your-hosted-node [default: 8080]
  -d, --download-from <NODE>  Download API from this node if not found
  -h, --help                  Print help
```

### Positional arg: `PACKAGE_ID`

Get the API of this package.
By default, list the names of all APIs.

### `--port`

short: `-p`

For nodes running on localhost, the port of the node; defaults to `8080`.
`--port` is overridden by `--url` if both are supplied.

### `--download-from`

short: `-d`

The mirror to download dependencies from (default: package `publisher`).
