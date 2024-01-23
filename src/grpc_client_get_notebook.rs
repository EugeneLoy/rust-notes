use std::env::args;
use tonic::Request;

use rust_notes::config::Config;
use rust_notes::grpc::proto;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = args().collect();
    let notebook_id: i32;
    if let [_, arg_1] = args.as_slice() {
        notebook_id = arg_1.parse().expect("Could not parse notebook id");
    } else {
        panic!("\
Invalid arguments. Syntax:
grpc_client_get_notebook <notebook_id>");
    }

    let config = Config::build();

    let mut client = proto::notes_service_client::NotesServiceClient::connect(
        format!("http://localhost:{}", config.grpc_port)
    ).await?;

    let request = Request::new(proto::GetNotebook {
        notebook_id: notebook_id
    });

    let response = client.get_notebook(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}