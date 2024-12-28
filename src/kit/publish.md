# `kit publish`

short: `kit p`

`kit publish` creates entries in the Kimap, publishing the given package according to the `app-store`s protocol.
It can also be used to update or unpublish previously-published packages.
`kit publish` directly writes to the Kimap: it does not interact with a Kinode.

## Example Usage

```bash
# Publish a package on the real network (Optimism mainnet).
kit publish --metadata-uri https://raw.githubusercontent.com/path/to/metadata.json --keystore-path ~/.foundry/keystores/dev --rpc wss://opt-mainnet.g.alchemy.com/v2/<ALCHEMY_API_KEY> --real

# Unublish a package.
kit publish --metadata-uri https://raw.githubusercontent.com/path/to/metadata.json --keystore-path ~/.foundry/keystores/dev --rpc wss://opt-mainnet.g.alchemy.com/v2/<ALCHEMY_API_KEY> --real --unpublish
```

See [Sharing with the World](../my_first_app/chapter_5.md) for a tutorial on how to use `kit publish`.

## Arguments

```
$ kit publish --help
Publish or update a package

Usage: kit publish [OPTIONS] --metadata-uri <URI> --keystore-path <PATH> --rpc <RPC_URI> [DIR]

Arguments:
  [DIR]  The package directory to publish [default: CWD]

Options:
  -k, --keystore-path <PATH>
          Path to private key keystore (choose 1 of `k`, `l`, `t`)
  -l, --ledger
          Use Ledger private key (choose 1 of `k`, `l`, `t`)
  -t, --trezor
          Use Trezor private key (choose 1 of `k`, `l`, `t`)
  -u, --metadata-uri <URI>
          URI where metadata lives
  -r, --rpc <RPC_URI>
          Ethereum Optimism mainnet RPC endpoint (wss://)
  -e, --real
          If set, deploy to real network [default: fake node]
      --unpublish
          If set, unpublish existing published package [default: publish a package]
  -g, --gas-limit <GAS_LIMIT>
          The ETH transaction gas limit [default: 1_000_000]
  -p, --priority-fee <MAX_PRIORITY_FEE_PER_GAS>
          The ETH transaction max priority fee per gas [default: estimated from network conditions]
  -f, --fee-per-gas <MAX_FEE_PER_GAS>
          The ETH transaction max fee per gas [default: estimated from network conditions]
  -h, --help
          Print help
```

### Positional arg: `DIR`

Publish the metadata for the package in this directory.

### `--metadata-uri`

short: `-u`

The URI hosting the `metadata.json`.
You must place the `metadata.json` somewhere public before publishing your package on Kimap.
A common place to host `metadata.json` is on your package's GitHub repo.
If you use GitHub, make sure to use the static link to the specific commit, not a branch-specific URL (e.g. `main`) that will change with new commits.
For example, `https://raw.githubusercontent.com/nick1udwig/chat/master/metadata.json` is not the correct link to use, because it will change when new commits are added.
You want to use a link like `https://raw.githubusercontent.com/nick1udwig/chat/191dce595ad00a956de04b9728f479dee04863c7/metadata.json` which will not change when new commits are added.

### `--keystore-path`

short: `-k`

Use private key from keystore given by path.
The keystore is a [Web3 Secret Storage file](https://ethereum.org/en/developers/docs/data-structures-and-encoding/web3-secret-storage/) that holds an encrypted copy of your private keys.
See the [Sharing with the World](../my_first_app/chapter_5.md) usage example for one way to create a keystore.

Must supply one and only one of `--keystore-path`, `--ledger`, or `--trezor`.

### `--ledger`

short: `-l`

Use private key from Ledger.

Must supply one and only one of `--keystore-path`, `--ledger`, or `--trezor`.

### `--trezor`

short: `-t`

Use private key from Trezor.

Must supply one and only one of `--keystore-path`, `--ledger`, or `--trezor`.

### `--rpc`

short: `-r`

The Ethereum RPC endpoint to use.
For fakenodes this runs by default at `ws://localhost:8545`.

### `--real`

short: `-e`

Manipulate the real (live) Kimap.
Default is to manipulate the fakenode Kimap.

### `--unpublish`

Remove a previously-published package.

### `--gas-limit`

short: `-g`

Set the gas limit for the transaction.

### `--priority-fee`

short: `-p`

Set the priority fee for the transaction.

### `--fee-per-gas`

short: `-f`

Set the price of gas for the transaction.
