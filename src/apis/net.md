# Net API

Most processes will not use this API directly.
Instead, processes will make use of the networking protocol simply by sending messages to processes running on other nodes.
This API is documented, rather, for those who wish to implement their own networking protocol.

The networking API is implemented in the `net:distro:sys` process.

For the specific networking protocol, see the [networking protocol](../system/networking_protocol.md) chapter.
This chapter is rather to describe the message-based API that the `net:distro:sys` process exposes.

`Net`, like all processes and runtime modules, is architected around a main message-receiving loop.
The received `Request`s are handled in one of three ways:

- If the `target.node` is "our domain", i.e. the domain name of the local node, and the `source.node` is also our domain, the message is parsed and treated as either a debugging command or one of the `NetActions` enum.

- If the `target.node` is our domain, but the `source.node` is not, the message is either parsed as the `NetActions` enum, or if it fails to parse, is treated as a "hello" message and printed in the terminal, size permitting. This "hello" protocol simply attempts to display the `message.body` as a UTF-8 string and is mostly used for network debugging.

- If the `source.node` is our domain, but the `target.node` is not, the message is sent to the target using the [networking protocol](../system/networking_protocol.md) implementation.

Let's look at `NetActions`. Note that this message type can be received from remote or local processes.
Different implementations of the networking protocol may reject actions depending on whether they were instigated locally or remotely, and also discriminate on which remote node sent the action.
This is, for example, where a router would choose whether or not to perform routing for a specific node<>node connection.

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NetAction {
    /// Received from a router of ours when they have a new pending passthrough for us.
    /// We should respond (if we desire) by using them to initialize a routed connection
    /// with the NodeId given.
    ConnectionRequest(NodeId),
    /// can only receive from trusted source, for now just ourselves locally,
    /// in the future could get from remote provider
    KnsUpdate(KnsUpdate),
    KnsBatchUpdate(Vec<KnsUpdate>),
    /// get a list of peers we are connected to
    GetPeers,
    /// get the [`Identity`] struct for a single peer
    GetPeer(String),
    /// get the [`NodeId`] associated with a given namehash, if any
    GetName(String),
    /// get a user-readable diagnostics string containing networking inforamtion
    GetDiagnostics,
    /// sign the attached blob payload, sign with our node's networking key.
    /// **only accepted from our own node**
    /// **the source [`Address`] will always be prepended to the payload**
    Sign,
    /// given a message in blob payload, verify the message is signed by
    /// the given source. if the signer is not in our representation of
    /// the PKI, will not verify.
    /// **the `from` [`Address`] will always be prepended to the payload**
    Verify {
        from: Address,
        signature: Vec<u8>,
    }
}

struct KnsUpdate {
    pub name: String, // actual username / domain name
    pub owner: String,
    pub node: String, // hex namehash of node
    pub public_key: String,
    pub ip: String,
    pub port: u16,
    pub routers: Vec<String>,
}
```

This type must be parsed from a request body using MessagePack.
`ConnectionRequest` is sent by remote nodes as part of the WebSockets networking protocol in order to ask a router to connect them to a node that they can't connect to directly.
This is responded to with either an `Accepted` or `Rejected` variant of `NetResponses`.

`KnsUpdate` and `KnsBatchUpdate` both are used as entry point by which the `net` module becomes aware of the Kinode PKI, or KNS.
In the current distro these are only accepted from the local node, and specifically the `kns_indexer` distro package.

`GetPeers` is used to request a list of peers that the `net` module is connected to. It can only be received from the local node.

`GetPeer` is used to request the `Identity` struct for a single peer. It can only be received from the local node.

`GetName` is used to request the `NodeId` associated with a given namehash. It can only be received from the local node.

`GetDiagnostics` is used to request a user-readable diagnostics string containing networking information. It can only be received from the local node.

`Sign` is used to request that the attached blob payload be signed with our node's networking key. It can only be received from the local node.

`Verify` is used to request that the attached blob payload be verified as being signed by the given source. It can only be received from the local node.


Finally, let's look at the type parsed from a `Response`.

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NetResponse {
    Accepted(NodeId),
    Rejected(NodeId),
    /// response to [`NetAction::GetPeers`]
    Peers(Vec<Identity>),
    /// response to [`NetAction::GetPeer`]
    Peer(Option<Identity>),
    /// response to [`NetAction::GetName`]
    Name(Option<String>),
    /// response to [`NetAction::GetDiagnostics`]. a user-readable string.
    Diagnostics(String),
    /// response to [`NetAction::Sign`]. contains the signature in blob
    Signed,
    /// response to [`NetAction::Verify`]. boolean indicates whether
    /// the signature was valid or not. note that if the signer node
    /// cannot be found in our representation of PKI, this will return false,
    /// because we cannot find the networking public key to verify with.
    Verified(bool),
}
```

This type must be also be parsed using MessagePack, this time from responses received by `net`.

In the future, `NetActions` and `NetResponses` may both expand to cover message types required for implementing networking protocols other than the WebSockets one.
