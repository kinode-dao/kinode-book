# HTTP API

In Nectar OS, incoming HTTP requests are handled by a Rust `warp` server in the core `http_server:sys:nectar` process.
This process handles binding (registering) routes, simple JWT-based authentication, and serving a `/login` page if auth is missing.

## Binding (Registering) HTTP Paths

Any process that you build can bind (register) any number of HTTP paths with `http_server`.
Every path that you bind will be automatically prepended with the current process' ID.
For example, bind the route `/messages` within a process called `main:my_package:myname.nec` like so:

```
use nectar_process_lib::{http::bind_http_path};

bind_http_path("/messages", true, false).unwrap();
```

Now, any HTTP requests to your node at `/main:my_package:myname.nec/messages` will be routed to your process.

The other two parameters to `bind_http_path` are `authenticated: bool` and `local_only: bool`.
`authenticated` means that `http_server` will check for an auth cookie (set at login/registration), and `local_only` means that `http_server` will only allow requests that come from `localhost`.

Incoming HTTP requests will come via `http_server` and have both an `body` and a `lazy_load_blob`.
The `lazy_load_blob` is the HTTP request body, and the `body` is an `IncomingHttpRequest`:

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

Usually, you will want to:
1) determine if an incoming request is a HTTP request.
2) figure out what kind of `IncomingHttpRequest` it is.
3) handle the request based on the path and method.

Here is an example from the `necdev` UI-enabled chat app template that handles both `POST` and `GET` requests to the `/messages` path:

```
fn handle_http_server_request(
    our: &Address,
    message_archive: &mut MessageArchive,
    source: &Address,
    body: &[u8],
    our_channel_id: &mut u32,
) -> anyhow::Result<()> {
    let Ok(server_request) = serde_json::from_slice::<HttpServerRequest>(body) else {
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
                        let Some(blob) = get_blob() else {
                            return Ok(());
                        };
                        print_to_terminal(0, "2");
                        handle_chat_request(
                            our,
                            message_archive,
                            our_channel_id,
                            source,
                            &blob.bytes,
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

## App-Specific Authentication

COMING SOON
