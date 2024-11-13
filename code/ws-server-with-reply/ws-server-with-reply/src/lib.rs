/// Simple example of running a WebSockets server, specifying reply type as Response.
/// Usage:
/// ```
/// # Start node.
/// kit f
///
/// # Start package from a new terminal.
/// kit bs ws-server-with-reply
///
/// # Connect from WS client script.
/// ./ws-server/ws-client.py
/// ```
use anyhow::{anyhow, Result};

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
    match serde_json::from_slice::<http::server::HttpServerRequest>(message.body())? {
        http::server::HttpServerRequest::Http(_) => {
            return Err(anyhow!("unexpected HTTP request"));
        }
        http::server::HttpServerRequest::WebSocketOpen { path, channel_id } => {
            assert_eq!(path, WS_PATH);
            assert_eq!(*connection, None);

            *connection = Some(channel_id.clone());

            Request::to("our@http-server:distro:sys".parse::<Address>()?)
                .body(serde_json::to_vec(
                    &http::server::HttpServerAction::WebSocketExtPushOutgoing {
                        channel_id,
                        message_type: http::server::WsMessageType::Binary,
                        desired_reply_type: http::server::MessageType::Response,
                    },
                )?)
                .expects_response(15)
                .blob(LazyLoadBlob {
                    mime: Some("application/json".to_string()),
                    bytes: rmp_serde::to_vec_named("ack client connection").unwrap(),
                })
                .send()?;
        }
        http::server::HttpServerRequest::WebSocketClose(channel_id) => {
            assert_eq!(*connection, Some(channel_id));

            *connection = None;
        }
        http::server::HttpServerRequest::WebSocketPush {
            channel_id,
            message_type,
        } => {
            assert_eq!(*connection, Some(channel_id));
            if message_type == http::server::WsMessageType::Close {
                println!("got Close push");
                return Ok(());
            }

            assert_eq!(message_type, http::server::WsMessageType::Binary);

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
    let mut server = http::server::HttpServer::new(5);
    server
        .bind_ws_path(
            WS_PATH,
            http::server::WsBindingConfig::new(false, false, false, false),
        )
        .unwrap();

    loop {
        match await_message() {
            Ok(message) => {
                if message.source().process == "http-server:distro:sys" {
                    if let Err(e) = handle_http_message(&our, &message, &mut connection) {
                        println!("{e}");
                    }
                }
            }
            Err(_send_error) => println!("got send error!"),
        }
    }
}
