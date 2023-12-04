# Chapter 2: Sending Some Messages, Using Some Tools

This chapter assumes you've completed the steps in Chapter 1 or otherwise have a basic Uqbar app in your code editor of choice. You should also have an Uqbar test node running such that you can quickly compile and test your code! Tight feedback loops when building: very important.

## What's already here

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

This is a basic Uqbar process that will print out any messages it receives. It's not very useful, but it's a good starting point for us to build on. The code here is common across nearly all Uqbar processes so it's worth understanding.

To interact with the Uqbar runtime, we use the `wit_bindgen::generate` macro to build a Rust interface to the WIT interface. See [uqbar.wit](./apis/uqbar_wit.md) for more details on what is imported by this. These are the necessary "system calls" for talking to other processes and runtime components.

We then define a `Component` struct and implement the `Guest` trait for it. This is just boilerplate for providing the singular `init()` function that the runtime will call when the process is started. The `init()` function is where we'll put our app logic. The `our` parameter is the address of the process, which you as a developer will define in your package metadata. We receive this as a raw `String`, but it will always be safe (and useful) to parse this into an `Address`.

## Sending a message

[joker_it's_about_sending_a_message.gif]

- ...

## Gathering capabilities

- in our package manifest:
    - request_messaging
    - grant_messaging

- from the WIT:
    - `get_capability()`
    - `share_capability()`

## Serving a frontend

- ...

## Using a database

- ...

## Saving a file

- ...

