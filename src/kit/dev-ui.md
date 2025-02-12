# `kit dev-ui`

short: `kit d`

`kit dev-ui` starts a web development server with hot reloading for the indicated UI-enabled package (or the current working directory), e.g.,

```
kit dev-ui foo
```

or

```
kit dev-ui
```

## Arguments

```
$ kit dev-ui --help
Start the web UI development server with hot reloading (same as `cd ui && npm i && npm run dev`)

Usage: kit dev-ui [OPTIONS] [DIR]

Arguments:
  [DIR]  The package directory to build (must contain a `ui` directory) [default: CWD]

Options:
  -p, --port <NODE_PORT>  localhost node port; for remote see https://book.kinode.org/hosted-nodes.html#using-kit-with-your-hosted-node [default: 8080]
      --release           If set, create a production build
  -s, --skip-deps-check   If set, do not check for dependencies
  -h, --help              Print help
```
