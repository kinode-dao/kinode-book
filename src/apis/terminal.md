# Terminal API
It is extremely rare for an app to have direct access to the terminal api.
Normally, the terminal will be used to call scripts, which will have access to the process in question.
For documentation on using, writing, publishing, and composing scripts, see the [terminal use documentation](../terminal.md), or for a quick start, the [script cookbook](../cookbook/writing_scripts.md).

The Kinode terminal is broken up into two segments: a Wasm app, called `terminal:terminal:sys`, and a runtime module called `terminal:distro:sys`.
The Wasm app is the central area where terminal logic and authority live.
It parses `Requests` by attempting to read the `body` field as a UTF-8 string, then parsing that string into various commands to perform.
The runtime module exists in order to actually use this app from the terminal which is launched by starting Kinode OS.
It manages the raw input and presents an interface with features such as command history, text manipulation, and shortcuts.

To "use" the terminal as an API, one simply needs the capability to message `terminal:terminal:sys`.
This is a powerful capability, equivalent to giving an application `root` authority over your node.
For this reason, users are unlikely to grant direct terminal access to most apps.

If one does have the capability to send `Request`s to the terminal, they can execute commands like so:
`script_name:package_name:publisher_name <ARGS>`

For example, the `hi` script, which pings another node's terminal with a message, can be called like so: `hi:terminal:sys default-router-1.os what's up?`.
In this case, the arguments are both `default-router-1.os` and the message `what's up?`.

Some commonly used scripts have shorthand aliases because they are invoked so frequently.
For example, `hi:terminal:sys` can be shortened to just `hi` as in: `hi default-router-1.os what's up?`.

The other most commonly used script is `m:terminal:sys`, or just `m` - which stands for `Message`. `m` let's you send a request to any node or application like so:
```bash
m john.os@proc:pkg:pub '{"foo":"bar"}'
```

Note that if your process has the ability to message the `terminal` app, then that process can call any script - of which there may be many on a machine, so we cannot possibly write all of them down in this document.
However, they will all have this standard calling convention of `<script-name> <ARGS>`.
