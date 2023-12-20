# Processes

### Overview

On Uqbar, processes are the building blocks for peer-to-peer applications. The Uqbar "microkernel" exlusively handles message-passing between `processes`, plus the startup and teardown of said processes. The following describes the message design as it relates to processes. Processes spawn with an ID: either a developer-selected string or a randomly-generated number as string. This identifier is namespaced by the package name that it was installed as, and to generate a globally-unique identifier for a process, it is combined with the name of the node that it is running on to build an `address`.

Package IDs (TODO: link to docs) look like this:

`my_cool_software:my_username.uq`

Process IDs (TODO: link to docs) look like this:

`process_one:my_cool_software:my_username.uq`

`8513024814:my_cool_software:my_username.uq`

Addresses (TODO: link to docs) look like this:

`some_user.uq@process_one:my_cool_software:my_username.uq`

Processes are compiled to wasm. They can be started once and complete immediately, or they can run forever. They can spawn other processes, and coordinate in arbitrarily complex ways by passing messages to one another.

### Process State

Uqbar processes can be stateless or stateful. In this case, state refers to data that is persisted between process instantiations. (Of course, it's also possible to consider processes as stateful vs. stateless based on their use of variables in-memory -- this is a separate concept.) Nodes get turned off, intentionally or otherwise. The kernel handles booting processes back up that were running previously, but their state is not persisted by default.

Instead, processes elect to persist data, and what to persist, when desired. Data might be persisted after every message ingested, after every X minutes, after a certain specific event, or never. When data is persisted, the kernel saves it to our abstracted filesystem, which not only persists data on disk, but also across arbitrarily many encrypted remote backups as configured at the user-system-level.

This design allows for ephemeral state that lives in-memory, or truly permanent state, encrypted across many remote backups, synchronized and safe. [Read more about filesystem persistence here](./filesystem.md).

### Requests and Responses

Processes communicate by passing messages, of which there are two kinds: `requests` and `responses`.

When a request or response is received, it has an `address` attached: the source of the message, including the ID of the process that produced the request, as well as the ID of the node itself.

The integrity of a source `address` differs between local and remote messages. If a message is local, it's validated by the local kernel, which can be trusted to label the process ID and node ID correctly. If a message is remote, only the node ID can be validated (via networking keys associated with each node ID). The process ID comes from the remote kernel, which could claim anything. (This is fine -- think of remote process IDs as a marker of what protocol is being spoken, rather than a discrete piece of code written by a specific developer.)

Requests can be issued at any time by a running process. A request can optionally expect a response. If it does, the request will be retained by the kernel, along with an optional `context` object created by the request's issuer. This request will be considered outstanding until the kernel receives a matching response, at which point that response will be delivered to the requester alongside the optional context. Contexts saved by the kernel enable very straightforward, async-await-style code inside processes.

Requests that wish to expect a response set a timeout value, after which, if no response was received, the initial request is returned to the process that issued it as an error. Send errors are handled in processes alongside other incoming messages.

If a process receives a request, that doesn't mean it must directly issue a response. The process can instead issue request(s) that inherit the context of the incipient request. If a request inherits from another request, responses to the child request will be returned to the parent request's issuer. This allows for arbitrarily complex request-response chains, particularly "middleware" processes.

Messages, both requests and responses, can contain arbitrary data, which must be interpreted by the process that receives it. The structure of a message contains ample hints about how best to do this:

First, messages contain a field labeled `ipc`. In order to cross the wasm boundary and be language-agnostic, this is simply a byte vector. To achieve composability, a process should be very clear, in code and documentation, about what it expects in the `ipc` field and how it gets parsed, usually into a language-level struct or object.

A message also contains a `payload`, another byte vector, used for opaque, arbitrary, or large data. Payloads, along with being a sort of "backup" field, are an optimization for shuttling messages across the wasm boundary. Unlike other message fields, the payload is only moved into a process if explicitly called (`get_payload()`). Processes can thus choose whether to ingest a payload based on the ipc/metadata/source/context of a given message. Payloads hold bytes alongside a `mime` field for explicit process-and-language-agnostic format declaration, if desired.

Lastly, messages contain an optional `metadata` field, expressed as a JSON-string, to enable middleware processes and other such things to manipulate the message without altering the IPC itself.

Messages that result in networking failures, like requests that time out, are returned to the process that created them as an error. There are only two kinds of send errors: Offline and Timeout. Offline means a message's remote target definitively cannot be reached. Timeout is multi-purpose: it's possible in the remote case that actual networking with that node is compromised, or that the (remote or local) process is simply taking too long to respond / not responding at all.

A send error will give the process the original message along with saved `context` if any, so the process can handle re-sending, crashing, or otherwise dealing with the failure as it sees fit. If the error comes from a response, the process may optionally try to send a response again: it will be directed towards the original outstanding request.

### Capabilities

Processes must acquire capabilities from the kernel in order to perform certain operations. Processes themselves can also produce capabilities in order to give them to other processes. For more information about the general capabilities-based security paradigm, [insert link to good article here].

The kernel gives out capabilities that allow a process to message another *local* process. It also gives a capability to a process allowing that process to send and receive messages over the network. A process can optionally mark itself as `public`, meaning that it can be messaged by any *local* process regardless of capabilities.

[See the capabilities chapter for more details.](./process-capabilities.md)

### Conclusion

This is a high-level overview of process semantics. In practice, processes are combined and shared in **packages**, which are generally synonymous with **apps**.

It's briefly discussed here that processes are compiled to wasm. The details of this are not covered in the Uqbar Book, but can be found in the documentation for the [Uqbar runtime](https://github.com/uqbar-dao/uqbar), which uses [Wasmtime](https://wasmtime.dev/), a WebAssembly runtime, to load, execute, and provide an interface for the subset of wasm processes that are valid Uqbar processes. The long term goal of the Uqbar runtime is to use [WASI](https://wasi.dev/) to provide a secure, sandboxed environment for processes to not only make use of the kernel features described in this document, but also to make full use of the entire WebAssembly ecosystem, including the ability to use sandboxed system calls provided by the host via WASI.
