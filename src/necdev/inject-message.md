# `necdev inject-message`

`necdev inject-message` injects the given message to the node running at given port/URL, e.g.,

```bash
necdev inject-message foo:foo:template.nec '{"Send": {"target": "fake2.nec", "message": "hello world"}}'
```

## Arguments

```bash
$ necdev inject-message --help
Inject a message to a running Nectar node

Usage: necdev inject-message [OPTIONS] <PROCESS> <JSON>

Arguments:
  <PROCESS>  Process to send message to
  <JSON>     Body in JSON format

Options:
  -p, --port <NODE_PORT>  Node port: for use on localhost (overridden by URL) [default: 8080]
  -u, --url <URL>         Node URL (overrides NODE_PORT)
  -n, --node <NODE_NAME>  Node ID (default: our)
  -b, --blob <PATH>       Send file at Unix path as bytes blob
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
necdev inject-message foo:foo:template.nec '{"Send": {"target": "fake.nec", "message": "wow, it works!"}}' --node fake2.nec
```

sent to the port running `fake.nec` will forward the message from `fake.nec`s HTTP server to `fake2@foo:foo:template.nec`.

### `--blob`

Path to file to include as `lazy_load_blob`.
