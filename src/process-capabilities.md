# Capability-Based Security
Capabilities are our security mechanism to provide fine-grained access control. They are a very simple struct:
```rust
pub struct Capability {
    pub issuer: Address,
    pub params: String, // JSON-string
}
```
The `issuer` represents the node+process that created this `Capability`, while the `params` encapsulate what authority/operations this capability gives you. `params` can encode permissions to read or write to a file in the vfs, the ability to message a particular process, an API key, and much more. Capabilities can either be granted directly to a process, or passed around and saved via messages.

## Startup Capabilities with `manifest.json`

When developing a process, the first encounter you will have with capabilities is with the `manifest.json` file, where capabilities are directly granted to a process on startup. Here is an example for the `chess` app:
```json
[
    {
        "process_name": "chess",
        "process_wasm_path": "/chess.wasm",
        "on_exit": "Restart",
        "request_networking": true,
        "request_capabilities": [
            "net:sys:uqbar"
        ],
        "grant_capabilities": [
            "http_server:sys:uqbar"
        ],
        "public": true
    }
]
```
By setting `request_networking: true`, the kernel will give it the `"networking"` capability. In the `request_capabilities` field, `chess` is asking for the capability to message `net:sys:uqbar`. Finally, in the `grant_capabilities` field, it is giving `http_server:sys:uqbar` the ability to message `chess`. 

When we boot the `chess` app, all of these capabilities will be granted throughout our node. If we were to print out `chess`' capabilities using `uqbar_process_lib::our_capabilities() -> Vec<Capability>`, we would see something like this:

```rust
[
    // obtained because of `request_networking: true`
    Capability { issuer: "our@kernel:sys:uqbar", params: "\"network\"" },
    // obtained because we asked for it in `request_capabilities`
    Capability { issuer: "our@net:sys:uqbar", params: "\"messaging\"" }
]
```

## Custom Capabilities

While the manifest fields are useful for getting a process started, it is not sufficient for creating and giving custom capabilities to other processes. To create our own capabilities, we can simply create a new one, and attach it to a `Request` or `Response` like so:

```rust
let my_new_cap = uqbar_process_lib::Capability::new(our, "\"my-new-capability\"");

Request::new()
    .to(a_different_process)
    .capabilities(vec![my_new_cap])
    .send();
```

On the other end, if a process wants to save and reuse that capability, they can do something like this:

```rust
uqbar_process_lib::save_capabilities(req.capabilities);
```
This call will automatically save the caps for later use. Next time you attach this cap to a message, whether that is for authentication with the `issuer`, or to share it with another process, it will reach the other side just fine, and they can check it using the exact same flow as above.