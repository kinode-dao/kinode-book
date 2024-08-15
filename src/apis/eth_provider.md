# ETH Provider API

**Note: Most processes will not use this API directly. Instead, they will use the `eth` portion of the[`process_lib`](../process_stdlib/overview.md) library, which papers over this API and provides a set of types and functions which are much easier to natively use.
This is mostly useful for re-implementing this module in a different client or performing niche actions unsupported by the library.**

Processes can send two kinds of requests to `eth:distro:sys`: `EthAction` and `EthConfigAction`.
The former only requires the capability to message the process, while the latter requires the root capability issued by `eth:distro:sys`.
Most processes will only need to send `EthAction` requests.

```rust
/// The Action and Request type that can be made to eth:distro:sys. Any process with messaging
/// capabilities can send this action to the eth provider.
///
/// Will be serialized and deserialized using `serde_json::to_vec` and `serde_json::from_slice`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EthAction {
    /// Subscribe to logs with a custom filter. ID is to be used to unsubscribe.
    /// Logs come in as alloy_rpc_types::pubsub::SubscriptionResults
    SubscribeLogs {
        sub_id: u64,
        chain_id: u64,
        kind: SubscriptionKind,
        params: Params,
    },
    /// Kill a SubscribeLogs subscription of a given ID, to stop getting updates.
    UnsubscribeLogs(u64),
    /// Raw request. Used by kinode_process_lib.
    Request {
        chain_id: u64,
        method: String,
        params: serde_json::Value,
    },
}
```

The `Request` containing this action should always expect a response, since every action variant triggers one and relies on it to be useful.
The ETH provider will respond with the following type:

```rust
/// The Response type which a process will get from requesting with an [`EthAction`] will be
/// of this type, serialized and deserialized using `serde_json::to_vec`
/// and `serde_json::from_slice`.
///
/// In the case of an [`EthAction::SubscribeLogs`] request, the response will indicate if
/// the subscription was successfully created or not.
#[derive(Debug, Serialize, Deserialize)]
pub enum EthResponse {
    Ok,
    Response { value: serde_json::Value },
    Err(EthError),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EthError {
    /// RPC provider returned an error
    RpcError(ErrorPayload),
    /// provider module cannot parse message
    MalformedRequest,
    /// No RPC provider for the chain
    NoRpcForChain,
    /// Subscription closed
    SubscriptionClosed(u64),
    /// Invalid method
    InvalidMethod(String),
    /// Invalid parameters
    InvalidParams,
    /// Permission denied
    PermissionDenied,
    /// RPC timed out
    RpcTimeout,
    /// RPC gave garbage back
    RpcMalformedResponse,
}
```

The `EthAction::SubscribeLogs` request will receive a response of `EthResponse::Ok` if the subscription was successfully created, or `EthResponse::Err(EthError)` if it was not.
Then, after the subscription is successfully created, the process will receive *Requests* from `eth:distro:sys` containing subscription updates.
That request will look like this:

```rust
/// Incoming `Request` containing subscription updates or errors that processes will receive.
/// Can deserialize all incoming requests from eth:distro:sys to this type.
///
/// Will be serialized and deserialized using `serde_json::to_vec` and `serde_json::from_slice`.
pub type EthSubResult = Result<EthSub, EthSubError>;

/// Incoming type for successful subscription updates.
#[derive(Debug, Serialize, Deserialize)]
pub struct EthSub {
    pub id: u64,
    pub result: SubscriptionResult,
}

/// If your subscription is closed unexpectedly, you will receive this.
#[derive(Debug, Serialize, Deserialize)]
pub struct EthSubError {
    pub id: u64,
    pub error: String,
}
```

Again, for most processes, this is the entire API.
The `eth` portion of the `process_lib` library will handle the serialization and deserialization of these types and provide a set of functions and types that are much easier to use.

### Config API

If a process has the `root` capability from `eth:distro:sys`, it can send `EthConfigAction` requests.
These actions are used to adjust the underlying providers and relays used by the module, and its settings regarding acting as a relayer for other nodes (public/private/granular etc).

The configuration of the ETH provider is persisted across two files named `.eth_providers` and `.eth_access_settings` in the node's home directory. `.eth_access_settings` is only created if the configuration is set past the default (private, empty allow/deny lists).

```rust
/// The action type used for configuring eth:distro:sys. Only processes which have the "root"
/// capability from eth:distro:sys can successfully send this action.
#[derive(Debug, Serialize, Deserialize)]
pub enum EthConfigAction {
    /// Add a new provider to the list of providers.
    AddProvider(ProviderConfig),
    /// Remove a provider from the list of providers.
    /// The tuple is (chain_id, node_id/rpc_url).
    RemoveProvider((u64, String)),
    /// make our provider public
    SetPublic,
    /// make our provider not-public
    SetPrivate,
    /// add node to whitelist on a provider
    AllowNode(String),
    /// remove node from whitelist on a provider
    UnallowNode(String),
    /// add node to blacklist on a provider
    DenyNode(String),
    /// remove node from blacklist on a provider
    UndenyNode(String),
    /// Set the list of providers to a new list.
    /// Replaces all existing saved provider configs.
    SetProviders(SavedConfigs),
    /// Get the list of current providers as a [`SavedConfigs`] object.
    GetProviders,
    /// Get the current access settings.
    GetAccessSettings,
    /// Get the state of calls and subscriptions. Used for debugging.
    GetState,
}

pub type SavedConfigs = HashSet<ProviderConfig>;

/// Provider config. Can currently be a node or a ws provider instance.
#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct ProviderConfig {
    pub chain_id: u64,
    pub trusted: bool,
    pub provider: NodeOrRpcUrl,
}

#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub enum NodeOrRpcUrl {
    Node {
        kns_update: crate::core::KnsUpdate,
        use_as_provider: bool, // false for just-routers inside saved config
    },
    RpcUrl(String),
}
```

`EthConfigAction` requests should always expect a response. The response body will look like this:
```rust
/// Response type from an [`EthConfigAction`] request.
#[derive(Debug, Serialize, Deserialize)]
pub enum EthConfigResponse {
    Ok,
    /// Response from a GetProviders request.
    /// Note the [`crate::core::KnsUpdate`] will only have the correct `name` field.
    /// The rest of the Update is not saved in this module.
    Providers(SavedConfigs),
    /// Response from a GetAccessSettings request.
    AccessSettings(AccessSettings),
    /// Permission denied due to missing capability
    PermissionDenied,
    /// Response from a GetState request
    State {
        active_subscriptions: HashMap<crate::core::Address, HashMap<u64, Option<String>>>, // None if local, Some(node_provider_name) if remote
        outstanding_requests: HashSet<u64>,
    },
}

/// Settings for our ETH provider
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessSettings {
    pub public: bool,           // whether or not other nodes can access through us
    pub allow: HashSet<String>, // whitelist for access (only used if public == false)
    pub deny: HashSet<String>,  // blacklist for access (always used)
}
```

A successful `GetProviders` request will receive a response of `EthConfigResponse::Providers(SavedConfigs)`, and a successful `GetAccessSettings` request will receive a response of `EthConfigResponse::AccessSettings(AccessSettings)`.
The other requests will receive a response of `EthConfigResponse::Ok` if they were successful, or `EthConfigResponse::PermissionDenied` if they were not.

All of these types are serialized to a JSON string via `serde_json` and stored as bytes in the request/response body.
[The source code for this API can be found in the `eth` section of the Kinode runtime library.](https://github.com/kinode-dao/kinode/blob/main/lib/src/eth.rs)
