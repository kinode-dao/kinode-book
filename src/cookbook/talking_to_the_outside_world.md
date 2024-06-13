# Talking to the Outside World

Kinode communicates with the Kinode network using the [Kinode Networking Protocol](../networking_protocol.md).
But nodes must also be able to communicate with the outside world.
These recipes will walk through these communication methods.
Briefly, Kinode can speak both HTTP and WebSockets, and can operate as a client or a server for both.
You can find the APIs for [HTTP client](../apis/http_client.md) and [server](../apis/http_server.md), as well as for [WebSockets](../apis/websocket.md) elsewhere.
This document focuses on simple usage examples of each.

## HTTP

### HTTP Client

```rust
{{#include ../code/http_client/http_client/src/lib.rs}}
```

[Full example package](https://github.com/kinode-dao/kinode-book/tree/main/src/code/http_client).

### HTTP Server

```rust
{{#include ../code/http_server/http_server/src/lib.rs}}
```

[Full example package](https://github.com/kinode-dao/kinode-book/tree/main/src/code/http_server).

## WebSockets

## WebSockets Client

The Kinode process:
```rust
{{#include ../code/ws_client/ws_client/src/lib.rs}}
```

An example WS server:
```python
{{#include ../code/ws_client/ws_server.py}}
```

[Full example package & client](https://github.com/kinode-dao/kinode-book/tree/main/src/code/ws_client).

## WebSockets Server

The Kinode process:
```rust
{{#include ../code/ws_server/ws_server/src/lib.rs}}
```

An example WS client:
```python
{{#include ../code/ws_server/ws_client.py}}
```

[Full example package & client](https://github.com/kinode-dao/kinode-book/tree/main/src/code/ws_server).

## WebSockets Server with Reply Type



