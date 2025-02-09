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
