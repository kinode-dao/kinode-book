# HTTP Server & Client

No server or web services backend would be complete without an HTTP interface.
Nectar nodes must be able to both create and serve HTTP requests.
This enables Nectar apps to read data from the web (and other Nectar nodes), and also serve both public and private websites and APIs.
The HTTP server is how most processes in the Nectar OS present their interface to the user, through an authenticated web browser.

The specification for the [server](./apis/http_server.md) and [client](./apis/http_client.md) APIs are available in the API reference.
These APIs are accessible via messaging the `http_server:sys:nectar` and `http_client:sys:nectar` runtime extensions, respectively.
The only [capability](./process-capabilities.md) required to use either process is the one to message it, granted by the kernel.

WebSocket server/client functionality is presented alongside HTTP.

At startup, the server task finds an open port, starting its search at 8080, to bind at and listen for HTTP and WebSocket requests.
All server functionality can be either authenticated or public.
If a given functionality is public, it is presented open to the world.
Note that the configuration of the Nectar node will still determine whether it is accessible over IPv4/IPv6 – Nectar OS does also not provide any DNS management for nodes.
Since direct nodes are expected to be accessible over IP, their HTTP server is likely to work, if the bound port is accessible.
However, indirect nodes are not expected to be accessible over IP, so in the near future, the HTTP server will include a proxying feature to allow indirect nodes to serve HTTP requests.


