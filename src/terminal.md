# Terminal

The [terminal syntax](https://github.com/kinode-dao/kinode?tab=readme-ov-file#terminal-syntax) is specified in the main Kinode repository.

## Commands

All commands in the [terminal](https://github.com/kinode-dao/kinode/tree/main/kinode/packages/terminal) are calling scripts — a special kind of process.
Kinode OS comes pre-loaded with a number of scripts useful for debugging and everyday use.
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
For your scripts to be usable by the terminal, you must include a `pkg/scripts.json` file, like [this one](https://github.com/kinode-dao/kinode/blob/main/kinode/packages/terminal/pkg/scripts.json).
Note that this is a core package and this file should not be edited, but rather you should create one in your own package.
For more discussion on package folder structure, look [here](https://book.kinode.org/my_first_app/chapter_1.html#exploring-the-package).

The JSON object in `scripts.json` describes the configuration for each script in your package.
Each top-level key represents the path of a process in your package, usually just `"myscript.wasm"`, `"echo.wasm"`, etc.

Within this JSON object, for each key (i.e., process) the value is an object that specifies the configuration for that particular process.
The object can contain the following fields:

- `root` (Boolean): Indicates whether the script has "root" privileges - meaning whether it gets *every* capability that the terminal has (not necessarily every capability in existence on your machine)
- `public` (Boolean): Determines if the script is publicly accessible by other processes
- `request_networking` (Boolean): Specifies whether the script will get networking capabilities
- `request_capabilities` (Array): An array that lists the capabilities requested by the script. Each element in the array can be either a string or an object.
The string represents a `ProcessId` that this script will be able to message. When an object is used, it specifies a different kind of capability from `issuer` with `params` as an arbitrary json object.
- `grant_capabilities` (Array of strings): An array of `ProcessId`s which represents which processes will be able to send a `Response` back to this script.
If this script is public, `grant_capabilities` can stay empty.

Processes may not necessarily use all these fields.
For instance, "m.wasm" only uses root, public, and `request_networking`, omitting `request_capabilities` and `grant_capabilities`.

### Example
This is a `scripts.json` that publishes a single script, `hi`, which doesn't receive `root` capabilities, is not `public`, can send messages over the network, will receive the capability to message `net:distro:sys`, and gives `net:distro:sys` the ability to message it back:
```json
{
    "hi.wasm": {
        "root": false,
        "public": false,
        "request_networking": true,
        "request_capabilities": [
            "net:distro:sys"
        ],
        "grant_capabilities": [
            "net:distro:sys"
        ]
    }
}
```
