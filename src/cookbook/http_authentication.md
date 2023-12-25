# HTTP API

In Nectar OS, incoming HTTP requests are handled by a Rust `warp` server in the core `http_server:sys:uqbar` process. This process handles binding (registering) routes, simple JWT-based authentication, and serving a `/login` page if auth is missing.

## Binding (Registering) HTTP Paths

Any process that you build can bind (register) any number of HTTP paths with `http_server`. Every path that you bind will be automatically prepended with the current process' ID. For example, if you bind the route `/messages` within a process called `main:my_package:myname.uq` like so:

```
use uqbar_process_lib::{http::bind_http_path};

bind_http_path("/messages", true, false).unwrap();
```

Any HTTP requests to your node at `/main:my_package:myname.uq/messages` will be routed to your process.

The other two parameters to `bind_http_path` are `authenticated: bool` and `local_only: bool`. `authenticated` means that `http_server` will check for an auth cookie (set at login/registration), and `local_only` means that `http_server` will only allow requests that come from `localhost`.

Incoming HTTP requests will come via `http_server` and have both an `ipc` and a `payload`. The `payload` is the HTTP request body, and the `ipc` is an `IncomingHttpRequest`:

```
pub struct IncomingHttpRequest {
    pub source_socket_addr: Option<String>, // will parse to SocketAddr
    pub method: String,                     // will parse to http::Method
    pub raw_path: String,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
}
```

Note that `raw_path` is the host and full path of the original HTTP request that came in.

## Handling HTTP Requests

Usually, you will want to determine if an incoming request is a HTTP request, figure out what kind of `IncomingHttpRequest` it is, and then handle it based on the path and method.

Here is an example from the `uqdev` chat app template that handles both `POST` and `GET` requests to the `/messages` path:

```
fn handle_http_server_request(
    our: &Address,
    message_archive: &mut MessageArchive,
    source: &Address,
    ipc: &[u8],
    our_channel_id: &mut u32,
) -> anyhow::Result<()> {
    let Ok(server_request) = serde_json::from_slice::<HttpServerRequest>(ipc) else {
        // Fail silently if we can't parse the request
        return Ok(());
    };

    match server_request {

        // IMPORTANT BIT:

        HttpServerRequest::Http(IncomingHttpRequest { method, raw_path, .. }) => {
            // Check the path
            if raw_path.ends_with(&format!("{}{}", our.process.to_string(), "/messages")) {
                // Match on the HTTP method
                match method.as_str() {
                    // Get all messages
                    "GET" => {
                        let mut headers = HashMap::new();
                        headers.insert("Content-Type".to_string(), "application/json".to_string());

                        send_response(
                            StatusCode::OK,
                            Some(headers),
                            serde_json::to_vec(&ChatResponse::History {
                                messages: message_archive.clone(),
                            })
                            .unwrap(),
                        )?;
                    }
                    // Send a message
                    "POST" => {
                        print_to_terminal(0, "1");
                        let Some(payload) = get_payload() else {
                            return Ok(());
                        };
                        print_to_terminal(0, "2");
                        handle_chat_request(
                            our,
                            message_archive,
                            our_channel_id,
                            source,
                            &payload.bytes,
                            true,
                        )?;

                        // Send an http response via the http server
                        send_response(StatusCode::CREATED, None, vec![])?;
                    }
                    _ => {
                        // Method not allowed
                        send_response(StatusCode::METHOD_NOT_ALLOWED, None, vec![])?;
                    }
                }
            }
        }

        _ => {}
    };

    Ok(())
}
```

`send_response` is a `process_lib` function that sends an HTTP response. The function signature is as follows:

```
pub fn send_response(
    status: StatusCode,
    headers: Option<HashMap<String, String>>,
    body: Vec<u8>,
) -> anyhow::Result<()>
```

## Serving Static Assets, such as a Web Page/App

The simplest way to serve a UI is using the `serve_ui` function from `process_lib`:

```
serve_ui(&our, "ui").unwrap();
```

`serve_ui` takes two arguments: `&our` (&Address) and the directory where the UI assets are stored. By convention, this is the `ui` directory inside of the `pkg` directory that will be uploaded when you install the process. There must be an `index.html` in the `"ui"` directory (or whatever your top-level directory is called).

Under the hood, `serve_ui` uses `http_bind_static_path` which caches files in memory with `http_server` to respond to HTTP requests more quickly. The signature for `http_bind_static_path` is below:

```
pub fn bind_http_static_path<T>(
    path: T,
    authenticated: bool,
    local_only: bool,
    content_type: Option<String>,
    content: Vec<u8>,
) -> anyhow::Result<()>
```

The two additional parameters are the `content_type` (an optional String) and the `content` (bytes). The content will be served at the named route with the `Content-Type` header set appropriately.

Note that `serve_ui` caches all files in `http_server`, so if your website or web app has hundreds of MBs of asset files (like high-res images), then you will want to use a different method to serve content. In this case, you would bind the `index.html` file to your main route, and then bind a given HTTP route to serve all of your assets like so:

```
serve_index_html(&our, "ui").unwrap();
bind_http_path("/assets/*", true, false).unwrap();
```

Then in your request handler, you can use `handle_ui_asset_request` to get the file whose path matches the HTTP route of the request:

```
let ipc = message.ipc();
if let Ok(http_request) = serde_json::from_slice::<HttpServerRequest>(ipc) {
    match http_request {
        HttpServerRequest::Http(IncomingHttpRequest { raw_path, .. }) => {
            if raw_path.contains(&format!("/{}/assets/", our.process.to_string())) {
                return handle_ui_asset_request(our, "ui", &raw_path);
            }
        }
        _ => {}
    }
}
```

`handle_ui_asset_request` takes our (&Address), the top-level directory that contains the files, and the `raw_path` of the incoming request. In this case, the `/assets` directory must be in the `/ui` directory which must be uploaded from `pkg` when the process is installed. So your project would look like this:

```
my_package
    /pkg
        /ui
            index.html
            /assets
```

## App-Specific Authentication

COMING SOON
