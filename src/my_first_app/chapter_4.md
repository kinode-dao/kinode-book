# Chapter 4: Frontend Time

After the last chapter, you should have a simple process that responds to two commands from the terminal. In this chapter, you'll add some basic HTTP logic to serve a frontend and accept an HTTP PUT request that contains a command.

If you're the type of person that prefers to learn by looking at a complete example, check out the [chess frontend chapter](../chess_app/frontend.md) for a fleshed-out example and a link to some frontend code.

## Adding HTTP request handling

Using the built-in HTTP server will require handling a new type of request in our main loop, and serving a response to it. The process_lib contains types and functions for doing so.

At the top of your process, import `http`, `get_payload`, and `Message` from `uqbar_process_lib` along with the rest of the imports. You'll use `get_payload()` to grab the body bytes of an incoming HTTP request.
```rust
use uqbar_process_lib::{
    await_message, call_init, get_payload, http, println, Address, Message, Request, Response,
};
```

Keep the custom IPC type the same, and keep using that for terminal input.

At the beginning of the init function, in order to receieve HTTP requests, you must use the `uqbar_process_lib::http` library to bind a new path. Binding a path will cause the process to receive all HTTP requests that match that path. You can also bind static content to a path using another function in the library.
```rust
// ...
fn my_init_fn(our: Address) {
    println!("{our}: started");
    // the first argument is the path to bind. Note that requests will be namespaced
    // under the process name, so this will be accessible at /my_process/

    // the second argument marks whether to serve the path only to authenticated clients,
    // and the third argument marks whether to only serve the path locally.
    // in order to skip authentication, set the second argument to false here.
    http::bind_http_path("/", false, false).unwrap();
    // ...
}
// ...
```

Now that you're handling multiple kinds of requests, let's refactor the loop to be more concise and move the request-specific logic to dedicated functions. Put this right under the bind command:
```rust
loop {
    match await_message() {
        Ok(message) => {
            if message.source().process == "http_server:sys:uqbar" {
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

Note that different apps will want to discriminate between incoming messages differently. This code doesn't check the `source.node` at all, for example.

The `handle_hello_message` will look just like what was in chapter 3. However, since this logic is no longer inside the main loop, return a boolean to indicate whether or not to exit out of the loop. Request handling can be separated out into as many functions is needed to keep the code clean.
```rust
/// Returns true if the process should exit.
fn handle_hello_message(message: &Message) -> bool {
    let Ok(ipc) = MyIPC::parse(message.ipc()) else {
        println!("received a message with weird IPC!");
        return false;
    };
    if message.is_request() {
        // Respond to a Hello with a Hello, and a Goodbye by exiting
        // the loop, which will cause the process to exit.
        match ipc {
            MyIPC::Hello(text) => {
                println!("got a Hello: {text}");
            }
            MyIPC::Goodbye => {
                println!("goodbye!");
                return true;
            }
        }
    } else {
        // we only expect Hello responses. If we get a Goodbye, ignore it.
        match ipc {
            MyIPC::Hello(text) => {
                println!("got a Hello response: {text}");
            }
            MyIPC::Goodbye => {}
        }
    }
    return false;
}
```

Finally, let's define `handle_http_message`.
```rust
fn handle_http_message(our: &Address, message: &Message) {

}
```

Instead of parsing our IPC type from the message, parse the type that the `http_server` process gives us. This type is defined in the `uqbar_process_lib::http` module for us:
```rust
// ...
let Ok(server_request) = http::HttpServerRequest::from_bytes(message.ipc()) else {
    println!("received a message with weird IPC!");
    return;
};
// ...
```

Next, you must parse out the HTTP request from the general type. This is necessary because the `HttpServerRequest` enum contains both HTTP protocol requests and requests related to WebSockets. Note that it's quite possible to streamline this series of request refinements if you're only interested in one type of request -- this example is overly thorough for demonstration purposes.

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

Finally, grab the payload from the request, send a 200 OK response to the client, and handle the payload, by sending a Request to ourselves with the payload as the IPC. This could be done in a different way, but this simple pattern is useful for letting HTTP requests masquerade as in-Uqbar requests.
```rust
// ...
let Some(body) = get_payload() else {
    println!("received a PUT HTTP request with no body, skipping");
    return;
};
http::send_response(http::StatusCode::OK, None, vec![]).unwrap();
Request::to(our).ipc(body.bytes).send().unwrap();
```

Putting it all together, you get a process which you can build and start, then use cURL to send Hello and Goodbye requests via HTTP PUTs! Here's the full code:
```rust
use serde::{Deserialize, Serialize};
use uqbar_process_lib::{
    await_message, call_init, get_payload, http, println, Address, Message, Request, Response,
};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

