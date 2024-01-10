# HTTP Client API

*See also: docs.rs for HTTP Client part of process_lib*

**Note: Most processes will not use this API directly. Instead, they will use the [`process_lib`](./process_stdlib/overview.md) library, which papers over this API and provides a set of types and functions which are much easier to natively use. This is mostly useful for re-implementing this module in a different client or performing niche actions unsupported by the library.**

The HTTP client is used by sending and receiving requests and responses.
From a process, you may send an `OutgoingHttpRequest` or a `WebSocketClientAction` to the `http_client:sys:nectar` process.
Both must be serialized to JSON and sent in the `body` of a request.

```rust
/// HTTP Request type that can be shared over WASM boundary to apps.
/// This is the one you send to the `http_client:sys:nectar` service.
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

```rust
/// WebSocket Client Request type that can be shared over WASM boundary to apps.
/// This is the one you send to the `http_client:sys:nectar` service.
#[derive(Debug, Serialize, Deserialize)]
pub enum WebSocketClientAction {
    Open {
        url: String,
        headers: HashMap<String, String>,
        channel_id: u32,
    },
    Push {
        channel_id: u32,
        message_type: WsMessageType,
    },
    Close {
        channel_id: u32,
    },
    Response {
        channel_id: u32,
        result: Result<(), WebSocketClientError>,
    },
}
```

If the HTTP client gets an `OutgoingHttpRequest`, it will send a response back to the sender process.
The response will be a `Result<HttpResponse, HttpClientError>` serialized to JSON. The process can await or ignore this response, although the desired information will be in the `HttpResponse` if the request was successful.

```rust
/// HTTP Response type that can be shared over WASM boundary to apps.
/// Respond to [`IncomingHttpRequest`] with this type.
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    // BODY is stored in the lazy_load_blob, as bytes
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum HttpClientError {
    #[error("http_client: request could not be parsed to HttpRequest: {}.", req)]
    BadRequest { req: String },
    #[error("http_client: http method not supported: {}", method)]
    BadMethod { method: String },
    #[error("http_client: url could not be parsed: {}", url)]
    BadUrl { url: String },
    #[error("http_client: http version not supported: {}", version)]
    BadVersion { version: String },
    #[error("http_client: failed to execute request {}", error)]
    RequestFailed { error: String },
}
```

If the HTTP client gets a `WebSocketClientAction`, it will send a response back to the sender process.
(*TODO: this API is pretty ugly! clean up!*)
The response will be a `Result<(), WebSocketClientError>` serialized to JSON.
(*or some other weird thing?*)
The process can await or ignore this response.

```rust
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum WebSocketClientError {
    #[error("websocket_client: request format incorrect: {}.", req)]
    BadRequest { req: String },
    #[error("websocket_client: url could not be parsed: {}", url)]
    BadUrl { url: String },
    #[error("websocket_client: failed to open connection {}", url)]
    OpenFailed { url: String },
    #[error("websocket_client: failed to send message {}", channel_id)]
    PushFailed { channel_id: u32 },
    #[error("websocket_client: failed to close connection {}", channel_id)]
    CloseFailed { channel_id: u32 },
}
```
