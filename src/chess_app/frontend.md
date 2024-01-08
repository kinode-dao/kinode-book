# Adding a Frontend

Here, we'll add a web frontend to the code from the [previous section](./chess_engine.md).

Creating a web frontend has two parts:
1. Altering the process code to serve and handle HTTP requests
2. Writing a webpage to interact with the process.
Here, you'll use React to make a single-page app that displays your current games and allows us to: create new games, resign from games, and make moves on the chess board.

JavaScript and React development aren't in the scope of this tutorial, so we'll provide that code [here](https://github.com/uqbar-dao/chess-ui).

The important part of the frontend for the purpose of this tutorial is the build, specifically the `pkg/ui` directory that will be loaded into the VFS during installation.
Serve these as static files, [which you can get here](https://github.com/uqbar-dao/chess-ui/tree/tutorial/tutorial_build) if you don't want to build them yourself.

Run `npm run build` in the `chess-ui` repo and copy the output `dist` folder into the `pkg` folder in your app, so it'll be ingested on-install.
This allows your process to fetch them from the virtual filesystem, as all files in `pkg` are mounted.
Rename it to `ui` so that you have the files in `pkg/ui`.
See the [VFS API overview](../apis/vfs.md) to see how to use files mounted in `pkg`.

Chess will use the `http_server` runtime module to serve a static frontend and receive HTTP requests from it.
You'll also use a WebSocket connection to send updates to the frontend when the game state changes.

In `my_chess/src/lib.rs`, inside `init()`:
```rust
...
use nectar_process_lib::http;
...
// Serve the index.html and other UI files found in pkg/ui at the root path.
http::serve_ui(&our, "ui").unwrap();

// Allow HTTP requests to be made to /games; they will be handled dynamically.
http::bind_http_path("/games", true, false).unwrap();

// Allow websockets to be opened at / (our process ID will be prepended).
http::bind_ws_path("/", true, false).unwrap();
...
```

The above code should be inserted into the `init()` function such that the frontend is served when the process starts.

The `http` library in [process_lib](../process_stdlib/overview.md) provides a simple interface for serving static files and handling HTTP requests.
Use `serve_ui` to serve the static files includeded in the process binary, and `bind_http_path` to handle requests to `/games`.
`serve_ui` takes two arguments: the process' `&Address` and the name of the folder inside `pkg` that contains the `index.html` and other associated UI files.
See [process_lib docs](../process_stdlib/overview.md) for more functions and documentation on their parameters.
These requests all serve HTTP that can only be accessed by a logged-in node user (the `true` parameter for `authenticated`) and can be accessed remotely (the `false` parameter for `local_only`).
This API is under active development!

Requests on the `/games` path will arrive as requests to your process, and you'll have to handle them and respond.
The request/response format can be imported from `http` in `process_lib`.
To do this, add a branch to the main request-handling function that takes requests from `http_server:sys:nectar`.

In `my_chess/src/lib.rs`, inside `handle_request()`:
```rust
...
    } else if message.source().node == our.node
        && message.source().process == "http_server:sys:nectar"
    {
        // receive HTTP requests and websocket connection messages from our server
        match serde_json::from_slice::<http::HttpServerRequest>(message.ipc())? {
            http::HttpServerRequest::Http(ref incoming) => {
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
            http::HttpServerRequest::WebSocketOpen { channel_id, .. } => {
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
    } else {
...
```

This code won't compile yet — you need a new function to handle HTTP requests, and a new state parameter to handle active frontend clients.

Before defining `handle_http_request`, you need to add a new state parameter in the process state.
The state will keep track of all connected clients in a `HashSet<u32>` and send updates to them when the game state changes.
You'll also need to update the `save_chess_state` and `load_chess_state` functions to handle this new state.

In `my_chess/src/lib.rs`:
```rust
...
#[derive(Debug, Serialize, Deserialize)]
struct ChessState {
    pub games: HashMap<String, Game>, // game is by opposing player id
    pub clients: HashSet<u32>,        // doesn't get persisted
}
...
```

`clients` now holds the channel IDs of all connected clients.
It'll be used to send updates over WebSockets to the frontend when the game state changes.
But wait!
This information shouldn't be persisted because those connections will disappear when the process is killed or the node running this process is turned off.
Instead, create another state type for persistence and convert to/from the in-memory one above when you save process state.

In `my_chess/src/lib.rs`:
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

Now, change the `handle_http_request` function to take incoming HTTP requests and return HTTP responses.
This will serve the same purpose as the `handle_local_request` function from the previous chapter, meaning that the frontend will produce actions and the backend will execute them.

An aside: As a process dev, you should be aware that HTTP resources served in this way can be accessed by *other processes running on the same node*, regardless of whether the paths are authenticated or not.
This can be a security risk: if your app is handling sensitive actions from the frontend, a malicious app could make those API requests instead.
You should never expect users to "only install non-malicious apps" — instead, use a *secure subdomain* to isolate your app's HTTP resources from other processes.
See the [HTTP Server API](../apis/http_server.md) for more details.

In `my_chess/src/lib.rs`:
```rust
...
use nectar_process_lib::get_payload;
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
                // reader note: can surface illegal move to player or something here
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

This is a lot of code.
Mostly, it just handles the different HTTP methods and returns the appropriate responses.
The only unfamiliar code here is the `get_payload()` function, which is used here to inspect the HTTP body.
See the HTTP API docs ([client](../apis/http_client.md), [server](../apis/http_server.md)) for more details.

Are you ready to play chess?
Almost there!
One more missing piece: the backend needs to send WebSocket updates to the frontend after each move in order to update the board without a refresh.
Since open channels are already tracked in process state, you just need to send a push to each open channel when a move occurs.

In `my_chess/src/lib.rs`, add a helper function:
```rust
...
use nectar_process_lib::Payload;
...
fn send_ws_update(
    our: &Address,
    game: &Game,
    open_channels: &HashSet<u32>,
) -> anyhow::Result<()> {
    for channel in open_channels {
        Request::new()
            .target((&our.node, "http_server", "sys", "nectar"))
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

Now, anywhere you receive an action from another node (in `handle_chess_request()`, for example), call `send_ws_update(&our, &game, &state.clients)?` to send an update to all connected clients.
You'll need to add `our` as a parameter to the handler function.
A good place to do this is right before saving the updated state.
Local moves from the frontend will update on their own.

Continue to [Putting Everything Together](./putting_everything_together.md) to see the full code and screenshots of the app in action.
