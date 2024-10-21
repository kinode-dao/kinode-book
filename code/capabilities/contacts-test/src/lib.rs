use crate::kinode::process::contacts;
use kinode_process_lib::{call_init, kiprintln, Address, Capability, Request};

wit_bindgen::generate!({
    path: "target/wit",
    world: "contacts-doria-dot-kino-v0",
    generate_unused_types: true,
    additional_derives: [PartialEq, serde::Deserialize, serde::Serialize],
});

call_init!(init);
fn init(our: Address) {
    kiprintln!("init");

    // ANCHOR: use_capabilities

    let contacts_process =
        Address::from((our.node(), "contacts", "capabilities-test", "doria.kino"));

    // All of these capabilities were requested in the manifest,
    // so we can create them here and attach them to our requests.
    // If they were not in the manifest or otherwise acquired,
    // we could still create the objects, but they would not be
    // attached to our requests and therefore the requests would fail.

    let read_names_cap = Capability::new(
        &contacts_process,
        serde_json::to_string(&contacts::Capabilities::ReadNameOnly).unwrap(),
    );

    let read_cap = Capability::new(
        &contacts_process,
        serde_json::to_string(&contacts::Capabilities::Read).unwrap(),
    );

    let add_cap = Capability::new(
        &contacts_process,
        serde_json::to_string(&contacts::Capabilities::Add).unwrap(),
    );

    let remove_cap = Capability::new(
        &contacts_process,
        serde_json::to_string(&contacts::Capabilities::Remove).unwrap(),
    );

    kiprintln!("requesting all names from contacts");

    let response = Request::to(&contacts_process)
        .body(serde_json::to_vec(&contacts::Request::GetNames).unwrap())
        .capabilities(vec![read_names_cap])
        .send_and_await_response(5)
        .unwrap()
        .unwrap();

    kiprintln!(
        "response: {:?}",
        serde_json::from_slice::<contacts::Response>(&response.body()).unwrap()
    );

    kiprintln!("requesting all names from contacts (without capability attached!)");

    let response = Request::to(&contacts_process)
        .body(serde_json::to_vec(&contacts::Request::GetNames).unwrap())
        // no cap
        .send_and_await_response(5)
        .unwrap()
        .unwrap();

    kiprintln!(
        "response: {:?}",
        serde_json::from_slice::<contacts::Response>(&response.body()).unwrap()
    );

    kiprintln!("adding contact to contacts");

    let response = Request::to(&contacts_process)
        .body(
            serde_json::to_vec(&contacts::Request::AddContact(
                "mothu-et-doria.os".to_string(),
            ))
            .unwrap(),
        )
        .capabilities(vec![add_cap])
        .send_and_await_response(5)
        .unwrap()
        .unwrap();

    kiprintln!(
        "response: {:?}",
        serde_json::from_slice::<contacts::Response>(&response.body()).unwrap()
    );

    kiprintln!("reading all contacts from contacts");

    let response = Request::to(&contacts_process)
        .body(serde_json::to_vec(&contacts::Request::GetAllContacts).unwrap())
        .capabilities(vec![read_cap])
        .send_and_await_response(5)
        .unwrap()
        .unwrap();

    kiprintln!(
        "response: {:?}",
        serde_json::from_slice::<contacts::Response>(&response.body()).unwrap()
    );

    kiprintln!("removing contact from contacts");

    let response = Request::to(&contacts_process)
        .body(
            serde_json::to_vec(&contacts::Request::RemoveContact(
                "mothu-et-doria.os".to_string(),
            ))
            .unwrap(),
        )
        .capabilities(vec![remove_cap])
        .send_and_await_response(5)
        .unwrap()
        .unwrap();

    kiprintln!(
        "response: {:?}",
        serde_json::from_slice::<contacts::Response>(&response.body()).unwrap()
    );

    // ANCHOR_END: use_capabilities
}
