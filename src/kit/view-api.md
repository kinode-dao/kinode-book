# `kit view-api`

`kit view-api` fetches the list of APIs or a specific API for the given package.
`view-api` relies on a node to do so, e.g.

```
kit view-api --port 8080
```

lists all the APIs of packages downloaded by the Kinode running at port 8080.

## Example Usage

```bash
# Fetch and display the API for the given package
kit view-api app_store:sys
```

## Discussion

Packages have the option to [expose their API using a WIT file](../process/wit-apis.md).
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
  -p, --port <NODE_PORT>  Node port: for use on localhost (overridden by URL) [default: 8080]
  -u, --url <URL>         Node URL (overrides NODE_PORT)
  -h, --help              Print help
```

### Positional arg: `PACKAGE_ID`

Get the API of this package.
By default, list the names of all APIs.

### `--port`

short: `-p`

For nodes running on localhost, the port of the node; defaults to `8080`.
`--port` is overridden by `--url` if both are supplied.

### `--url`

short: `-u`

The URL the node is hosted at.
Can be either localhost or remote.
`--url` overrides `--port` if both are supplied.
