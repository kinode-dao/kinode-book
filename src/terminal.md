# Terminal

## Commands

All commands in the terminal are calling scripts - a special kind of process.
KinodeOS comes pre-loaded with a number of scripts useful for debugging and everyday use.
These scripts are fully named `<SCRIPT>:terminal:sys` e.g `hi:terminal:sys`, but the distro [aliases](#alias---alias-a-script-name) these to short names, in this case just `hi`, for convenience.


### `hi` - ping another kinode
```bash
Usage: hi <KNS_ID> <MESSAGE>
Arguments:
  <KNS_ID>  id of the node you want to message, e.g. some-node.os
  <MESSAGE> any string
Example:
hi other-node.os Hello other-node.os! how are you?
```

### `m` - message a process
```bash
Usage: m <ADDRESS> <BODY>
Arguments:
  <ADDRESS> kns addresss e.g. some-node.os@process:pkg:publisher.os
  <BODY>    json payload wrapped in single quotes, e.g. '{"foo": "bar"}'
Options:
  -a, --await <SECONDS> await the response, timing out after SECONDS
Example:
  m -a 5 our@foo:bar:baz '{"some payload": "value"}'
    - this will  await the response and print it out
  m our@foo:bar:baz '{"some payload": "value"}'
    - this one will not await the response or print it out
```

### `top` - display information about processes
```bash
Usage: top [PROCESS_ID]
Arguments:
  [PROCESS_ID] optional process id, just print information about this process
Example:
  top
    - this prints all information for all processes
  top terminal:terminal:sys
    - this prints information for just the requested process
```

### `alias` - alias a script name
```bash
Usage: alias <NAME> [SCRIPT]
Arguments:
  <NAME>   the name you want to assign the script to
  [SCRIPT] the script-id
Example:
  alias my-script my-script:my-package:my-name.os
    - this lets you call my-script in the terminal as a shorthand
  alias my-script
    - this removes the my-script alias
```

### `cat` - print the contents of a file in your vfs
```bash
Usage: cat <FILE_PATH>
Arguments:
  <FILE_PATH> the file path in your vfs
Example:
  cat terminal:sys/pkg/scripts.json
```

### `echo` - print the argument
`echo` is mostly an example script for developers to look at.
```bash
Usage: echo <MESSAGE>
Arguments:
  <MESSAGE> any string
Example:
  echo Hello World!
```

For more information on writing your own scripts, see the [cookbook](./cookbook/writing_scripts.md).

## Packaging Scripts with `scripts.json`
For your scripts to be usable by the terminal, you must include a `pkg/scripts.json` file.
The JSON object in `scripts.json` describes the configuration for each script in your package.
Each top-level key represents the path of the WASM module in your package, usually just `"myscript.wasm"`, `"echo.wasm"`, etc.

The value for each module is an object that specifies the configuration for that particular module.
The object can contain the following fields:

- `root` (Boolean): Indicates whether the script has "root" privileges - meaning whether it gets *every* capability that the terminal has (not necessarily every capability in existence on your machine)
- `public` (Boolean): Determines if the script is publicly accessible by other processes
- `requestNetworking` (Boolean): Specifies whether the script will get networking capabilities
- `requestCapabilities` (Array): An array that lists the capabilities requested by the script. Each element in the array can be either a string or an object. The string represents a `ProcessId` that this script will be able to message. When an object is used, it specifies a different kind of capability from `issuer` with `params` as an arbitrary json object.
- `grantCapabilities` (Array): An array `ProcessId`s as strings which represent which processes will be able to message this script back

Modules may not necessarily use all these fields. For instance, "m.wasm" only uses root, public, and requestNetworking, omitting requestCapabilities and grantCapabilities.

### Example
This is a `scripts.json` that publishes a single script, `hi`, which doesn't receive `root` capabilities, is not `public`, can send messages over the network, will receive the capability to message `net:distro:sys`, and gives `net:distro:sys` the ability to message it back:
```json
{
    "hi.wasm": {
        "root": false,
        "public": false,
        "requestNetworking": true,
        "requestCapabilities": [
            "net:distro:sys"
        ],
        "grantCapabilities": [
            "net:distro:sys"
        ]
    }
}
```

## Piping and Composing Scripts
### Using Pipes in the Terminal
The terminal lets you pipe the previous command's output into another script.
For exapmle:
```bash
echo default-router-1.os Hello! |1 hi
```
Since the `echo` script just returns its arguments, we are piping the string `default-router-1.os Hello!` into `hi`, so the end result is just 
```bash
hi default-router-1.os Hello!
```
This is exactly the same as a unix pipe except for the number that comes after the pipe.
This number after the pipe represents timeout in seconds.
In this case: wait for `echo` to respond, and if it does not after 1 second, abort.
Any `u64` value is a valid timeout value.

### Writing Your Script to be Pipe-able
Not all scripts are pipable.
For a script's output to be pipeable you must take the output and `.send()` it in a `Response`.
The [cookbook](./cookbook/writing_scripts.md) describes how every script begins with a `Request` that contains arguments in the `body`.
If you want the outputs of your script to be composable via pipes, you must terminate your script with a `Response` to this initial request.
As a simple example, let's look at the `echo` code:
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

    let _ = Response::new().body(args).send();
}
```

All we do here is get the args, and send a response back with the exact same args.
If instead of sending a `Response` we simply `println`d the arguments out, then we wouldn't be able to compose `echo` with pipes.
