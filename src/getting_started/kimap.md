# Kimap and KNS

Kimap is an onchain namespace for the Kinode operating system.
It serves as the base-level shared global state that all nodes use to share critical signaling data with the entire network.
Kimap is organized as a hierarchical path system and has mutable and immutable keys.

Historically, discoverability of both *peers* and *content* has been a major barrier for peer-to-peer developers.
Discoverability can present both social barriers (finding a new user on a game or chat) and technical obstacles (automatically acquiring networking information for a particular username).
Many solutions have been designed to address this problem, but so far, the ``devex'' (developer experience) of deploying centralized services has continued to outcompete the p2p discoverability options available.
Kimap aims to change this by providing a single, shared, onchain namespace that can be used to resolve to arbitrary elements of the Kinode network.

1. All keys are strings containing exclusively characters 0-9, a-z (lowercase), - (hyphen).
2. A key may be one of two types, a name-key or a data-key.
3. Every name-key may create sub-entries directly beneath it.
4. Every name-key is an ERC-721[^1] NFT (non-fungible token), with a connected token-bound account[^2] with a counterfactual address.
5. The implementation of the token-bound account may be set when a name-key is created.
6. If the parent entry of a name-key has a token-bound account implementation set (a "gene"), then the name-key will automatically inherit this implementation.
7. Every name-key may inscribe data in data-keys directly beneath it.
8. A data-key may be mutable (a "note", prepended with `~`) or immutable (a "fact", prepended with `!`).

[^1]: https://eips.ethereum.org/EIPS/eip-721
[^2]: https://ercs.ethereum.org/ERCS/erc-6551

See the Kinode whitepaper for a full specification which goes into detail regarding token-bound accounts, sub-entry management, the use of data keys, and protocol extensibility.

Kimap is tightly integrated into the operating system. At the runtime level, networking identities are verified against the kimap namespace.
In userspace, programs such as the App Store make use of kimap by storing and reading data from it to define global state, such as apps available for download.

## KNS: Kinode Name System

One of the most important features of a peer-to-peer network is the ability to maintain a unique and persistent identity.
This identity must be self-sovereign, unforgeable, and easy to discover by peers.
Kinode uses a PKI (public-key infrastructure) that runs *within* kimap to achieve this.
It should be noted that, in our system, the concepts of `domain`, `identity`, and `username` are identical and interchangeable.

Also important to understanding KNS identities is that other onchain identity protocols can be absorbed and supported by KNS.
The KNS is not an attempt at replacing or competing with existing onchain identity primitives such as ENS and Lens.
This has already been done for ENS protocol.

Kinode names are registered by a wallet and owned in the form of an NFT like any other kimap namespace entry.
They contain metadata necessary to cover both:

- **Domain provenance** - to demonstrate that the NFT owner has provenance of a given Kinode identity.
- **Domain resolution** - to be able to route messages to a given identity on the Kinode network.

It's easy enough to check for provenance of a given Kinode identity.
If you have a Kinode domain, you can prove ownership by signing a message with the wallet that owns the domain.
However, to effectively use your Kinode identity as a domain name for your personal server, KNS domains have routing information, similar to a DNS record, that points to an IP address.

### Domain Resolution

A KNS identity can either be `direct` or `indirect`.
When users first boot a node, they may decide between these two types as they create their initial identity.
Direct nodes share their literal IP address and port in their metadata, allowing other nodes to message them directly.
Again, this is similar to registering a WWW domain name and pointing it at your web server.
However, running a direct node is both technically demanding (you must maintain the ability of your machine to be accessed remotely) and a security risk (you must open ports on the server to the public internet).
Therefore, indirect nodes are the best choice for the majority of users that choose to run their own node.

Instead of sharing their IP and port, indirect nodes simply post a list of _routers_ onchain.
These routers are other _direct_ nodes that have agreed to forward messages to indirect nodes.
When a node wants to send a message to an indirect node, it first finds the node onchain, and then sends the message to one of the routers listed in the node's metadata.
The router is responsible for forwarding the message to the indirect node and similarly forwarding messages from that node back to the network at large.

### Specification Within Kimap

The definition of a node identity in the KNS protocol is any kimap entry that has:

1. A `~net-key` note AND
2. Either:
   a. A `~routers` note OR
   b. An `~ip` note AND at least one of:
      - `~tcp-port` note
      - `~udp-port` note
      - `~ws-port` note
      - `~wt-port` note

Direct nodes are those that publish an `~ip` and one or more of the port notes.
Indirect nodes are those that publish `~routers`.

The data stored at `~net-key` must be 32 bytes corresponding to an Ed25519 public key.
This is a node's signing key which is used across a variety of domains to verify ownership, including in the end-to-end encrypted networking protocol between nodes.
The owner of a namespace entry/node identity may rotate this key at any time by posting a transaction to kimap mutating the data stored at `~net-key`.

The bytes at a `~routers` entry must parse to an array of UTF-8 strings.
These strings should be node identities.
Each node in the array is treated by other participants in the networking protocol as a router for the parent entry.
Routers should themselves be direct nodes.
If a string in the array is not a valid node identity, or it is a valid node identity but not a direct one, that router will not be used by the networking protocol.
Further discussion of the networking protocol specification can be found [here](../system/networking_protocol.md).

The bytes at an `~ip` entry must be either 4 or 16 big-endian bytes.
A 4-byte entry represents a 32-bit unsigned integer and is interpreted as an IPv4 address.
A 16-byte entry represents a 128-bit unsigned integer and is interpreted as an IPv6 address.

Lastly, the bytes at any of the following port entries must be 2 big-endian bytes corresponding to a 16-bit unsigned integer:

1. `~tcp-port` sub-entry
2. `~udp-port` sub-entry
3. `~ws-port` sub-entry
4. `~wt-port` sub-entry

These integers are translated to port numbers.
In practice, port numbers used are between 9000 and 65535.
Ports between 8000-8999 are usually saved for HTTP server use.