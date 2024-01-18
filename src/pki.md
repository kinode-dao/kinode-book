# Public Key Infrastructure

The following is a high level overview of Kinode's public key infrastructure, the Kinode Identity System, or NDNS.
You can find a more general discussion of the Kinode [identity system](./identity_system.md) here.

## Identity Registration

The NDNS Registry and Resolver are coupled in the same contract, the `NDNSRegistryResolver`.
This contract issues nodes on the NDNS network and records the data.osessary for a node to interact with other nodes.

At a high level, the PKI depends on two elements: public keys and networking information.

1. The networking public key is used to encrypt and decrypt communications with other nodes.
When nodes first connect, they engage in an initial handshake ceremony (TODO: describe) to create an encryption channel using both of their public keys.
It is this credential that verifies the identity of each nodes.
2. Networking information depends on whether a node is direct or routed (for more, see [networking protocol](./networking_protocol.md)).

Direct nodes send and receive networking traffic directly to and from all nodes on the network. In doing so they must provide their IP address and one or more of:
* WebSockets port
* WebTransport port
* TCP port
* UDP port

Indirect nodes instead specify one or more "router" nodes.
These router nodes communicate between indirect nodes and the network at large.

## Name Registration

The `DotNecRegistrar` (AKA `.os`) is responsible for registering all `.os` domain names.
It is also responsible for authorizing alterations to `.os` node records managed by the NDNSRegistryResolver. (Todo: just confused by this)
`DotNecRegistrar` implements ERC721 tokenization logic for the names it is charged with, so all `.os` names are NFTs that may be transferred to and from any address.
There is currently a minimum length of 9 characters for Kinode IDs.

`DotNecRegistrar` allows users to create subdomains underneath any `.os` name they own.
Initially this grants them control over the subdomain, as a holder of the parent domain, but they may choose to irreversibly revoke this control if they desire to.
This applies at each level of subdomain.
