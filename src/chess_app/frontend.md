# Adding a Frontend

Here, you'll add a web frontend to the code from the [previous section](./chess_engine.md).

Creating a web frontend has two parts:
1. Altering the process code to serve and handle HTTP requests
2. Writing a webpage to interact with the process.
Here, you'll use React to make a single-page app that displays your current games and allows us to: create new games, resign from games, and make moves on the chess board.

JavaScript and React development aren't in the scope of this tutorial, so you can find that code [here](https://github.com/kinode-dao/chess-ui).

The important part of the frontend for the purpose of this tutorial is how to set up those pre-existing files to be built and installed by `kit`.
When files found in the `ui/` directory, if a `package.json` file is found with a `build:copy` field in `scripts`, `kit` will run that to build the UI (see [here](https://github.com/kinode-dao/chess-ui/blob/82419ea0e53e6d86d6dc6c8ed7f656c3ab51fdc8/package.json#L10)).
The `build:copy` in that file builds the UI and then places the resulting files into the `pkg/ui/` directory where they will be installed by `kit start-package`.
This allows your process to fetch them from the virtual filesystem, as all files in `pkg/` are mounted.
See the [VFS API overview](../apis/vfs.md) to see how to use files mounted in `pkg/`.
Additional UI dev info can be found [here](../apis/frontend_development.md).

Get the chess UI files and place them in the proper place (next to `pkg/`):
```bash
# run in the top-level directory of your my-chess package
git clone https://github.com/kinode-dao/chess-ui ui
```

Chess will use the built-in HTTP server runtime module to serve a static frontend and receive HTTP requests from it.
You'll also use a WebSocket connection to send updates to the frontend when the game state changes.

In `my-chess/src/lib.rs`, inside `init()`:
```rust
use kinode_process_lib::{http::server, homepage};

// add ourselves to the homepage
homepage::add_to_homepage("My Chess App", None, Some("/"), None);

// create an HTTP server struct with which to manipulate `http-server:distro:sys`
let mut http-server = server::HttpServer::new(5);
let http_config = server::HttpBindingConfig::default();

// Serve the index.html and other UI files found in pkg/ui at the root path.
http-server
    .serve_ui(&our, "ui", vec!["/"], http_config.clone())
    .expect("failed to serve ui");

// Allow HTTP requests to be made to /games; they will be handled dynamically.
http-server
    .bind_http_path("/games", http_config.clone())
    .expect("failed to bind /games");

// Allow websockets to be opened at / (our process ID will be prepended).
http-server
    .bind_ws_path("/", server::WsBindingConfig::default())
    .expect("failed to bind ws");
```

The above code should be inserted into the `init()` function such that the frontend is served when the process starts.

The `http` library in [process_lib](../process_stdlib/overview.md) provides a simple interface for serving static files and handling HTTP requests.
Use `serve_ui` to serve the static files included in the process binary, and `bind_http_path` to handle requests to `/games`.
`serve_ui` takes five arguments: the process `Address`, the name of the folder inside `pkg` that contains the `index.html` and other associated UI files, the path(s) on which to serve the UI (usually just `["/"]`), and the `HttpBindingConfig` to use.
See [process_lib docs](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/) for more functions and documentation on their parameters.
These requests all serve HTTP that can only be accessed by a logged-in node user (the `true` parameter for `authenticated` in `HttpBindingConfig`) and can be accessed remotely (the `false` parameter for `local_only`).

Requests on the `/games` path will arrive as requests to your process, and you'll have to handle them and respond.
To do this, add a branch to the main request-handling function that takes requests from *our* `http-server:distro:sys`.

In `my-chess/src/lib.rs`, inside the part of `handle_request()` that handles local requests:
```rust
...
    // if the message is from the HTTP server runtime module, we should handle it
    // as an HTTP request and not a chess request
    if message.source().process == "http-server:distro:sys" {
        return handle_http_request(state, http-server, message);
    }
...
```

Now, write the `handle_http_request` function to take incoming HTTP requests and return HTTP responses.
This will serve the same purpose as the `handle_local_request` function from the previous chapter, meaning that the frontend will produce actions and the backend will execute them.

An aside: As a process dev, you should be aware that HTTP resources served in this way can be accessed by *other processes running on the same node*, regardless of whether the paths are authenticated or not.
This can be a security risk: if your app is handling sensitive actions from the frontend, a malicious app could make those API requests instead.
You should never expect users to "only install non-malicious apps" — instead, use a *secure subdomain* to isolate your app's HTTP resources from other processes.
See the [HTTP Server API](../apis/http_server.md) for more details.

In `my-chess/src/lib.rs`:
```rust
/// Handle HTTP requests from our own frontend.
fn handle_http_request(
    state: &mut ChessState,
    http-server: &mut server::HttpServer,
    message: &Message,
) -> anyhow::Result<()> {
    let request = http-server.parse_request(message.body())?;

    // the HTTP server helper struct allows us to pass functions that
    // handle the various types of requests we get from the frontend
    http-server.handle_request(
        request,
        |incoming| {
            // client frontend sent an HTTP request, process it and
            // return an HTTP response
            // these functions can reuse the logic from handle_local_request
            // after converting the request into the appropriate format!
            match incoming.method().unwrap_or_default() {
                http::Method::GET => handle_get(state),
                http::Method::POST => handle_post(state),
                http::Method::PUT => handle_put(state),
                http::Method::DELETE => handle_delete(state, &incoming),
                _ => (
                    server::HttpResponse::new(http::StatusCode::METHOD_NOT_ALLOWED),
                    None,
                ),
            }
        },
        |_channel_id, _message_type, _message| {
            // client frontend sent a websocket message
            // we don't expect this! we only use websockets to push updates
        },
    );

    Ok(())
}
```

Of course, we must now implement the `handle_get`, `handle_post`, `handle_put`, and `handle_delete` functions.
These will parse the incoming requests, convert them to our `ChessRequest` format, use the function defined in the last chapter to apply them to our state machine, and return the appropriate HTTP responses.

```rust
/// On GET: return all active games
fn handle_get(state: &mut ChessState) -> (server::HttpResponse, Option<LazyLoadBlob>) {
    (
        server::HttpResponse::new(http::StatusCode::OK),
        Some(LazyLoadBlob {
            mime: Some("application/json".to_string()),
            bytes: serde_json::to_vec(&state.games).expect("failed to serialize games!"),
        }),
    )
}

/// On POST: create a new game
fn handle_post(state: &mut ChessState) -> (server::HttpResponse, Option<LazyLoadBlob>) {
    let Some(blob) = get_blob() else {
        return (
            server::HttpResponse::new(http::StatusCode::BAD_REQUEST),
            None,
        );
    };
    let Ok(blob_json) = serde_json::from_slice::<serde_json::Value>(&blob.bytes) else {
        return (
            server::HttpResponse::new(http::StatusCode::BAD_REQUEST),
            None,
        );
    };
    let Some(game_id) = blob_json["id"].as_str() else {
        return (
            server::HttpResponse::new(http::StatusCode::BAD_REQUEST),
            None,
        );
    };

    let player_white = blob_json["white"]
        .as_str()
        .unwrap_or(state.our.node.as_str())
        .to_string();
    let player_black = blob_json["black"].as_str().unwrap_or(game_id).to_string();

    match handle_local_request(
        state,
        &ChessRequest::NewGame(NewGameRequest {
            white: player_white,
            black: player_black,
        }),
    ) {
        Ok(game) => (
            server::HttpResponse::new(http::StatusCode::OK)
                .header("Content-Type", "application/json"),
            Some(LazyLoadBlob {
                mime: Some("application/json".to_string()),
                bytes: serde_json::to_vec(&game).expect("failed to serialize game!"),
            }),
        ),
        Err(e) => (
            server::HttpResponse::new(http::StatusCode::BAD_REQUEST),
            Some(LazyLoadBlob {
                mime: Some("application/text".to_string()),
                bytes: e.to_string().into_bytes(),
            }),
        ),
    }
}

/// On PUT: make a move
fn handle_put(state: &mut ChessState) -> (server::HttpResponse, Option<LazyLoadBlob>) {
    let Some(blob) = get_blob() else {
        return (
            server::HttpResponse::new(http::StatusCode::BAD_REQUEST),
            None,
        );
    };
    let Ok(blob_json) = serde_json::from_slice::<serde_json::Value>(&blob.bytes) else {
        return (
            server::HttpResponse::new(http::StatusCode::BAD_REQUEST),
            None,
        );
    };

    let Some(game_id) = blob_json["id"].as_str() else {
        return (
            server::HttpResponse::new(http::StatusCode::BAD_REQUEST),
            None,
        );
    };
    let Some(move_str) = blob_json["move"].as_str() else {
        return (
            server::HttpResponse::new(http::StatusCode::BAD_REQUEST),
            None,
        );
    };

    match handle_local_request(
        state,
        &ChessRequest::Move(MoveRequest {
            game_id: game_id.to_string(),
            move_str: move_str.to_string(),
        }),
    ) {
        Ok(game) => (
            server::HttpResponse::new(http::StatusCode::OK)
                .header("Content-Type", "application/json"),
            Some(LazyLoadBlob {
                mime: Some("application/json".to_string()),
                bytes: serde_json::to_vec(&game).expect("failed to serialize game!"),
            }),
        ),
        Err(e) => (
            server::HttpResponse::new(http::StatusCode::BAD_REQUEST),
            Some(LazyLoadBlob {
                mime: Some("application/text".to_string()),
                bytes: e.to_string().into_bytes(),
            }),
        ),
    }
}

/// On DELETE: end the game
fn handle_delete(
    state: &mut ChessState,
    request: &server::IncomingHttpRequest,
) -> (server::HttpResponse, Option<LazyLoadBlob>) {
    let Some(game_id) = request.query_params().get("id") else {
        return (
            server::HttpResponse::new(http::StatusCode::BAD_REQUEST),
            None,
        );
    };
    match handle_local_request(state, &ChessRequest::Resign(game_id.to_string())) {
        Ok(game) => (
            server::HttpResponse::new(http::StatusCode::OK)
                .header("Content-Type", "application/json"),
            Some(LazyLoadBlob {
                mime: Some("application/json".to_string()),
                bytes: serde_json::to_vec(&game).expect("failed to serialize game!"),
            }),
        ),
        Err(e) => (
            server::HttpResponse::new(http::StatusCode::BAD_REQUEST),
            Some(LazyLoadBlob {
                mime: Some("application/text".to_string()),
                bytes: e.to_string().into_bytes(),
            }),
        ),
    }
}
```

Are you ready to play chess?
Almost there!
One more missing piece: the backend needs to send WebSocket updates to the frontend after each move in order to update the board without a refresh.
Since open channels are already tracked in `HttpServer`, you just need to send a push to each open channel when a move occurs.

In `my-chess/src/lib.rs`, add a helper function:
```rust
fn send_ws_update(http-server: &mut server::HttpServer, game: &Game) {
    http-server.ws_push_all_channels(
        "/",
        server::WsMessageType::Binary,
        LazyLoadBlob {
            mime: Some("application/json".to_string()),
            bytes: serde_json::json!({
                "kind": "game_update",
                "data": game,
            })
            .to_string()
            .into_bytes(),
        },
    )
}
```

Now, anywhere you receive an action from another node (in `handle_chess_request()`, for example), call `send_ws_update(&our, &game, &state.clients)?` to send an update to all connected clients.
A good place to do this is right after saving the updated state.
Local moves from the frontend will update on their own.

Finally, add requests for `http-server` and `vfs` messaging capabilities to the `manifest.json`:
```json
...
"request_capabilities": [
    "http-server:distro:sys",
    "vfs:distro:sys"
],
...
```

Continue to [Putting Everything Together](./putting_everything_together.md) to see the full code and screenshots of the app in action.
