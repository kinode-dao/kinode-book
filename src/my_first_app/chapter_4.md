# Frontend Time

After the last section, you should have a simple process that responds to two commands from the terminal.
In this section, you'll add some basic HTTP logic to serve a frontend and accept an HTTP PUT request that contains a command.

If you're the type of person that prefers to learn by looking at a complete example, check out the [chess frontend section](../chess_app/frontend.md) for a fleshed-out example and a link to some frontend code.

## Adding HTTP request handling

Using the built-in HTTP server will require handling a new type of request in our main loop, and serving a response to it.
The [process_lib](../process_stdlib/overview.md) contains types and functions for doing so.

At the top of your process, import `http`, `get_blob`, and `Message` from [`kinode_process_lib`](../process_stdlib/overview.md) along with the rest of the imports.
You'll use `get_blob()` to grab the `body` bytes of an incoming HTTP request.
```rust
use kinode_process_lib::{
    await_message, call_init, get_blob, http, println, Address, Message, Request, Response,
};
```

Keep the custom `body` type (i.e. `MyBody`) the same, and keep using that for terminal input.

At the beginning of the init function (here `my_init_fn()`), in order to receive HTTP requests, you must use the [`kinode_process_lib::http`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/http/index.html) library to bind a new path.
Binding a path will cause the process to receive all HTTP requests that match that path.
You can also bind static content to a path using another function in the library.

```rust
// ...
fn my_init_fn(our: Address) {
    println!("{our}: started");

    http::bind_http_path("/", false, false).unwrap();
    // ...
}
// ...
```

`http::bind_http_path("/", false, false)` arguments mean the following:
- The first argument is the path to bind. 
Note that requests will be namespaced under the process name, so this will be accessible at e.g. `/my_process_name/`.
- The second argument marks whether to serve the path only to authenticated clients
In order to skip authentication, set the second argument to false here.
- The third argument marks whether to only serve the path locally.

Now that you're handling multiple kinds of requests, let's refactor the loop to be more concise and move the request-specific logic to dedicated functions.
Put this right under the bind command:
```rust
loop {
    match await_message() {
        Ok(message) => {
            if message.source().process == "http_server:distro:sys" {
                handle_http_message(&our, &message);
            } else {
                if handle_hello_message(&message) {
                    break;
                }
            }
        }
        Err(_send_error) => {
            println!("got send error!");
        }
    }
}
```

Note that different apps will want to discriminate between incoming messages differently.
This code doesn't check the [`source`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/enum.Message.html#method.source)[`.node`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/kinode/process/standard/struct.Address.html#method.node) at all, for example.

The `handle_hello_message` will look just like what was in [Section 5.3.](./chapter_3.md)
However, since this logic is no longer inside the main loop, return a boolean to indicate whether or not to exit out of the loop.
Request handling can be separated out into as many functions is needed to keep the code clean.
```rust
/// Returns true if the process should exit.
fn handle_hello_message(message: &Message) -> bool {
    let Ok(body) = MyBody::parse(message.body()) else {
        println!("received a message with weird `body`!");
        return false;
    };
    if message.is_request() {
        // Respond to a Hello with a Hello, and a Goodbye by exiting
        // the loop, which will cause the process to exit.
        match body {
            MyBody::Hello(text) => {
                println!("got a Hello: {text}");
            }
            MyBody::Goodbye => {
                println!("goodbye!");
                return true;
            }
        }
    } else {
        // we only expect Hello responses. If we get a Goodbye, ignore it.
        match body {
            MyBody::Hello(text) => {
                println!("got a Hello response: {text}");
            }
            MyBody::Goodbye => {}
        }
    }
    return false;
}
```

### Handling an HTTP Message

Finally, let's define `handle_http_message`.
```rust
fn handle_http_message(our: &Address, message: &Message) {

}
```

Instead of directly parsing the `body` type from the message, parse the type that the `http_server` process gives us. 
This type is defined in the `kinode_process_lib::http` module for us:
```rust
// ...
let Ok(server_request) = http::HttpServerRequest::from_bytes(message.body()) else {
    println!("received a message with weird `body`!");
    return;
};
// ...
```

