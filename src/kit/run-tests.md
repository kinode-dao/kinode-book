# `kit run-tests`

short: `kit t`

`kit run-tests` runs the tests specified by the given `.toml` file, or `tests.toml`, e.g.,

```
kit run-tests my_tests.toml
```

or

```
kit run-tests
```

to run the current working directory's `tests.toml` or the current package's `test/`.

## Discussion

`kit run-tests` runs a series of tests specified  by [a `.toml` file](#teststoml).
Each test is run in a fresh environment of one or more fake nodes.
A test can setup one or more packages before running a series of test packages.
Each test package is [a single-process package that accepts and responds with certain messages](#test-package-interface).

Tests are orchestrated from the outside of the node by `kit run-tests` and run on the inside of the node by the [`tester`](https://github.com/kinode-dao/kinode/tree/main/kinode/packages/tester) core package.
For a given test, the `tester` package runs the specified test packages in order.
Each test package must respond to the `tester` package with a `Pass` or `Fail`.
The `tester` package stops on the first `Fail`, or responds with a `Pass` if all tests `Pass`.
If a given test `Pass`es, the next test in the series is run.

Examples of tests are the [Kinode Book's code examples](https://github.com/kinode-dao/kinode-book/tree/main/code) and [`kit`s templates](https://github.com/kinode-dao/kit/tree/master/src/new/templates/rust).

## Arguments

```
$ kit run-tests --help
Run Kinode tests

Usage: kit run-tests [PATH]

Arguments:
  [PATH]  Path to tests configuration file [default: tests.toml]

Options:
  -h, --help  Print help
```
