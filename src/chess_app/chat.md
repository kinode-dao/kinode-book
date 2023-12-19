# Extension 1: Chat

So, at this point you've got a working chess game with a frontend. There are a number of obvious improvements to the program to be made, as listed at the end of the last chapter. The best way to understand those improvements is to start exploring other areas of the docs, such as the chapters on capabilities-based security and the networking protocol, for error handling.

This chapter will instead focus on how to *extend* an existing program with new functionality. Chat is a basic feature for a chess program, but will touch the existing code in many places. This will give you a good idea of how to extend your own programs.

We need to alter at least 4 things about the program:
- the request-response types it can handle (i.e. the protocol itself)
- the incoming request handler for HTTP requests, to receive chats sent by our node
- the outgoing websocket update, to send received chats to the frontend
- the frontend, to display the chat

We'll handle them in that order. First, let's look at the types we use for request-response now:
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

