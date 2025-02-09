# `kit build-start-package`

short: `kit bs`

`kit build-start-package` builds, installs and starts the indicated package directory, or the current working directory if none supplied, e.g.,

```
kit build-start-package foo
```

or

```
kit build-start-package
```

## Discussion

`kit build-start-package` runs [`kit build`](./build.md) followed by [`kit start-package`](./start-package.md).

## Arguments

```
$ kit build-start-package --help
Build and start a Kinode package

Usage: kit build-start-package [OPTIONS] [DIR]

Arguments:
  [DIR]  The package directory to build [default: /home/nick/git/kit]

Options:
  -p, --port <NODE_PORT>
          localhost node port; for remote see https://book.kinode.org/hosted-nodes.html#using-kit-with-your-hosted-node [default: 8080]
  -d, --download-from <NODE>
          Download API from this node if not found
  -w, --world <WORLD>
          Fallback WIT world name
  -l, --local-dependency <DEPENDENCY_PACKAGE_PATH>
          Path to local dependency package (can specify multiple times)
  -a, --add-to-api <PATH>
          Path to file to add to api.zip (can specify multiple times)
      --no-ui
          If set, do NOT build the web UI for the process; no-op if passed with UI_ONLY
      --ui-only
          If set, build ONLY the web UI for the process
  -i, --include <INCLUDE>
          Build only these processes/UIs (can specify multiple times) (default: build all)
  -e, --exclude <EXCLUDE>
          Build all but these processes/UIs (can specify multiple times) (default: build all)
  -s, --skip-deps-check
          If set, do not check for dependencies
      --features <FEATURES>
          Pass these comma-delimited feature flags to Rust cargo builds
  -r, --reproducible
          Make a reproducible build using Docker
  -f, --force
          Force a rebuild
  -v, --verbose
          If set, output stdout and stderr
  -h, --help
          Print help
```
