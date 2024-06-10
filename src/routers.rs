use actix_web::{get,Responder, HttpRequest};

use owo_colors::OwoColorize;

#[get("/ip")]
pub async fn get_ip(req: HttpRequest) ->impl Responder {
    let peer_addr = req.peer_addr().expect("Cant Get Peer Addr");
    println!("[Get Ip] {}",peer_addr.green());
    return format!("{}\n",peer_addr.ip());
}
