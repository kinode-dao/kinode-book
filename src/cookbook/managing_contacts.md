# Managing Contacts

Like iOS and Android, Kinode OS includes a handy contacts system primitive, called `contacts:contacts:sys`.
Using it is optional, but as a peer-to-peer application developer, importing existing contacts is a great way to bootstrap your protocol.
Given the proper capabilities, an app can get the list of existing contacts, get information about a specific contact or all contacts, add new contacts, edit information about contacts, and remove contacts.

Each contact is a valid node identity that's been registered in [kimap](../getting_started/kimap.md).
Each contact has a map of fields which are labeled by a string key and contain a JSON value.

Here is the full [WIT API](../system/process/wit_apis.md) for `contacts:contacts:sys`:
```wit
interface contacts {
    enum capabilities {
        read-name-only,
        read,
        add,
        remove,
    }

    variant request {
        get-names,                                // requires read-names-only
        get-all-contacts,                         // requires read
        get-contact(string),                      // requires read
        add-contact(string),                      // requires add
        // tuple<node, field, value>
        add-field(tuple<string, string, string>), // requires add
        remove-contact(string),                   // requires remove
        // tuple<node, field>
        remove-field(tuple<string, string>),      // requires remove
    }

    variant response {
        get-names(list<string>),
        get-all-contacts, // JSON all-contacts dict in blob
        get-contact,      // JSON contact dict in blob
        add-contact,
        add-field,
        remove-contact,
        remove-field,
        error(string),    // any failed request will receive this response
    }
}

world contacts-sys-v0 {
    import contacts;
    include process-v0;
}
```

As described in the comments, each request requires a specific capability.
Acquiring these capabilities is as simple as including them along with the messaging capability for `contacts:contacts:sys` in the manifest for your package, like so:
```json
"request_capabilities": [
    "contacts:contacts:sys",
    {
        "process": "contacts:contacts:sys",
        "params": "ReadNameOnly"
    },
    {
        "process": "contacts:contacts:sys",
        "params": "Read"
    },
    {
        "process": "contacts:contacts:sys",
        "params": "Add"
    },
    {
        "process": "contacts:contacts:sys",
        "params": "Remove"
    }
],
```

Only request capabilities that your package actually needs.
Users may reject installing an app that requests add or remove that they would otherwise feel comfortable allowing to read from contacts.
`ReadNameOnly` is a good capability to request if the main purpose of using `contacts:contacts:sys` is simply to grab a list of node identities that the user might want to see content in your protocol from or play a game with.

_________

To use the contacts primitive to get a list of existing contacts, follow these steps:

1. Download or copy the WIT API file into your package `/api` folder

2. Generate your WIT bindings to include this API ([note that you can compose this with additional APIs if desired](../cookbook/package_apis.md))

```rust
wit_bindgen::generate!({
    path: "target/wit",
    world: "contacts-sys-v0",
    generate_unused_types: true,
    additional_derives: [PartialEq, serde::Deserialize, serde::Serialize],
});
```

3. Request the proper capability in your `manifest.json`

```json
"request_capabilities": [
    "contacts:contacts:sys",
    {
        "process": "contacts:contacts:sys",
        "params": "ReadNameOnly"
    }
],
```

4. In your process, create the capability and use it to make a request

```rust
use crate::kinode::process::contacts;
use kinode_process_lib::{kiprintln, Address, Capability, Request};

let contacts_process = Address::from((our.node(), "contacts", "contacts", "sys"));

let read_names_cap = Capability::new(
    &contacts_process,
    serde_json::to_string(&contacts::Capabilities::ReadNameOnly).unwrap(),
);

let response = Request::to(&contacts_process)
    .body(serde_json::to_vec(&contacts::Request::GetNames).unwrap())
    .capabilities(vec![read_names_cap])
    .send_and_await_response(5)
    .unwrap()
    .unwrap();

// the response will be returned as a list of node identities, represented as strings
if let Ok(contacts::Response::GetNames(names)) = serde_json::from_slice(&response.body()) {
    kiprintln!("contacts: {:?}", names);
}
```
