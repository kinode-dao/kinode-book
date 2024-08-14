/// Simple example of using the WebSockets client.
/// Usage:
/// ```
/// # Start node.
/// kit f
///
/// # Start WS server from a new terminal.
/// ./ws_client/ws_server.py
///
/// # Start package from a new terminal.
/// kit bs ws_client
/// ```
use anyhow::{anyhow, Result};

use kinode_process_lib::{
    await_message, call_init, get_blob, http, println, Address, LazyLoadBlob, Message,
};
#[cfg(feature = "test")]
use kinode_process_lib::{OnExit, Request};

wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v0",
});

const WS_URL: &str = "ws://localhost:8765";
const CONNECTION: u32 = 0;

fn handle_http_message(message: &Message, connection: &u32) -> Result<()> {
    match serde_json::from_slice::<http::HttpClientRequest>(message.body())? {
        http::HttpClientRequest::WebSocketClose { channel_id } => {
            assert_eq!(*connection, channel_id);
        }
        http::HttpClientRequest::WebSocketPush {
            channel_id,
            message_type,
        } => {
            assert_eq!(*connection, channel_id);
            if message_type == http::WsMessageType::Close {
                println!("got Close push");
                return Ok(());
            }

            assert_eq!(message_type, http::WsMessageType::Text);

            let Some(blob) = get_blob() else {
                return Err(anyhow!("got WebSocketPush with no blob"));
            };
            println!("Received from server: {:?}", String::from_utf8(blob.bytes));

            http::send_ws_client_push(
                connection.clone(),
                http::WsMessageType::Text,
                LazyLoadBlob {
                    mime: Some("application/json".to_string()),
                    bytes: serde_json::to_vec("Hello from client").unwrap(),
                },
            );
        }
    }
    Ok(())
}

fn talk_to_ws() -> Result<()> {
    let connection = CONNECTION;
    http::open_ws_connection(WS_URL.to_string(), None, connection)?;

    match await_message() {
        Ok(message) => {
            if message.source().process == "http_client:distro:sys" {
                if let Err(e) = handle_http_message(&message, &connection) {
                    println!("{e}");
                }
            }
        }
        Err(_send_error) => println!("got send error!"),
    }
    Ok(())
}

#[cfg(feature = "test")]
fn talk_to_ws_test() -> Result<()> {
    println!("in test");
    let message = await_message()?;
    let parent_address = message.source();
    println!("got parent {parent_address:?}");

    match talk_to_ws() {
        Ok(_) => {}
        Err(e) => println!("error talking to ws: {e}"),
    }

    Request::to(parent_address)
        .body(serde_json::to_vec(&Ok::<(), ()>(())).unwrap())
        .send()
        .unwrap();
    OnExit::None.set().unwrap();
    println!("done");

    Ok(())
}

call_init!(init);
fn init(our: Address) {
    println!("{}: begin", our.process());

    #[cfg(not(feature = "test"))]
    match talk_to_ws() {
        Ok(_) => {}
        Err(e) => println!("error talking to ws: {e}"),
    }

    #[cfg(feature = "test")]
    match talk_to_ws_test() {
        Ok(_) => {}
        Err(e) => println!("error talking to ws: {e}"),
    }
}
