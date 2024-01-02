# Public Key Infrastructure

The following is a high level overview of Uqbar's public key infrastucture, the Uqbar Identity System, or QNS. 
You can find a more general discussion of the Uqbar [identity system](./identity_system.md) here.

## Identity Registration

The QNS Registry and Resolver are coupled in the same contract, the `QNSRegistryResolver`. 
This contract issues nodes on the QNS network and records the data necessary to for a node to interact with other nodes.  

At a high level, the PKI depends on two elements: public keys and networking information. 

1. The networking public key is used to encrypt and decrpyt communications with other nodes. 
When nodes first connect, they engage in an initial handshake ceremony (TODO: describe) to create an encryption channel using both of their public keys. 
It is this credential that verifies the identity of each nodes.

2. Networking information depends on whether a node is direct or routed (for more, see [networking protocol](./networking_protocol.md)). 

Direct nodes send and receive networking traffic directly to and from all nodes on the network. In doing so they must provide their: 
* IP address
* WebSockets port
* WebTransport port
* TCP port
* UDP port

Indirect nodes instead delegate node to serve as its entrypoint on the network such that it need not publicly reveal its own IP information to any nodes other than its chosen routers. For this option, they set a list of nodes that may fulfill this role for them.

## Name Registration 

The `DotUqRegistrar` (AKA `.uq`) is responsible for registering all .uq domain names and authorizing alterations to these nodes when a user attempts to change a record for a node on the QNSRegistryResolver. (Todo: just confused by this)
It implements ERC721 tokenization logic for the names it is charged with, so all Uq names are NFTs that may be transferred to and from any address. 
There is currently a minimum length of 9 characters for Uqbar IDs.

It allows users to create subdomains underneath any Uq name they own. 
Initially this grants them control over the subdomain, as a holder of the parent domain, but they may choose to irreversibly revoke this control if they desire to. 
This applies at each level of subdomain.