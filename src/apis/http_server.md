# HTTP Server API

See also: [docs.rs for HTTP Server part of `process_lib`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/http/index.html).

**Note: Most processes will not use this API directly. Instead, they will use the [`process_lib`](../process_stdlib/overview.md) library, which papers over this API and provides a set of types and functions which are much easier to natively use. This is mostly useful for re-implementing this module in a different client or performing niche actions unsupported by the library.**

The HTTP server is used by sending and receiving requests and responses.
From a process, you may send an `HttpServerAction` to the `http_server:distro:sys` process.

```rust
/// Request type sent to `http_server:distro:sys` in order to configure it.
/// You can also send [`type@HttpServerAction::WebSocketPush`], which
/// allows you to push messages across an existing open WebSocket connection.
///
/// If a response is expected, all HttpServerActions will return a Response
/// with the shape Result<(), HttpServerActionError> serialized to JSON.
#[derive(Debug, Serialize, Deserialize)]
pub enum HttpServerAction {
    /// Bind expects a lazy_load_blob if and only if `cache` is TRUE. The lazy_load_blob should
    /// be the static file to serve at this path.
    Bind {
        path: String,
        /// Set whether the HTTP request needs a valid login cookie, AKA, whether
        /// the user needs to be logged in to access this path.
        authenticated: bool,
        /// Set whether requests can be fielded from anywhere, or only the loopback address.
        local_only: bool,
        /// Set whether to bind the lazy_load_blob statically to this path. That is, take the
        /// lazy_load_blob bytes and serve them as the response to any request to this path.
        cache: bool,
    },
    /// SecureBind expects a lazy_load_blob if and only if `cache` is TRUE. The lazy_load_blob should
    /// be the static file to serve at this path.
    ///
    /// SecureBind is the same as Bind, except that it forces requests to be made from
    /// the unique subdomain of the process that bound the path. These requests are
    /// *always* authenticated, and *never* local_only. The purpose of SecureBind is to
    /// serve elements of an app frontend or API in an exclusive manner, such that other
    /// apps installed on this node cannot access them. Since the subdomain is unique, it
    /// will require the user to be logged in separately to the general domain authentication.
    SecureBind {
        path: String,
        /// Set whether to bind the lazy_load_blob statically to this path. That is, take the
        /// lazy_load_blob bytes and serve them as the response to any request to this path.
        cache: bool,
    },
    /// Unbind a previously-bound HTTP path
    Unbind { path: String },
    /// Bind a path to receive incoming WebSocket connections.
    /// Doesn't need a cache since does not serve assets.
    WebSocketBind {
        path: String,
        authenticated: bool,
        encrypted: bool,
        extension: bool,
    },
    /// SecureBind is the same as Bind, except that it forces new connections to be made
    /// from the unique subdomain of the process that bound the path. These are *always*
    /// authenticated. Since the subdomain is unique, it will require the user to be
    /// logged in separately to the general domain authentication.
    WebSocketSecureBind {
        path: String,
        encrypted: bool,
        extension: bool,
    },
    /// Unbind a previously-bound WebSocket path
    WebSocketUnbind { path: String },
    /// Processes will RECEIVE this kind of request when a client connects to them.
    /// If a process does not want this websocket open, they should issue a *request*
    /// containing a [`type@HttpServerAction::WebSocketClose`] message and this channel ID.
    WebSocketOpen { path: String, channel_id: u32 },
    /// When sent, expects a lazy_load_blob containing the WebSocket message bytes to send.
    WebSocketPush {
        channel_id: u32,
        message_type: WsMessageType,
    },
    /// When sent, expects a `lazy_load_blob` containing the WebSocket message bytes to send.
    /// Modifies the `lazy_load_blob` by placing into `WebSocketExtPushData` with id taken from
    /// this `KernelMessage` and `kinode_message_type` set to `desired_reply_type`.
    WebSocketExtPushOutgoing {
        channel_id: u32,
        message_type: WsMessageType,
        desired_reply_type: MessageType,
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
        kinode_message_type: MessageType,
        blob: Vec<u8>,
    },
    /// Sending will close a socket the process controls.
    WebSocketClose(u32),
}

/// The possible message types for WebSocketPush. Ping and Pong are limited to 125 bytes
/// by the WebSockets protocol. Text will be sent as a Text frame, with the lazy_load_blob bytes
/// being the UTF-8 encoding of the string. Binary will be sent as a Binary frame containing
/// the unmodified lazy_load_blob bytes.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WsMessageType {
    Text,
    Binary,
    Ping,
    Pong,
    Close,
}
```

