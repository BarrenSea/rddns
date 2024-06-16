use crate::routers;
use crate::Config;
use actix_web::{web, App, HttpServer};
use owo_colors::OwoColorize;
use std::sync::Arc;
#[derive(Debug)]
pub struct Server {
    config: Arc<Config>,
}

impl Server {
    pub fn new(config: Arc<Config>) -> Self {
        return Server { config };
    }
    pub async fn run(&self) -> () {
        let state = web::Data::new(State::new(self.config.clone()));
        HttpServer::new(move || {
            App::new()
                .app_data(state.clone())
                .service(routers::get_ip)
                .service(routers::ddns)
        })
        .bind(self.config.address())
        .expect(&format!("{}", "Unable To Bind Server !".red()))
        .workers(self.config.workers())
        .run()
        .await
        .expect(&format!("{}", "Unable To Run the Server !".red()));
    }
}

pub struct State {
    config: Arc<Config>,
}

impl State {
    pub fn new(config: Arc<Config>) -> State {
        return State { config };
    }
    pub fn config(&self) -> Arc<Config> {
        return self.config.clone();
    }
}
