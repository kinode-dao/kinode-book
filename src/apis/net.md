# Net API

Most processes will not use this API directly.
Instead, processes will make use of the networking protocol simply by sending messages to processes running on other nodes.
This API is documented, rather, for those who wish to implement their own networking protocol.

The networking API is implemented in the `net:distro:sys` process.

For the specific networking protocol, see the [networking protocol](../networking_protocol.md) chapter.
This chapter is rather to describe the message-based API that the `net:distro:sys` process exposes.

`Net`, like all processes and runtime modules, is architected around a main message-receiving loop.
The received `Request`s are handled in one of three ways:

- If the `target.node` is "our domain", i.e. the domain name of the local node, and the `source.node` is also our domain, the message is parsed and treated as either a debugging command or one of the `NetActions` enum.

- If the `target.node` is our domain, but the `source.node` is not, the message is either parsed as the `NetActions` enum, or if it fails to parse, is treated as a "hello" message and printed in the terminal, size permitting. This "hello" protocol simply attempts to display the `message.body` as a UTF-8 string and is mostly used for network debugging.

- If the `source.node` is our domain, but the `target.node` is not, the message is sent to the target using the [networking protocol](../networking_protocol.md) implementation.

Let's look at `NetActions`. Note that this message type can be received from remote or local processes.
Different implementations of the networking protocol may reject actions depending on whether they were instigated locally or remotely, and also discriminate on what remote node sent the action.
This is, for example, where a router would choose whether or not to perform routing for a specific node<>node connection.

```rust
enum NetActions {
    /// Received from a router of ours when they have a new pending passthrough for us.
    /// We should respond (if we desire) by using them to initialize a routed connection
    /// with the NodeId in the string given.
    ConnectionRequest(String),
    /// can only receive from trusted source, for now just ourselves locally,
    /// in the future could get from remote provider
    KnsUpdate(KnsUpdate),
    KnsBatchUpdate(Vec<KnsUpdate>),
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


Finally, let's look at the type parsed from a `Response`.

```rust
/// For now, only sent in response to a ConnectionRequest.
enum NetResponses {
    Accepted(NodeId),
    Rejected(NodeId),
}
```

This type must be also be parsed using MessagePack, this time from responses received by `net`.

In the future, `NetActions` and `NetResponses` may both expand to cover message types required for implementing networking protocols other than the WebSockets one.
