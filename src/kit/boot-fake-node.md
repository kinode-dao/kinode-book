# `kit boot-fake-node`

`kit boot-fake-node` starts a "fake" (i.e. not connected to the live network) node, e.g.,

```
kit boot-fake-node
```

By default, `boot-fake-node` fetches a prebuilt binary and launches the node using it.
Alternatively, `boot-fake-node` can use a local binary or build a local Kinode core repo and use the resulting binary.

## Example Usage

You can start a network of fake nodes that can communicate with each other (but not the live network).
You'll need to start a new terminal for each fake node.
For example, to start two fake nodes, `fake.os` and `fake2.os`:

```
kit boot-fake-node

# In a new terminal
kit boot-fake-node --home /tmp/kinode-fake-node-2 -p 8081 --fake-node-name fake2.os

# Send a message from fake2.os to fake.os
# In the terminal of fake2.os:
hi fake.os hello!

# You should see "hello!" in the first node's terminal
```

## Discussion

Fake nodes make development easier.
A fake node is not connected to the network, but otherwise behaves the same as a live node.
Fake nodes are connected to each other on your local machine through a network router that passes messages between them.
Fake nodes also clean up after themselves, so you don't have to worry about state from a previous iterations messing up the current one.
If you wish to persist a state of a fake node between boots, you can do so with `--persist`.
Thus, fake nodes are an excellent testing ground during development for fast iteration.

There are some cases where fake nodes are not appropriate.
The weakness of fake nodes is also their strength: they are not connected to the live network.
Though this lack of connectivity makes them easy to spin up and throw away, the downside is no access to services on the network, like remote LLMs.

## Arguments

```
$ kit boot-fake-node --help
Boot a fake node for development

Usage: kit boot-fake-node [OPTIONS]

Options:
  -r, --runtime-path <PATH>
          Path to Kinode core repo or runtime binary (overrides --version)
  -v, --version <VERSION>
          Version of Kinode binary to use (overridden by --runtime-path) [default: latest] [possible values: latest, v0.5.3-alpha, v0.5.2-alpha, v0.5.1-alpha]
  -p, --port <NODE_PORT>
          The port to run the fake node on [default: 8080]
  -h, --home <HOME>
          Where to place the home directory for the fake node [default: /tmp/kinode-fake-node]
  -f, --fake-node-name <NODE_NAME>
          Name for fake node [default: fake.os]
      --network-router-port <NETWORK_ROUTER_PORT>
          The port to run the network router on (or to connect to) [default: 9001]
      --rpc <RPC_ENDPOINT>
          Ethereum RPC endpoint (wss://)
      --testnet
          If set, use Sepolia testnet
      --persist
          If set, do not delete node home after exit
      --password <PASSWORD>
          Password to login [default: secret]
      --release
          If set and given --runtime-path, compile release build [default: debug build]
      --verbosity <VERBOSITY>
          Verbosity of node: higher is more verbose [default: 0]
      --help
          Print help
```

### `--runtime-path`

short: `-r`

Pass to boot a fake node from a local binary or build a local Kinode core repo and use the resulting binary, e.g.

```
kit boot-fake-node --runtime-path ~/git/kinode
```

for a system with the Kinode core repo living at `~/git/kinode`.

Overrides `--version`.

### `--version`

short: `-v`

Fetch and run a specific version of the binary; defaults to most recent version (here, `0.5.0`).
Overridden by `--runtime-path`.

### `--port`

short: `-p`

Run the fake node on this port; defaults to `8080`.

### `--home`

short: `-h`

Path to place fake node home directory at; defaults to `/tmp/kinode-fake-node`.

### `--fake-node-name`

short: `-f`

The name of the fake node; defaults to `fake.os`.

### `--network-router-port`

Run the fake node network router on this port; defaults to `9001`.
Additional fake nodes must point to the same port to connect to the fake node network.

### `--rpc`

The Ethereum RPC endpoint to use, if desired.

### `--testnet`

Connect to the Sepolia testnet rather than the Optimism mainnet.

### `--persist`

Persist the node home directory after exit, rather than cleaning it up.

### `--password`

The password of the fake node; defaults to `secret`.

### `--release`

If `--runtime-path` is given, build the runtime for release; default is debug. The tradeoffs between the release and default version are described [here](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html?highlight=release#building-for-release).

### `--verbosity`

Set the verbosity of the node; higher is more verbose; default is `0`.
