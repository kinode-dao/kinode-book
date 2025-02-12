# `kit new`

short: `kit n`

`kit new` creates a Kinode package template at the specified path, e.g.,

```
kit new foo
```

creates the default template (a Rust chat app with no UI) in the `foo/` directory.

The package name must be "Kimap-safe": contain only a-z, 0-9, and `-`.

## Example Usage

```bash
# Create the default template: rust chat with no UI
kit new my-rust-chat

# Create rust chat with UI
kit new my-rust-chat-with-ui --ui
```

## Discussion

You can create a variety of templates using `kit new`.
Currently, one language is supported: `rust`.
Ask us in the [Discord](https://discord.gg/mYDj74NkfP) about `python`, and `javascript` templates.
Four templates are currently supported, as described in the [following section](./new.html#existshas-ui-enabled-version).
In addition, some subset of these templates also have a UI-enabled version.

### Exists/Has UI-enabled Version

The following table specifies whether a template "Exists/Has UI-enabled version" for each language/template combination:

Language     | `chat`  | `echo` | `fibonacci` | `file-transfer`
------------ | ------- | ------ | ----------- | ---------------
`rust`       | yes/yes | yes/no | yes/no      | yes/no

Brief description of each template:

- `chat`: A simple chat app.
- `echo`: Echos back any message it receives.
- `fibonacci`: Computes the n-th Fibonacci number.
- `file-transfer`: Allows for file transfers between nodes.

## Arguments

```
$ kit new --help
Create a Kinode template package

Usage: kit new [OPTIONS] <DIR>

Arguments:
  <DIR>  Path to create template directory at (must contain only a-z, 0-9, `-`)

Options:
  -a, --package <PACKAGE>      Name of the package (must contain only a-z, 0-9, `-`) [default: DIR]
  -u, --publisher <PUBLISHER>  Name of the publisher (must contain only a-z, 0-9, `-`, `.`) [default: template.os]
  -l, --language <LANGUAGE>    Programming language of the template [default: rust] [possible values: rust]
  -t, --template <TEMPLATE>    Template to create [default: chat] [possible values: blank, chat, echo, fibonacci, file-transfer]
      --ui                     If set, use the template with UI
  -h, --help                   Print help
```
