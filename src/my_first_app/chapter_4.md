# Frontend Time

After the last section, you should have a simple process that responds to two commands from the terminal.
In this section, you'll add some basic HTTP logic to serve a frontend and accept an HTTP PUT request that contains a command.

If you're the type of person that prefers to learn by looking at a complete example, check out the [chess frontend section](../chess_app/frontend.md) for a real example application and a link to some frontend code.

## Adding HTTP request handling

Using the built-in HTTP server will require handling a new type of Request in our main loop, and serving a Response to it.
The [`process_lib`](../process_stdlib/overview.md) contains types and functions for doing so.

At the top of your process, import [`get_blob`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/kinode/process/standard/fn.get_blob.html), [`homepage`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/homepage/index.html), and [`http`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/http/index.html) from [`kinode_process_lib`](../process_stdlib/overview.md) along with the rest of the imports.
You'll use `get_blob()` to grab the `body` bytes of an incoming HTTP request.
```rust
{{#include ../../code/mfa_fe_demo/mfa_fe_demo/src/lib.rs:4:7}}
```

Keep the custom WIT-defined `MfaRequest` the same, and keep using that for terminal input.

At the beginning of the `init()` function, in order to receive HTTP requests, you must use the [`kinode_process_lib::http`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/http/index.html) library to bind a new path.
Binding a path will cause the process to receive all HTTP requests that match that path.
You can also bind static content to a path using another function in the library.

```rust
...
{{#include ../../code/mfa_fe_demo/mfa_fe_demo/src/lib.rs:91:94}}
...
```

[`http::bind_http_path("/", false, false)`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/http/fn.bind_http_path.html) arguments mean the following:
- The first argument is the path to bind.
Note that requests will be namespaced under the process name, so this will be accessible at e.g. `/my-process-name/`.
- The second argument marks whether to serve the path only to authenticated clients
In order to skip authentication, set the second argument to false here.
- The third argument marks whether to only serve the path locally.

To handle different kinds of Requests (or Responses), wrap them in a meta `Req` or `Res`:
```rust
{{#include ../../code/mfa_fe_demo/mfa_fe_demo/src/lib.rs:25:30}}
```
and `match` on it in the top-level `handle_message()`:
```rust
{{#include ../../code/mfa_fe_demo/mfa_fe_demo/src/lib.rs:74:88}}
```

