# `kit new`

`kit new` creates a Kinode package template at the specified path, e.g.,

```
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
kit new my_py_fib --language python --template fibonacci
```

## Discussion

You can create a variety of templates using `kit new`.
Currently, three languages are supported: `rust` (the default), `python`, and `javascript`.
Four templates are currently supported, as described in the [following section](./new.html#existshas-ui-enabled-version).
In addition, some subset of these templates also have a UI-enabled version.

### Exists/Has UI-enabled Version

The following table specifies whether a template "Exists/Has UI-enabled version" for each language/template combination:

Language     | `chat`  | `echo` | `fibonacci` | `file_transfer`
------------ | ------- | ------ | ----------- | ---------------
`rust`       | yes/yes | yes/no | yes/no      | yes/no
`python`     | yes/no  | yes/no | yes/no      | no/no
`javascript` | yes/no  | yes/no | yes/no      | no/no

Brief description of each template:

- `chat`: A simple chat app.
- `echo`: Echos back any message it receives.
- `fibonacci`: Computes the n-th Fibonacci number.
- `file_transfer`: Allows for file transfers between nodes.

## Arguments

```
$ kit new --help
Create a Kinode template package

Usage: kit new [OPTIONS] <DIR>

Arguments:
  <DIR>  Path to create template directory at

Options:
  -a, --package <PACKAGE>      Name of the package [default: DIR]
  -u, --publisher <PUBLISHER>  Name of the publisher [default: template.os]
  -l, --language <LANGUAGE>    Programming language of the template [default: rust] [possible values: rust, python, javascript]
  -t, --template <TEMPLATE>    Template to create [default: chat] [possible values: chat, echo, fibonacci, file_transfer]
      --ui                     If set, use the template with UI
  -h, --help                   Print help
```

### Positional arg: `DIR`

Create the template package in this directory.
By default the package name is set to the name specified here, if not supplied by `--package`.

### `--package`

short: `-a`

Name of the package; defaults to `DIR`.
Must be URL-safe.

### `--publisher`

short: `-u`

Name of the publisher; defaults to `template.os`.
Must be URL-safe.

### `--language`

short: `-l`

Template language; defaults to `rust`.
Currently supports `rust`, `python`, and `javascript`.

### `--template`

short: `-t`

Which template to create; defaults to `chat`.
Options are outlined in [Exists/Has UI-enabled version](./new.html#existshas-ui-enabled-version).

### `--ui`

Create the template with a UI.
Currently, only `rust` `chat` has UI support.
