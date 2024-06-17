<div align="center">

[![](https://img.shields.io/crates/d/rddns.svg)](https://crates.io/crates/rddns)
[![](https://img.shields.io/github/forks/barrensea/rddns.svg)](https://github.com/BarrenSea/rddns/fork)
[![](https://img.shields.io/github/repo-size/barrensea/rddns.svg)](https://github.com/BarrenSea/rddns)
[![](https://img.shields.io/github/stars/barrensea/rddns.svg)](https://github.com/BarrenSea/rddns)
[![](https://img.shields.io/github/commit-activity/t/barrensea/rddns.svg)](https://github.com/BarrenSea/rddns)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org)


</div>

<p align="center">
  <a href="https://github.com/barrensea/rddns" rel="noopener">
 <img width=200px height=200px src="./imgs/barrensea.jpg"></a>

 <h3 align="center"><a href="https://github.com/barrensea/rddns">Rddns</a></h3>
  <p align="center">
    Rust DDNS For NSD
    <br />
    <br />
    <a href="https://matrix.to/#/#algori:mozilla.org">Matrix</a>
  </p>
</p>

# RDDNS
A DDNS Server with NSD DNS Server
## Use
### Server
You Can Build from `source`
``` shell
git clone https://github.com/barrensea/rddns && cd rddns
cargo build --release
./target/release/rddns --nds /etc/nsd/zone/example.zone
```

Or use `cargo install`
``` shell
cargo instasll rddns
```

### Client
You Can Use *Curl* with the server
#### DDNS
``` shell
curl -X POST http://your-nsd-server-domain/ddns/your-domain-want-to-change/new-address/password
```


##### Example
``` shell
curl -XPOST http://123.com/ddns/www.123.com./123.123.123.123/password
curl -XPOST http://123.com/ddns/www/123.123.123.123/mypassword
curl -XPOST http://123.com/ddns/www/$(curl -XGET http://123.com/ip)/mypassword
```

#### Get your ip
``` shell
curl -XGET http://your-nsd-server-domain/ddns/your-domain-want-to-change/new-address
```

##### Example
``` shell
curl http://123.com/ip
```



## Help
```
DDNS Server and Client for Nsd

Usage: rddns [OPTIONS] --nsd <nsd> --auth <AUTH>

Options:
  -l, --listen <listen>    [default: 0.0.0.0:3000]
  -n, --nsd <nsd>
  -w, --workers <WORKERS>  [default: 1]
  -a, --auth <AUTH>
  -h, --help               Print help
  -V, --version            Print version

```
