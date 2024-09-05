/// Simple example of running a WebSockets server, specifying reply type as Response.
/// Usage:
/// ```
/// # Start node.
/// kit f
///
/// # Start package from a new terminal.
/// kit bs ws_server_with_reply
///
/// # Connect from WS client script.
/// ./ws_server/ws_client.py
/// ```
use anyhow::{anyhow, Result};

use kinode_process_lib::kernel_types::MessageType;
use kinode_process_lib::{
    await_message, call_init, get_blob, http, println, Address, LazyLoadBlob, Message, Request,
};

wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v0",
});

const WS_PATH: &str = "/";

fn handle_http_message(
    _our: &Address,
    message: &Message,
    connection: &mut Option<u32>,
) -> Result<()> {
    match serde_json::from_slice::<http::HttpServerRequest>(message.body())? {
        http::HttpServerRequest::Http(_) => {
            return Err(anyhow!("unexpected HTTP request"));
        }
        http::HttpServerRequest::WebSocketOpen { path, channel_id } => {
            assert_eq!(path, WS_PATH);
            assert_eq!(*connection, None);

            *connection = Some(channel_id.clone());

            Request::to("our@http_server:distro:sys".parse::<Address>()?)
                .body(serde_json::to_vec(
                    &http::HttpServerAction::WebSocketExtPushOutgoing {
                        channel_id,
                        message_type: http::WsMessageType::Binary,
                        desired_reply_type: MessageType::Response,
                    },
                )?)
                .expects_response(15)
                .blob(LazyLoadBlob {
                    mime: Some("application/json".to_string()),
                    bytes: rmp_serde::to_vec_named("ack client connection").unwrap(),
                })
                .send()?;
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

            assert_eq!(message_type, http::WsMessageType::Binary);

            let Some(blob) = get_blob() else {
                return Err(anyhow!("got WebSocketPush with no blob"));
            };
            println!(
                "got Text from WS: {:?}",
                rmp_serde::from_slice::<String>(&blob.bytes)
            );
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