Next, you must parse out the HTTP request from the `HttpServerRequest`.
This is necessary because the `HttpServerRequest` enum contains both HTTP protocol requests and requests related to WebSockets.
If your application only needs to handle one type of request (e.g., only HTTP requests), you could simplify the code by directly handling that type without having to check for a specific request type from the `HttpServerRequest` enum each time.
This example is overly thorough for demonstration purposes.

```rust
// ...
let Some(http_request) = server_request.request() else {
    println!("received a WebSocket message, skipping");
    return;
};
// ...
```

Now, check the HTTP method in order to only handle PUT requests:
```rust
// ...
if http_request.method().unwrap() != http::Method::PUT {
    println!("received a non-PUT HTTP request, skipping");
    return;
}
// ...
```

Finally, grab the `blob` from the request, send a `200 OK` response to the client, and handle the `blob` by sending a `Request` to ourselves with the `blob` as the `body`.
This could be done in a different way, but this simple pattern is useful for letting HTTP requests masquerade as in-Kinode requests.
```rust
// ...
let Some(body) = get_blob() else {
    println!("received a PUT HTTP request with no body, skipping");
    return;
};
http::send_response(http::StatusCode::OK, None, vec![]);
Request::to(our).body(body.bytes).send().unwrap();
```

Putting it all together, you get a process which you can build and start, then use cURL to send `Hello` and `Goodbye` requests via HTTP PUTs!

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
use serde::{Deserialize, Serialize};
use kinode_process_lib::{
    await_message, call_init, get_blob, http, println, Address, Message, Request,
};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
});

#[derive(Serialize, Deserialize)]
enum MyBody {
    Hello(String),
    Goodbye,
}

impl MyBody {
    fn hello(text: &str) -> Vec<u8> {
        serde_json::to_vec(&MyBody::Hello(text.to_string())).unwrap()
    }

    fn goodbye() -> Vec<u8> {
        serde_json::to_vec(&MyBody::Goodbye).unwrap()
    }

    fn parse(bytes: &[u8]) -> Result<MyBody, serde_json::Error> {
        serde_json::from_slice::<MyBody>(bytes)
    }
}

call_init!(my_init_fn);
fn my_init_fn(our: Address) {
    println!("{our}: started");

    http::bind_http_path("/", false, false).unwrap();

    Request::to(&our)
        .body(MyBody::hello("hello world"))
        .send()
        .unwrap();

    loop {
        match await_message() {
            Ok(message) => {
                if message.source().process == "http_server:distro:sys" {
                    handle_http_message(&our, &message);
                } else {
                    if handle_hello_message(&message) {
                        break;
                    }
                }
            }
            Err(_send_error) => {
                println!("got send error!");
            }
        }
    }
}

/// Handle a message from the HTTP server.
fn handle_http_message(our: &Address, message: &Message) {
    let Ok(server_request) = http::HttpServerRequest::from_bytes(message.body()) else {
        println!("received a message with weird `body`!");
        return;
    };
    let Some(http_request) = server_request.request() else {
        println!("received a WebSocket message, skipping");
        return;
    };
    if http_request.method().unwrap() != http::Method::PUT {
        println!("received a non-PUT HTTP request, skipping");
        return;
    }
    let Some(body) = get_blob() else {
        println!("received a PUT HTTP request with no body, skipping");
        return;
    };
    http::send_response(http::StatusCode::OK, None, vec![]);
    Request::to(our).body(body.bytes).send().unwrap();
}

