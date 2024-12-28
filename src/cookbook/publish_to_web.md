# Publishing a Website or Web App

Publishing a website or web app is quite simple.
There are helper functions to make this a one-line call if you have properly uploaded the relevant files from your development `/pkg` directory.

All of these functions expect you to place your `index.html` within a directory in `/pkg`.
In the following examples, that directory would be `/pkg/ui`.
All other files should be in a directory called `assets` inside of `ui`, so `/pkg/ui/assets`.
The structure should look like this:

```
my-package
└── pkg
    └── ui
        ├── assets
        └── index.html
```

## Serving Static Assets

The simplest way to serve a UI is using the [`http::HttpServer::serve_ui()`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/http/server/struct.HttpServer.html#method.serve_ui) method from `process_lib`:

```rs
let mut server = http::server::HttpServer::new(5);
server
    .serve_ui(
        &our,
        "ui",
        vec!["/"],
        http::server::HttpBindingConfig::new(true, false, false, None),
    )
    .unwrap();
```

This will serve the `index.html` in the specified folder at the home path of your process.
If your process is called `main:my-package:myusername.os` and your Kinode is running locally on port 8080, then the UI will be served at `http://localhost:8080/main:my-package:myusername.os`.

`serve_ui` takes four arguments:
1. The process' `&Address`
2. The name of the folder inside `pkg` that contains the `index.html` and other associated UI files.
   By convention, this is the `ui` directory inside of the `pkg` directory that will be uploaded when you install the process.
   There must be an `index.html` in the `"ui"` directory (or whatever your top-level directory is called).
3. The path(s) on which to serve the UI (usually `["/"]`)
4. The configuration for the binding:
   - Whether the UI requires authentication
   - Whether the UI is local-only
   - Whether the content is static (not relevant here)
   - Whether to serve as a secure subdomain

Under the hood, `serve_ui` uses [`bind_http_static_path`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/http/server/struct.HttpServer.html#method.bind_http_static_path) which caches files in memory with `http-server` to respond to HTTP requests more quickly.
The two additional parameters are the `content_type` (an optional String) and the `content` (bytes).
The content will be served at the named route with the `Content-Type` header set appropriately.

Note that `serve_ui` caches all files in `http-server`, so if your website or web app has hundreds of MBs of asset files (like high-res images), then you will want to use a different method to serve content.
For example, see the [`docs:docs:nick.kino` application](https://github.com/nick1udwig/docs/blob/master/docs/src/lib.rs).
