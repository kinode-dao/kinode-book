# `kit run-tests`

`kit run-tests` runs the tests specified by the given `.toml` file, or `tests.toml`, e.g.,

```bash
kit run-tests my_tests.toml
```

or

```bash
kit run-tests
```

to run the current working directory's `tests.toml`.

## Example Usage

## Discussion

## Arguments

```bash
$ kit t --help
Run Nectar tests

Usage: kit run-tests [PATH]

Arguments:
  [PATH]  Path to tests configuration file [default: tests.toml]

Options:
  -h, --help  Print help
```

### Optional positional arg: `PATH`

Path to `.toml` file specifying tests to run; defaults to `tests.toml` in current working directory.

## `tests.toml`

The testing protocol is specified by a `.toml` file.

TODO
