# kit

`kit` is a tool**kit** that make development on Kinode OS ergonomic.

These documents describe some ways you can use these tools, but do not attempt to be completely exhaustive.
You are encouraged to make use of the `--help` flag, which can be used for the top-level `kit`:

```
$ kit --help
Development toolkit for Kinode OS

Usage: kit <COMMAND>

Commands:
  boot-fake-node       Boot a fake node for development [aliases: f]
  build                Build a Kinode package [aliases: b]
  build-start-package  Build and start a Kinode package [aliases: bs]
  dev-ui               Start the web UI development server with hot reloading (same as `cd ui && npm i && npm start`) [aliases: d]
  inject-message       Inject a message to a running Kinode [aliases: i]
  new                  Create a Kinode template package [aliases: n]
  run-tests            Run Kinode tests [aliases: t]
  remove-package       Remove a running package from a node [aliases: r]
  setup                Fetch & setup kit dependencies
  start-package        Start a built Kinode process [aliases: s]
  update               Fetch the most recent version of kit
  help                 Print this message or the help of the given subcommand(s)

Options:
  -v, --version  Print version
  -h, --help     Print help
```

or for any of the subcommands, e.g.:

```bash
kit new --help
```

The first chapter of the [Build and Deploy an App tutorial](../my_first_app/chapter_1.md) shows the `kit` tools in action.

## Getting kit

To get `kit`, run

```bash
cargo install --git https://github.com/kinode-dao/kit
```

To update, run that same command or

```bash
kit update
```

You can find the source for `kit` at [https://github.com/kinode-dao/kit](https://github.com/kinode-dao/kit).

## Table of Contents

* [`kit boot-fake-node`](./boot-fake-node.md)
* [`kit new`](./new.md)
* [`kit build`](./build.md)
* [`kit start-package`](./start-package.md)
* [`kit dev-ui`](./dev-ui.md)
* [`kit inject-message`](./inject-message.md)
* [`kit run-tests`](./run-tests.md)
