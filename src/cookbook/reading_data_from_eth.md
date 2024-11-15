# Reading Data from ETH

For the purposes of this cookbook entry, all reads will be done from Ethereum Mainnet, but the same methods can easily be used on other networks by changing the `chain_id` parameter.

<div class="warning">
If a node does not have a provider for the given chain ID, calls and subscriptions will fail.
To fix this, add some code on either the frontend or backend of your app that handles these failures by prompting the user to add a provider for the desired chain.
</div>

Using the provider system starts in a process by importing the `eth` library from `kinode_process_lib`:
```rust
use kinode_process_lib::eth;
```

Then, create a new `Provider` object with the desired chain ID and timeout:
```rust
let provider = eth::Provider::new(chain_id, 30);
```
The timeout set here will apply to all requests sent through the provider.
If a request takes longer than the timeout, the request will fail with a timeout error.
Generally, ETH calls can take longer than other requests in Kinode, because the call must be sent to an external RPC that may or may not be fast.
Note also that an RPC endpoint will generally take longer to respond to larger calls.
If you need to adjust the timeout or chain ID, simply create another provider object with the new desired parameters.

Calling various functions on the `Provider` allows the process to execute RPC calls like `get_balance`, `get_logs`, and `send_raw_transaction`.
Here's an example of reading the current block number from Ethereum:
```rust
let provider = eth::Provider::new(1, 5);

match provider.get_block_number() {
    Ok(block_number) => {
        println!("latest block number: {block_number}");
    }
    Err(e) => {
        println!("failed to get block number: {e:?}");
    }
}
```

Here's an example of using a `Filter` to first fetch logs, then create a subscription to a contract's events:
```rust
const EVENTS: [&str; 3] = [
    "AppRegistered(uint256,string,bytes,string,bytes32)",
    "AppMetadataUpdated(uint256,string,bytes32)",
    "Transfer(address,address,uint256)",
];

let provider = eth::Provider::new(1, 30);

let filter = eth::Filter::new()
        .address(eth::Address::from_str("0x18c39eB547A0060C6034f8bEaFB947D1C16eADF1").unwrap())
        .from_block(0)
        .to_block(eth::BlockNumberOrTag::Latest)
        .events(EVENTS);

match eth_provider.get_logs(&filter) {
    Ok(logs) => {
        // do something with the logs, perhaps save them somewhere?
    },
    Err(_) => {
        println!("failed to fetch logs!");
    }
}

match eth_provider.subscribe(1, filter) {
    Ok(()) => {
        println!("subscribed to events!");
    },
    Err(e) => {
        println!("failed to subscribe to events! we should try again..");
    }
}
```

There are a few important things to note when subscribing to contract events and fetching event logs:

1. Subscription updates will come in the form of `Request`s from `eth:distro:sys`. The body of these requests will be JSON that deserializes to `Result<eth::EthSub, eth::EthSubError>`. See the [ETH API documentation](../apis/eth_provider.md) for more information on these types.

2. The `get_logs` call is usually limited by RPC providers to a certain amount of data. For example, [Alchemy](https://docs.alchemy.com/reference/eth-getlogs) limits a request to either 10,000 total log items OR a 2,000-block range. For this reason, your app should be prepared to break calls up into multiple chunks.

3. A good strategy for efficiently fetching logs is to save them in a data structure inside your app, and then only fetch logs that are newer than the last log you saved.

4. If a subscription fails, it makes sense to try resubscribing, but keep in mind that events might occur between the failure and the resubscribe. A good strategy is to fetch logs for this time period.

For a full example of an app that uses the ETH Provider in a critical use-case, check out the [kns_indexer](https://github.com/kinode-dao/kinode/blob/main/kinode/packages/kns_indexer/kns_indexer/src/lib.rs) in the Kinode repo.