/// Returns true if the process should exit.
fn handle_hello_message(message: &Message) -> bool {
    let Ok(body) = MyBody::parse(message.body()) else {
        println!("received a message with weird `body`!");
        return false;
    };
    if message.is_request() {
        // Respond to a Hello with a Hello, and a Goodbye by exiting
        // the loop, which will cause the process to exit.
        match body {
            MyBody::Hello(text) => {
                println!("got a Hello: {text}");
            }
            MyBody::Goodbye => {
                println!("goodbye!");
                return true;
            }
        }
    } else {
        // we only expect Hello responses. If we get a Goodbye, ignore it.
        match body {
            MyBody::Hello(text) => {
                println!("got a Hello response: {text}");
            }
            MyBody::Goodbye => {}
        }
    }
    return false;
}
```

A cURL command to send a `Hello` request looks like this.
Make sure to replace the URL with your node's local port and the correct process name.
Note: if you had not set `authenticated` to false in the bind command, you would need to add an `Authorization` header to this request with the [JWT](https://jwt.io/) cookie of your node.
This is saved in your browser automatically on login.

```bash
curl -X PUT -H "Content-Type: application/json" -d '{"Hello": "greetings"}' "http://localhost:8080/my_process:my_package:template.os"
```

## Serving a static frontend

If you just want to serve an API, you've seen enough now to handle PUTs and GETs to your heart's content.
But the classic personal node app also serves a webpage that provides a user interface for your program.

You *could* add handling to root `/` path to dynamically serve some HTML on every GET.
But for maximum ease and efficiency, use the static bind command on `/` and move the PUT handling to `/api`. 
To do this, edit the bind commands in `my_init_fn` to look like this:

```rust
http::bind_http_path("/api", true, false).unwrap();
http::serve_index_html(&our, "ui", true, false, vec!["/"]).unwrap();
```

Note that you are setting `authenticated` to `true` in the `serve_index_html` and `bind_http_path` calls. 
The result of this is that the webpage will be able to get served by the browser, but not by the raw cURL request.

Now you can add a static `index.html` file to the package.
UI files are stored in the `ui/` directory and built into the application by `kit build` automatically.
Create a `ui/` directory in the package root, and then a new file in `ui/index.html` with the following contents.
**Make sure to replace the fetch URL with your process ID!**

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
  </head>
  <body>
    <main>
        <h1>This is a website!</h1>
        <p>Enter a message to send to the process:</p>
        <form id="hello-form" class="col">
        <input id="hello" required="" name="hello" placeholder="hello world" value="">
        <button> PUT </button>
      </form>
    </main>
	<script>
        async function say_hello(text) {
          const result = await fetch("/my_process:my_package:template.os/api", {
            method: "PUT",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ "Hello": text }),
          });
          console.log(result);
        }


        document.addEventListener("DOMContentLoaded", () => {
          const form = document.getElementById("hello-form");
          form.addEventListener("submit", (e) => {
            e.preventDefault();
            e.stopPropagation();
            const text = document.getElementById("hello").value;
            say_hello(text);
          });
        });
    </script>
  </body>
</html>
```

This is a super barebones `index.html` that provides a form to make requests to the `/api` endpoint.
Additional UI dev info can be found [here](../apis/frontend_development.md).

Next, add two more entries to `manifest.json`: messaging capabilities to the [VFS](../files.md) which is required to store and access the UI `index.html`, and the `homepage` capability which is required to add our app to the user's homepage (next section):
```json
...
"request_capabilities": [
    "vfs:distro:sys",
    "homepage:homepage:sys",
    ...
],
...
```

After saving `ui/index.html`, rebuilding the program, and starting the package again with `kit bs`, you should be able to navigate to your `http://localhost:8080/my_process:my_package:template.os` and see the form page.
Because you now set `authenticated` to `true` in the `/api` binding, the webpage will still work, but cURL will not.

The user will navigate to `/` to see the webpage, and when they make a PUT request, it will automatically happen on `/api` to send a message to the process.

This frontend is now fully packaged with the process — there are no more steps!
Of course, this can be made arbitrarily complex with various frontend frameworks that produce a static build.

In the next and final section, learn about the package metadata and how to share this app across the Kinode network.


## (Optional) Extra Credit: Homepage Icon and Widget

This section is optional!
You're free to go.
However, if you insist upon staying, you will learn how to customize your app icon with a clickable link to your frontend, and how to create a widget to display on the homepage.

### Adding Our App to the Home Page

#### Encoding an Icon

Choosing an emblem is a difficult task.
Thankfully, we have chosen one for you.
Let's use this gosling:

![gosling](./assets/gosling.png)

Or, you may elect to use your own.
No issue.

On the command line, encode your image as base64, and prepend `data:image/png;base64,`:

```sh
echo "data:image/png;base64," > icon
base64 < gosling.png >> icon
```

Then, move `icon` next to `lib.rs` in your app's directory.
Finally, include the icon data in your `lib.rs` file, early on, just after the imports: 

```rs
const ICON: &str = include_str!("./icon");
```

#### Clicking the Button

