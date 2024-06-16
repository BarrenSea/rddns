//use tokio::fs::OpenOptions;
use super::server::State;
use actix_web::web;
use owo_colors::OwoColorize;
use std::fs::File;
use std::io::Read;
use std::io::{self};
//use tokio::fs::File;
//use tokio::io::{self, AsyncWriteExt};
//use tokio::io::AsyncReadExt;

pub async fn read_from_nsd(data: web::Data<State>, domain: &str, address: &str) -> io::Result<()> {
    //    println!("将要从{}读取",(**data).config().path());
    let mut file = File::open((**data).config().path()).expect(&format!(
        "{}{}\n",
        "Can not Open ".red(),
        (**data).config().path().red()
    ));
    //    let mut file = OpenOptions::new().write(true).open((**data).config().path()).await?;
    let mut content = String::new();
    file.read_to_string(&mut content).expect(&format!(
        "{}{}\n",
        "Can not Read From ".red(),
        (**data).config().path().red()
    ));
    //    println!("原始文件{}\n",content);
    let mut is_zone_region = false;
    let mut new = String::new();
    // 遍历zone文件每行
    for line in content.lines() {
        //	eprintln!("第{}次循环",index);
        // 跳过其他区域
        if !is_zone_region {
            if line.contains(")") {
                is_zone_region = true;
                new.push_str(&line);
                new.push('\n');
            } else {
                new.push_str(&line);
                new.push('\n');
                continue;
            }
        // zone区域
        } else {
            // 以空格分割
            let mut i = line.split_ascii_whitespace();
            // 匹配到域名
            let nth0 = i.nth(0).expect(&format!(
                "{}{}\n",
                "Can not Read From ".red(),
                (**data).config().path().red()
            ));
            if nth0 == domain {
                println!("{} {}", "Find ".green(), domain.purple());
                new.push_str(domain);
                new.push('\t');
                new.push_str(&i.next().unwrap());
                new.push('\t');
                new.push_str(&i.next().unwrap());
                new.push('\t');
                new.push_str(&i.next().unwrap());
                new.push('\t');
                new.push_str(address);
                new.push('\n');
            }
            // 未匹配到域名
            else {
                new.push_str(&line);
                new.push('\n');
            }
        }
    }
    //    println!("新字符串{}",&new);
    std::fs::write((**data).config().path(), new).expect(&format!(
        "{}{}\n",
        "Can not Write To ".red(),
        (**data).config().path().red()
    ));
    //    file.write(new.as_bytes()).expect("写入失败");
    //    println!("写入完成");
    println!("{}", "Succuess To Write".purple());
    return Ok(());
}
