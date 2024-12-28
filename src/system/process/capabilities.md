# Capability-Based Security

Capabilities are a security paradigm in which an ability that is usually handled as a *permission* (i.e. certain processes are allowed to perform an action if they are saved on an "access control list") are instead handled as a *token* (i.e. the process that possesses token can perform a certain action).
These unforgeable tokens (as enforced by the kernel) can be passed to other owners, held by a given process, and checked for.

On Kinode, each process has an associated set of capabilities, which are each represented internally as an arbitrary JSON object with a source process:

```rust
pub struct Capability {
    pub issuer: Address,
    pub params: String, // JSON-string
}
```
The kernel abstracts away the process of ensuring that a capability is not forged.
As a process developer, if a capability comes in on a message or is granted to you by the kernel, you are guaranteed that it is legitimate.

Runtime processes, including the kernel itself, the filesystem, and the HTTP client, issue capabilities to processes.
Then, when a request is made by a process, the responder verifies the process's capability.
If the process does not have the capability to make such a request, it will be denied.

To give a concrete example: the filesystem can read/write, and it has the capabilities for doing so.
The FS may issue capabilities to processes to read/write to certain drives.
A process can request to read/write somewhere, and then the FS checks if that process has the required capability.
If it does, the FS does the read/write; if not, the request will be denied.

[System level capabilities](#startup-capabilities-with-manifestjson) like the above can only be given when a process is first installed.


## Startup Capabilities with `manifest.json`

When developing an application, `manifest.json` will be your first encounter with capabilties. With this file, capabilities are directly granted to a process on startup.
Upon install, the package manager (also referred to as "app store") surfaces these requested capabilities to the user, who can then choose to grant them or not.
Here is a `manifest.json` example for the `chess` app:
```json
[
    {
        "process_name": "chess",
        "process_wasm_path": "/chess.wasm",
        "on_exit": "Restart",
        "request_networking": true,
        "request_capabilities": [
            "net:distro:sys"
        ],
        "grant_capabilities": [
            "http-server:distro:sys"
        ],
        "public": true
    }
]
```
By setting `request_networking: true`, the kernel will give it the `"networking"` capability. In the `request_capabilities` field, `chess` is asking for the capability to message `net:distro:sys`.
Finally, in the `grant_capabilities` field, it is giving `http-server:distro:sys` the ability to message `chess`.

When booting the `chess` app, all of these capabilities will be granted throughout your node.
If you were to print out `chess`' capabilities using `kinode_process_lib::our_capabilities() -> Vec<Capability>`, you would see something like this:

```rust
[
    // obtained because of `request_networking: true`
    Capability { issuer: "our-node.os@kernel:distro:sys", params: "\"network\"" },
    // obtained because we asked for it in `request_capabilities`
    Capability { issuer: "our-node.os@net:distro:sys", params: "\"messaging\"" }
]
```
Note that [userspace capabilities](#userspace-capabilities), those *created by other processes*, can also be requested in a package manifest, though it's not guaranteed that the user will have installed the process that can grant the capability.
Therefore, when a userspace process uses the capabilities system, it should have a way to grant capabilities through its `body` protocol, as described below.

## Userspace Capabilities

While the manifest fields are useful for getting a process started, it is not sufficient for creating and giving custom capabilities to other processes.
To create your own capabilities, simply declare a new one and attach it to a `Request` or `Response` like so:

```rust
let my_new_cap = kinode_process_lib::Capability::new(our, "\"my-new-capability\"");

Request::new()
    .to(a_different_process)
    .capabilities(vec![my_new_cap])
    .send();
```

On the other end, if a process wants to save and reuse that capability, they can do something like this:

```rust
kinode_process_lib::save_capabilities(req.capabilities);
```
This call will automatically save the caps for later use.
Next time you attach this capability to a message, whether that is for authentication with the `issuer`, or to share it with another process, it will reach the other side just fine, and they can check it using the exact same flow.

For a code example of creating and using capabilities in userspace, see [this cookbook recipe](../../cookbook/creating_and_using_capabilities.md).
