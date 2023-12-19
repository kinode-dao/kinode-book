# Adding a Frontend

Here, we'll take the code from the [previous section](./chess_engine.md) and add a web frontend to it.

There are two parts to this project: Altering the process code to serve and handle HTTP requests, and writing a webpage to interact with the process. We'll be using React to make a single-page app that will show our current games and let us make moves, plus make new games and resign from existing ones.

JavaScript and React development aren't in the scope of this tutorial, so we'll provide that code [here](https://github.com/uqbar-dao/chess-ui).

The important part is the built result, specifically, `index.html`, `index.js`, and `index.css`. We will be serving these as static files, [which you can get here](https://github.com/uqbar-dao/chess-ui/tree/tutorial/tutorial_build) if you don't want to build them yourself.

Add the files to the `pkg` folder in your app, so they'll be ingested on-install. This allows your process to fetch them from the virtual filesystem, as all files in `pkg` are mounted. However... for simplicity's sake, we can use the `include_str!` macro to embed the files directly into our process binary and serve them that way. See the [VFS API overview](../apis/vfs.md) to see how to use files mounted in `pkg`.

In `src/lib.rs`:
```rust
...
const CHESS_HTML: &str = include_str!("../pkg/chess.html");
const CHESS_JS: &str = include_str!("../pkg/index.js");
const CHESS_CSS: &str = include_str!("../pkg/index.css");
...
```

Chess will use the http_server runtime module to serve a static frontend and receive HTTP requests from it. We'll also use a WebSocket connection to send updates to the frontend when the game state changes.

In `src/lib.rs`, inside `init()`:
```rust
...
use uqbar_process_lib::http;
...
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
// Allow websockets to be opened at / (our process ID will be prepended).
http::bind_ws_path("/", true, false).unwrap();
...
```

This code will go in the init() function such that the frontend is served when the process starts.

The `http` library in [process_lib](../process_stdlib/overview.md) provides a simple interface for serving static files and handling HTTP requests. We use `bind_http_static_path` to serve the static files we included in our process binary, and `bind_http_path` to handle requests to `/games`. See process_lib docs for more functions and documentation on their parameters. These requests all serve HTTP that can only be accessed by a logged-in node user (the `true` parameter for `authenticated`), and can be accessed remotely (the `false` parameter for `local_only`). This API is under active development!

Requests on the /games path will come in as requests to our process, and we'll have to handle them and give a response. The request/response format can be imported from `http` in `process_lib`. To do this, we'll add a branch to our main request-handling function that takes requests from our `http_server:sys:uqbar`.

In `src/lib.rs`, inside `handle_request()`:
```rust
...
else if message.source().node == our.node
        && message.source().process == "http_server:sys:uqbar"
    {
        // receive HTTP requests and websocket connection messages from our server
        match serde_json::from_slice::<http::HttpServerRequest>(message.ipc())? {
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
            http::HttpServerRequest::WebSocketOpen { path, channel_id } => {
                // We know this is authenticated and unencrypted because we only
                // bound one path, the root path. So we know that client
                // frontend opened a websocket and can send updates
                state.clients.insert(channel_id);
                Ok(())
            }
            http::HttpServerRequest::WebSocketClose(channel_id) => {
                // client frontend closed a websocket
                state.clients.remove(&channel_id);
                Ok(())
            }
            http::HttpServerRequest::WebSocketPush { .. } => {
                // client frontend sent a websocket message
                // we don't expect this! we only use websockets to push updates
                Ok(())
            }
        }
    }
...
```

This code will have some errors -- we need a new function to handle HTTP requests, and a new state parameter to handle active frontend clients.

The `handle_http_request` function will be defined below. It takes the request, and returns a `Result` that we can match on to send a response. We'll also handle websocket open/close messages here, and ignore websocket push messages. We add a new `state` parameter here:

In `src/lib.rs`:
```rust
...
#[derive(Debug, Serialize, Deserialize)]
struct ChessState {
    pub games: HashMap<String, Game>, // game is by opposing player id
    pub clients: HashSet<u32>,        // doesn't get persisted
}
...
```

`clients` now holds the channel IDs of all connected clients. We'll use this to send updates over WebSockets to the frontend when the game state changes. But wait! We shouldn't persist this information, because those connections will die when our process is killed or the node is turned off. So we'll create another state type for persistence and convert to/from the in-memory one above.

In `src/lib.rs`:
```rust
...
use std::collections::{HashMap, HashSet};
...
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
...
```

Now, we just need a `handle_http_request` function to take incoming HTTP requests and return HTTP responses. This will serve pretty much exactly the same purpose as the `handle_local_request` function from the previous chapter, meaning that the frontend will produce actions and we'll execute them.

*An aside: As a process dev, you should be aware that HTTP resources served in this way can be accessed by _other processes running on the same node_, regardless of whether the paths are authenticated or not. This can be a security risk: if your app is handling sensitive actions from the frontend, a malicious app could make those API requests instead. You should never expect users to "only install non-malicious apps" -- instead, use a _secure subdomain_ to isolate your app's HTTP resources from other processes. See the [HTTP Server API](../apis/http_server.md) for more details.*

In `src/lib.rs`:
```rust
...
use uqbar_process_lib::get_payload;
...
fn handle_http_request(
    our: &Address,
    state: &mut ChessState,
    http_request: &http::IncomingHttpRequest,
) -> anyhow::Result<()> {
    if http_request.path()? != "games" {
        return http::send_response(
            http::StatusCode::NOT_FOUND,
            None,
            "Not Found".to_string().as_bytes().to_vec(),
        );
    }
    match http_request.method.as_str() {
        // on GET: give the frontend all of our active games
        "GET" => http::send_response(
            http::StatusCode::OK,
            Some(HashMap::from([(
                String::from("Content-Type"),
                String::from("application/json"),
            )])),
            serde_json::to_vec(&state.games)?,
        ),
        // on POST: create a new game
        "POST" => {
            let Some(payload) = get_payload() else {
                return http::send_response(http::StatusCode::BAD_REQUEST, None, vec![]);
            };
            let payload_json = serde_json::from_slice::<serde_json::Value>(&payload.bytes)?;
            let Some(game_id) = payload_json["id"].as_str() else {
                return http::send_response(http::StatusCode::BAD_REQUEST, None, vec![]);
            };
            if let Some(game) = state.games.get(game_id)
                && !game.ended
            {
                return http::send_response(http::StatusCode::CONFLICT, None, vec![]);
            };

            let player_white = payload_json["white"]
                .as_str()
                .unwrap_or(our.node.as_str())
                .to_string();
            let player_black = payload_json["black"]
                .as_str()
                .unwrap_or(game_id)
                .to_string();

            // send the other player a new game request
            let Ok(msg) = Request::new()
                .target((game_id, our.process.clone()))
                .ipc(serde_json::to_vec(&ChessRequest::NewGame {
                    white: player_white.clone(),
                    black: player_black.clone(),
                })?)
                .send_and_await_response(5)? else {
                    return Err(anyhow::anyhow!("other player did not respond properly to new game request"))
                };
            // if they accept, create a new game
            // otherwise, should surface error to FE...
            if serde_json::from_slice::<ChessResponse>(msg.ipc())? != ChessResponse::NewGameAccepted
            {
                return Err(anyhow::anyhow!("other player rejected new game request"));
            }
            // create a new game
            let game = Game {
                id: game_id.to_string(),
                turns: 0,
                board: Board::start_pos().fen(),
                white: player_white,
                black: player_black,
                ended: false,
            };
            let body = serde_json::to_vec(&game)?;
            state.games.insert(game_id.to_string(), game);
            save_chess_state(&state);
            http::send_response(
                http::StatusCode::OK,
                Some(HashMap::from([(
                    String::from("Content-Type"),
                    String::from("application/json"),
                )])),
                body,
            )
        }
        // on PUT: make a move
        "PUT" => {
            let Some(payload) = get_payload() else {
                return http::send_response(http::StatusCode::BAD_REQUEST, None, vec![]);
            };
            let payload_json = serde_json::from_slice::<serde_json::Value>(&payload.bytes)?;
            let Some(game_id) = payload_json["id"].as_str() else {
                return http::send_response(http::StatusCode::BAD_REQUEST, None, vec![]);
            };
            let Some(game) = state.games.get_mut(game_id) else {
                return http::send_response(http::StatusCode::NOT_FOUND, None, vec![]);
            };
            if (game.turns % 2 == 0 && game.white != our.node)
                || (game.turns % 2 == 1 && game.black != our.node)
            {
                return http::send_response(http::StatusCode::FORBIDDEN, None, vec![]);
            } else if game.ended {
                return http::send_response(http::StatusCode::CONFLICT, None, vec![]);
            }
            let Some(move_str) = payload_json["move"].as_str() else {
                return http::send_response(http::StatusCode::BAD_REQUEST, None, vec![]);
            };
            let mut board = Board::from_fen(&game.board).unwrap();
            if !board.apply_uci_move(move_str) {
                // TODO surface illegal move to player or something here
                return http::send_response(http::StatusCode::BAD_REQUEST, None, vec![]);
            }
            // send the move to the other player
            // check if the game is over
            // if so, update the records
            let Ok(msg) = Request::new()
                .target((game_id, our.process.clone()))
                .ipc(serde_json::to_vec(&ChessRequest::Move {
                    game_id: game_id.to_string(),
                    move_str: move_str.to_string(),
                })?)
                .send_and_await_response(5)? else {
                    return Err(anyhow::anyhow!("other player did not respond properly to our move"))
                };
            if serde_json::from_slice::<ChessResponse>(msg.ipc())? != ChessResponse::MoveAccepted {
                return Err(anyhow::anyhow!("other player rejected our move"));
            }
            // update the game
            game.turns += 1;
            if board.checkmate() || board.stalemate() {
                game.ended = true;
            }
            game.board = board.fen();
            // update state and return to FE
            let body = serde_json::to_vec(&game)?;
            save_chess_state(&state);
            // return the game
            http::send_response(
                http::StatusCode::OK,
                Some(HashMap::from([(
                    String::from("Content-Type"),
                    String::from("application/json"),
                )])),
                body,
            )
        }
        // on DELETE: end the game
        "DELETE" => {
            let Some(game_id) = http_request.query_params.get("id") else {
                return http::send_response(http::StatusCode::BAD_REQUEST, None, vec![]);
            };
            let Some(game) = state.games.get_mut(game_id) else {
                return http::send_response(http::StatusCode::BAD_REQUEST, None, vec![]);
            };
            // send the other player an end game request
            Request::new()
                .target((game_id.as_str(), our.process.clone()))
                .ipc(serde_json::to_vec(&ChessRequest::Resign(our.node.clone()))?)
                .send()?;
            game.ended = true;
            let body = serde_json::to_vec(&game)?;
            save_chess_state(&state);
            http::send_response(
                http::StatusCode::OK,
                Some(HashMap::from([(
                    String::from("Content-Type"),
                    String::from("application/json"),
                )])),
                body,
            )
        }
        // Any other method will be rejected.
        _ => http::send_response(http::StatusCode::METHOD_NOT_ALLOWED, None, vec![]),
    }
}
...
```

This is a lot of code, but it's mostly just handling the different HTTP methods and returning the appropriate responses. The only new thing here is the `get_payload()` function, which lets us get the HTTP body. See the HTTP API docs ([client](../apis/http_client.md), [server](../apis/http_server.md)) for more details.

Are we ready to play chess? Almost! One more missing piece: the backend needs to send WebSocket updates to the frontend after each move, so the board gets updated without a refresh. We already keep track of open channels in our process state, and we'll just send a push to each open channel when something happens.

In `src/lib.rs`, add a helper function:
```rust
...
use uqbar_process_lib::Payload;
...
fn send_ws_update(
    our: &Address,
    game: &Game,
    open_channels: &HashSet<u32>,
) -> anyhow::Result<()> {
    for channel in open_channels {
        Request::new()
            .target((&our.node, "http_server", "sys", "uqbar"))
            .ipc(serde_json::to_vec(
                &http::HttpServerAction::WebSocketPush {
                    channel_id: *channel,
                    message_type: http::WsMessageType::Binary,
                },
            )?)
            .payload(Payload {
                mime: Some("application/json".to_string()),
                bytes: serde_json::json!({
                    "kind": "game_update",
                    "data": game,
                }).to_string().into_bytes(),
            })
            .send()?;
    }
    Ok(())
}
```

Now, anywhere we receive an action from another node (so in `handle_chess_update()`), call `send_ws_update(&our, &game, &state.clients)?` to send an update to all connected clients. You'll need to add `our` as a parameter to the handler function. A good place to do this is right before we save our updated state. Moves that we make ourselves from the frontend will update on their own.

Continue to [Putting Everything Together](./putting_everything_together.md) to see the full code and screenshots of the app in action.