use crate::routers;
use crate::Config;
use actix_web::{App, HttpServer};
use owo_colors::OwoColorize;
#[derive(Debug)]
pub struct Server {
    config: Config,
}

impl Server {
    pub fn new(config: Config) -> Self {
        return Server { config };
    }
    pub async fn run(&self) -> () {
        println!(
            "Server Running On {} with {} workers use {} as nsd zone file",
            format!("{}", self.config.address().yellow()),
            format!("{}", self.config.workers().blue()),
            format!("{}", self.config.path().purple())
        );
        HttpServer::new(|| App::new().service(routers::get_ip).service(routers::ddns))
            .bind(&self.config.address())
            .expect(&format!("{}", "Unable To Bind Server !".red()))
            .workers(self.config.workers())
            .run()
            .await
            .expect(&format!("{}", "Unable To Run the Server !".red()));
    }
    pub fn config(&self) -> &Config {
        return &self.config;
    }
}
