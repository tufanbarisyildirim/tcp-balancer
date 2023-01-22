pub struct Upstream {
    pub ip: String,
    pub port: u16,
    pub is_live: bool,
}

impl Upstream {
   pub fn new(addr: &str) -> Upstream {
        let mut parts = addr.split(':');
        let ip = parts.next().unwrap().to_string();
        let port = parts.next().unwrap().parse().unwrap();
        Upstream {
            ip,
            port,
            is_live: true,
        }
    }
}