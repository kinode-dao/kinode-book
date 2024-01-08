# Chapter 2: Sending Some Messages, Using Some Tools

This chapter assumes you've completed the steps outlined in [Chapter 1](./my_first_app/chapter_1) to construct your dev environment or otherwise have a basic Uqbar app open in your code editor of choice. You should also be actively running an Uqbar test node such that you can quickly compile and test your code! Tight feedback loops when building: very important.

## What's already here

In your `src/lib.rs`, your template should already have some basic code that looks something like this:

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

This code enacts a basic Uqbar process that will print out any messages it receives. It's not very useful on its own, but it's a good starting point for us to build on. The code here is common across nearly all Uqbar processes, so it's worth understanding.

To interact with the Uqbar runtime, we use the `wit_bindgen::generate` macro to build a Rust interface to the WIT interface. See [uqbar.wit](./apis/uqbar_wit.md) for more details on what is imported by this (TODO: needs clarification based on what is written in uqbar.wit). These imports are the necessary "system calls" for talking to other processes and runtime components in Uqbar.

A process must then have a `Component` struct and implement the `Guest` trait for it. This is just boilerplate for providing the singular `init()` function that the runtime will call when the process is started. The `init()` function is where we'll put our app logic. The `our` parameter is the address of the process, which you as a developer will define in your package metadata. We receive this address as a raw `String`, but it will always be safe (and useful) to parse this into an `Address`. (TODO link to address docstring)

## Sending a message

[joker_it's_about_sending_a_message.gif]

- ...

## Gathering capabilities

- in our package manifest:
    - request_capabilities
    - grant_capabilities

- from the WIT:
    - `save_capabilities()`

## Serving a frontend

- ...

## Using a database

- ...

## Saving a file

- ...

