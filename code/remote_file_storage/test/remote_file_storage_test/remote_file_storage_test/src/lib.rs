use crate::kinode::process::server::{get_file, list_files, put_file};
use crate::kinode::process::tester::{
    FailResponse, Request as TesterRequest, Response as TesterResponse, RunRequest,
};
use kinode_process_lib::{
    await_message, call_init, get_blob, println, vfs::File, Address, Response,
};

mod tester_lib;

wit_bindgen::generate!({
    path: "target/wit",
    world: "remote-file-storage-test-template-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn run_test(our: &Address) -> anyhow::Result<()> {
    let message = await_message()?;

    if !message.is_request() {
        fail!("mfa_data_demo_test");
    }
    let source = message.source();
    if our.node != source.node {
        return Err(anyhow::anyhow!(
            "rejecting foreign Message from {:?}",
            source,
        ));
    }
    let TesterRequest::Run(RunRequest {
        input_node_names: node_names,
        ..
    }) = message.body().try_into()?;
    if node_names.len() != 2 {
        fail!("mfa_data_demo_test");
    }

    let file_path = "tester:sys/pkg/manifest.json";
    let worker = &node_names[1];

    // put file onto worker
    if put_file(worker, file_path, "manifest.json").is_err() {
        fail!("file_storage_test");
    }

    // check file is on worker
    let Ok(files) = list_files(worker) else {
        fail!("file_storage_test");
    };
    if files != vec!["manifest.json"] {
        fail!("file_storage_test");
    }

    // read out the file on worker from master
    if get_file(worker, "manifest.json").is_err() {
        fail!("file_storage_test");
    }
    let Some(contents) = get_blob() else {
        fail!("file_storage_test");
    };

    // compare result to original file contents
    let file = File {
        path: file_path.to_string(),
        timeout: 5,
    };
    let expected_contents = file.read()?;

    if contents.bytes() != expected_contents {
        fail!("file_storage_test");
    }

    Response::new().body(TesterResponse::Run(Ok(()))).send()?;
    Ok(())
}

call_init!(init);
fn init(our: Address) {
    println!("begin");

    match run_test(&our) {
        Ok(()) => {}
        Err(e) => {
            println!("mfa_data_demo_test: error: {e:?}");
            fail!("mfa_data_demo_test");
        }
    };
}
