use tokio::net::TcpListener;
    
#[tokio::test]
async fn health_check_works() {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("Failed to bind the random port"); 
    let address = format!("http://{}", listener.local_addr().unwrap());
    tokio::spawn( async move { zero2prod::run(listener).await.expect("Failed to spawn our app");
    });

    //Creating new clientRequest
    let client = reqwest::Client::new();

    // Response
    let res = client.get(&format!("{}/check_health", address)).send().await.expect("Failed to execute request.");
    // Assert
    assert!(res.status().is_success());
    assert_eq!(res.content_length(), Some(0));
}

