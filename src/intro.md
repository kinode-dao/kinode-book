# Introduction

Uqbar provides the 4 basic primitives needed for p2p applications. We've identified and built these primitives in order to create robust abstractions, cutting away the usual boilerplate and complexity while preserving flexibility. These 4 basic primitives are:

- Networking: passing messages from peer to peer.

- Filesystem: storing data and persisting it forever.

- Global State: reading shared global state (blockchain) and composing actions with it (transactions).

and most importantly,
- Applications: writing and distributing software that runs on privately-held personal server nodes.

The focus of this book will be how to build and deploy applications on Uqbar. Applications are composed of processes, which hold state and pass messages. Uqbar's microkernel handles the startup and teardown of processes, as well as message-passing between them, both locally and across the network. Processes are programs compiled to WASM which export a single `init()` function. They can be started once and complete immediately, or run forever.

Peers in Uqbar are identified by their onchain username in the "QNS": uQbar Name System, which is modeled after ENS. The modular architecture of the QNS allows for any Ethereum NFT, including ENS names themselves, to be used as an identity, once it is linked to a QNS entry.

Since data persistence and blockchain access are such fundamental primitives for p2p apps, we've built them into the kernel. The filesystem is abstracted away from the developer, and data is automatically persisted across arbitrarily many encrypted remote backups as configured at the user-system-level. Accessing global state in the form of the Ethereum blockchain is easier than ever before, with chain reads and writes taken care of by state-of-the-art built-in system runtime modules.

A few other I/O primitives also come with the kernel: HTTP server and client, and a simple key-value store. All of these tools can be used together to build performant and self-custodied full-stack applications. Finally, by the end of this book, you will learn how to deploy said applications to the Uqbar network, where they will be discoverably by any user with a Uqbar node.
