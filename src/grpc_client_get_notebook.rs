use tonic::Request;
use crate::config::Config;
use crate::grpc::proto;

mod grpc;
mod repository;
mod config;
mod schema;
mod model;


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