# Introduction

Uqbar is a peer-to-peer app framework and node network that provides the four basic primitives needed for p2p applications. We've identified and built these primitives in order to create robust abstractions (TODO: clarify), cutting away the usual boilerplate and complexity of p2p software development while preserving flexibility. These four basic primitives are:

- Networking: passing messages from peer to peer.

- Filesystem: storing data and persisting it forever.

- Global State: reading shared global state (blockchain) and composing actions with this state (transactions).

and most importantly,
- Applications: writing and distributing software that runs on privately-held personal server nodes.

The focus of this book will be how to build and deploy applications on Uqbar. Applications are composed of processes, which hold state and pass messages. Uqbar's microkernel handles the startup and teardown of processes, as well as message-passing between processes, both locally and across the network. Processes are programs compiled to wasm, which export a single `init()` function. They can be started once and complete immediately, or they can run "forever".

Peers in Uqbar are identified by their onchain username in the "QNS": uQbar Name System, which is modeled after ENS. The modular architecture of the QNS allows for any Ethereum NFT, including ENS names themselves, to generate a unique Uqbar idenity once it is linked to a QNS entry.

Data persistence and blockchain access, as fundamental primitieves for p2p apps, are built directly into the kernel. The filesystem is abstracted away from the developer, and data is automatically persisted across an arbitrary number of encrypted remote backups as configured at the user-system-level. Accessing global state in the form of the Ethereum blockchain is now trivial, with chain reads and writes handled by built-in system runtime modules.

Several other I/O primitives also come with the kernel: an HTTP server and client framework, as well as a simple key-value store. Together, these tools can be used to build performant and self-custodied full-stack applications.

Finally, by the end of this book, you will learn how to deploy said applications to the Uqbar network, where they will be discoverable and installable by any user with an Uqbar node.
