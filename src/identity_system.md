# Identity System

One of the most important features of a peer-to-peer network is the ability to maintain a unique and persistent identity. This identity must be self-sovereign, unforgeable, and easy to share and index on. Uqbar uses a domain system similar to [ENS](https://ens.domains/) to achieve this.

Like ENS, Uqbar domains (so far we're calling our system `QNS`) are registered by a wallet and owned in the form of an NFT. However, they never expire, and contain metadata necessary to not only show provenance of a given identity, but also route messages to it on the Uqbar network.

What does this look like? It's easy enough to check for provenance of a given identity. If you have a Uqbar domain, you can prove that you own it by signing a message with the wallet that owns it. But to essentially use this like a domain name for your personal server, QNS domains have routing information similar to a DNS record that points at an IP address.

A QNS domain can either be `direct` or `indirect`. This decision is made when you boot your node for the first time and set your networking information using a transaction. Direct nodes share their literal IP address and port in their metadata, which means that other nodes can send them messages directly. Again, this is like registering a WWW domain name and pointing it at your web server. However, running a node like this is both technically demanding and a security risk, so indirect nodes are the best choice for the majority of users that choose to run their own node.

Instead of sharing an IP and port, indirect nodes simply post a list of *routers* onchain. These routers are other *direct* nodes that have agreed to forward messages for them. When a node wants to send a message to an indirect node, it first looks up the node onchain, then sends the message to one of the routers listed in the node's metadata. The router is responsible for forwarding the message to the indirect node and similarly forwarding messages from that node back to the network at large.

For more information about the architectural specifics of the networking protocol, see [Networking Protocol](./networking_protocol.md). The main takeway for the dentity system is that *domain provenance* and *domain resolution* are unified by QNS.

We recognize that there are a massive variety of identities that people choose to operate under, many of which have been around for years. The great thing about NFT-based identities is that they are composable. There are a number of tools to do this, the most basic of which revolve around simply grouping together multiple ID NFTs by nature of them being in the same wallet. QNS provides the utility of Uqbar networking, but can and should be paired with existing identity solutions for things like profile images, social reputation, and more.

So far, like .eth for ENS, the QNS domain space is fixed inside the `.uq` top-level domain. However, we reserve the ability to expand this in the future, and governance of the Uqbar protocol will include the ability to manage this. We're excited to potentially link various TLDs to existing NFT communities and other identity systems.