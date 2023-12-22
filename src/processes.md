# Processes

### Overview

On Uqbar, processes are the building blocks for peer-to-peer applications. The Uqbar "microkernel" exlusively handles message-passing between `processes`, plus the startup and teardown of said processes. This section describes the message design as it relates to processes. Processes spawn with an ID: either a developer-selected string or a randomly-generated number as string. This identifier is put into a namespace under the package name that it was installed as (TODO: clarify "put into a namespace" and "under the package name it was installed as), and to generate a globally-unique identifier for a process, it (TODO: the package name?) is combined with the name of the node that it is running on to build an `address`.

Package IDs (TODO: link to docs) look like this:

`my_cool_software:my_username.uq`

Process IDs (TODO: link to docs) look like this:

`process_one:my_cool_software:my_username.uq`

`8513024814:my_cool_software:my_username.uq`

Addresses (TODO: link to docs) look like this:

`some_user.uq@process_one:my_cool_software:my_username.uq`

Processes are compiled to Wasm. They can be started once and complete immediately, or they can run forever. They can spawn other processes, and coordinate in arbitrarily complex ways by passing messages to one another.

### Process State

Uqbar processes can be stateless or stateful. In this case, state refers to data that is persisted between process instantiations. Nodes get turned off, intentionally or otherwise. The kernel handles rebooting processes that were running previously, but their state is not persisted by default.

Instead, processes elect to persist data, and what data to persist, when desired. Data might be persisted after every message ingested, after every X minutes, after a certain specific event, or never. When data is persisted, the kernel saves it to our abstracted filesystem, which not only persists data on disk, but also across arbitrarily many encrypted remote backups as configured at the user-system-level.

This design allows for ephemeral state that lives in-memory, or truly permanent state, encrypted across many remote backups, synchronized and safe. [Read more about filesystem persistence here](./filesystem.md).

### Requests and Responses

Processes communicate by passing messages, of which there are two kinds: `requests` and `responses`.

When a request or response is received, it has an attached `address`, which consists of: the source of the message, including the ID of the process that produced the request, as well as the ID of the originating node.

The integrity of a source `address` differs between local and remote messages. If a message is local, the validity of its source is ensured by the local kernel, which can be trusted to label the process ID and node ID correctly. If a message is remote, only the node ID can be validated (via networking keys associated with each node ID). The process ID comes from the remote kernel, which could claim any process ID. (This is fine -- think of remote process IDs as a marker of what protocol is being spoken, rather than a discrete piece of code written by a specific developer.) (TODO: This justification isn't clear to me, and neither is the importance of the difference in source address "integrity")

Requests can be issued at any time by a running process. A request can optionally expect a response. If it does, the request will be retained by the kernel, along with an optional `context` object created by the request's issuer. This request will be considered outstanding until the kernel receives a matching response, at which point that response will be delivered to the requester alongside the optional context. Contexts saved by the kernel enable very straightforward, async-await-style code inside processes.

Requests that expect a response set a timeout value, after which, if no response is received, the initial request is returned to the process that issued it as an error. Send errors are handled in processes alongside other incoming messages.

If a process receives a request, that doesn't mean it must directly issue a response. The process can instead issue request(s) that inherit the context of the incipient request. If a request inherits context from another request, its responses to the child request will be returned to the parent request's issuer. This allows for arbitrarily complex request-response chains, particularly useful for "middleware" processes.

Messages, both requests and responses, can contain arbitrary data, which must be interpreted by the process that receives it. The structure of a message contains hints about how best to do this:

First, messages contain a field labeled `ipc`. In order to cross the Wasm boundary and be language-agnostic, this (TODO: the contents of the ipc field?) is simply a byte vector. To achieve composability (TODO: here meaning the ability to cross the WASM boundary?), a process should be very clear, in code and documentation, about what it expects in the `ipc` field and how it gets parsed, usually into a language-level struct or object. (TODO: unclear to me what types of things might go in the ipc field and exactly HOW it helps messages cross the boundary)

A message also contains a `payload`, another byte vector, used for opaque, arbitrary, or large data. Payloads, along with being a sort of "backup" field, are an optimization for shuttling messages across the Wasm boundary (TODO: this whole section is a bit blurry for me—I think maybe easiest for us to talk it out, but unclear to me WHAT exactly goes in the ipc/payload fields and how it is used). Unlike other message fields, the payload is only moved into a process if explicitly called with (`get_payload()`). Processes can thus choose whether to ingest a payload based on the ipc/metadata/source/context of a given message. Payloads hold bytes alongside a `mime` field for explicit process-and-language-agnostic format declaration, if desired.

Lastly, messages contain an optional `metadata` field, expressed as a JSON-string, to enable middleware processes and other such things to manipulate the message without altering the IPC itself.

Messages that result in networking failures, like requests that time out, are returned to the process that created them as an error. There are only two kinds of send errors: Offline and Timeout. Offline means a message's remote target definitively cannot be reached. Timeout is multi-purpose: for remote nodes, it may indicate compromised networking; for both remote and local nodes, it may indicate that a process is simply failing to respond in the required time. 

A send error will return to the originating process the initial message, along with and saved `context`, so that the process can re-send the message, crashing, or otherwise handle the failure as the developer desires. If the error results from a response, the process may optionally try to re-send a response again (TODO: the same response?): it will be directed towards the original outstanding request.

### Capabilities

Processes must acquire capabilities from the kernel in order to perform certain operations. Processes themselves can also produce capabilities in order to give them to other processes. For more information about the general capabilities-based security paradigm, [insert link to good article here].

The kernel gives out capabilities that allow a process to message another *local* process. It also gives a capability allowing processes to send and receive messages over the network. A process can optionally mark itself as `public`, meaning that it can be messaged by any *local* process regardless of capabilities.

[See the capabilities chapter for more details.](./process-capabilities.md)

### Conclusion

This is a high-level overview of process semantics. In practice, processes are combined and shared in **packages**, which are generally synonymous with **apps**.

It's briefly discussed here that processes are compiled to Wasm. The details of this are not covered in the Uqbar Book, but can be found in the documentation for the [Uqbar runtime](https://github.com/uqbar-dao/uqbar), which uses [Wasmtime](https://wasmtime.dev/), a WebAssembly runtime, to load, execute, and provide an interface for the subset of Wasm processes that are valid Uqbar processes. The long term goal of the Uqbar runtime is to use [WASI](https://wasi.dev/) to provide a secure, sandboxed environment for processes to not only make use of the kernel features described in this document, but also to make full use of the entire WebAssembly ecosystem, including the ability to use sandboxed system calls provided by the host via WASI.
