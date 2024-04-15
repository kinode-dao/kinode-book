# Extension 1: Chat

So, at this point you've got a working chess game with a frontend.
There are a number of obvious improvements to the program to be made, as listed at the end of the [last chapter](./putting_everything_together.md).
The best way to understand those improvements is to start exploring other areas of the docs, such as the chapters on [capabilities-based security](../process/capabilities.md) and the [networking protocol](../networking_protocol.md), for error handling.

This chapter will instead focus on how to *extend* an existing program with new functionality.
Chat is a basic feature for a chess program, but will touch the existing code in many places.
This will give you a good idea of how to extend your own programs.

You need to alter at least 4 things about the program:
- The request-response types it can handle (i.e. the protocol itself)
- The incoming request handler for HTTP requests, to receive chats sent by `our` node
- The outgoing websocket update, to send received chats to the frontend
- The frontend, to display the chat

Handling them in that order, first, look at the types used for request-response now:
```rust
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
```

These types need to be exhaustive, since incoming messages will be fed into a `match` statement that uses `ChessRequest` and `ChessResponse`.
For more complex apps, one could introduce a new type that serves as an umbrella over multiple "kinds" of message, but since a simple chat will only be a few extra entries into the existing types, it's unnecessary for this example.

In order to add chat, the request type above will need a new variant, something like `Message(String)`.
It doesn't need a `from` field, since that's just the `source` of the message!

A new response type will make the chat more robust, by acknowledging received messages.
Something like `MessageAck` will do, with no fields — since this will be sent in response to a `Message` request, the sender will know which message it's acknowledging.

The new types will look like this:
```rust
#[derive(Debug, Serialize, Deserialize)]
enum ChessRequest {
    NewGame { white: String, black: String },
    Move { game_id: String, move_str: String },
    Resign(String),
    Message(String),
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
enum ChessResponse {
    NewGameAccepted,
    NewGameRejected,
    MoveAccepted,
    MoveRejected,
    MessageAck,
}
```

If you are modifying these types inside the finished chess app from this tutorial, your IDE should indicate that there are a few errors now: these new message types are not handled in their respective `match` statements.
Those errors, in `handle_chess_request` and `handle_local_request`, are where you'll need logic to handle messages other nodes send to this node, and messages this node sends to others, respectively.

In `handle_chess_request`, the app receives requests from other nodes.
A reasonable way to handle incoming messages is to add them to a vector of messages that's saved for each active game.
The frontend could reflect this by adding a chat box next to each game, and displaying all messages sent over that game's duration.

To do that, the `Game` struct must be altered to hold such a vector.

```rust
struct Game {
    pub id: String, // the node with whom we are playing
    pub turns: u64,
    pub board: String,
    pub white: String,
    pub black: String,
    pub ended: bool,
    /// messages stored in order as (sender, content)
    pub messages: Vec<(String, String)>,
}
```

Then in the main switch statement in `handle_chess_request`:
```rust
...
ChessRequest::Message(content) => {
    // Earlier in this code, we define game_id as the source node.
    let Some(game) = state.games.get_mut(game_id) else {
        return Err(anyhow::anyhow!("no game with {game_id}"));
    };
    game.messages.push((game_id.to_string(), content.to_string()));
    Ok(())
}
...
```

In `handle_local_request`, the app sends requests to other nodes.
Note, however, that requests to message `our`self don't really make sense — what should really happen is that the chess frontend performs a PUT request, or sends a message over a websocket, and the chess backend process turns that into a message request to the other player.
So instead of handling `Message` requests in `handle_local_request`, the process should reject or ignore them:

```rust
ChessRequest::Message(_) => {
    Ok(())
}
```

Instead, the chess backend will handle a new kind of PUT request in `handle_http_request`, such that the local frontend can be used to send messages in games being played.

This is the current (super gross!!) code for handling PUT requests in `handle_http_request`:
```rust
// on PUT: make a move
"PUT" => {
    let Some(blob) = get_blob() else {
        return http::send_response(http::StatusCode::BAD_REQUEST, None, vec![]);
    };
    let blob_json = serde_json::from_slice::<serde_json::Value>(&blob.bytes)?;
    let Some(game_id) = blob_json["id"].as_str() else {
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
    let Some(move_str) = blob_json["move"].as_str() else {
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
        .body(serde_json::to_vec(&ChessRequest::Move {
            game_id: game_id.to_string(),
            move_str: move_str.to_string(),
        })?)
        .send_and_await_response(5)?
    else {
        return Err(anyhow::anyhow!(
            "other player did not respond properly to our move"
        ));
    };
    if serde_json::from_slice::<ChessResponse>(msg.body())? != ChessResponse::MoveAccepted {
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
```

Let's modify this to handle more than just making moves.
Note that there's an implicit JSON structure enforced by the code above, where PUT requests from your frontend look like this:

```json
{
    "id": "game_id",
    "move": "e2e4"
}
```

An easy way to allow messages is to match on whether the key `"move"` is present, and if not, look for the key `"message"`.
This could also easily be codified as a Rust type and deserialized.

Now, instead of assuming `"move"` exists, let's add a branch that handles the `"message"` case.
This is a modification of the code above:
```rust
// on PUT: make a move OR send a message
"PUT" => {
    // ... same as the previous snippet ...
    let Some(move_str) = blob_json["move"].as_str() else {
        let Some(message) = blob_json["message"].as_str() else {
            return http::send_response(http::StatusCode::BAD_REQUEST, None, vec![]);
        };
        // handle sending message to another player
        let Ok(_ack) = Request::new()
            .target((game_id, our.process.clone()))
            .body(serde_json::to_vec(&ChessRequest::Message(message.to_string()))?)
            .send_and_await_response(5)?
        else {
            // Reader Note: handle a failed message send!
            return Err(anyhow::anyhow!(
                "other player did not respond properly to our message"
            ));
        };
        game.messages.push((our.node.clone(), message.to_string()));
        let body = serde_json::to_vec(&game)?;
        save_chess_state(&state);
        // return the game
        return http::send_response(
            http::StatusCode::OK,
            Some(HashMap::from([(
                String::from("Content-Type"),
                String::from("application/json"),
            )])),
            body,
        );
    };
    //
    // ... the rest of the move-handling code, same as previous snippet ...
    //
}
```

That's it.
A simple demonstration of how to extend the functionality of a given process.
There are a few key things to keep in mind when doing this, if you want to build stable, maintainable, upgradable applications:

- By adding chat, you changed the format of the "chess protocol" implicitly declared by this program.
If a user is running the old code, their version won't know how to handle the new `Message` request type we added.
**Depending on the serialization/deserialization strategy used, this might even create incompatibilities with the other types of requests.**
This is a good reason to use a serialization strategy that allows for "unknown" fields, such as JSON.
If you're using a binary format, you'll need to be more careful about how you add new fields to existing types.

- It's *okay* to break backwards compatibility with old versions of an app, but once a protocol is established, it's best to stick to it or start a new project.
Backwards compatibility can always be achieved by adding a version number to the request/response type(s) directly.
That's a simple way to know which version of the protocol is being used and handle it accordingly.

- By adding a `messages` field to the `Game` struct, you changed the format of the state that gets persisted.
If a user was running the previous version of this process, and upgrades to this version, the old state will fail to properly deserialize.
If you are building an upgrade to an existing app, you should always test that the new version can appropriately handle old state.
If you have many versions, you might need to make sure that state types from *any* old version can be handled.
Again, inserting a version number that can be deserialized from persisted state is a useful strategy.
The best way to do this depends on the serialization strategy used.
