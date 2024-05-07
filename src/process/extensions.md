# Extensions

Extensions supplement and compliment Kinode processes.
Kinode processes have many features that make them good computational units, but they also have constraints.
Extensions remove the constraints (e.g., not all libraries can be built to Wasm) while maintaining the advantages (e.g., the integration with the Kinode Request/Response system).
The cost of extensions is that they are not as nicely bundled within the Kinode system: they must be run separately.

## What is an Extension?

Extensions are [WebSocket](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API) clients that connect to a paired Kinode process to extend library, language, or hardware support.

Kinode processes are [Wasm components](https://component-model.bytecodealliance.org/design/why-component-model.html), which leads to advantages and disadvantages.
The rest of the book (and in particular the [processes chapter](./processes.md)) discusses the advantages (e.g., integration with the Kinode Request/Response system and the capabilities security model).
Two of the main disadvantages are:
1. Only certain libraries and languages can be used.
2. Hardware accelerators like GPUs are not easily accessible.

Extensions solve both of these issues, since an extension runs natively.
Any language with any library supported by the bare metal host can be run as long as it can speak WebSockets.

## Downsides of Extensions

Extensions enable use cases that pure processes lack.
However, they come with a cost.
Processes are contained and managed by your Kinode, but extensions are not.
Extensions are independent servers that run alongside your Kinode.
They do not yet have a Kinode-native distribution channel.

As such, extensions should only be used when absolutely necessary.
Processes are more stable, maintainable, and easily upgraded.
Only write an extension if there is no other choice.

## How to Write an Extension?

An extension is composed of two parts: a Kinode package and the extension itself.
They communicate with each other over a WebSocket connection that is managed by Kinode.
Check out the [examples below](#examples) to see some working extensions.

### The WebSocket protocol

The process [binds a WebSocket](#bind-an-extension-websocket), so Kinode acts as the WebSocket server.
The extension acts as a client, connecting to the WebSocket served by the Kinode process.

The process sends [`HttpServerAction::WebSocketExtPushOutgoing`](https://github.com/kinode-dao/kinode/blob/main/lib/src/http/server_types.rs#L123-L142) Requests to the `http_server`(look [here](https://book.kinode.org/http_server_and_client.html
) and [here](https://book.kinode.org/apis/http_server.html)) to communicate with the extension (see the `enum` defined at the bottom of this section).

Table 1: `HttpServerAction::WebSocketExtPushOutgoing` Inputs
Field Name           | Description
-------------------- | -----------
`channel_id`         | Given in a WebSocket message after a client connects.
`message_type`       | The WebSocketMessage type — recommended to be [`WsMessageType::Binary`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/http/enum.WsMessageType.html).
`desired_reply_type` | The Kinode [`Message`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/enum.Message.html) type that the extension should return — `Request` or `Response`.

The [`lazy_load_blob`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/kinode/process/standard/struct.LazyLoadBlob.html) is the payload for the WebSocket message.

The `http_server` converts the Request into a [`HttpServerAction::WebSocketExtPushData`](https://github.com/kinode-dao/process_lib/blob/main/src/http.rs#L129-L148), [MessagePack](https://msgpack.org)s it, and sends it to the extension.
Specifically, it attaches the Message's `id`, copies the `desired_reply_type` to the `kinode_message_type` field, and copies the `lazy_load_blob` to the `blob` field.

The extension replies with a [MessagePack](https://msgpack.org)ed `HttpServerAction::WebSocketExtPushData`.
It should copy the `id` and `kinode_message_type` of the message it is serving into those same fields of the reply.
The `blob` is the payload.

```rust
pub enum HttpServerAction {
    //...
    /// When sent, expects a `lazy_load_blob` containing the WebSocket message bytes to send.
    /// Modifies the `lazy_load_blob` by placing into `WebSocketExtPushData` with id taken from
    /// this `KernelMessage` and `kinode_message_type` set to `desired_reply_type`.
    WebSocketExtPushOutgoing {
        channel_id: u32,
        message_type: WsMessageType,
        desired_reply_type: Message,
    },
    /// For communicating with the ext.
    /// Kinode's http_server sends this to the ext after receiving `WebSocketExtPushOutgoing`.
    /// Upon receiving reply with this type from ext, http_server parses, setting:
    /// * id as given,
    /// * message type as given (Request or Response),
    /// * body as HttpServerRequest::WebSocketPush,
    /// * blob as given.
    WebSocketExtPushData {
        id: u64,
        kinode_message_type: Message,
        blob: Vec<u8>,
    },
    //...
}
```

### The Package

The package is, minimally, a single process that serves as interface between Kinode and the extension. Each extension must come with a corresponding Kinode package. 

Specifically, the interface process must:
1. Bind an extension WebSocket: this will be used to communicate with the extension.
2. Handle Kinode messages: e.g., Requests to be passed to the extension for processing.
3. Handle WebSocket messages: these will come from the extension.

'Interface process' will be used interchangeably with 'package' throughout this page.

#### Bind an Extension WebSocket

The [`kinode_process_lib`](../process_stdlib/overview.md) provides an easy way to bind an extension WebSocket:

```
kinode_process_lib::http::bind_ext_path("/")?;
```

which, for a process with process ID `process:package:publisher.os`, serves a WebSocket server for the extension to connect to at `ws://localhost:8080/process:package:publisher.os`.
Passing a different endpoint like `bind_ext_path("/foo")` will append to the WebSocket endpoint like `ws://localhost:8080/process:package:publisher.os/foo`.

#### Handle Kinode Messages

Like any Kinode process, the interface process must handle Kinode messages.
These are how other Kinode processes will make Requests that are served by the extension:
1. Process A sends Request.
2. Interface process receives Request, optionally does some logic, sends Request on to extension via WS.
3. Extension does computation, replies on WS.
4. Interface process receives Response, optionally does some logic, sends Response on to process A.

The [WebSocket protocol section](#the-websocket-protocol) above discusses how to send messages to the extension over WebSockets.
Briefly, a `HttpServerAction::WebSocketExtPushOutgoing` Request is sent to the `http_server`, with the payload in the `lazy_load_blob`.

It is recommended to use the following protocol:
1. Use the [`WsMessageType::Binary`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/http/enum.WsMessageType.html) WebSocket message type and use MessagePack to (de)serialize your messages.
   [MessagePack](https://msgpack.org) is space-efficient and well supported by a variety of languages.
   Structs, dictionaries, arrays, etc. can be (de)serialized in this way.
   The extension must support MessagePack anyways, since the `HttpServerAction::WebSocketExtPushData` is (de)serialized using it.
2. Set `desired_reply_type` to [`Message::Response`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/enum.Message.html#variant.Response) type.
   Then the extension can indicate its reply is a Response, which will allow your Kinode process to properly route it back to the original requestor.
3. 'If possible, the original requestor should serialize the `lazy_load_blob`, and the type of `lazy_load_blob` should be defined accordingly.'
   Then, all the interface process needs to do is `inherit` the `lazy_load_blob` in its `http_server` Request.
   This increases efficiency since it avoids bringing those bytes across the Wasm boundry between the process and the runtime (see more discussion [here](./processes.md#message-structure)).

#### Handle WebSocket Messages

At a minimum, the interface process must handle:

Table 2: [`HttpServerRequest`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/http/enum.HttpServerRequest.html) Variants
`HttpServerRequest` variant | Description
--------------------------- | -----------
`WebSocketOpen`             | Sent when an extension connects. Provides the `channel_id` of the WebSocket connection, needed to message the extension: store this!
`WebSocketClose`            | Sent when the WebSocket closes. A good time to clean up the old `channel_id`, since it will no longer be used.
`WebSocketPush`             | Used for sending payloads between interface and extension.

Although the extension will send a `HttpServerAction::WebSocketExtPushData`, the `http_server` converts that into a `HttpServerRequest::WebSocketPush`.
The `lazy_load_blob` then contains the payload from the extension, which can either be processed in the interface or `inherit`ed and passed back to the original requestor process.

### The Extension

The extension is, minimally, a WebSocket client that connects to the Kinode interface process.
It can be written in any language and it is run natively on the host as a "side car" — a separate binary.

The extension should first connect to the interface process.
The recommended pattern is to then iteratively accept and process messages from the WebSocket.
Messages come in as MessagePack'd `HttpServerAction::WebSocketExtPushData` and must be replied to in the same format.
The `blob` field is recommended to also be MessagePack'd.
The `id` and `kinode_message_type` should be mirrored by the extension: what it receives in those fields should be copied in its reply.

## Examples

Find some working examples of runtime extensions below:

* [An untrusted Python code runner](https://github.com/hosted-fornet/kinode-python)
* [A framework for evaluating ML models](https://github.com/hosted-fornet/kinode-ml)
