# Scripts
Scripts are just processes.
They are written almost exactly like applications, with a few key differences:
- Scripts always terminate, while apps may run indefinitely.
When writing a script, you cannot control the `OnExit` behavior like you can with an application
- Scripts are called with an initial set of arguments (passed in via the `terminal`)
- Scripts are registered in the `scripts.json` file instead of the `manifest.json` file

## Writing a Script
Consider the simplest possible script: `echo`, which takes in an argument, and prints it out again:
```rust
use kinode_process_lib::{await_next_request_body, call_init, println, Address, Response};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

call_init!(init);

fn init(_our: Address) {
    let Ok(args) = await_next_request_body() else {
        println!("echo: failed to get args, aborting");
        return;
    };

    match String::from_utf8(args.clone()) {
        Ok(s) => println!("{}", s),
        Err(_) => println!("{:?}", args),
    }
}
```
From writing applications, this should look very familiar - the imports, `wit_bindgen::generate!`, `call_init!`, `init(our: Address)`, etc. are all exactly the same.
The first unique thing about scripts is that we will have no `loop` where we `await_message`.
Instead, our initial arguments will come from a single message from the terminal - which we get by calling `await_next_message_body()`.
Next, all we do is `String`ify the message body, and print it out.

Arbitrary logic can be put below `await_next_message_body` - just like an app, you can fire-off a number of requests, choose to await their responses, handle errors, etc. just like normal.

In this case, `echo` sends simply reserializes the body to a `String` and prints it out.

## Publishing a Script
Unlike processes associated with a long-running application, which will be put into the `manifest.json`, scripts must be registered in a separate `scripts.json` file.
While very similar, there are a few important differences; here's an example that could live in your packages `pkg/scripts.json` file:
```json
{
    "echo.wasm": {
        "root": false,
        "public": false,
        "requestNetworking": false,
        "requestCapabilities": [],
        "grantCapabilities": []
    }
}
```
This `scripts.json` file corresponds to a package which publishes a single script, `echo`, which doesn't request `root` capabilities, or any capabilities for that matter.
The keys of this object are the process paths inside of the `pkg/` folder.
The name of the script will be the file path, with `.wasm` taken off.
The object that `echo.wasm` points to is very similar to `manifest.json`, with a few things removed, and `root` has been added:
- `root` means that all the capabilities held by the `terminal:terminal:sys` are passed to this script (this is powerful, and rarely needed)
- `public`: same as `manfiest.json` - corresponds to whether or not other processes can message `echo.wasm` without the messsaging cap
- `requestNetworking`: same as `manfiest.json` - corresponds to whether or not this script will need to send messaages over the network
- `requestCapabilities`: same as `manifest.json` - a list of capabilities that will be granted to this script on startup (NOTE if you have `root`, there is no reason to populate `requestCapabilities` as well)
- `grantCapabilities`: same as `manifest.json` - a list of messaging caps to `echo.wasm` to be given to other processes on startup
As long as you have a `scripts.json` file, your scripts will be callable from the terminal when someone else downloads your package

## Calling a Script
Calling a script is very easy, simply type in the terminal `my_script:my_package:publisher <ARGS>` in the terminal.
For instance, the `echo` script is published as part of `terminal:sys`, so you can call
```bash
echo:terminal:sys Hello World!
```

## Aliasing a Script
If you are going to be calling your script very often, you can alias it to something shorter like so:
```bash
alias echo echo:terminal:sys
```
so now you can call `echo` like `echo Hello World!`.

To remove the alias, simply run:
```bash
alias echo
```
