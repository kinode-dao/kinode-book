# `kit inject-message`

short: `kit i`

`kit inject-message` injects the given message to the node running at given port/URL, e.g.,

```bash
kit inject-message foo:foo:template.os '{"Send": {"target": "fake2.os", "message": "hello world"}}'
```

## Discussion

`kit inject-message` injects the given message into the given node.
It is useful for:
1. Testing processes from the outside world during development
2. Injecting data into the node
3. Combining the above with `bash` or other scripting.
For example, using the [`--blob`](#--blob) flag you can directly inject the contents of a file.
You can script in the outside world, dump the result to a file, and inject it with `inject-message`.

By default, `inject-message` expects a Response from the target process.
To instead "fire and forget" a message and exit immediately, use the [`--non-block`](#--non-block) flag.

## Arguments

```
$ kit inject-message --help
Inject a message to a running Kinode

Usage: kit inject-message [OPTIONS] <PROCESS> <BODY_JSON>

Arguments:
  <PROCESS>    PROCESS to send message to
  <BODY_JSON>  Body in JSON format

Options:
  -p, --port <NODE_PORT>  localhost node port; for remote see https://book.kinode.org/hosted-nodes.html#using-kit-with-your-hosted-node [default: 8080]
  -n, --node <NODE_NAME>  Node ID (default: our)
  -b, --blob <PATH>       Send file at Unix path as bytes blob
  -l, --non-block         If set, don't block on the full node response
  -h, --help              Print help
```
