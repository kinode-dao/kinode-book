# Processes

### Overview

On Nectar OS, processes are the building blocks for peer-to-peer applications.
The Nectar runtime handles message-passing between processes, plus the startup and teardown of said processes.
This section describes the message design as it relates to processes.

Processes have a globally unique identifier, or "address", composed of four elements.
First, the publisher's node.
Second, the package name.
Third, the process identifier.
Processes spawn with their own identifier: either a developer-selected string or a randomly-generated number as string.
And finally, the node the process is running on (your node).

Package IDs (TODO: link to docs) look like:

```
my_cool_software:my_username.nec
```

Process IDs (TODO: link to docs) look like:

```
process_one:my_cool_software:my_username.nec
8513024814:my_cool_software:my_username.nec
```

Addresses (TODO: link to docs) look like:

```
some_user.nec@process_one:my_cool_software:my_username.nec
```

Processes are compiled to Wasm.
They can be started once and complete immediately, or they can run forever.
They can spawn other processes, and coordinate in arbitrarily complex ways by passing messages to one another.

### Process State

Nectar processes can be stateless or stateful.
In this case, state refers to data that is persisted between process instantiations.
Nodes get turned off, intentionally or otherwise.
The kernel handles rebooting processes that were running previously, but their state is not persisted by default.

Instead, processes elect to persist data, and what data to persist, when desired.
Data might be persisted after every message ingested, after every X minutes, after a certain specific event, or never.
When data is persisted, the kernel saves it to our abstracted filesystem, which not only persists data on disk, but also across arbitrarily many encrypted remote backups as configured at the user-system-level.

This design allows for ephemeral state that lives in-memory, or truly permanent state, encrypted across many remote backups, synchronized and safe. [Read more about filesystem persistence here](./files.md).

### Requests and Responses

Processes communicate by passing messages, of which there are two kinds: `requests` and `responses`.

#### Addressing

When a request or response is received, it has an attached `address`, which consists of: the source of the message, including the ID of the process that produced the request, as well as the ID of the originating node.

The integrity of a source `address` differs between local and remote messages.
If a message is local, the validity of its source is ensured by the local kernel, which can be trusted to label the process ID and node ID correctly.
If a message is remote, only the node ID can be validated (via networking keys associated with each node ID).
The process ID comes from the remote kernel, which could claim any process ID.
This is fine — merely consider remote process IDs a *claim* about the initiating process rather than an infallible ID like in the local case.

#### Please Respond

