# `kit inject-message`

`kit inject-message` injects the given message to the node running at given port/URL, e.g.,

```bash
kit inject-message foo:foo:template.nec '{"Send": {"target": "fake2.nec", "message": "hello world"}}'
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

```bash
$ kit i --help
Inject a message to a running Nectar node

Usage: kit inject-message [OPTIONS] <PROCESS> <BODY_JSON>

Arguments:
  <PROCESS>    PROCESS to send message to
  <BODY_JSON>  Body in JSON format

Options:
  -p, --port <NODE_PORT>  Node port: for use on localhost (overridden by URL) [default: 8080]
  -u, --url <URL>         Node URL (overrides NODE_PORT)
  -n, --node <NODE_NAME>  Node ID (default: our)
  -b, --blob <PATH>       Send file at Unix path as bytes blob
  -l, --non-block         If set, don't block on the full node response
  -h, --help              Print help
```

### First positional arg: `PROCESS`

The process to send the injected message to in the form of `<process_name>:<package_name>:<publisher>`.

### Second positional arg: `BODY_JSON`

The message body.

### `--port`

For nodes running on localhost, the port of the node; defaults to `8080`.
`--port` is overridden by `--url` if both are supplied.

### `--url`

The URL the node is hosted at.
Can be either localhost or remote.
`--url` overrides `--port` if both are supplied.

### `--node`

Node to target (i.e. the node portion of the address).
E.g.

```bash
kit inject-message foo:foo:template.nec '{"Send": {"target": "fake.nec", "message": "wow, it works!"}}' --node fake2.nec
```

sent to the port running `fake.nec` will forward the message from `fake.nec`s HTTP server to `fake2@foo:foo:template.nec`.

### `--blob`

Path to file to include as `lazy_load_blob`.

### `--non-block`

Don't block waiting for a Response from target process.
Instead, inject the message and immediately return.
