# Publishing a Website or Web App

Publishing a website or web app is quite simple.
There are helper functions to make this a one-line call if you have properly uploaded the relevant files from your development `/pkg` directory.

All of these functions expect you to place your `index.html` within a directory in `/pkg`.
In the following examples, that directory would be `/pkg/ui`.
All other files should be in a directory called `assets` inside of `ui`, so `/pkg/ui/assets`.
The structure should look like this:

```
my_package
└── pkg
    └── ui
        ├── assets
        └── index.html
```

## Serving Static Assets

The simplest way to serve a UI is using the `serve_ui` function from `process_lib`:

```
serve_ui(&our, "ui", true, false, vec!["/"]).unwrap();
```

This will serve the `index.html` in the specified folder at the home path of your process.
If your process is called `main:my_package:myusername.os` and your Kinode is running locally on port 8080,
then the UI will be served at `http://localhost:8080/main:my_package:myusername.os`.

`serve_ui` takes five arguments: the process' `&Address`, the name of the folder inside `pkg` that contains the `index.html` and other associated UI files, whether the UI requires authentication, whether the UI is local-only, and the path(s) on which to serve the UI (usually `["/"]`).
By convention, this is the `ui` directory inside of the `pkg` directory that will be uploaded when you install the process.
There must be an `index.html` in the `"ui"` directory (or whatever your top-level directory is called).

Under the hood, `serve_ui` uses `http_bind_static_path` which caches files in memory with `http_server` to respond to HTTP requests more quickly.
The signature for `http_bind_static_path` is below:

```
pub fn bind_http_static_path<T>(
    path: T,
    authenticated: bool,
    local_only: bool,
    content_type: Option<String>,
    content: Vec<u8>,
) -> anyhow::Result<()>
```

The two additional parameters are the `content_type` (an optional String) and the `content` (bytes).
The content will be served at the named route with the `Content-Type` header set appropriately.

Note that `serve_ui` caches all files in `http_server`, so if your website or web app has hundreds of MBs of asset files (like high-res images), then you will want to use a different method to serve content.
In this case, you would bind the `index.html` file to your main route, and then bind a given HTTP route to serve all of your assets like so:

```
serve_index_html(&our, "ui", true, false, vec!["/"]).unwrap();
bind_http_path("/assets/*", true, false).unwrap();
```

Then in your request handler, you can use `handle_ui_asset_request` to get the file whose path matches the HTTP route of the request:

```
let body = message.body();
if let Ok(http_request) = serde_json::from_slice::<HttpServerRequest>(body) {
    match http_request {
        HttpServerRequest::Http(IncomingHttpRequest { raw_path, .. }) => {
            if raw_path.contains(&format!("/{}/assets/", our.process.to_string())) {
                return handle_ui_asset_request(our, "ui", &raw_path);
            }
        }
        _ => {}
    }
}
```

`handle_ui_asset_request` takes our (&Address), the top-level directory that contains the files, and the `raw_path` of the incoming request.
In this case, the `/assets` directory must be in the `/ui` directory which must be uploaded from `pkg` when the process is installed.
So your project would look like this:

```
my_package
└── pkg
    └── ui
        ├── assets
        └── index.html
```
