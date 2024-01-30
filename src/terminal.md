# Terminal

## Basic Usage and Utilities

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
