use tokio::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    let port = listener.local_addr().unwrap();

    println!("Listening at port:{}", port);
    run(listener).await?;
    tokio::signal::ctrl_c().await?;
    Ok(())
}
