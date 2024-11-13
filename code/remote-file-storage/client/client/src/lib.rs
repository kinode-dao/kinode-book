use clap::{Parser, Subcommand};

use crate::kinode::process::server::{get_file, list_files, put_file};
use kinode_process_lib::{await_next_message_body, call_init, get_blob, println, Address};

wit_bindgen::generate!({
    path: "target/wit",
    world: "client-template-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Take a file from local VFS and store on remote `host`.
    PutFile {
        host: String,
        #[arg(short, long)]
        path: String,
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Retrieve a file from remove `host`.
    GetFile {
        host: String,
        #[arg(short, long)]
        name: String,
    },
    /// List all files we have stored on remote `host`.
    ListFiles { host: String },
}

fn handle_put_file(host: &str, path: &str, name: &str) -> anyhow::Result<()> {
    match put_file(host, path, name) {
        Err(e) => Err(anyhow::anyhow!("{e}")),
        Ok(_) => {
            println!("Successfully PutFile {path} to host {host}.");
            Ok(())
        }
    }
}

fn handle_get_file(host: &str, name: &str) -> anyhow::Result<()> {
    match get_file(host, name) {
        Err(e) => Err(anyhow::anyhow!("{e}")),
        Ok(_) => {
            if let Some(blob) = get_blob() {
                if let Ok(contents) = String::from_utf8(blob.bytes().to_vec()) {
                    println!("Successfully GetFile {name} from host {host}:\n\n{contents}");
                    return Ok(());
                }
            }
            println!("Successfully GetFile {name} from host {host}.");
            Ok(())
        }
    }
}

fn handle_list_files(host: &str) -> anyhow::Result<()> {
    match list_files(host) {
        Err(e) => Err(anyhow::anyhow!("{e}")),
        Ok(paths) => {
            println!("{paths:#?}");
            Ok(())
        }
    }
}

fn execute() -> anyhow::Result<()> {
    let body = await_next_message_body()?;
    let body_string = format!("client {}", String::from_utf8(body)?);
    let args = body_string.split(' ');
    match Args::try_parse_from(args)?.command {
        Some(Command::PutFile {
            ref host,
            ref path,
            name,
        }) => handle_put_file(
            host,
            path,
            &name.unwrap_or_else(|| path.split('/').last().unwrap().to_string()),
        )?,
        Some(Command::GetFile { ref host, ref name }) => handle_get_file(host, name)?,
        Some(Command::ListFiles { ref host }) => handle_list_files(host)?,
        None => {}
    }
    Ok(())
}

call_init!(init);
fn init(_our: Address) {
    match execute() {
        Ok(_) => {}
        Err(e) => println!("error: {e:?}"),
    }
}
