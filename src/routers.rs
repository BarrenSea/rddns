use actix_web::{get, post, HttpRequest, Responder};

use owo_colors::OwoColorize;

#[get("/ip")]
pub async fn get_ip(req: HttpRequest) -> impl Responder {
    let peer_addr = req.peer_addr().expect("Cant Get Peer Addr");
    println!("[Get Ip] {}", peer_addr.yellow());
    return format!("{}\n", peer_addr.ip());
}

#[post("/ddns/{domain}/{address}")]
pub async fn ddns(req: HttpRequest, domain: String, address: String) -> impl Responder {
    return format!("{}\n", req.peer_addr().unwrap().ip());
}
