# Install `kit`


These documents describe some ways you can use these tools, but do not attempt to be completely exhaustive.
You are encouraged to make use of the `--help` flag, which can be used for the top-level `kit` command:

```
$ kit --help
Development toolkit for Kinode

Usage: kit <COMMAND>

Commands:
  boot-fake-node       Boot a fake node for development [aliases: f]
  build                Build a Kinode package [aliases: b]
  build-start-package  Build and start a Kinode package [aliases: bs]
  dev-ui               Start the web UI development server with hot reloading (same as `cd ui && npm i && npm run dev`) [aliases: d]
  inject-message       Inject a message to a running Kinode [aliases: i]
  new                  Create a Kinode template package [aliases: n]
  remove-package       Remove a running package from a node [aliases: r]
  reset-cache          Reset kit cache (Kinode core binaries, logs, etc.)
  run-tests            Run Kinode tests [aliases: t]
  setup                Fetch & setup kit dependencies
  start-package        Start a built Kinode process [aliases: s]
  update               Fetch the most recent version of kit
  help                 Print this message or the help of the given subcommand(s)

Options:
  -v, --version  Print version
  -h, --help     Print help
```

or for any of the subcommands, e.g.:

```
kit new --help
```

The first chapter of the [My First Kinode App tutorial](../my_first_app/chapter_1.md) shows the `kit` tools in action.

## Getting kit

To get `kit`, run

```
cargo install --git https://github.com/kinode-dao/kit
```

To update, run that same command or

```
kit update
```

You can find the source for `kit` at [https://github.com/kinode-dao/kit](https://github.com/kinode-dao/kit).

## Logging

Logs are printed to the terminal and stored, by default, at `/tmp/kinode-kit-cache/logs/log.log`.
The default logging level is `info`.
Other logging levels are: `debug`, `warning` and `error`.

These defaults can be changed by setting environment variables:

Environment Variable | Description
-------------------- | -----------
`KIT_LOG_PATH`       | Set log path (default `/tmp/kinode-kit-cache/logs/log.log`).
`RUST_LOG`           | Set log level (default `info`).

For example, in Bash:

```bash
export RUST_LOG=info
```
