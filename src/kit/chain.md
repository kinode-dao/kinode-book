# kit chain

`kit chain` starts a local fakechain with foundry's [anvil](https://github.com/foundry-rs/foundry/tree/master/crates/anvil), e.g.,

```
kit chain
```

The default port is `8545` and the chain ID is `31337`.

## Discussion

`kit chain` starts an anvil node with the arguments `--load-state kinostate.json`.
This json file includes the [KNS](https://github.com/kinode-dao/KNS) & app_store contracts, and is included in the `kit` binary.

The [kinostate.json](https://github.com/kinode-dao/kit/blob/master/src/chain/kinostate.json) file can be found written into /tmp/kinode-kit-cache/kinostate-{hash}.json upon running the command.

Note that while the kns_indexer and app_store apps in fake nodes use this chain to index events, any events loaded from a json statefile, aren't replayed upon restarting anvil.

## Arguments

```
$ kit c --help
Start a local chain for development

Usage: kit chain [OPTIONS]

Options:
  -p, --port <PORT>  Port to run the chain on [default: 8545]
  -h, --help         Print help
```

### `--port`

Port to run anvil fakechain on.
Defaults to `8545`.
