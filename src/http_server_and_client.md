# HTTP Server & Client

No server or web services backend would be complete without an HTTP interface.
Kinodes must be able to both create and serve HTTP requests.
This enables Kinode apps to read data from the web (and other Kinodes), and also serve both public and private websites and APIs.
The HTTP server is how most processes in the Kinode present their interface to the user, through an authenticated web browser.

The specification for the [server](./apis/http_server.md) and [client](./apis/http_client.md) APIs are available in the API reference.
These APIs are accessible via messaging the [`http_server:distro:sys`](https://github.com/kinode-dao/kinode/blob/main/kinode/src/http/server.rs) and [`http_client:distro:sys`](https://github.com/kinode-dao/kinode/blob/main/kinode/src/http/client.rs) runtime modules, respectively.
The only [capability](./process/capabilities.md) required to use either process is the one to message it, granted by the kernel.

WebSocket server/client functionality is presented alongside HTTP.

At startup, the server either:

1. Binds to the port given at the commandline, or
2. Searches for an open port (starting at 8080, if not, then 8081, etc.).

The server then binds this port, listening for HTTP and WebSocket requests.

You can find usage examples [here](./cookbook/talking_to_the_outside_world.md).

## Private and Public Serving

All server functionality can be either private (authenticated) or public.
If a given functionality is public, the Kinode serves HTTP openly to the world; if it is authenticated, you need your node's password so that your node can generate a cookie that grants you access.

## Direct and Indirect Nodes

Since direct nodes are expected to be accessible over IP, their HTTP server is likely to work if the bound port is accessible.
Note that direct nodes will need to do their own IP/DNS configuration, as Kinode doesn't provide any DNS management.

However, Kinode provides indirect nodes for users who don't want to do this config, as indirect nodes are not expected to be accessible over IP. For more, see [Domain Resolution](https://book.kinode.org/identity_system.html#domain-resolution).
