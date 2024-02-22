# ETH API

Useful [helper functions](#helpers) can be found in the [kinode_process_lib](https://github.com/kinode-dao/process_lib)

The runtime module for eth in it's core is quite simple, it connects to an rpc endpoint, or forwards requests to a node that has done so.

It uses [alloy](https://github.com/alloy-rs/alloy) and [alloy-core](https://github.com/alloy-rs/core) for types.

The Requests are of type `EthAction`, which has an action for creating a subscription, killing one, and a raw eth_rpc request type, taking a method and params.

Responses are one of the following, an `Ok`, an `Err(EthError)`, or a `Response { value: json }`.

If you have a subscription open, updates come in as `EthSub`s with a corresponding sub_id, and the result as a `SubscriptionResult`.

## Core API

```rust
pub enum EthAction {
    /// Subscribe to logs with a custom filter. ID is to be used to unsubscribe.
    /// Logs come in as alloy_rpc_types::pubsub::SubscriptionResults
    SubscribeLogs {
        sub_id: u64,
        kind: SubscriptionKind,
        params: Params,
    },
    /// Kill a SubscribeLogs subscription of a given ID, to stop getting updates.
    UnsubscribeLogs(u64),
    /// Raw request. Used by kinode_process_lib.
    Request {
        method: String,
        params: serde_json::Value,
    },
}
/// Incoming subscription update.
pub struct EthSub {
    pub id: u64,
    pub result: SubscriptionResult,
}

pub enum EthResponse {
    Ok,
    Response { value: serde_json::Value },
    Err(EthError),
}

pub enum EthError {
    /// Underlying transport error
    TransportError(String),
    /// Subscription closed
    SubscriptionClosed(u64),
    /// The subscription ID was not found, so we couldn't unsubscribe.
    SubscriptionNotFound,
    /// Invalid method
    InvalidMethod(String),
    /// Permission denied
    PermissionDenied(String),
    /// Internal RPC error
    RpcError(String),
}
```

## Helpers

```rust
pub fn get_transaction_count(address: Address, tag: Option<BlockId>) -> anyhow::Result<U256>
```

```rust
pub fn get_block_by_hash(hash: BlockHash, full_tx: bool) -> anyhow::Result<Option<Block>>
```

```rust
pub fn get_block_by_number(number: BlockNumberOrTag, full_tx: bool) -> anyhow::Result<Option<Block>>
```

```rust
pub fn get_storage_at(address: Address, key: U256, tag: Option<BlockId>) -> anyhow::Result<Bytes>
```

```rust
pub fn get_code_at(address: Address, tag: BlockId) -> anyhow::Result<Bytes>
```

```rust
pub fn get_transaction_by_hash(hash: TxHash) -> anyhow::Result<Option<Transaction>>
```

```rust

pub fn get_transaction_receipt(hash: TxHash) -> anyhow::Result<Option<TransactionReceipt>>
```

```rust

pub fn estimate_gas(tx: TransactionRequest, block: Option<BlockId>) -> anyhow::Result<U256>
```

```rust

pub fn get_accounts() -> anyhow::Result<Vec<Address>>
```

```rust

pub fn get_fee_history(block_count: U256, last_block: BlockNumberOrTag, reward_percentiles: Vec<f64>) -> 
anyhow::Result<FeeHistory>
```

```rust

pub fn call(tx: TransactionRequest, block: Option<BlockId>) -> anyhow::Result<Bytes>
```

```rust

pub fn send_raw_transaction(tx: Bytes) -> anyhow::Result<TxHash>
```

```rust

pub fn getlogs_and_subscribe(sub_id: u64, filter: Filter) -> anyhow::Result<()>
```

```rust
pub fn subscribe(sub_id: u64, filter: Filter) -> anyhow::Result<()>
```

```rust
pub fn unsubscribe(sub_id: u64) -> anyhow::Result<()>
```
