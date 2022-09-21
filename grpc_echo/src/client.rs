use echo::EchoResponse;
use futures::executor::block_on;
use tokio::task;
use tonic::transport::Channel;

use crate::echo::echo_client::EchoClient;

pub mod echo {
    tonic::include_proto!("echo");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EchoClient::connect("http://[::1]:50080").await?;

    let request = tonic::Request::new(
        echo::EchoRequest {
            input:"anybody here?".to_string(), 
            echo_delay_millis: 0
        });

    let response = client.hello(request).await?;
    println!("RESPONSE={:?}", response);

    //let _local = task::LocalSet::new();
    let responses = block_on(
        send_parallel_requests(&client, 10, 1000));
    println!("RESPONSES={:?}", responses);
    Ok(())
}

async fn send_parallel_requests(client: &EchoClient<Channel>, requests_number: usize, echo_delay_millis: u64)
     // -> Vec<Result<EchoResponse, tonic::Status /*Box<dyn std::error::Error>*/>> 
     -> Vec<Result<tonic::Response<EchoResponse>, tonic::Status>>
{
    let mut handles = vec![]; // with_capacity()
    for i in 0..requests_number {
        let message = format!("request #{}", i);
        let request = tonic::Request::new(
            echo::EchoRequest {
                input: message,
                echo_delay_millis: echo_delay_millis,
            }
        );
    
        //let r = client.clone().hello(request).await;
        let mut client_clone = client.clone();
        let handle = task::spawn(
            async move { client_clone.hello(request).await });
        handles.push(handle);
    }

    //let _responses = futures::future::join_all(handles).await;
    
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.unwrap());
    }

    results
}