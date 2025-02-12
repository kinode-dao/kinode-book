# `kit boot-real-node`

short: `kit e`

`kit boot-real-node` starts a Kinode connected to the live network, e.g.,

```
kit boot-real-node
```

By default, `boot-real-node` fetches a prebuilt binary and launches the node using it.
Alternatively, `boot-real-node` can build a local Kinode core repo and use the resulting binary.

## Example Usage

You can create a new node, creating a home directory at, e.g., `~/<my-new-node-name>.os`, using

```
kit boot-real-node --home ~/<my-new-node-name>.os
```

or you can boot an existing node with home directory at, e.g., `~/<my-old-node-name>.os`, using

```
kit boot-real-node --home ~/<my-old-node-name>.os
```

## Discussion

`kit boot-real-node` makes it easier to run a node by reducing the number of steps to download the Kinode core binary and launch a node.
Be cautious using `boot-real-node` before Kinode core `1.0.0` launch without specifying the `--version` flag: the default `--version latest` may use a new major version of Kinode core!

## Arguments

```
$ kit boot-real-node --help
Boot a real node

Usage: kit boot-real-node [OPTIONS] --home <HOME>

Options:
  -r, --runtime-path <PATH>    Path to Kinode core repo (overrides --version)
  -v, --version <VERSION>      Version of Kinode binary to use (overridden by --runtime-path) [default: latest] [possible values: latest, v0.8.7, v0.8.6, v0.8.5]
  -p, --port <NODE_PORT>       The port to run the real node on [default: 8080]
  -o, --home <HOME>            Path to home directory for real node
      --rpc <RPC_ENDPOINT>     Ethereum Optimism mainnet RPC endpoint (wss://)
      --release                If set and given --runtime-path, compile release build [default: debug build]
      --verbosity <VERBOSITY>  Verbosity of node: higher is more verbose [default: 0]
  -h, --help                   Print help
```
