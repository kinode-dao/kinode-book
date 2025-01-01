# Glossary

Kinode uses a variety of technical terms.
The glossary defines those terms.

## Address

[Processes](#process) have a globally-unique [address](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/kinode/process/standard/struct.Address.html) to and from which [messages](#message) can be routed.

## App Store

The Kinode App Store is the place where users download Kinode apps and where devs distribute Kinode [packages](#package).


## Blob

See [LazyLoadBlob](#LazyLoadBlob).

## Capability

Kinode uses [capabilities](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/kinode/process/standard/struct.Capability.html) to restrict what [processes](#process) can do.
A capability is issued by a process (the "issuer") and signed by the [kernel](#kernel).
The holder of a capability can then attach that capability to a message.
The kernel will confirm it is valid by checking the signature.
If valid, it will be passed to the recipient of the message.
Only trust capabilties from [local](#local) holders!
A [remote](#remote) [node](#node) need not follow the rules above (i.e. it may run a modified [runtime](#runtime)).

There are system-level and userspace-level capabilities.

System-level capabilities are of two types:
- `"messaging"`, which allows the holder to send messages to the issuer.
- `"net"`, which allows the holder to send/receive messages over the network to/from remote nodes.

System-level capabilities need not be attached explicitly to messages.
They are requested and granted at process start-time in the [manifest](manifest).

Userspace-level capabilities are defined within a process.
They are issued by that process.
Holders must explictly attach these capabilities to their messages to the issuing process.
The issuer must define the logic that determines what happens if a sender has or does not have a capability.
E.g., the system Contacts app defines capabilities [here](https://github.com/kinode-dao/kinode/blob/main/kinode/packages/contacts/api/contacts%3Asys-v0.wit#L2-L7) and the logic that allows/disallows access given a sender's capabilities [here](https://github.com/kinode-dao/kinode/blob/main/kinode/packages/contacts/contacts/src/lib.rs#L291-L314).

## Inherit

## Kernel

The Kinode microkernel is responsible for:
1. Starting and stopping [processes](#process).
2. Routing [messages](#message).
3. Enforcing [capabilities](#capability).

## Kimap

Kimap is the onchain component of Kinode.
Kimap is a path-value map.
Protocols can be defined on Kimap.

Examples:

The KNS protocol stores contact information for all [Kinodes](#node) in Kimap entries.
That contact information looks like:
1. A public key.
2. Either an IP address or a list of other nodes that will route messages to that node.
The `kns-indexer` Kinode [process](#process) reads the Kimap, looking for these specific path/entries, and then uses that information to contact other nodes offchain.

The Kinode [App Store](#app-store) protocol stores the app metadata URI and hash of that metadata in Kimap entries.
The `app-store` Kinode process reads the Kimap, looking for these specific path/entries, and then uses that information to coordinate:
1. Fetching app information.
2. Finding mirrors to download from (over the Kinode network or HTTP).
3. Confirming those mirrors gave the expected files.
4. Fetching and installing updates, if desired, when made available.

Read more [here](./getting_started/kimap.md).

## Kimap-safe

A String containing only a-z, 0-9, `-`, and, for a publisher [node](#node), `.`.

## LazyLoadBlob

An optional part of a [message](#message) that is "loaded lazily".
The purpose of he [LazyLoadBlob](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/kinode/process/standard/struct.LazyLoadBlob.html) is to avoid the cost of repeatedly bringing large data across the Wasm boundary in a [message-chain](#message-chain) when only the ends of the chain need to access the data.

## Local

Of or relating to our [node](#node).
Contrasts with [remote](#remote).

E.g., a local [process](#process) is one that is running on our node.
[Messages](#message) sent to a local process need not traverse the Kinode network.

Capabilities attached to messages received from a local process can be trusted since the [kernel](#kernel) can be trusted.

## Manifest

## Message

Kinode [processes](#process) communicate with each other by passing messages.
Messages are [addressed](#address) to a local or remote [process](#process), contain some content, and have a variety of associated metadata.
Messages can be [requests](#request) or [responses](#response).
Messages can set off [message-chains](#message-chain) of requests and responses.
A process that sends a request must specify the address of the recipient.
In contrast, a response will be routed automatically to the sender of the most recently-received request in the message-chain that expected a response.

## Message-chain

## Module

A module, or runtime module, is similar to a [process](#process).
[Messages](#message) are [addressed](#address) to and received from a module just like a process.
The difference is that processes are [Wasm components](#wasm-component), which restricts them in a number of ways, e.g., to be single-threaded.
Runtime modules do not have these same restrictions.
As such they provide some useful features for processes, such as access to the Kinode network, a virtual file system, databases, the Ethereum blockchain, an HTTP server and client, etc.

## Node

A node (sometimes referred to as a Kinode) is a server running the Kinode [runtime](#runtime).
It communicates with other nodes over the Kinode network using [message](#message) passing.
It has a variety of [runtime modules](#module) and also runs userspace [processes](#process) which are [Wasm components](#component).

## Package

An "app".
A set of one-or-more [processes](#process) along with one-or-more UIs.
Packages can be distributed using the Kinode [App Store](#app-store).

Packages have a unique [identity](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/kinode/process/standard/struct.PackageId.html).

## PackageId

## Process

Kinode processes are the code bundles that make up userspace.
Kinode processes are [Wasm components](#wasm-component) that use either the [Kinode process WIT file](https://github.com/kinode-dao/kinode-wit/blob/v1.0.0/kinode.wit) or that define their own [WIT](#wit) file that [wraps the Kinode process WIT file](./cookbook/package_apis.md).

Processes have a unique [identity](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/kinode/process/standard/struct.ProcessId.html) and a globally unique [address](#address).

## ProcessId

## Remote

Of or relating to someone else's [node](#node).
Contrasts with [local](#local).

E.g., a remote [process](#process) is one that is running elsewhere.
[Messages](#message) sent to a remote process must traverse the Kinode network.

Capabilities attached to messages received from a remote process cannot be trusted since the [kernel](#kernel) run by that remote node might be modified.
E.g., the hypothetical modified kernel might take all capabilities issued to any process it runs and give it to all processes it runs.

## Request

A [message](#message) that requires the [address](#address) of the recipient.
A [request](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/struct.Request.html) can start off a messsage-chain if the sender sets metadata that indicates it expects a [response](#response).

## Response

A [message](#message) that is automatically routed to the sender of the most recently-received [request](#request) in the [message-chain](#message-chain) that expected a [response](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/struct.Response.html).

## Runtime

## Wasm Component

[The WebAssembly Component Model](https://component-model.bytecodealliance.org/) is a standard that builds on top of WebAssembly and WASI.
Wasm components define their interfaces using [WIT](#wit).
Kinode [processes](#process) are Wasm components.

## WIT

WIT is the [Wasm Interface Type](https://component-model.bytecodealliance.org/design/wit.html).
WIT is used to define the interface for a [Wasm component](#wasm-component).
Kinode [processes](#process) must use either the [Kinode process WIT file](https://github.com/kinode-dao/kinode-wit/blob/v1.0.0/kinode.wit) or define their own WIT file that [wraps the Kinode process WIT file](./cookbook/package_apis.md)
