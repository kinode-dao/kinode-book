# Design Philosophy
The following is a high-level overview of NectarOS's design philosophy, along with the rationale for fundamental design choices.

### Decentralized Software Requires a Shared Computing Environment
A single shared computing environment enables software to coordinate directly between users, services, and other pieces of software in a common language.
Therefore, the best way to enable decentralized software is to provide an easy-to-use, general purpose node (that can run on anything from laptops to data centers) that runs the same operating system as all other nodes on the network.
This environment must integrate with existing protocols, blockchains, and services to create a new set of protocols that operate peer-to-peer within the node network.

### Decentralization is Broad
A wide array of companies and services benefit from some amount of decentralized infrastructure, even those operating in a largely centralized context.
Additionally, central authority and centralized data are often essential to the proper function of a particular service, including those with decentralized properties.
The Nectar environment must be flexible enough to serve the vast majority of the decentralization spectrum.

### Blockchains are not Databases
To use blockchains as mere databases would negate their unique value.
Blockchains are consensus tools, and exist in a spectrum alongside other consensus strategies such as Raft, lockstep protocols, CRDTs, and simple gossip.
All of these are valid consensus schemes, and peer-to-peer software, such as that built on Nectar, must choose the correct strategy for a particular task, program, or application.

### Decentralized Software Outcompetes Centralized Software through Permissionlessness and Composability
Therefore, any serious decentralized network must identify and prioritize the features that guarantee permissionless and composable development.
Those features include:
* a persistent software environment (software can run forever once deployed)
* client diversity (more actors means fewer monopolies)
* perpetual backwards-compatibility
* a robust node network that ensures individual ownership of software and data

### Decentralized Software Requires Decentralized Governance
The above properties are achieved by governance.
Successful protocols launched on Nectar will be ones that decentralize their governance in order to maintain these properties.
We believe that systems that don't proactively specify their point of control will eventually centralize, even if unintentionally.
The governance of Nectar itself must be designed to encourage decentralization, playing a role in the publication and distribution of userspace software protocols.
In practice, this looks like an on-chain permissionless App Store.

### Good Products Use Existing Tools
Nectar is a novel combination of existing technologies, protocols, and ideas.
Our goal is not to create a new programming language or consensus algorithm, but to build a new execution environment that integrates the best of existing tools.
Our current architecture relies on the following systems:
* ETH: a trusted execution layer
* Rust: a performant, expressive, and popular programming language
* Wasm: a portable, powerful binary format for executable programs
* Wasmtime: a standalone Wasm runtime

In addition, Nectar is inspired by the [Bytecode Alliance](https://bytecodealliance.org/) and their vision for secure, efficient, and modular software.
We make extensive use of their tools and standards.