Here, the [logic that was previously](./chapter_3.md#handling-messages) in `handle_message()` is now factored out into `handle_mfa_request()` and `handle_mfa_response()`:

```rust
{{#include ../../code/mfa_fe_demo/mfa_fe_demo/src/lib.rs:32:47}}

...

{{#include ../../code/mfa_fe_demo/mfa_fe_demo/src/lib.rs:66:72}}
```

As a side-note, different apps will want to discriminate between incoming messages differently.
For example, to restrict what senders are accepted (say to your own node or to some set of allowed nodes), your process can branch on the [`source().node`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/enum.Message.html#method.source).

### Handling an HTTP Message

Finally, define `handle_http_message()`.
```rust
{{#include ../../code/mfa_fe_demo/mfa_fe_demo/src/lib.rs:49:64}}
```

Walking through the code, first, you must parse out the HTTP request from the `HttpServerRequest`.
This is necessary because the `HttpServerRequest` enum contains both HTTP protocol requests and requests related to WebSockets.
If your application only needs to handle one type of request (e.g., only HTTP requests), you could simplify the code by directly handling that type without having to check for a specific request type from the `HttpServerRequest` enum each time.
This example is overly thorough for demonstration purposes.

```rust
{{#include ../../code/mfa_fe_demo/mfa_fe_demo/src/lib.rs:50:52}}
```

Next, check the HTTP method in order to only handle PUT requests:
```rust
{{#include ../../code/mfa_fe_demo/mfa_fe_demo/src/lib.rs:53:55}}
```

Finally, grab the `blob` from the request, send a `200 OK` response to the client, and handle the `blob` by sending a `Request` to ourselves with the `blob` as the `body`.
```rust
{{#include ../../code/mfa_fe_demo/mfa_fe_demo/src/lib.rs:56:62}}
```
This could be done in a different way, but this simple pattern is useful for letting HTTP requests masquerade as in-Kinode requests.

Putting it all together, you get a process that you can build and start, then use cURL to send `Hello` and `Goodbye` requests via HTTP PUTs!

### Requesting Capabilities

Also, remember to request the capability to message `http_server` in `manifest.json`:
```json
...
"request_capabilities": [
    "http_server:distro:sys"
],
...
```

### The Full Code

```rust
{{#include ../../code/mfa_fe_demo/mfa_fe_demo/src/lib.rs}}
```

Use the following cURL command to send a `Hello` Request
Make sure to replace the URL with your node's local port and the correct process name.
Note: if you had set `authenticated` to true in `bind_http_path()`, you would need to add an `Authorization` header to this request with the [JWT](https://jwt.io/) cookie of your node.
This is saved in your browser automatically on login.

```bash
curl -X PUT -d '{"Hello": "greetings"}' http://localhost:8080/mfa_fe_demo:mfa_fe_demo:template.os/api
```

You can find the full code [here](https://github.com/kinode-dao/kinode-book/tree/main/src/code/mfa_fe_demo).

There are a few lines we haven't covered yet: learn more about [serving a static frontend](#serving-a-static-frontend) and [adding a homepage icon and widget](#adding-a-homepage-icon-and-widget) below.

## Serving a static frontend

If you just want to serve an API, you've seen enough now to handle PUTs and GETs to your heart's content.
But the classic personal node app also serves a webpage that provides a user interface for your program.

You *could* add handling to root `/` path to dynamically serve some HTML on every GET.
But for maximum ease and efficiency, use the static bind command on `/` and move the PUT handling to `/api`.
To do this, edit the bind commands in `my_init_fn` to look like this:

```rust
{{#include ../../code/mfa_fe_demo/mfa_fe_demo/src/lib.rs:95:96}}
```

Here you are setting `authenticated` to `false` in the [`bind_http_path()`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/http/fn.bind_http_path.html) call, but to `true` in the [`serve_index_html`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/http/fn.serve_index_html.html) call.
This means the API is public; if instead you want the webpage to be served exclusively by the browser, change `authenticated` to `true` in `bind_http_path()` as well.

You must also add a static `index.html` file to the package.
UI files are stored in the `ui/` directory and built into the application by `kit build` automatically.
Create a `ui/` directory in the package root, and then a new file in `ui/index.html` with the following contents.
**Make sure to replace the fetch URL with your process ID!**

```html
{{#include ../../code/mfa_fe_demo/ui/index.html}}
```

This is a super barebones `index.html` that provides a form to make requests to the `/api` endpoint.
Additional UI dev info can be found [here](../apis/frontend_development.md).

Next, add two more entries to `manifest.json`: messaging capabilities to the [VFS](../system/files.md) which is required to store and access the UI `index.html`, and the `homepage` capability which is required to add our app to the user's homepage (next section):
```json
...
{{#include ../../code/mfa_fe_demo/pkg/manifest.json:7:11}}
...
```

After saving `ui/index.html`, rebuilding the program, and starting the package again with `kit bs`, you should be able to navigate to your `http://localhost:8080/mfa_fe_demo:mfa_fe_demo:template.os` and see the form page.
Because you now set `authenticated` to `true` in the `/api` binding, the webpage will still work, but cURL will not.

The user will navigate to `/` to see the webpage, and when they make a PUT request, it will automatically happen on `/api` to send a message to the process.

This frontend is now fully packaged with the process — there are no more steps!
Of course, this can be made arbitrarily complex with various frontend frameworks that produce a static build.

In the next and final section, learn about the package metadata and how to share this app across the Kinode network.


## Adding a Homepage Icon and Widget

In this section, you will learn how to customize your app icon with a clickable link to your frontend, and how to create a widget to display on the homepage.

### Adding the App to the Home Page

#### Encoding an Icon

Choosing an emblem is a difficult task.
You may elect to use your own, or use this one:

![gosling](../assets/gosling.png)

On the command line, encode your image as base64, and prepend `data:image/png;base64,`:

```bash
echo "data:image/png;base64,$(base64 < gosling.png)" | tr -d '\n' > icon
```

Then, move `icon` next to `lib.rs` in your app's `src/` directory.
Finally, include the icon data in your `lib.rs` file just after the imports:

```rust
{{#include ../../code/mfa_fe_demo/mfa_fe_demo/src/lib.rs:18}}
```

#### Clicking the Button

The Kinode process lib exposes an [`add_to_homepage()`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/homepage/fn.add_to_homepage.html) function that you can use to add your app to the homepage.

In your `init()`, add the following line:
This line in the `init()` function adds your process, with the given icon, to the homepage:

```rust
{{#include ../../code/mfa_fe_demo/mfa_fe_demo/src/lib.rs:96}}
```

### Writing a Widget

A widget is an HTML iframe.
Kinode apps can send widgets to the `homepage` process, which will display them on the user's homepage.
They are quite simple to configure.
In `add_to_homepage()`, the final field optionally sets the widget:

```rust
{{#include ../../code/mfa_fe_demo/mfa_fe_demo/src/lib.rs:96}}
```
which uses the `WIDGET` constant, here:
```rust
{{#include ../../code/mfa_fe_demo/mfa_fe_demo/src/lib.rs:20:23}}
```

After another `kit bs`, you should be able to reload your homepage and see your app icon under "All Apps", as well as your new widget.
To dock your app, click the heart icon on it.
Click the icon itself to go to the UI served by your app.

For an example of a more complex widget, see the source code of our [app store widget](#widget-case-study-app-store), below.

#### Widget Case Study: App Store

The app store's [widget](https://github.com/kinode-dao/kinode/blob/3719ab38e19143a7bcd501fd245c7a10b2239ee7/kinode/packages/app_store/app_store/src/http_api.rs#L59C1-L133C2) makes a single request to the node, to determine the apps that are listed in the app store.
It then creates some HTML to display the apps in a nice little list.

```html
<html>
{{#webinclude https://raw.githubusercontent.com/kinode-dao/kinode/3719ab38e19143a7bcd501fd245c7a10b2239ee7/kinode/packages/app_store/app_store/src/http_api.rs 62:130}}
</html>
```
