//use std::io::{self, BufReader};
// use std::fs::File;
use super::server::State;
use actix_web::web;
use owo_colors::OwoColorize;
use tokio::fs::File;
use tokio::io;
use tokio::io::AsyncReadExt;

pub async fn read_from_nsd(data: web::Data<State>, domain: &str, address: &str) -> io::Result<()> {
    let mut file = File::open((**data).config().path()).await?;
    let mut content = String::new();
    file.read_to_string(&mut content).await?;
    let mut is_zone_region = false;
    for i in content.lines() {
        if !is_zone_region {
            if i.contains(")") {
                is_zone_region = true;
            } else {
                continue;
            }
        } else {
            let mut i = i.split_ascii_whitespace();
            if i.nth(0).expect("无法正常读取nsd").contains(domain) {
                println!("[{}]\t{domain}", "匹配到域名".purple());
                println!("原地址是{}", i.nth(3).expect(""));
                println!("欲替换地址是{}", address.yellow());
            }
        }
    }
    //    println!("{}",content);
    return Ok(());
}
