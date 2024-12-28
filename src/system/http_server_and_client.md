# HTTP Server & Client

No server or web services backend would be complete without an HTTP interface.
Kinode can both create and serve HTTP requests.
As a result, Kinode apps can read data from the web (and other Kinodes), and also serve both public and private websites and APIs.
The HTTP server is how most processes in Kinode present their interface to the user, through an authenticated web browser.

The specification for the [server](../apis/http_server.md) and [client](../apis/http_client.md) APIs are available in the API reference.
These APIs are accessible via messaging the [`http-server:distro:sys`](https://github.com/kinode-dao/kinode/blob/main/kinode/src/http/server.rs) and [`http-client:distro:sys`](https://github.com/kinode-dao/kinode/blob/main/kinode/src/http/client.rs) runtime modules, respectively.
The only [`capability`](../system/process/capabilities.md) required to use either process is the one to message it, granted by the kernel.
It is recommended to interact with the `http-server` and `http-client` using the [`kinode_process_lib`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/http/index.html)

WebSocket server/client functionality is presented alongside HTTP.

At startup, the server either:

1. Binds to the port given at the commandline, or
2. Searches for an open port (starting at 8080, if not, then 8081, etc.).

The server then binds this port, listening for HTTP and WebSocket requests.

You can find usage examples [here](../cookbook/talking_to_the_outside_world.md).
See also [`kit new`](../kit/new.md)s `chat` with GUI template which you can create using
```
kit new my-chat --ui
```

## Private and Public Serving

All server functionality can be either private (authenticated) or public.
If a given functionality is public, the Kinode serves HTTP openly to the world; if it is authenticated, you need your node's password so that your node can generate a cookie that grants you access.

## Direct and Indirect Nodes

Since direct nodes are expected to be accessible over IP, their HTTP server is likely to work if the bound port is accessible.
Note that direct nodes will need to do their own IP/DNS configuration, as Kinode doesn't provide any DNS management.

Indirect nodes may not be accessible over IP, so their HTTP server may or may not function outside the local network.
