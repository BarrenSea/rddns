use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Config {
    #[arg(short, long, default_value = "0.0.0.0:3000")]
    address: String,
    #[arg(name = "nsd", short, long)]
    nsd_file: String,
    #[arg(short, long, default_value_t = 1)]
    workers: usize,
}

impl Config {
    pub fn address(&self) -> &str {
        return &self.address;
    }
    pub fn workers(&self) -> usize {
        return self.workers;
    }
    pub fn path(&self) -> &str {
        return &self.nsd_file;
    }
}
