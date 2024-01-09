# Networking Protocol

### 1. Protocol Overview and Motivation

The Nectar networking protocol is designed to be performant, reliable, private, and purely peer-to-peer, while still enabling access for nodes without a static public IP address.

The networking protocol is NOT designed to be all-encompassing, that is, the only way that two Nectar nodes will ever communicate.
Many Nectar runtimes will provide userspace access to HTTP server/client capabilities, TCP sockets, and much more.
Some applications will choose to use such facilities to communicate.
This networking protocol is merely a common language that every Nectar node is guaranteed to speak.
For this reason, it is the protocol on which system processes will communicate, and it will be a reasonable default for most applications.

In order for nodes to attest to their identity without any central authority, all networking information is made available onchain.
Networking information can take two forms: direct or routed.
The former allows for completely direct peer-to-peer connections, and the latter allows nodes without a physical network configuration that permits direct connections to route messages through a peer.

The networking protocol can be implemented in multiple underlying protocols, but it will use WebSockets for the initial implementation.
Since our protocol is encrypted, a secure underlying connection with TLS or HTTPS is never necessary.
WebSockets are prioritized since we expect to quickly build Nectar nodes that run purely in-browser.

### 2. Onchain Networking Information

All nodes must publish an Ed25519 EdDSA networking public key onchain.
A new key transaction may be posted at any time, but because agreement on networking keys is required to establish a connection and send messages between nodes, changes to onchain networking information will temporarily disrupt networking.
Therefore, all nodes must have robust access to the onchain PKI, meaning: multiple backup options and multiple pathways to read onchain data.
Because it may take time for a new networking key to proliferate to all nodes, (anywhere from seconds to days depending on chain indexing access) a node that changes its networking key should expect downtime immediately after doing so.

Nodes that wish to make direct connections must post a URL and port onchain.
The data structure will resemble this:
```rust
"ws_routing": ["147.135.114.167", 9002]
```
Indirect nodes can leave this field null, and instead indicate which other nodes will rout for them.
The full data structure of a network identity onchain will resemble this, where usernames are unique:
```rust
"username": "squid",
"networking_key": "6077987c998066ed7dea3e30555add0523482475c705fb92c0c8e78307b8e62c",
"ws_routing": null,
"allowed_routers": ["loach", ...]
```

*Note: the TLD of the username is not included in the onchain data, as it is set by the registry contract that issues the username.*

Nodes with onchain network information will be referred to as direct nodes, and ones without will be referred to as indirect nodes.

If a node is indirect, it must initiate a connection with at least one of its allowed routers in order to begin networking.
Until such a connection is established, the indirect node is offline.
In practice, an indirect node that wants reliable access to the network should (1) have many routers listed onchain and (2) connect to as many of them as possible on startup.


### 3. Handshake

XX OUT OF DATE


### 4. Sending Messages

XX OUT OF DATE



### 5. Connection Maintenance and Errors

The system’s networking module seeks to abstract away the many complexities of p2p networking from app developers.
To this end, it reduces all networking issues to either Offline or Timeout.

If a peer is direct, i.e. they have networking information published onchain, determining their offline status is simple: try to create a connection and send a message; it will throw an offline error if this message fails. If a message is un-acked for more than the timeout counter, it will throw a timeout.

If a peer is non-public, i.e. they have routers, multiple attempts must be made before either an offline or timeout error is thrown.

