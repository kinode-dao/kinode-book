# HTTP Client API

See also: [docs.rs for HTTP Client part of `process_lib`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/http/index.html).

**Note: Most processes will not use this API directly. Instead, they will use the [`process_lib`](../process_stdlib/overview.md) library, which papers over this API and provides a set of types and functions which are much easier to natively use. This is mostly useful for re-implementing this module in a different client or performing niche actions unsupported by the library.**

The HTTP client is used for sending and receiving HTTP requests and responses.
It is also used for connecting to a websocket endpoint as a client.
From a process, you may send an `HttpClientAction` to the `http_client:distro:sys` process.
The action must be serialized to JSON and sent in the `body` of a request.
`HttpClientAction` is an `enum` type that includes both HTTP and websocket actions.

```rust
/// Request type that can be shared over WASM boundary to apps.
/// This is the one you send to the `http_client:distro:sys` service.
#[derive(Debug, Serialize, Deserialize)]
pub enum HttpClientAction {
    Http(OutgoingHttpRequest),
    WebSocketOpen {
        url: String,
        headers: HashMap<String, String>,
        channel_id: u32,
    },
    WebSocketPush {
        channel_id: u32,
        message_type: WsMessageType,
    },
    WebSocketClose {
        channel_id: u32,
    },
}
```

The websocket actions, `WebSocketOpen`, `WebSocketPush`, and `WebSocketClose` all require a `channel_id`.
The `channel_id` is used to identify the connection, and must be unique for each connection from a given process.
Two or more connections can have the same `channel_id` if they are from different processes.
`OutgoingHttpRequest` is used to send an HTTP request.

```rust
/// HTTP Request type that can be shared over WASM boundary to apps.
/// This is the one you send to the `http_client:distro:sys` service.
#[derive(Debug, Serialize, Deserialize)]
pub struct OutgoingHttpRequest {
    pub method: String,          // must parse to http::Method
    pub version: Option<String>, // must parse to http::Version
    pub url: String,             // must parse to url::Url
    pub headers: HashMap<String, String>,
    // BODY is stored in the lazy_load_blob, as bytes
    // TIMEOUT is stored in the message expect_response
}
```

All requests to the HTTP client will receive a response of `Result<HttpClientResponse, HttpClientError>` serialized to JSON.
The process can await or ignore this response, although the desired information will be in the `HttpClientResponse` if the request was successful.
An HTTP request will have an `HttpResponse` defined in the [`http_server`](./http_server.md) module.
A websocket request (open, push, close) will simply respond with a `HttpClientResponse::WebSocketAck`.

```rust
/// HTTP Client Response type that can be shared over WASM boundary to apps.
/// This is the one you receive from the `http_client:distro:sys` service.
#[derive(Debug, Serialize, Deserialize)]
pub enum HttpClientResponse {
    Http(HttpResponse),
    WebSocketAck,
}
```

```rust
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum HttpClientError {
    // HTTP errors, may also be applicable to OutgoingWebSocketClientRequest::Open
    #[error("http_client: request is not valid HttpClientRequest: {}.", req)]
    BadRequest { req: String },
    #[error("http_client: http method not supported: {}", method)]
    BadMethod { method: String },
    #[error("http_client: url could not be parsed: {}", url)]
    BadUrl { url: String },
    #[error("http_client: http version not supported: {}", version)]
    BadVersion { version: String },
    #[error("http_client: failed to execute request {}", error)]
    RequestFailed { error: String },

    // WebSocket errors
    #[error("websocket_client: failed to open connection {}", url)]
    WsOpenFailed { url: String },
    #[error("websocket_client: failed to send message {}", req)]
    WsPushFailed { req: String },
    #[error("websocket_client: failed to close connection {}", channel_id)]
    WsCloseFailed { channel_id: u32 },
}
```

The HTTP client can also receive external websocket messages over an active client connection.
These incoming websocket messages are processed and sent as `HttpClientRequest` to the process that originally opened the websocket.
The message itself is accessible with `get_blob()`.

```rust
/// WebSocket Client Request type that can be shared over WASM boundary to apps.
/// This comes from an open websocket client connection in the `http_client:distro:sys` service.
#[derive(Debug, Serialize, Deserialize)]
pub enum HttpClientRequest {
    WebSocketPush {
        channel_id: u32,
        message_type: WsMessageType,
    },
    WebSocketClose {
        channel_id: u32,
    },
}
```
