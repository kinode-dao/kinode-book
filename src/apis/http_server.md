# HTTP Server API

*See also: docs.rs for HTTP Server part of process_lib*

**Note: Most processes will not use this API directly. Instead, they will use the [`process_lib`](./process_stdlib/overview.md) library, which papers over this API and provides a set of types and functions which are much easier to natively use.**

The HTTP server is used by sending and receiving requests and responses.
From a process, you may send an `HttpServerAction` to the `http_server:sys:nectar` process.

```rust
/// Request type sent to `http_server:sys:nectar` in order to configure it.
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
    /// Bind a path to receive incoming WebSocket connections.
    /// Doesn't need a cache since does not serve assets.
    WebSocketBind {
        path: String,
        authenticated: bool,
        encrypted: bool,
    },
    /// SecureBind is the same as Bind, except that it forces new connections to be made
    /// from the unique subdomain of the process that bound the path. These are *always*
    /// authenticated. Since the subdomain is unique, it will require the user to be
    /// logged in separately to the general domain authentication.
    WebSocketSecureBind { path: String, encrypted: bool },
    /// When sent, expects a lazy_load_blob containing the WebSocket message bytes to send.
    WebSocketPush {
        channel_id: u32,
        message_type: WsMessageType,
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

This struct must be serialized to JSON and placed in the `body` of a requests to `http_server:sys:nectar`.
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

Certain actions will cause the HTTP server to send the process requests in the future.
If a process uses `Bind` or `SecureBind`, future HTTP requests to that path will be sent to the process, which is expected to issue a response that can then be sent to the client.

**Note: Paths bound using the HTTP server are *always* prefixed by the ProcessId of the process that bound them.**

**Note 2: If a process creates a static binding by setting `cache` to `true`, the HTTP server will serve whatever bytes were in the accompanying `lazy_load_blob` to all GET requests on that path.**

If a process uses `WebSocketBind` or `WebSocketSecureBind`, future WebSocket connections to that path will be sent to the process, which is expected to issue a response that can then be sent to the client.

The incoming request, whether the binding is for HTTP or WebSocket, will look like this:
```rust
/// HTTP Request type that can be shared over WASM boundary to apps.
/// This is the one you receive from the `http_server:sys:nectar` service.
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
    pub source_socket_addr: Option<String>, // will parse to SocketAddr
    pub method: String,                     // will parse to http::Method
    pub raw_path: String,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    // BODY is stored in the lazy_load_blob, as bytes
}
```

Processes that use the HTTP server should expect to field this request type, serialized to JSON.
To respond, issue a response with the structure in the body, serialized to JSON:

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
If a process wants to send data over an open WebSocket connection, it should issue a `HttpServerAction::WebSocketPush` request with the appropriate `channel_id`.