The Kinode process lib exposes an [`add_to_homepage`](https://docs.rs/kinode_process_lib/0.8.3/kinode_process_lib/homepage/fn.add_to_homepage.html) function that you can use to add your app to the homepage.

In your `my_init_fn`, add the following line:

```rs
homepage::add_to_homepage(
    "My App Name", // the name of your app  
    ICON, // the icon data (base64 encoded, prepended with "data:image/png;base64,")
    "/", // the path to your app's UI (/my_process:my_package:template.os/ is prepended automatically)
).unwrap();
```

Now, you can build and reinstall your package with `kit bs`, reload your node's homepage in the browser, and see your app icon under "All Apps"!
To dock your app, click the heart icon on it.
Click the icon itself to go to the UI served by your app.

### Writing a Widget 

A widget is an HTML iframe. 
Kinode apps can send widgets to the `homepage` process, which will display them on the user's homepage.
They are quite simple to configure.
In `add_to_homepage`, add an additional field: 

```rs
// inside the init function again

// you can embed an external URL
let widget: String = "<iframe src='https://example.com'></iframe>".to_string();
// or you can embed your own HTML
let widget: String = "<iframe><html><body><h1>Hello, Kinode!</h1></body></html></iframe>".to_string();

homepage::add_to_homepage(
    "My App Name",
    ICON,
    "/",
    widget, // the widget to display on the homepage
).unwrap();
```

After another `kit bs`, you should be able to reload your homepage and see your new widget.
For an example of a more complex widget, you can check out the source code of our app store widget.

#### Widget Case Study: App Store

The app store's [widget](https://github.com/kinode-dao/kinode/blob/3719ab38e19143a7bcd501fd245c7a10b2239ee7/kinode/packages/app_store/app_store/src/http_api.rs#L59C1-L133C2) makes a single request to the node, to determine the apps that are listed in the app store.
It then creates some HTML to display the apps in a nice little list.
The Tailwind CSS library is included in the HTML to make the UI look nice, but you don't need to do this, and we are going to remove it eventually because it's data-intensive.

```rs
fn make_widget() -> String {
    return r#"<html>
<head>
    <script src="https://cdn.tailwindcss.com"></script>
    <style>
        .app {
            width: 100%;
        }

        .app-image {
            background-size: cover;
            background-repeat: no-repeat;
            background-position: center;
        }

        .app-info {
            max-width: 67%
        }

        @media screen and (min-width: 500px) {
            .app {
                width: 49%;
            }
        }
    </style>
</head>
<body class="text-white overflow-hidden">
    <div
        id="latest-apps"
        class="flex flex-wrap p-2 gap-2 items-center backdrop-brightness-125 rounded-xl shadow-lg h-screen w-screen overflow-y-auto"
        style="
            scrollbar-color: transparent transparent;
            scrollbar-width: none;
        "
    >
    </div>
    <script>
        document.addEventListener('DOMContentLoaded', function() {
            fetch('/main:app_store:sys/apps/listed', { credentials: 'include' })
                .then(response => response.json())
                .then(data => {
                    const container = document.getElementById('latest-apps');
                    data.forEach(app => {
                        if (app.metadata) {
                            const a = document.createElement('a');
                            a.className = 'app p-2 grow flex items-stretch rounded-lg shadow bg-white/10 hover:bg-white/20 font-sans cursor-pointer';
                            a.href = `/main:app_store:sys/app-details/${app.package}:${app.publisher}`
                            a.target = '_blank';
                            a.rel = 'noopener noreferrer';
                            const iconLetter = app.metadata_hash.replace('0x', '')[0].toUpperCase();
                            a.innerHTML = `<div
                                class="app-image rounded mr-2 grow"
                                style="
                                    background-image: url('${app.metadata.image || `/icons/${iconLetter}`}');
                                    height: 92px;
                                    width: 92px;
                                    max-width: 33%;
                                "
                            ></div>
                            <div class="app-info flex flex-col grow">
                                <h2 class="font-bold">${app.metadata.name}</h2>
                                <p>${app.metadata.description}</p>
                            </div>`;
                                container.appendChild(a);
                        }
                    });
                })
                .catch(error => console.error('Error fetching apps:', error));
        });
    </script>
</body>
</html>"#
        .to_string();
}
```