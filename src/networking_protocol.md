# Networking Protocol

Third Draft: revised 9/20/23

### 1. Protocol Overview and Motivation

The Uqbar networking protocol is designed to be performant, reliable, private, and purely peer-to-peer, while still enabling access for nodes that don’t have a static public IP address.

It is NOT designed to be all-encompassing, that is, the only way that two Uqbar nodes will ever communicate. Many Uqbar runtimes will provide userspace access to HTTP server/client capabilities, TCP sockets, and much more. Therefore, some applications will choose to use such facilities to communicate. This networking protocol is merely a common language that every Uqbar node is guaranteed to speak. For this reason it is the protocol on which system processes will communicate, and will be a reasonable default for most applications.

In order for nodes to attest to their identity without any central authority, all networking information is made available onchain. Networking information can take two forms: direct or routed. The former allows for completely direct peer-to-peer connections, and the latter allows nodes without a physical network configuration that allows for direct connections to route messages through a peer.

The networking protocol can be implemented in multiple underlying protocols, but we have chosen WebSockets for the initial implementation. Other options to explore in the future are raw TCP sockets, UDP, and WebRTC. Since our protocol is encrypted, a secure underlying connection with TLS or HTTPS is never necessary. WebSockets are prioritized since we expect to quickly build Uqbar nodes that run purely in-browser.

### 2. Onchain Networking Information

All nodes must publish a networking public key onchain. This is an Ed25519 EdDSA public key. A new key can be posted at any time, but all messages sent across the network after a connection is established must be sent with the key known by the target peer. Note that all nodes should have robust access to the onchain PKI, which can mostly be done over the network itself but should have fallback “clearweb” endpoints available. Because it may take time for a new networking key to proliferate, a node that changes its networking key might expect handshakes to fail if it attempts to make a connection right away.

Nodes that wish to make direct connections must post a URL and port onchain. The data structure will look something like this:
```rust
    "ws_routing": ["147.135.114.167", 9002]
```
Nodes that will network behind routing can leave this field null, and instead fill out a field which indicates what other nodes will route for them. The full data structure of a network identity onchain will look something like this, where usernames are unique:
```rust
    "username": "squid",
    "networking_key": "6077987c998066ed7dea3e30555add0523482475c705fb92c0c8e78307b8e62c",
    "ws_routing": null,
    "allowed_routers": ["loach"]
```
Nodes with onchain network information will be referred to as public nodes, and ones without will be referred to as private nodes.

If a node is private, it must initiate a connection with at least one of its allowed routers in order to begin networking. Until such a connection is established, the private node is offline. In practice, a private node that wants reliable access to the network should (1) have many routers listed onchain and (2) connect to as many of them as possible on startup.


### 3. Handshake

There is a handshake sequence that must be performed before sending Uqbar messages, in order to enable symmetric encryption between communicating nodes. Once the handshake is performed, those two nodes can communicate directly or through any router, as long as each one holds onto the handshake data.

If a node receives an encrypted message (see the message format below) that it cannot decrypt, either because it does not have a saved decryption key for the message’s sender, or the one it has did not work, it should respond with a handshake message, which lets the sender know they need to re-shake.

Once a handshake is received, it is responded to with a HandshakeAck, which is the same data structure, but marked as a response so as to never create a looping problem. Handshakes look like this:
```rust
Handshake {
    from: String,
    target: String,
    id_signature: Vec<u8>,
    ephemeral_public_key: Vec<u8>,
    ephemeral_public_key_signature: Vec<u8>,
    nonce: Vec<u8>,
}
```
The nonce field is to be filled out by the node initiating the connection. If filled out by the responding node, it will be ignored. The initiating node can use whatever strategy it wants to generate the nonce.

The id_signature is an EdDSA signature, created by the node’s networking public key, on the byte serialization of the Identity data structure that exists onchain for that node. This ensures that connections agree on the current state of each participant’s PKI information.

The ephemeral_public_key is generated as part of the Handshake and used in a Diffe-Hellman key exchange. This key is also signed by the node’s networking key. By signing this key with its networking key, a node verifies that it is in fact sending all subsequent messages, and the ephemeral key exchange allows all subsequent messages to be encrypted and only decrypted by the target.

The exact WebSockets route taken depends on the public/private nature of both nodes involved.

If both nodes are public, the initiator connects directly to the target. If a public node is connecting to a private node, it must establish a connection with one of its routers, then request a connection to the private node and send them a handshake. The connection-initiating node should try available routers until it successfully establishes a connection. If a private node is connecting to a public node, it can either connect directly, revealing its IP address, or privately through a router using the same method as above.  Two private nodes will always connect through a router.


### 4. Sending Messages

Once both nodes have sent a handshake, the connection is complete. All messages are encrypted using the shared secret created by the Diffe-Hellman key exchange performed in the handshake messages. Different implementations can execute different expiration strategies around handshakes, but if a node changes its onchain routing information, that always triggers a new handshake.

This is the exact message type sent as binary data between nodes:
```rust
NetworkMessage {
    Ack(u64),
    Nack(u64),
    Msg {
        from: String,
        to: String,
        id: u64,
        contents: Vec<u8>,
    },
    Handshake(Handshake),
    HandshakeAck(Handshake),
    Error(NetworkError),
}
```
Acks are used to confirm delivery of a Msg. Nacks are used to confirm that although the message was delivered over the network to *some* node in the chain (either router or destination), it was not successfully decrypted and passed on to the kernel of the target. This could happen due to a disconnect between router and target, a decryption error of some kind, or some kind of invalid message format.

Msg contents are always encrypted.

Handshakes are exchanged in-order, so the initiator sends a Handshake, then the responder sends a HandshakeAck.


### 5. Connection Maintenance and Errors

The system’s networking module seeks to abstract away the many complexities of p2p networking from app developers. To this end, it reduces all networking issues to either Offline or Timeout.

Timeouts are caused by a message not being delivered within X seconds (X is TBD, somewhere between 5 and 15 seconds probably). This measurement applies to individual routing strategies, and can stack.

If a peer is public, i.e. they have direct networking information published onchain, determining their offline status is simple: try to create a connection and send a message, throwing an offline error if this fails. If a message is un-acked for more than the timeout counter, throw a timeout.

If a peer is non-public, i.e. they have routers, multiple attempts must be made before either an offline or timeout error is thrown. Routers will forward errors produced by their connection attempt to the target, which is why the Error type is inside the NetworkMessage enum.

