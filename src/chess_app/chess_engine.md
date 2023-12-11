# Chess Engine

Chess is a good example for an Uqbar app because the basic game logic is already readily available. There are thousands of high-quality libraries across many languages that can be imported into a wasm app that runs on Uqbar. We'll be using [pleco](https://github.com/pleco-rs/Pleco).

In your `src/lib.rs`, you should already have something like this from the template:

```rust
use uqbar_process_lib::{println, receive, Address, Message};

wit_bindgen::generate!({
    path: "../wit",
    world: "process",
    exports: {
        world: Component,
    },
});

struct Component;

impl Guest for Component {
    fn init(our: String) {
        let our = Address::from_str(&our).unwrap();
        println!("{our}: start");

        loop {
            let _ = receive().map(|(source, message)| {
                let Message::Request(req) = message else { return };
                println!(
                    "{our}: got message from {}: {}",
                    source.process.process(),
                    String::from_utf8_lossy(&req.ipc)
                );
            });
        }
    }
}
```

Add `pleco = "0.5"` to your `Cargo.toml` dependencies (latest version at the time of writing). Then, import pleco at the top of lib.rs with `use pleco::Board;`. Now, we have access to a chess board and can manipulate it with code such as this:

```rust
let mut board = Board::start_pos();
let move_str = "e2e4";
board.apply_uci_move(move_str)?;
```

