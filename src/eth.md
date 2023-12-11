# ETH read and write

## QNS - Ubar Name System - Public Key Infrastructure

### QNSRegistryResolver

The QNS Registry and Resolver are coupled in the same contract, the QNSRegistryResolver. This takes care of issuing nodes on QNS network and recording the basic records necessary to interact as and with nodes on the network. 

At a high level, the following is the important information for identities in the PKI.

1. The public networking key. This is responsible for proving the node is what it says it is. It is used to encrypt packages to be sent and to decrypt packages that are received. When they first connect to each other, nodes engage in an initial handshake ceremony to create an encryption channel between the two of them using both of their public keys.

2. Networking information.
    - Routing information. A node in the network may choose not to directly accept and send packages from its own IP address, instead delegating this role to a specific node in the network that does advertise its own IP information. In this way, a node may maintain an easy connection to the network when its device may change IP addresses frequently, or if they would prefer to conceal their personal IP information. A node may list several routers. 
    - Networking information. When a node decides to send and receive network traffic directly at its own IP, it provides at least some of the following information, maybe more. The minimum is IP and WebSockets port.
        1. IP address
        2. WebSockets port
        3. WebTransport port
        4. TCP port
        5. UDP port