# `kit new`

`kit new` creates a Kinode package template at the specified path, e.g.,

```bash
kit new foo
```

creates the default template (a Rust chat app with no UI) in the `foo/` directory.

## Example Usage

```bash
# Create the default template: rust chat with no UI
kit new my_rust_chat

# Create rust chat with UI
kit new my_rust_chat_with_ui --ui

# Create fibonacci in python
kit new my_py_fib -l python -t fibonacci
```

## Discussion

You can create a variety of templates using `kit new`.
Currently, three languages are supported: `rust` (the default), `python`, and `javascript`.
Two templates are currently supported: `chat`, a simple chat application, and `fibonacci`, which computes Fibonacci numbers.
In addition, some subset of these templates also have a UI-enabled version.

The following table describes the matrix of "Exists/Has UI-enabled version" for each template/language combination:

### Exists/Has UI-enabled vesion

Language     | `chat`  | `echo` | `fibonacci`
------------ | ------- | ------ | -----------
`rust`       | yes/yes | yes/no | yes/no
`python`     | yes/no  | yes/no | yes/no
`javascript` | yes/no  | yes/no | yes/no

## Arguments

```bash
$ kit n --help
Create a Kinode template package

Usage: kit new [OPTIONS] <DIR>

Arguments:
  <DIR>  Path to create template directory at

Options:
  -a, --package <PACKAGE>      Name of the package [default: DIR]
  -u, --publisher <PUBLISHER>  Name of the publisher [default: template.os]
  -l, --language <LANGUAGE>    Programming language of the template [default: rust] [possible values: rust, python, javascript]
  -t, --template <TEMPLATE>    Template to create [default: chat] [possible values: chat, echo, fibonacci]
      --ui                     If set, use the template with UI
  -h, --help                   Print help
```

### Positional arg: `DIR`

Where to create the template package.
The package name is set to this by default if not supplied by `--package`.

### `--package`

Name of the package; defaults to `DIR`.
Must be URL-safe.

### `--publisher`

Name of the publisher; defaults to `template.uq`.
Must be URL-safe.

### `--language`

Template language; defaults to `rust`.
Currently support `rust`, `python`, and `javascript`.

### `--template`

Which template to create; defaults to `chat`.
Currently have `chat`, a simple chat application, `echo`, an application that prints and responds with the received message, and `fibonacci`, a naive fibonacci-number-computer.

### `--ui`

Create the template with a UI.
Currently, only `rust` `chat` has UI support.
