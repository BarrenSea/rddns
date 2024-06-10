use crate::Config;
use actix_web::{App,HttpServer};
use crate::routers;
use owo_colors::OwoColorize;
#[derive(Debug)]
pub struct Server{
    config :Config,
}

impl Server {
    pub fn new(config: Config) ->Self {
	return Server{config};
    }
    pub async fn run(&self) -> () {
	HttpServer::new(|| {
	    App::new().service(routers::get_ip)
	})
	    .bind(&self.config.address()).expect(&format!("{}","Unable To Bind Server !".red())).workers(self.config.workers()).run().await.expect(&format!("{}","Unable To Run the Server !".red()));

    }
}
