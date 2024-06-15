// pub struct Nsd {
//     domain: String,
//     server: String,
//     record: String,
// }

// use std::io::{self, BufReader};
// use std::fs::File;
// use std::io::BufRead;
// pub async fn read_from_nsd(path : String) ->io::Result<()> {
//     let file = File::open(&path)?;
//     let buffer = BufReader::new(file);
//     for line in buffer.lines() {
// 	println!("{}",line.unwrap());
//     }
//     return Ok(());
// }
