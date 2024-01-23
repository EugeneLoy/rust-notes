use tonic::Request;

use rust_notes::config::Config;
use rust_notes::grpc::proto;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::build();

    let mut client = proto::notes_service_client::NotesServiceClient::connect(
        format!("http://[::1]:{}", config.grpc_port)
    ).await?;

    let request = Request::new(proto::GetNotebook {
        notebook_id: 1.into()
    });

    let response = client.get_notebook(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}