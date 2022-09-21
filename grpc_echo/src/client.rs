use echo::EchoResponse;
use tokio::task;
use tonic::transport::Channel;

use crate::echo::echo_client::EchoClient;

pub mod echo {
    tonic::include_proto!("echo");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EchoClient::connect("http://[::1]:50080").await?;

    let request = tonic::Request::new(echo::EchoRequest {
        input: "anybody here?".to_string(),
        echo_delay_millis: 0,
    });

    let response = client.hello(request).await?;
    println!("response for a single request\n{:?}", response);
    println!("-------------");

    let responses = send_parallel_requests(&client, 10, 1000).await?;
    println!("responses for multiple parallel requests\n{:?}", responses);
    Ok(())
}

type EchoResult = Result<tonic::Response<EchoResponse>, tonic::Status>;
async fn send_parallel_requests(
    client: &EchoClient<Channel>,
    requests_number: usize,
    echo_delay_millis: u64,
) -> Result<Vec<EchoResult>, Box<dyn std::error::Error>> {
    let mut handles = Vec::with_capacity(requests_number);
    for i in 0..requests_number {
        let message = format!("request #{}", i);
        let request = tonic::Request::new(echo::EchoRequest {
            input: message,
            echo_delay_millis: echo_delay_millis,
        });

        let mut client_clone = client.clone();
        let handle = task::spawn(async move { client_clone.hello(request).await });
        handles.push(handle);
    }

    let mut results = Vec::with_capacity(requests_number);
    for handle in handles {
        results.push(handle.await?);
    }

    Ok(results)
}
