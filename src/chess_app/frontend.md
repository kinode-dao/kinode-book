# Adding a Frontend

Here, we'll take the code from the [previous section](./chess_engine.md) and add a web frontend to it.

There are two parts to this project: Altering the process code to serve and handle HTTP requests, and writing a webpage to interact with the process. We'll be using React to make a single-page app that will show our current games and let us make moves, plus make new games and resign from existing ones.

JavaScript and React development aren't in the scope of this tutorial, so we'll provide that code [here](https://github.com/uqbar-dao/chess-ui).

The important part is the built result, specifically, `index.html`, `index.js`, and `index.css`. We will be serving these as static files, [which you can get here](https://github.com/uqbar-dao/chess-ui/tree/tutorial/tutorial_build) if you don't want to build them yourself.

Add the files to the `pkg` folder in your app, so they'll be ingested on-install. This allows your process to fetch them from the virtual filesystem, as all files in `pkg` are mounted. However... for simplicity's sake, we can use the `include_str!` macro to embed the files directly into our process binary and serve them that way. See the [VFS API overview](../apis/vfs.md) to see how to use files mounted in `pkg`.

```rust
const CHESS_HTML: &str = include_str!("../pkg/chess.html");
const CHESS_JS: &str = include_str!("../pkg/index.js");
const CHESS_CSS: &str = include_str!("../pkg/index.css");
```

Chess will use the http_server runtime module to serve a static frontend and receive HTTP requests from it. We'll also use a WebSocket connection to send updates to the frontend when the game state changes.

```rust
// serve static page at /index.html, /index.js, /index.css
// dynamically handle requests to /games
http::bind_http_static_path(
    "/",
    true,  // only serve for ourselves
    false, // can access remotely
    Some("text/html".to_string()),
    CHESS_HTML
        .replace("${node}", &our.node)
        .replace("${process}", &our.process.to_string())
        .as_bytes()
        .to_vec(),
)
.unwrap();
http::bind_http_static_path(
    "/index.js",
    true,
    false,
    Some("text/javascript".to_string()),
    CHESS_JS.as_bytes().to_vec(),
)
.unwrap();
http::bind_http_static_path(
    "/index.css",
    true,
    false,
    Some("text/css".to_string()),
    CHESS_CSS.as_bytes().to_vec(),
)
.unwrap();
http::bind_http_path("/games", true, false).unwrap();
```

This code will go in the init() function such that the frontend is served when the process starts.

The `http` library in [process_lib](../process_stdlib/overview.md) provides a simple interface for serving static files and handling HTTP requests. We use `bind_http_static_path` to serve the static files we included in our process binary, and `bind_http_path` to handle requests to `/games`. See process_lib docs for more functions and documentation on their parameters. These requests all serve HTTP that can only be accessed by a logged-in node user (the `true` parameter for `authenticated`), and can be accessed remotely (the `false` parameter for `local_only`). This API is under active development!

Requests on the /games path will come in as requests to our process, and we'll have to handle them and give a response. The request/response format can be imported from `http` in `process_lib`. To do this, we'll add a branch to our main request-handling function that takes requests from our `http_server:sys:uqbar`.

```rust
...
else if source.process == "http_server:sys:uqbar" && source.node == our.node {
    // receive HTTP requests and websocket connection messages from our server
    match serde_json::from_slice::<http::HttpServerRequest>(&request.ipc)? {
        http::HttpServerRequest::Http(incoming) => {
            match handle_http_request(our, state, incoming) {
                Ok(()) => Ok(()),
                Err(e) => {
                    println!("chess: error handling http request: {:?}", e);
                    http::send_response(
                        http::StatusCode::SERVICE_UNAVAILABLE,
                        None,
                        "Service Unavailable".to_string().as_bytes().to_vec(),
                    )
                }
            }
        }
        http::HttpServerRequest::WebSocketOpen(channel_id) => {
            // client frontend opened a websocket
            state.clients.insert(channel_id);
            Ok(())
        }
        http::HttpServerRequest::WebSocketClose(channel_id) => {
            // client frontend closed a websocket
            state.clients.remove(&channel_id);
            Ok(())
        }
        http::HttpServerRequest::WebSocketPush { message_type, .. } => {
            // client frontend sent a websocket message
            // we don't expect this! we only use websockets to push updates
            Ok(())
        }
    }
}
...
```

The `handle_http_request` function will be defined below. It takes the request, and returns a `Result` that we can match on to send a response. We'll also handle websocket open/close messages here, and ignore websocket push messages. Note that we have a new `state` parameter here:

```rust
#[derive(Debug, Serialize, Deserialize)]
struct ChessState {
    pub games: HashMap<String, Game>, // game is by opposing player id
    pub clients: HashSet<u32>,        // doesn't get persisted
}
```

`clients` now holds the channel IDs of all connected clients. We'll use this to send updates over WebSockets to the frontend when the game state changes. However, we shouldn't persist this information, because those connections will die when our process is killed or the node is turned off. So we'll create another state type for persistence and convert to/from the in-memory one above.

```rust
#[derive(Debug, Serialize, Deserialize)]
struct StoredChessState {
    pub games: HashMap<String, Game>, // game is by opposing player id
}

fn save_chess_state(state: &ChessState) {
    set_state(&bincode::serialize(&state.games).unwrap());
}

fn load_chess_state() -> ChessState {
    match get_typed_state(|bytes| Ok(bincode::deserialize::<HashMap<String, Game>>(bytes)?)) {
        Some(games) => ChessState {
            games,
            clients: HashSet::new(),
        },
        None => ChessState {
            games: HashMap::new(),
            clients: HashSet::new(),
        },
    }
}
```

Now, we just need a `handle_http_request` function to take incoming HTTP requests and return HTTP responses. This will serve pretty much exactly the same purpose as the `handle_local_request` function from the previous chapter, meaning that the frontend will produce actions and we'll execute them.

*An aside: As a process dev, you should be aware that HTTP resources served in this way can be accessed by _other processes running on the same node_, regardless of whether the paths are authenticated or not. This can be a security risk: if your app is handling sensitive actions from the frontend, a malicious app could make those API requests instead. You should never expect users to "only install non-malicious apps" -- instead, use a _secure subdomain_ to isolate your app's HTTP resources from other processes. See the [HTTP Server API](../apis/http_server.md) for more details.*

```rust
