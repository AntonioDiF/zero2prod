use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port = "8000";
    let address = format!("127.0.0.1:{}", &port);
    let listener = TcpListener::bind(&address).expect("Failed to bind to random port");
    run(listener)?.await
}
