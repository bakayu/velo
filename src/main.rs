use std::net::TcpListener;

use velo::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to address");
    run(listener)?.await
}
