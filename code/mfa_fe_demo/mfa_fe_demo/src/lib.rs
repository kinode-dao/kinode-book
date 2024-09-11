use anyhow::{anyhow, Result};

use crate::kinode::process::mfa_data_demo::{Request as MfaRequest, Response as MfaResponse};
use kinode_process_lib::{
    await_message, call_init, get_blob, homepage, http, println, Address, Message, Request,
    Response,
};

wit_bindgen::generate!({
    path: "target/wit",
    world: "mfa-data-demo-template-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

// base64-encoded bytes prepended with image type like `data:image/png;base64,`, e.g.
// echo "data:image/png;base64,$(base64 < gosling.png)" | tr -d '\n' > icon
const ICON: &str = include_str!("./icon");

// you can embed an external URL
// const WIDGET: &str = "<iframe src='https://example.com'></iframe>";
// or you can embed your own HTML
const WIDGET: &str = "<html><body><h1>Hello, Kinode!</h1></body></html>";

#[derive(Debug, serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto)]
#[serde(untagged)] // untagged as a meta-type for all incoming responses
enum Req {
    MfaRequest(MfaRequest),
    HttpRequest(http::server::HttpServerRequest),
}

fn handle_mfa_request(request: &MfaRequest) -> Result<bool> {
    match request {
        MfaRequest::Hello(text) => {
            println!("got a Hello: {text}");
            Response::new()
                .body(MfaResponse::Hello("hello to you too!".to_string()))
                .send()?
        }
        MfaRequest::Goodbye => {
            println!("goodbye!");
            Response::new().body(MfaResponse::Goodbye).send()?;
            return Ok(true);
        }
    }
    Ok(false)
}

fn handle_http_request(our: &Address, request: http::server::HttpServerRequest) -> Result<()> {
    let Some(http_request) = request.request() else {
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
    Request::to(our).body(body.bytes).send().unwrap();
    Ok(())
}

fn handle_mfa_response(response: MfaResponse) -> Result<()> {
    match response {
        MfaResponse::Hello(text) => println!("got a Hello response: {text}"),
        MfaResponse::Goodbye => println!("got a Goodbye response"),
    }
    Ok(())
}

fn handle_message(our: &Address, message: &Message) -> Result<bool> {
    if message.is_request() {
        match message.body().try_into()? {
            Req::MfaRequest(ref mfa_request) => {
                return Ok(handle_mfa_request(mfa_request)?);
            }
            Req::HttpRequest(http_request) => {
                handle_http_request(our, http_request)?;
            }
        }
    } else {
        handle_mfa_response(message.body().try_into()?)?;
    }
    Ok(false)
}

call_init!(init);
fn init(our: Address) {
    println!("begin");

    let server_config = http::server::HttpBindingConfig::default().authenticated(false);
    let mut server = http::server::HttpServer::new(5);
    server.bind_http_path("/api", server_config).unwrap();
    server
        .serve_file(
            &our,
            "ui",
            vec!["/"],
            http::server::HttpBindingConfig::default(),
        )
        .unwrap();
    homepage::add_to_homepage("My First App", Some(ICON), Some("/"), Some(WIDGET));

    Request::to(&our)
        .body(MfaRequest::Hello("hello world".to_string()))
        .expects_response(5)
        .send()
        .unwrap();

    loop {
        match await_message() {
            Err(send_error) => println!("got SendError: {send_error}"),
            Ok(ref message) => match handle_message(&our, message) {
                Err(e) => println!("got error while handling message: {e:?}"),
                Ok(should_exit) => {
                    if should_exit {
                        return;
                    }
                }
            },
        }
    }
}
