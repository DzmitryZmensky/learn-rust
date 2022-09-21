use std::time::Duration;

use echo::{EchoRequest, EchoResponse, echo_server};
use tokio::time::sleep;
use tonic::{transport::Server, Request, Response, Status};

pub mod echo {
    tonic::include_proto!("echo");
}

#[derive(Debug, Default)]
pub struct EchoService {}

#[tonic::async_trait]
impl echo_server::Echo for EchoService {
    async fn hello(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        println!("Got a request: {:?}", request);
        let req = request.into_inner();
        sleep(Duration::from_millis(req.echo_delay_millis)).await;
        let reply = EchoResponse { 
            output: format!("echo: {}", req.input) 
        };
        
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50080".parse()?;
    let echo_service = EchoService::default();

    Server::builder()
        .add_service(echo_server::EchoServer::new(echo_service))
        .serve(addr)
        .await?;

    Ok(())
}