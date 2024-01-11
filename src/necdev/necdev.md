# NecDev

`necdev` is a suite of tools that make development on NectarOS ergonomic.

These documents describe some ways you can use these tools, but do not attempt to be completely exhaustive.
You are encouraged to make use of the `--help` flag, which can be used for the top-level `necdev`:

```bash
necdev --help
```

or for any of the subcommands, e.g.:

```bash
necdev new --help
```

## Getting NecDev

To get `necdev`, run

```bash
cargo install --git https://github.com/uqbar-dao/necdev
```

To update, run that same command or

```bash
necdev update
```

You can find the source for `necdev` at [https://github.com/uqbar-dao/necdev](https://github.com/uqbar-dao/necdev).

## Table of Contents

* [`necdev boot-fake-node`](./boot-fake-node.md)
* [`necdev new`](./new.md)
* [`necdev build`](./build.md)
* [`necdev start-package`](./start-package.md)
* [`necdev dev-ui`](./dev-ui.md)
* [`necdev inject-message`](./inject-message.md)
* [`necdev run-tests`](./run-tests.md)
