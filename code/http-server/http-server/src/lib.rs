/// Simple example of running an HTTP server.
/// Usage:
/// ```
/// # Start node.
/// kit f
///
/// # Start package from a new terminal.
/// kit bs http-server
///
/// # Send an HTTP request.
/// curl -X PUT -d '{"Hello": "greetings"}' http://localhost:8080/http-server:http-server:template.os
/// ```
use anyhow::{anyhow, Result};

use kinode_process_lib::{await_message, call_init, get_blob, http, println, Address, Message};

wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v1",
});

/// Handle a message from the HTTP server.
fn handle_http_message(message: &Message) -> Result<()> {
    let Ok(server_request) = http::server::HttpServerRequest::from_bytes(message.body()) else {
        return Err(anyhow!("received a message with weird `body`!"));
    };
    let Some(http_request) = server_request.request() else {
        return Err(anyhow!("received a WebSocket message, skipping"));
    };
    if http_request.method().unwrap() != http::Method::PUT {
        return Err(anyhow!("received a non-PUT HTTP request, skipping"));
    }
    let Some(body) = get_blob() else {
        return Err(anyhow!(
            "received a PUT HTTP request with no body, skipping"
        ));
    };
    http::server::send_response(http::StatusCode::OK, None, vec![]);
    println!(
        "{:?}",
        serde_json::from_slice::<serde_json::Value>(&body.bytes)
    );
    Ok(())
}

call_init!(init);
fn init(_our: Address) {
    println!("begin");

    let mut server = http::server::HttpServer::new(5);
    server
        .bind_http_path(
            "/",
            http::server::HttpBindingConfig::new(false, false, false, None),
        )
        .unwrap();

    loop {
        match await_message() {
            Ok(message) => {
                if message.source().process == "http-server:distro:sys" {
                    if let Err(e) = handle_http_message(&message) {
                        println!("{e}");
                    }
                }
            }
            Err(_send_error) => println!("got send error!"),
        }
    }
}