This struct must be serialized to JSON and placed in the `body` of a requests to `http_server:distro:sys`.
For actions that take additional data, such as `Bind` and `WebSocketPush`, it is placed in the `lazy_load_blob` of that request.

After handling such a request, the HTTP server will always give a response of the shape `Result<(), HttpServerError>`, also serialized to JSON. This can be ignored, or awaited and handled.

```rust
/// Part of the Response type issued by http_server
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum HttpServerError {
    #[error(
        "http_server: request could not be parsed to HttpServerAction: {}.",
        req
    )]
    BadRequest { req: String },
    #[error("http_server: action expected blob")]
    NoBlob,
    #[error("http_server: path binding error: {:?}", error)]
    PathBindError { error: String },
    #[error("http_server: WebSocket error: {:?}", error)]
    WebSocketPushError { error: String },
}
```

Certain actions will cause the HTTP server to send requests to the process in the future.
If a process uses `Bind` or `SecureBind`, that process will need to field future requests from the HTTP server. The server will handle incoming HTTP protocol messages to that path by sending an `HttpServerRequest` to the process which performed the binding, and will expect a response that it can then send to the client.

**Note: Paths bound using the HTTP server are *always* prefixed by the ProcessId of the process that bound them.**

**Note 2: If a process creates a static binding by setting `cache` to `true`, the HTTP server will serve whatever bytes were in the accompanying `lazy_load_blob` to all GET requests on that path.**

If a process uses `WebSocketBind` or `WebSocketSecureBind`, future WebSocket connections to that path will be sent to the process, which is expected to issue a response that can then be sent to the client.

Bindings can be removed using `Unbind` and `WebSocketUnbind` actions.
Note that the HTTP server module will persist bindings until the node itself is restarted (and no later), so unbinding paths is usually not necessary unless cleaning up an old static resource.

The incoming request, whether the binding is for HTTP or WebSocket, will look like this:
```rust
/// HTTP Request type that can be shared over WASM boundary to apps.
/// This is the one you receive from the `http_server:distro:sys` service.
#[derive(Debug, Serialize, Deserialize)]
pub enum HttpServerRequest {
    Http(IncomingHttpRequest),
    /// Processes will receive this kind of request when a client connects to them.
    /// If a process does not want this websocket open, they should issue a *request*
    /// containing a [`type@HttpServerAction::WebSocketClose`] message and this channel ID.
    WebSocketOpen {
        path: String,
        channel_id: u32,
    },
    /// Processes can both SEND and RECEIVE this kind of request
    /// (send as [`type@HttpServerAction::WebSocketPush`]).
    /// When received, will contain the message bytes as lazy_load_blob.
    WebSocketPush {
        channel_id: u32,
        message_type: WsMessageType,
    },
    /// Receiving will indicate that the client closed the socket. Can be sent to close
    /// from the server-side, as [`type@HttpServerAction::WebSocketClose`].
    WebSocketClose(u32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomingHttpRequest {
    pub source_socket_addr: Option<String>,   // will parse to SocketAddr
    pub method: String,                       // will parse to http::Method
    pub url: String,                          // will parse to url::Url
    pub bound_path: String,                   // the path that was originally bound
    pub headers: HashMap<String, String>,
    pub url_params: HashMap<String, String>, // comes from route-recognizer
    pub query_params: HashMap<String, String>,
    // BODY is stored in the lazy_load_blob, as bytes
}
```

Processes that use the HTTP server should expect to field this request type, serialized to JSON.
The process must issue a response with this structure in the body, serialized to JSON:

```rust
/// HTTP Response type that can be shared over WASM boundary to apps.
/// Respond to [`IncomingHttpRequest`] with this type.
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    // BODY is stored in the lazy_load_blob, as bytes
}
```

This response is only required for HTTP requests.
`WebSocketOpen`, `WebSocketPush`, and `WebSocketClose` requests do not require a response.
If a process is meant to send data over an open WebSocket connection, it must issue a `HttpServerAction::WebSocketPush` request with the appropriate `channel_id`.
Find discussion of the `HttpServerAction::WebSocketExt*` requests in the [extensions document](../process/extensions.md).
