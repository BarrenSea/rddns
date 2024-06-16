use clap::Parser;
use rddns::{Config, Server};
use std::sync::Arc;
#[tokio::main]
async fn main() {
    let config = Arc::new(Config::parse());
    let server: Server = Server::new(config);
    println!("{:?}", server);
    server.run().await;
}
