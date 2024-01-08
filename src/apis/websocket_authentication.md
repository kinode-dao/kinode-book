# WebSocket API

In Nectar OS, WebSocket connects are made with a Rust `warp` server in the core `http_server:sys:nectar` process.
Each connection is assigned a `channel_id` that can be bound to a given process using a `WsRegister` message.
The process receives the `channel_id` for pushing data into the WebSocket, and any subsequent messages from that client will be forwarded to the bound process.

## Opening a WebSocket Channel from a Client

To open a WebSocket channel, connect to the main route on the node `/` and send a `WsRegister` message as either text or bytes.

The simplest way to connect from a browser is to use the `@uqbar/client-encryptor-api` like so:

```
const api = new NectarEncryptorApi({
  nodeId: window.our.node, // this is set if the /our.js script is present in index.html
  processId: "my_package:my_package:template.uq",
  onOpen: (_event, api) => {
    console.log('Connected to nectar node')
    // Send a message to the node via WebSocket
    api.send({ data: 'Hello World' })
  },
})
```

`@uqbar/client-encryptor-api` is available here: [https://www.npmjs.com/package/@uqbar/client-encryptor-api](https://www.npmjs.com/package/@uqbar/client-encryptor-api)

Simple JavaScript/JSON example:

```
function getCookie(name) {
    const cookies = document.cookie.split(';');
    for (let i = 0; i < cookies.length; i++) {
        const cookie = cookies[i].trim();
        if (cookie.startsWith(name)) {
            return cookie.substring(name.length + 1);
        }
    }
}

const websocket = new WebSocket("http://localhost:8080/");

const message = JSON.stringify({
    "auth_token": getCookie(`nectar-auth_${nodeId}`),
    "target_process": "my_package:my_package:template.uq",
    "encrypted": false,
});

websocket.send(message);
```

## Handling Incoming WebSocket Messages

Incoming WebSocket messages will be enums of `HttpServerRequest` with type `WebSocketOpen`, `WebSocketPush`, or `WebSocketClose`.

You will want to store the `channel_id` that comes in with `WebSocketOpen` so that you can push data to that WebSocket.
If you expect to have more than one client connected at a time, then you will most likely want to store the channel IDs in a Set (Rust `HashSet`).

With a `WebSocketPush`, the incoming message will be on the `Payload`, accessible with `get_payload()`.

`WebSocketClose` will have the `channel_id` of the closed channel, so that you can remove it from wherever you are storing it.

A full example:

```
fn handle_http_server_request(
    our: &Address,
    message_archive: &mut MessageArchive,
    source: &Address,
    ipc: &[u8],
    channel_ids: &mut HashSet,
) -> anyhow::Result<()> {
    let Ok(server_request) = serde_json::from_slice::<HttpServerRequest>(ipc) else {
        // Fail silently if we can't parse the request
        return Ok(());
    };

    match server_request {
        HttpServerRequest::WebSocketOpen { channel_id, .. } => {
            // Set our channel_id to the newly opened channel
            // Note: this code could be improved to support multiple channels
            channel_ids.insert(channel_id);
        }
        HttpServerRequest::WebSocketPush { .. } => {
            let Some(payload) = get_payload() else {
                return Ok(());
            };

            handle_chat_request(
                our,
                message_archive,
                our_channel_id,
                source,
                &payload.bytes,
                false,
            )?;
        }
        HttpServerRequest::WebSocketClose(_channel_id) => {
          channel_ids.remove(channel_id);
        }
        HttpServerRequest::Http(IncomingHttpRequest { method, raw_path, .. }) => {
            // Handle incoming HTTP requests here
        }
    };

    Ok(())
}
```

## Pushing Data to a Client via WebSocket

Pushing data to a connected WebSocket is very simple. Call the `send_ws_push` function from `process_lib`:

```
pub fn send_ws_push(
    node: String,
    channel_id: u32,
    message_type: WsMessageType,
    payload: Payload,
) -> anyhow::Result<()>
```

`node` will usually be `our.node` (although you can also send a WS push to another node's `http_server`!), `channel_id` is the client you want to send to, `message_type` will be either `WsMessageType::Text` or `WsMessageType::Binary`, and `payload` will be a standard `Payload` with an optional `mime` field and required `bytes` field.

If you would prefer to send the request without the helper function, this is that what `send_ws_push` looks like under the hood:

```
Request::new()
    .target(Address::new(
        node,
        ProcessId::from_str("http_server:sys:nectar").unwrap(),
    ))
    .ipc(
        serde_json::json!(HttpServerRequest::WebSocketPush {
            channel_id,
            message_type,
        })
        .to_string()
        .as_bytes()
        .to_vec(),
    )
    .payload(payload)
    .send()?;
```
