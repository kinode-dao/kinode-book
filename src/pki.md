# Public Key Infrastructure

## QNS - Ubar Name System - Public Key Infrastructure

### QNSRegistryResolver

The QNS Registry and Resolver are coupled in the same contract, the QNSRegistryResolver. This takes care of issuing nodes on QNS network and recording the basic records necessary to interact as and with nodes on the network. 

At a high level, the following is the important information for identities in the PKI.

1. The public networking key. 
    - The networking public key is used to encrypt and decrpyt communications with other nodes. When they first connect to each other, nodes engage in an initial handshake ceremony to create an encryption channel between the two of them using both of their public keys. It is the credential that assures the correct party is in control of the identity.

2. Networking information.
    - Direct Node
        Networking information. When a node decides to send/receive network traffic to/from all nodes on the network directly, it provides at least some of the following information, maybe more. The minimum is IP and WebSockets port.
            1. IP address
            2. WebSockets port
            3. WebTransport port
            4. TCP port
            5. UDP port
    - Indirect Node or Routed Node
        A node may choose to delegate another node to serve as its entrypoint on the network such that it will not have to reveal its own ip information publicly to any nodes other than its chosen routers. For this option, they set a list of nodes that may fulfill this role for them.

### DotUqRegistrar

The DotUqRegistrar (.uq) is responsible for registering all .uq domain names and authorizing access for alterations to these nodes when a user attempts to change a record for a node on the QNSRegistryResolver. It implements ERC721 tokenization logic for the names it is charged with, so all Uq names are NFTs that may be transferred to and from any address. There is a minimum length of 9 characters for a new name at this time.

It allows users to create subdomains underneath any Uq name they own. Initially this grants them control over the subdomain, as a holder of the parent domain, but they may choose to irreversibly revoke this control if they desire to. This applies at each level of subdomain.