#[derive(Serialize, Deserialize)]
enum MyIPC {
    Hello(String),
    Goodbye,
}

impl MyIPC {
    fn hello(text: &str) -> Vec<u8> {
        serde_json::to_vec(&MyIPC::Hello(text.to_string())).unwrap()
    }

    fn goodbye() -> Vec<u8> {
        serde_json::to_vec(&MyIPC::Goodbye).unwrap()
    }

    fn parse(bytes: &[u8]) -> Result<MyIPC, serde_json::Error> {
        serde_json::from_slice::<MyIPC>(bytes)
    }
}

call_init!(my_init_fn);

fn my_init_fn(our: Address) {
    println!("{our}: started");

    http::bind_http_path("/", false, false).unwrap();

    Request::new()
        .target(&our)
        .ipc(MyIPC::hello("hello world"))
        .send()
        .unwrap();

    loop {
        match await_message() {
            Ok(message) => {
                if message.source().process == "http_server:sys:uqbar" {
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
    let Ok(server_request) = http::HttpServerRequest::from_bytes(message.ipc()) else {
        println!("received a message with weird IPC!");
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
    let Some(body) = get_payload() else {
        println!("received a PUT HTTP request with no body, skipping");
        return;
    };
    http::send_response(http::StatusCode::OK, None, vec![]).unwrap();
    Request::to(our).ipc(body.bytes).send().unwrap();
}

/// Returns true if the process should exit.
fn handle_hello_message(message: &Message) -> bool {
    let Ok(ipc) = MyIPC::parse(message.ipc()) else {
        println!("received a message with weird IPC!");
        return false;
    };
    if message.is_request() {
        // Respond to a Hello with a Hello, and a Goodbye by exiting
        // the loop, which will cause the process to exit.
        match ipc {
            MyIPC::Hello(text) => {
                println!("got a Hello: {text}");
            }
            MyIPC::Goodbye => {
                println!("goodbye!");
                return true;
            }
        }
    } else {
        // we only expect Hello responses. If we get a Goodbye, ignore it.
        match ipc {
            MyIPC::Hello(text) => {
                println!("got a Hello response: {text}");
            }
            MyIPC::Goodbye => {}
        }
    }
    return false;
}
```

A cURL command to send a Hello request looks like this. Make sure to replace the URL with your node's local port and the correct process name. Note: if you had not set `authenticated` to false in the bind command, you would need to add an `Authorization` header to this request with the JWT cookie of your node. This is saved in your browser automatically on login.
```bash
curl -X PUT -H "Content-Type: application/json" -d '{"Hello": "greetings"}' "http://localhost:8080/tutorial:tutorial:template.uq"
```

## Serving a static frontend

If you just want to serve an API, you've seen enough now to handle PUTs and GETs to your heart's content. But the classic personal node app also serves a webpage that provides a user interface for your program.

You *could* add handling to our `/` path to dynamically serve some HTML on every GET. But for maximum ease and efficiency, use the static bind command on `/` and move our PUT handling to `/api`. To do this, edit the bind commands in `my_init_fn` to look like this:
```rust
http::bind_http_path("/api", true, false).unwrap();
http::serve_index_html(&our, "static").unwrap();
```

Now you can add a static `index.html` file to the package. Create a new file in `pkg/static/index.html` with the following contents. **Make sure to replace the fetch URL with your process ID!**
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
          const result = await fetch("/tutorial:tutorial:template.uq/api", {
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

This is a super barebones `index.html` that provides a form to make requests to our /api endpoint. After saving this file to `pkg/static/index.html`, rebuilding the program, and starting the package again, you should be able to navigate to you `http://localhost:8080/<process_id>` and see the form page. Note that you can now set `authenticated` to `true` in the /api binding and the webpage will still work, but cURL will not.

This frontend is now fully packaged with the process -- there are no more steps! Of course, this can be made arbitrarily complex with various frontend frameworks that produce a static build.

In the next and final chapter, we'll quickly go over the package metadata and discuss how to share this app across the Uqbar network.
