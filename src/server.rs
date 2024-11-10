// mod blocks;
// mod hashable;
// mod library;

use payments::greet_server::{Greet, GreetServer};
use payments::{HelloResponse, NoParam};

use tonic::{transport::Server, Request, Response, Status};

// use payments::greet_client::GreetClient; // Adjust the import based on your generated Rust gRPC code
// use tonic::transport::Channel;

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct GreetService {}

#[tonic::async_trait]
impl Greet for GreetService {
    async fn say_hello(
        &self,
        request: Request<NoParam>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = HelloResponse {
            message: format!("Sent {}", req.message).into(), // Include the request message
        };

        Ok(Response::new(reply))
    }
}

// #[tokio::main]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50052".parse().unwrap();
    println!("Server listening on: http://{}", addr);

    let btc_service = GreetService::default();
    let server = Server::builder()
        .add_service(GreetServer::new(btc_service))
        .serve(addr);

    server.await?;

    Ok(())
}
