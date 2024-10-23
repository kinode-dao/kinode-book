# Scripts
Scripts are just processes.
They are written almost exactly like applications, with a few key differences:
- Scripts always terminate, while apps may run indefinitely.
- When writing a script, you cannot control the `OnExit` behavior like you can with an application
- Scripts are called with an initial set of arguments (passed in via the terminal)
- Scripts are registered in the `scripts.json` file instead of the `manifest.json` file

## Writing a Script
Consider the simplest possible script: `echo` (found in the runtime [here](https://github.com/kinode-dao/kinode/blob/main/kinode/packages/terminal/echo)), which takes in an argument and prints it out again:
```rust
use kinode_process_lib::{script, Address};

wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v0",
});

script!(init);
fn init(_our: Address, args: String) -> String {
    args
}

```
From writing applications, this should look very familiar - the imports, `wit_bindgen::generate!`, etc.

Note the use of a macro `script!` instead of the usual `call_init!`.
This macro handles the boilerplate associated with script processes:
- Creating an init function, just like all processes
- Awaiting an initial request from the terminal, which provides the script with its arguments
- Parsing the body of that request into a string
- Returning a string to be either printed or sent as a response, depending on how the script was called

If you want to create an advanced script, consider looking at the source code of the `script!` macro in [process_lib](https://github.com/kinode-dao/process_lib/blob/9a53504693676094ba06f601312457675d10ca8a/src/scripting/mod.rs#L11).

## Publishing a Script
Unlike processes associated with a long-running application, which will be put into the `manifest.json`, scripts must be registered in a separate `scripts.json` file.
While very similar, there are a few important differences; here's an example that could live in your packages `pkg/scripts.json` file:
```json
{
    "echo.wasm": {
        "root": false,
        "public": false,
        "request_networking": false,
        "request_capabilities": [],
        "grant_capabilities": [],
        "wit_version": 0
    },
}
```
This `scripts.json` file corresponds to a package which publishes a single script, `echo`, which doesn't request `root` capabilities, or any capabilities for that matter.
The keys of this object are the process paths inside of the `pkg/` folder.
The name of the script will be the file path, with `.wasm` taken off.
The object that `echo.wasm` points to is very similar to `manifest.json`, with a few things removed, and `root` has been added:
- `root` means that all the capabilities held by `terminal:terminal:sys` are passed to this script (this is powerful, and rarely needed)
- `public`: same as `manifest.json` - corresponds to whether or not other processes can message `echo.wasm` without the messsaging cap
- `request_networking`: same as `manifest.json` - corresponds to whether or not this script will need to send messaages over the network
- `request_capabilities`: same as `manifest.json` - a list of capabilities that will be granted to this script on startup (NOTE if you have `root`, there is no reason to populate `request_capabilities` as well)
- `grant_capabilities`: same as `manifest.json` - a list of messaging caps to `echo.wasm` to be given to other processes on startup
As long as you have a `scripts.json` file, your scripts will be callable from the terminal when someone else downloads your package.

## Calling a Script
After having called `kit bs`, simply type `my-script:my-package:publisher <ARGS>` in the terminal.
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
