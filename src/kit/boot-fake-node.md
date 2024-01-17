# `kit boot-fake-node`

`kit boot-fake-node` starts a "fake" (i.e. not connected to the live network) node, e.g.,

```bash
kit boot-fake-node
```

By default, `boot-fake-node` fetches a prebuilt binary and launches the node using it.
Alternatively, `boot-fake-node` can use a local binary or build a local Nectar core repo and use the resulting binary.

## Example Usage

You can start a network of fake nodes that can communicate with each other (but not the live network).
You'll need to start a new terminal for each fake node.
For example, to start two fake nodes, `fake.nec` and `fake2.nec`:

```bash
kit boot-fake-node

# In a new terminal
kit boot-fake-node -h /tmp/nectar-fake-node-2 -p 8081 -f fake2.nec
```

## Discussion

Fake nodes make development easier.
A fake node is not connected to the network, but otherwise behaves the same as a live node.
Fake nodes are connected to each other on your local machine through a network router that passes messages between them.
Fake nodes also clean up after themselves, so you don't have to worry about state from a previous iterations messing up the current one.
Thus, fake nodes are an excellent testing ground during development for fast iteration.

There are some cases where fake nodes are not appropriate.
One is for testing persistence of a package.
Because fake nodes clean up after themselves, they will not persist data from run to run.
Another weakness of fake nodes is also their strength: they are not connected to the live network.
Though this lack of connectivity makes them easy to spin up and throw away, the downside is no access to services on the network, like remote LLMs.

## Arguments

```bash
$ kit f --help
Boot a fake node for development

Usage: kit boot-fake-node [OPTIONS]

Options:
  -r, --runtime-path <PATH>
          Path to Nectar core repo or runtime binary (overrides --version)
  -v, --version <VERSION>
          Version of Nectar binary to use (overridden by --runtime-path) [default: 0.4.0]
  -p, --port <NODE_PORT>
          The port to run the fake node on [default: 8080]
  -h, --home <HOME>
          Where to place the home directory for the fake node [default: /tmp/nectar-fake-node]
  -f, --fake-node-name <NODE_NAME>
          Name for fake node [default: fake.nec]
      --network-router-port <NETWORK_ROUTER_PORT>
          The port to run the network router on (or to connect to) [default: 9001]
      --rpc <RPC_ENDPOINT>
          Ethereum RPC endpoint (wss://)
      --persist
          If set, do not delete node home after exit
      --password <PASSWORD>
          Password to login [default: secret]
      --help
          Print help
```

### `--runtime-path`

Pass to run a local binary or build a local Nectar core repo and use the resulting binary, e.g.

```bash
kit boot-fake-node --runtime-path ~/git/nectar
```

for a system with the Nectar core repo living at `~/git/nectar`.

Overrides `--version`.

### `--version`

Fetch and run a specific version of the binary; defaults to most recent version (here, `0.5.0`).
Overridden by `--runtime-path`.

### `--port`

Run the fake node on this port; defaults to `8080`.

### `--home`

Path to place fake node home directory at; defaults to `/tmp/nectar-fake-node`.

### `--fake-node-name`

The name of the fake node; defaults to `fake.nec`.

### `--network-router-port`

Run the fake node network router on this port; defaults to `9001`.
Additional fake nodes must point to the same port to connect to the fake node network.

### `--rpc`

The Ethereum RPC endpoint to use, if desired.

### `--persist`

Persist the node home directory after exit, rather than cleaning it up.

### `--password`

The password of the fake node; defaults to `secret`.
