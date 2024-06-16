use super::nsd::*;
use super::server::State;
use actix_web::{
    get, post,
    web::{self, Data},
    HttpRequest, Responder,
};
use owo_colors::OwoColorize;

#[get("/ip")]
pub async fn get_ip(req: HttpRequest) -> impl Responder {
    let peer_addr = req.peer_addr().expect("Cant Get Peer Addr");
    println!("[Get Ip] {}", peer_addr.yellow());
    return format!("{}\n", peer_addr.ip());
}

#[post("/ddns/{domain}/{address}")]
pub async fn ddns(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    data: Data<State>,
) -> impl Responder {
    let peer_addr = req
        .peer_addr()
        .expect("Unable To Get The Address Of Peer Address");
    println!(
        "[DDNS Request] {} Request {} for {}\n",
        format!("{}", peer_addr.purple()),
        format!("{}", path.0).blue(),
        format!("{}", path.1).green()
    );
    read_from_nsd(data, &path.0, &path.1)
        .await
        .expect("nsd处理失败");
    return format!(
        "{} Request {} for {}\n",
        format!("{}", peer_addr.ip()).yellow(),
        format!("{}", path.0).blue(),
        format!("{}", path.1).green()
    );
}