Requests can be issued at any time by a running process.
A request can optionally expect a response.
If it does, the request will be retained by the kernel, along with an optional `context` object created by the request's issuer.
A request will be considered outstanding until the kernel receives a matching response, at which point that response will be delivered to the requester alongside the optional `context`.
`context`s allow responses to be disambiguated when handled asynchronously, for example, when some information about the request must be used in handling the response.
Responses can also be handled in an async-await style, discussed [below](#awaiting-a-response).

Requests that expect a response set a timeout value, after which, if no response is received, the initial request is returned to the process that issued it as an error.
[Send errors](#errors) are handled in processes alongside other incoming messages.

##### Inheriting a Response

If a process receives a request, that doesn't mean it must directly issue a response.
The process can instead issue request(s) that "inherit" from the incipient request, continuing its lineage.
If a request does not expect a response and also "inherits" from another request, responses to the child request will be returned to the parent request's issuer.
This allows for arbitrarily complex request-response chains, particularly useful for "middleware" processes.

There is one other use of inheritance, discussed below: [passing data in request chains cheaply](#inheriting-a-lazy_load_blob).

##### Awaiting a Response

When sending a request, a process can await a response to that specific request, queueing other messages in the meantime.
Awaiting a response leads to easier-to-read code:
* The response is handled in the next line of code, rather than in a separate iteration of the message-handling loop
* Therefore, the `context` need not be set.
The downside of awaiting a response is that all other messages to a process will be queued until that response is received and handled.

As such, certain applications lend themselves to blocking with an await, and others don't.
A rule of thumb is: await responses (because simpler code) except when a process needs to performantly handle other messages in the meantime.

For example, if a file-transfer process can only transfer one file at a time, requests can simply await responses, since the only possible next message will be a response to the request just sent.
In contrast, if a file-transfer process can transfer more than one file at a time, requests that await responses will block others in the meantime; for performance it may make sense to write the process fully asynchronously.
The constraint on awaiting is a primary reason why it is desirable to [spawn child processes](#spawning-child-processes).
Continuing the file-transfer example, by spawning one child "worker" process per file to be transferred, each worker can use the await mechanic to simplify the code, while not limiting performance.

#### Message Structure

Messages, both requests and responses, can contain arbitrary data, which must be interpreted by the process that receives it.
The structure of a message contains hints about how best to do this:

First, messages contain a field labeled `body`, which holds the actual contents of the message.
In order to cross the Wasm boundary and be language-agnostic, the `body` field is simply a byte vector.
To achieve composability between processes, a process should be very clear, in code and documentation, about what it expects in the `body` field and how it gets parsed, usually into a language-level struct or object.

A message also contains a `lazy_load_blob`, another byte vector, used for opaque, arbitrary, or large data.
`lazy_load_blob`s, along with being suitable location for miscellaneous message data, are an optimization for shuttling messages across the Wasm boundary.
Unlike other message fields, the `lazy_load_blob` is only moved into a process if explicitly called with (`get_blob()`).
Processes can thus choose whether to ingest a `lazy_load_blob` based on the `body`/`metadata`/`source`/`context` of a given message.
`lazy_load_blob`s hold bytes alongside a `mime` field for explicit process-and-language-agnostic format declaration, if desired.
See [inheriting a `lazy_load_blob`](#inheriting-a-lazy_load_blob) for a discussion of why lazy loading is useful.

Lastly, messages contain an optional `metadata` field, expressed as a JSON-string, to enable middleware processes and other such things to manipulate the message without altering the IPC itself.

##### Inheriting a `lazy_load_blob`

The reason `lazy_load_blob`s are not automatically loaded into a process is that an intermediate process may not need to access the blob.
If process A sends a message with a blob to process B, process B can send a message that inherits to process C.
If process B does not attach a new `lazy_load_blob` to that inheriting message, the original blob from process A will be attached and accessible to C.

For example, consider again the file-transfer process discussed [above](#awaiting-a-response).
Say one node, `send.nec`, is transferring a file to another node, `recv.nec`.
The process of sending a file chunk will look something like:
1. `recv.nec` sends a request for chunk N
2. `send.nec` receives the request and itself makes a request to the filesystem for the piece of the file
3. `send.nec` receives a response from the filesystem with the piece of the file in the `lazy_load_blob`;
   `send.nec` sends a response that inherits the blob back to `recv.nec` without itself having to load the blob, saving the compute and IO required to move the blob across the Wasm boundary.

This is the second functionality of inheritance; the first is discussed above: [eliminating the need for bucket-brigading of responses](#inheriting-a-response).

#### Errors

Messages that result in networking failures, like requests that timeout, are returned to the process that created them as an error.
There are only two kinds of send errors: Offline and Timeout.
Offline means a message's remote target definitively cannot be reached.
Timeout is multi-purpose: for remote nodes, it may indicate compromised networking; for both remote and local nodes, it may indicate that a process is simply failing to respond in the required time.

A send error will return to the originating process the initial message, along with any optional `context`, so that the process can re-send the message, crash, or otherwise handle the failure as the developer desires.
If the error results from a response, the process may optionally try to re-send a response: it will be directed towards the original outstanding request.

### Capabilities

Processes must acquire capabilities from the kernel in order to perform certain operations.
Processes themselves can also produce capabilities in order to give them to other processes.
For more information about the general capabilities-based security paradigm, [insert link to good article here].

The kernel gives out capabilities that allow a process to message another *local* process.
It also gives a capability allowing processes to send and receive messages over the network.
A process can optionally mark itself as `public`, meaning that it can be messaged by any *local* process regardless of capabilities.

[See the capabilities chapter for more details.](./process-capabilities.md)

### Spawning child processes

A process can spawn "child" processes -- in which case the spawner is known as the "parent".
As discussed [above](#awaiting-a-response), one of the primary reasons to write an application with multiple processes is to enable both simple code and high performance.

Child processes can be used to:
1. Run code that may crash without risking crashing the parent
2. Run compute-heavy code without blocking the parent
3. Run IO-heavy code without blocking the parent
4. Break out code that is more easily written with awaits to avoid blocking the parent

### Conclusion

This is a high-level overview of process semantics.
In practice, processes are combined and shared in **packages**, which are generally synonymous with **apps**.

It's briefly discussed here that processes are compiled to Wasm.
The details of this are not covered in the Nectar Book, but can be found in the documentation for the [Nectar runtime](https://github.com/uqbar-dao/nectar), which uses [Wasmtime](https://wasmtime.dev/), a WebAssembly runtime, to load, execute, and provide an interface for the subset of Wasm processes that are valid Nectar processes.
Pragmatically, processes can be compiled using the [`necdev` tools](https://github.com/uqbar-dao/necdev).
The long term goal of the Nectar runtime is to use [WASI](https://wasi.dev/) to provide a secure, sandboxed environment for processes to not only make use of the kernel features described in this document, but also to make full use of the entire WebAssembly ecosystem, including the ability to use sandboxed system calls provided by the host via WASI.
