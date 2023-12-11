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

Requests on the /games path will come in as requests to our process, and we'll have to handle them and give a response. The request/response format can be imported from `http` in `process_lib`.