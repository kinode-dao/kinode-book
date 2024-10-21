use crate::kinode::process::contacts;
use kinode_process_lib::{
    await_message, call_init, get_typed_state, kiprintln, set_state, Address, Capability,
    LazyLoadBlob, Message, NodeId, Response,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Contact(HashMap<String, serde_json::Value>);

#[derive(Debug, Serialize, Deserialize)]
struct Contacts(HashMap<NodeId, Contact>);

#[derive(Debug, Serialize, Deserialize)]
struct ContactsState {
    our: Address,
    contacts: Contacts,
}

impl ContactsState {
    fn new(our: Address) -> Self {
        get_typed_state(|bytes| serde_json::from_slice(bytes)).unwrap_or(Self {
            our,
            contacts: Contacts(HashMap::new()),
        })
    }

    fn save(&self) {
        set_state(&serde_json::to_vec(&self).expect("Failed to serialize contacts state!"));
    }

    fn contacts(&self) -> &Contacts {
        &self.contacts
    }

    fn get_contact(&self, node: NodeId) -> Option<&Contact> {
        self.contacts.0.get(&node)
    }

    fn add_contact(&mut self, node: NodeId) {
        self.contacts.0.insert(node, Contact(HashMap::new()));
        self.save();
    }

    fn remove_contact(&mut self, node: NodeId) {
        self.contacts.0.remove(&node);
        self.save();
    }

    fn add_field(&mut self, node: NodeId, field: String, value: serde_json::Value) {
        self.contacts
            .0
            .entry(node)
            .or_insert_with(|| Contact(HashMap::new()))
            .0
            .insert(field, value);
        self.save();
    }

    fn remove_field(&mut self, node: NodeId, field: String) {
        if let Some(contact) = self.contacts.0.get_mut(&node) {
            contact.0.remove(&field);
        }
        self.save();
    }
}

wit_bindgen::generate!({
    path: "target/wit",
    world: "contacts-doria-dot-kino-v0",
    generate_unused_types: true,
    additional_derives: [PartialEq, serde::Deserialize, serde::Serialize],
});

call_init!(initialize);
fn initialize(our: Address) {
    kiprintln!("started");

    let mut state: ContactsState = ContactsState::new(our);

    main_loop(&mut state);
}

fn main_loop(state: &mut ContactsState) {
    loop {
        match await_message() {
            Err(_send_error) => {
                // ignore send errors, local-only process
                continue;
            }
            Ok(Message::Request {
                source,
                body,
                capabilities,
                ..
            }) => {
                // ignore messages from other nodes -- technically superfluous check
                // since manifest does not acquire networking capability
                if source.node() != state.our.node {
                    continue;
                }
                handle_request(&body, capabilities, state);
            }
            _ => continue, // ignore responses
        }
    }
}

fn handle_request(body: &[u8], capabilities: Vec<Capability>, state: &mut ContactsState) {
    let (response, blob) = handle_contacts_request(state, body, Some(capabilities));
    let mut response = Response::new().body(serde_json::to_vec(&response).unwrap());
    if let Some(blob) = blob {
        response = response.blob(blob);
    }
    response.send().unwrap();
}

fn handle_contacts_request(
    state: &mut ContactsState,
    request_bytes: &[u8],
    capabilities: Option<Vec<Capability>>,
) -> (contacts::Response, Option<LazyLoadBlob>) {
    let Ok(request) = serde_json::from_slice::<contacts::Request>(request_bytes) else {
        return (
            contacts::Response::Error("Malformed request".to_string()),
            None,
        );
    };
    // ANCHOR: check_capabilities
    // each request requires one of read-name-only, read, add, or remove
    if let Some(capabilities) = capabilities {
        let required_capability = Capability::new(
            &state.our,
            serde_json::to_string(&match request {
                contacts::Request::GetNames => contacts::Capabilities::ReadNameOnly,
                contacts::Request::GetAllContacts | contacts::Request::GetContact(_) => {
                    contacts::Capabilities::Read
                }
                contacts::Request::AddContact(_) | contacts::Request::AddField(_) => {
                    contacts::Capabilities::Add
                }
                contacts::Request::RemoveContact(_) | contacts::Request::RemoveField(_) => {
                    contacts::Capabilities::Remove
                }
            })
            .unwrap(),
        );
        if !capabilities.contains(&required_capability) {
            return (
                contacts::Response::Error("Missing capability".to_string()),
                None,
            );
        }
    }
    // ANCHOR_END: check_capabilities

    match request {
        contacts::Request::GetNames => (
            contacts::Response::GetNames(
                state
                    .contacts()
                    .0
                    .keys()
                    .map(|node| node.to_string())
                    .collect(),
            ),
            None,
        ),
        contacts::Request::GetAllContacts => (
            contacts::Response::GetAllContacts,
            Some(LazyLoadBlob::new(
                Some("application/json"),
                serde_json::to_vec(state.contacts()).unwrap(),
            )),
        ),
        contacts::Request::GetContact(node) => (
            contacts::Response::GetContact,
            Some(LazyLoadBlob::new(
                Some("application/json"),
                serde_json::to_vec(&state.get_contact(node)).unwrap(),
            )),
        ),
        contacts::Request::AddContact(node) => {
            state.add_contact(node);
            (contacts::Response::AddContact, None)
        }
        contacts::Request::AddField((node, field, value)) => {
            let Ok(value) = serde_json::from_str::<serde_json::Value>(&value) else {
                return (
                    contacts::Response::Error("Malformed value".to_string()),
                    None,
                );
            };
            state.add_field(node, field, value);
            (contacts::Response::AddField, None)
        }
        contacts::Request::RemoveContact(node) => {
            state.remove_contact(node);
            (contacts::Response::RemoveContact, None)
        }
        contacts::Request::RemoveField((node, field)) => {
            state.remove_field(node, field);
            (contacts::Response::RemoveField, None)
        }
    }
}