The [pleco docs](https://github.com/pleco-rs/Pleco#using-pleco-as-a-library) show everything you can do. But this isn't very interesting! We want to be able to play chess with other people. Let's start by creating a persisted state for the chess app and an IPC format for sending messages to other nodes.

```rust
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
enum ChessRequest {
    NewGame { white: String, black: String },
    Move { game_id: String, move_str: String },
    Resign(String),
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
enum ChessResponse {
    NewGameAccepted,
    NewGameRejected,
    MoveAccepted,
    MoveRejected,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Game {
    pub id: String, // the node with whom we are playing
    pub turns: u64,
    pub board: String,
    pub white: String,
    pub black: String,
    pub ended: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChessState {
    pub games: HashMap<String, Game>, // game is by opposing player id
}
```

Here's a simple Request/Response interface and a persistable game state.

Creating explicit `ChessRequest` and `ChessResponse` types is the easiest way to get reliable and easy to parse messages between two processes. It makes message-passing very simple. If you get a request, you can deserialize it to `ChessRequest` and ignore or throw an error if that fails. If you get a response, you can do the same but with `ChessResponse`. And every request and response that you send can be serialized in kind. More advanced apps can take on different structures, but a top-level enum to ser/de and match on is always a good idea.

We've also got a state struct here which can be persisted using the `set_state` and `get_state` commands exposed by Uqbar's runtime. Note that the `Game` struct here has `board` as a `String`. This is because the `Board` type from pleco doesn't implement `Serialize` or `Deserialize`. We'll have to convert it to a string using `fen()` before persisting it, and convert it back to a `Board` with `Board::from_fen()` when we load it from state.

This process will have a version of init() that creates an event loop and handles ChessRequests, but first, it's important to note that these types already bake in some assumptions about our "chess protocol" as it were. Keep in mind that requests can either expect a response, or be fired and forgotten. And unless a response is expected, there's no way to know if that request was received or not. In a game like chess, pretty much every action has a logical response. Otherwise there's no way to easily alert the user that their counterparty has gone offline, or started to otherwise ignore our moves. Partially for the sake of the tutorial, I've decided to make three kinds of requests and only have two of them expect a response. In our code, `NewGame` and `Move` requests will always await a response, blocking until they receive one (or the request times out). `Resign`, however, will be fire-and-forget -- a "real" game would probably want to wait for a response here too, but it's also important to let one player resign and thus clear their state *without* that resignation being "accepted" by a non-responsive player, so production-grade resignation logic is non-trivial.

*An aside: when building consumer-grade peer-to-peer apps, you'll find that there are in fact very few "trivial" interaction patterns. Something as simple as resigning from a one-on-one game, which would be a single POST request in a client-frontend <> server-backend architecture, requires well-thought-out negotiations to ensure that both players can walk away with a clean state machine, regardless of whether the counterparty is cooperating. Adding more "players" to the mix makes this even more complex. To keep things clean, leverage the request/response pattern and the `context` field to store information about how to handle a given response, if you're not awaiting it in-place.*

Here's the full code for the CLI version of the app. You can build it and install it on a node using `uqdev`. You can interact with it in the terminal, primitively, like so:
`/a our@chess2:chess2:ben.uq`
`/m {"NewGame": {"white": "<your_node_id>", "black": "<other_node_id>"}}`
`/m {"Move": {"game_id": "<other_node_id>", "move_str": "e2e4"}}`
(If you want to make a better CLI app, consider parsing IPC as a string...)

You might notice a problem with this app... there's no way to see your games! A fun project would be to add a CLI command that shows you, in-terminal, the board for a given game_id. But in the [next chapter](./frontend.md), we'll add a frontend to this app so you can see your games in a browser.

`Cargo.toml`:
```toml
[package]
name = "my_chess_app"
version = "0.1.0"
edition = "2021"

[profile.release]
panic = "abort"
opt-level = "s"
lto = true

[dependencies]
anyhow = "1.0"
base64 = "0.13"
bincode = "1.3.3"
pleco = "0.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
url = "*"
# NOTE: you can replace this with a newer version of the repo, but the tutorial may break slightly!
uqbar_process_lib = { git = "ssh://git@github.com/uqbar-dao/process_lib.git", rev = "b2dbec7" }
wit-bindgen = { git = "https://github.com/bytecodealliance/wit-bindgen", rev = "5390bab780733f1660d14c254ec985df2816bf1d" }

[lib]
crate-type = ["cdylib"]

[package.metadata.component]
package = "uqbar:process"
```

`src/lib.rs`:
```rust
#![feature(let_chains)]
use pleco::Board;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uqbar_process_lib::{
    get_typed_state, println, receive, set_state, Address, Message, NodeId, Request, Response,
};

extern crate base64;

// Boilerplate: generate the wasm bindings for an Uqbar app
wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});
struct Component;

//
// Our "chess protocol" request/response format. We'll always serialize these
// to a byte vector and send them over IPC.
//

#[derive(Debug, Serialize, Deserialize)]
enum ChessRequest {
    NewGame { white: String, black: String },
    Move { game_id: String, move_str: String },
    Resign(String),
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
enum ChessResponse {
    NewGameAccepted,
    NewGameRejected,
    MoveAccepted,
    MoveRejected,
}

//
// Our serializable state format.
//

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Game {
    pub id: String, // the node with whom we are playing
    pub turns: u64,
    pub board: String,
    pub white: String,
    pub black: String,
    pub ended: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChessState {
    pub games: HashMap<String, Game>, // game is by opposing player id
}

/// Helper function to serialize and save the process state.
fn save_chess_state(state: &ChessState) {
    set_state(&bincode::serialize(&state.games).unwrap());
}

/// Helper function to deserialize the process state. Note that we use a helper function
/// from process_lib to fetch a typed state, which will return None if the state does
/// not exist OR fails to deserialize. In either case, we'll make an empty new state.
fn load_chess_state() -> ChessState {
    match get_typed_state(|bytes| Ok(bincode::deserialize::<HashMap<String, Game>>(bytes)?)) {
        Some(games) => ChessState { games },
        None => ChessState {
            games: HashMap::new(),
        },
    }
}

impl Guest for Component {
    fn init(our: String) {
        let our = Address::from_str(&our).unwrap();
        // A little printout to show in terminal that the process has started.
        println!(
            "{} by {}: start",
            our.process.process_name, our.process.publisher_node
        );

        // Grab our state, then enter the main event loop.
        let mut state: ChessState = load_chess_state();
        main_loop(&our, &mut state);
    }
}

fn main_loop(our: &Address, state: &mut ChessState) {
    loop {
        // Call receive() to wait for any incoming messages.
        // If we get a network error, make a print and throw it away.
        // In a high-quality consumer-grade app, we'd want to explicitly handle
        // this and surface it to the user.
        let Ok((source, message)) = receive() else {
            println!("{our}: got network error");
            continue;
        };
        // We never expect any responses *here*, because for every
        // chess protocol request, we *await* its response in-place.
        // This is appropriate for direct node<>node comms, less
        // appropriate for other circumstances...
        let Message::Request(request) = message else {
            continue;
        };
        // Deserialize the request IPC to our format, and throw it away if it
        // doesn't fit.
        let Ok(chess_request) = serde_json::from_slice::<ChessRequest>(&request.ipc) else {
            println!("{our}: got invalid request");
            continue
        };
        // Call a function to handle the request.
        match handle_request(&our, &source, &chess_request, state) {
            Ok(()) => continue,
            Err(e) => println!("{our}: error handling request: {:?}", e),
        }
    }
}

/// Handle chess protocol messages from ourself *or* other nodes.
fn handle_request(
    our: &Address,
    source: &Address,
    chess_request: &ChessRequest,
    state: &mut ChessState,
) -> anyhow::Result<()> {
    // If the request is from another node, handle it as an incoming request.
    // Note that we can enforce the ProcessId as well, but it shouldn't be a trusted
    // piece of information, since another node can easily spoof any ProcessId on a request.
    // It can still be useful simply as a protocol-level switch to handle different kinds of
    // requests from the same node, with the knowledge that the remote node can finagle with
    // which ProcessId a given message can be from. It's their code, after all.
    if source.node != our.node {
        handle_chess_request(&source.node, state, chess_request)
    // ...and if the request is from ourselves, handle it as our own!
    // Note that since this is a local request, we *can* trust the ProcessId.
    // Here, we'll accept messages from the local terminal so as to make this a "CLI" app.
    } else if source.node == our.node && source.process == "terminal:terminal:uqbar" {
        handle_local_request(our, state, chess_request)
    } else {
        // If we get a request from ourselves that isn't from the terminal, we'll just
        // throw it away. This is a good place to put a printout to show that we've
        // received a request from ourselves that we don't know how to handle.
        return Err(anyhow::anyhow!(
            "got request from not-the-terminal, ignoring"
        ));
    }
}

/// handle chess protocol messages from other nodes
fn handle_chess_request(
    source_node: &NodeId,
    state: &mut ChessState,
    action: &ChessRequest,
) -> anyhow::Result<()> {
    println!("chess: handling action from {source_node}: {action:?}");

    // For simplicity's sake, we'll just use the node we're playing with as the game id.
    // This limits us to one active game per partner.
    let game_id = source_node;

    match action {
        ChessRequest::NewGame { white, black } => {
            // Make a new game with source.node
            // This will replace any existing game with source.node!
            if state.games.contains_key(game_id) {
                println!("chess: resetting game with {game_id} on their request!");
            }
            let game = Game {
                id: game_id.to_string(),
                turns: 0,
                board: Board::start_pos().fen(),
                white: white.to_string(),
                black: black.to_string(),
                ended: false,
            };
            // Use our helper function to persist state after every action.
            // The simplest and most trivial way to keep state. You'll want to
            // use a database or something in a real app, and consider performance
            // when doing intensive data-based operations.
            state.games.insert(game_id.to_string(), game);
            save_chess_state(&state);
            // Send a response to tell them we've accepted the game.
            // Remember, the other player is waiting for this.
            Response::new()
                .ipc(serde_json::to_vec(&ChessResponse::NewGameAccepted)?)
                .send()
        }
        ChessRequest::Move { ref move_str, .. } => {
            // Get the associated game, and respond with an error if
            // we don't have it in our state.
            let Some(game) = state.games.get_mut(game_id) else {
                // If we don't have a game with them, reject the move.
                return Response::new()
                    .ipc(serde_json::to_vec(&ChessResponse::MoveRejected)?)
                    .send()
            };
            // Convert the saved board to one we can manipulate.
            let mut board = Board::from_fen(&game.board).unwrap();
            if !board.apply_uci_move(move_str) {
                // Reject invalid moves!
                return Response::new()
                    .ipc(serde_json::to_vec(&ChessResponse::MoveRejected)?)
                    .send();
            }
            game.turns += 1;
            if board.checkmate() || board.stalemate() {
                game.ended = true;
            }
            // Persist state.
            game.board = board.fen();
            save_chess_state(&state);
            // Send a response to tell them we've accepted the move.
            Response::new()
                .ipc(serde_json::to_vec(&ChessResponse::MoveAccepted)?)
                .send()
        }
        ChessRequest::Resign(_) => {
            // They've resigned. The sender isn't waiting for a response to this,
            // so we don't need to send one.
            match state.games.get_mut(game_id) {
                Some(game) => {
                    game.ended = true;
                    save_chess_state(&state);
                }
                None => {}
            }
            Ok(())
        }
    }
}

/// Handle actions we are performing. Here's where we'll send_and_await various requests.
fn handle_local_request(
    our: &Address,
    state: &mut ChessState,
    action: &ChessRequest,
) -> anyhow::Result<()> {
    match action {
        ChessRequest::NewGame { white, black } => {
            // Create a new game. We'll enforce that one of the two players is us.
            if white != &our.node && black != &our.node {
                return Err(anyhow::anyhow!("cannot start a game without us!"));
            }
            let game_id = if white == &our.node { black } else { white };
            // If we already have a game with this player, throw an error.
            if let Some(game) = state.games.get(game_id)
                && !game.ended
            {
                return Err(anyhow::anyhow!("already have a game with {game_id}"));
            };
            // Send the other player a NewGame request
            // The request is exactly the same as what we got from terminal.
            // We'll give them 5 seconds to respond...
            let response = Request::new()
                .target((game_id.as_ref(), our.process.clone()))
                .ipc(serde_json::to_vec(&action)?)
                .send_and_await_response(5)?;
            // If they accept, create a new game -- otherwise, error out.
            let Ok((_source, Message::Response((resp, _context)))) = response else {
                return Err(anyhow::anyhow!("other player did not respond properly to new game request"));
            };
            let resp = serde_json::from_slice::<ChessResponse>(&resp.ipc)?;
            if resp != ChessResponse::NewGameAccepted {
                return Err(anyhow::anyhow!("other player rejected new game request!"));
            }
            // New game with default board.
            let game = Game {
                id: game_id.to_string(),
                turns: 0,
                board: Board::start_pos().fen(),
                white: white.to_string(),
                black: black.to_string(),
                ended: false,
            };
            state.games.insert(game_id.to_string(), game);
            save_chess_state(&state);
            Ok(())
        }
        ChessRequest::Move { game_id, move_str } => {
            // Make a move. We'll enforce that it's our turn. The game_id is the
            // person we're playing with.
            let Some(game) = state.games.get_mut(game_id) else {
                return Err(anyhow::anyhow!("no game with {game_id}"));
            };
            if (game.turns % 2 == 0 && game.white != our.node)
                || (game.turns % 2 == 1 && game.black != our.node)
            {
                return Err(anyhow::anyhow!("not our turn!"));
            } else if game.ended {
                return Err(anyhow::anyhow!("that game is over!"));
            }
            let mut board = Board::from_fen(&game.board).unwrap();
            if !board.apply_uci_move(move_str) {
                return Err(anyhow::anyhow!("illegal move!"));
            }
            // Send the move to the other player, then check if the game is over.
            // The request is exactly the same as what we got from terminal.
            // We'll give them 5 seconds to respond...
            let response = Request::new()
                .target((game_id.as_ref(), our.process.clone()))
                .ipc(serde_json::to_vec(&action)?)
                .send_and_await_response(5)?;
            let Ok((_source, Message::Response((resp, _context)))) = response else {
                return Err(anyhow::anyhow!("other player did not respond properly to our move"));
            };
            let resp = serde_json::from_slice::<ChessResponse>(&resp.ipc)?;
            if resp != ChessResponse::MoveAccepted {
                return Err(anyhow::anyhow!("other player rejected our move"));
            }
            game.turns += 1;
            if board.checkmate() || board.stalemate() {
                game.ended = true;
            }
            game.board = board.fen();
            save_chess_state(&state);
            Ok(())
        }
        ChessRequest::Resign(ref with_who) => {
            // Resign from a game with a given player.
            let Some(game) = state.games.get_mut(with_who) else {
                return Err(anyhow::anyhow!("no game with {with_who}"));
            };
            // send the other player an end game request -- no response expected
            Request::new()
                .target((with_who.as_ref(), our.process.clone()))
                .ipc(serde_json::to_vec(&action)?)
                .send()?;
            game.ended = true;
            save_chess_state(&state);
            Ok(())
        }
    }
}
```
