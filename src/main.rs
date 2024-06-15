use clap::Parser;
use rddns::{Config, Server};
#[tokio::main]
async fn main() {
    let server: Server = Server::new(Config::parse());
    server.run().await;
}
