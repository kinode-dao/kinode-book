/// Simple example of running a WebSockets server.
/// Usage:
/// ```
/// # Start node.
/// kit f
///
/// # Start package from a new terminal.
/// kit bs ws_server
///
/// # Connect from WS client script.
/// ./ws_server/ws_client.py
/// ```
use anyhow::{anyhow, Result};

use kinode_process_lib::{
    await_message, call_init, get_blob, http, println, Address, LazyLoadBlob, Message,
};

wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v0",
});

const WS_PATH: &str = "/";

fn handle_http_message(
    our: &Address,
    message: &Message,
    connection: &mut Option<u32>,
) -> Result<()> {
    match serde_json::from_slice::<http::HttpServerRequest>(message.body())? {
        http::HttpServerRequest::Http(_) => {
            return Err(anyhow!("unexpected HTTP request"));
        }
        http::HttpServerRequest::WebSocketOpen { path, channel_id } => {
            assert_eq!(path, our.process.to_string());
            assert_eq!(*connection, None);

            *connection = Some(channel_id);

            http::send_ws_push(
                channel_id,
                http::WsMessageType::Text,
                LazyLoadBlob {
                    mime: Some("application/json".to_string()),
                    bytes: serde_json::to_vec("ack client connection").unwrap(),
                },
            );
        }
        http::HttpServerRequest::WebSocketClose(channel_id) => {
            assert_eq!(*connection, Some(channel_id));

            *connection = None;
        }
        http::HttpServerRequest::WebSocketPush {
            channel_id,
            message_type,
        } => {
            assert_eq!(*connection, Some(channel_id));
            if message_type == http::WsMessageType::Close {
                println!("got Close push");
                return Ok(());
            }

            assert_eq!(message_type, http::WsMessageType::Text);

            let Some(blob) = get_blob() else {
                return Err(anyhow!("got WebSocketPush with no blob"));
            };
            println!("got Text from WS: {:?}", String::from_utf8(blob.bytes));
        }
    }
    Ok(())
}

call_init!(init);
fn init(our: Address) {
    println!("begin");

    let mut connection: Option<u32> = None;
    http::bind_ws_path(WS_PATH, false, false).unwrap();

    loop {
        match await_message() {
            Ok(message) => {
                if message.source().process == "http_server:distro:sys" {
                    if let Err(e) = handle_http_message(&our, &message, &mut connection) {
                        println!("{e}");
                    }
                }
            }
            Err(_send_error) => println!("got send error!"),
        }
    }
}
