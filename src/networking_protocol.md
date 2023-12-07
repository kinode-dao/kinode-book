# Networking Protocol

Third Draft: revised 9/20/23

### 1. Protocol Overview and Motivation

The Uqbar networking protocol is designed to be performant, reliable, private, and purely peer-to-peer, while still enabling access for nodes that don’t have access to a static public IP address.

The networking protocol is NOT designed to be all-encompassing, that is, the only way that two Uqbar nodes will ever communicate. Many Uqbar runtimes will provide userspace access to HTTP server/client capabilities, TCP sockets, and much more. Therefore, some applications will choose to use such facilities to communicate. This networking protocol is merely a common language that every Uqbar node is guaranteed to speak. For this reason it is the protocol on which system processes will communicate, and it will be a reasonable default for most applications.

In order for nodes to attest to their identity without any central authority, all networking information is made available onchain. Networking information can take two forms: direct or routed. The former allows for completely direct peer-to-peer connections, and the latter allows nodes without a physical network configuration that allows for direct connections to route messages through a peer.

The networking protocol can be implemented in multiple underlying protocols, but it will use WebSockets for the initial implementation. Other options to explore in the future are raw TCP sockets, UDP, and WebRTC. Since our protocol is encrypted, a secure underlying connection with TLS or HTTPS is never necessary. WebSockets are prioritized since we expect to quickly build Uqbar nodes that run purely in-browser.

### 2. Onchain Networking Information

All nodes must publish a networking public key onchain. This is an Ed25519 EdDSA public key. A new key may be posted at any time, but all messages sent across the network after a connection (TODO: between nodes?) is established must be sent with the key known by the target peer. Note that all nodes should have robust (TODO: what does robust mean here?)access to the onchain PKI, which can mostly be done over the network itself but should have fallback “clearweb” endpoints available (TODO: clarify). Because it may take time (TODO: how much time are we talking?) for a new networking key to proliferate, a node that changes its networking key might expect handshakes to fail if it attempts to make a connection right away.

Nodes that wish to make direct connections must post a URL and port onchain. The data structure will look something (TODO: let's avoid somethings) like this:
```rust
    "ws_routing": ["147.135.114.167", 9002]
```
Nodes that will network behind routing can leave this field null, and instead indicate which other nodes will rout for them. The full data structure of a network identity onchain will look something like this, where usernames are unique:
```rust
    "username": "squid",
    "networking_key": "6077987c998066ed7dea3e30555add0523482475c705fb92c0c8e78307b8e62c",
    "ws_routing": null,
    "allowed_routers": ["loach"]
```
Nodes with onchain network information will be referred to as public nodes, and ones without will be referred to as private nodes.

If a node is private, it must initiate a connection with at least one of its allowed routers in order to begin networking. Until such a connection is established, the private node is offline. In practice, a private node that wants reliable access to the network should (1) have many routers listed onchain and (2) connect to as many of them as possible on startup.


### 3. Handshake

XX OUT OF DATE


### 4. Sending Messages

XX OUT OF DATE



### 5. Connection Maintenance and Errors

The system’s networking module seeks to abstract away the many complexities of p2p networking from app developers. To this end, it reduces all networking issues to either Offline or Timeout.

Timeouts are caused by a message not being delivered within X seconds (X is TBD, somewhere between 5 and 15 seconds probably). This measurement applies to individual routing strategies and can stack.

If a peer is public, i.e. they have direct networking information published onchain, determining their offline status is simple: try to create a connection and send a message, throwing an offline error if this fails. If a message is un-acked for more than the timeout counter, throw a timeout.

If a peer is non-public, i.e. they have routers, multiple attempts must be made before either an offline or timeout error is thrown. Routers will forward errors produced by their connection attempt to the target, which is why the Error type is inside the NetworkMessage enum.

