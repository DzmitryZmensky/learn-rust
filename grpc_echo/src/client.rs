use crate::echo::echo_client::EchoClient;

pub mod echo {
    tonic::include_proto!("echo");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EchoClient::connect("http://[::1]:50080").await?;

    let request = tonic::Request::new(
        echo::EchoRequest {
            input: "anybody here?".to_string()
        }
    );

    let response = client.hello(request).await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}