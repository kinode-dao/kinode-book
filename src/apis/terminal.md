# Terminal API

The Kinode terminal is broken up into two segments: a Wasm app, called `terminal:terminal:sys`, and a runtime module called `terminal:distro:sys`.
The Wasm app is the central area where terminal logic and authority live.
It parses `Requests` by attempting to read the `body` field as a UTF-8 string, then parsing that string into various commands (usually denoted by a `/`) to perform.
The runtime module exists in order to actually use this app from the terminal which is launched by starting Kinode OS.
It manages the raw input and presents an interface with features such as command history, text manipulation, and shortcuts.

To "use" the terminal as an API, one must simply send a `Request` to the `terminal:terminal:sys` module.
This is a powerful capability, as it allows the process to send a `Request` to the terminal and have it be parsed and executed.
For this reason, users are unlikely to grant direct terminal access to most apps.

If one does have the capability to send `Request`s to the terminal, they can use the following commands:

```
/hi <node_id> <message>
```
Send a raw network message to another node.

```
/app <address>
/a <address>
```
Set the terminal to send all subsequent messages to a certain process.

```
/app clear
/a clear
```
Remove the set process.

```
/message <address> <request_body>
/m <address> <request_body>
```
Send a `Request` with the given body to the given address.
If `/app` or `/a` has been used to set a process, the address parameter here must be omitted, and the set one will be used instead.

The plaintext format of an `Address` looks like <node_id>`@`<process_id>.
`ProcessId` is a triple of the form <process_name>`:`<package_name>`:`<publisher_name>.

Example address:
```
some_user.os@process_one:my_cool_software:my_username.os
```
