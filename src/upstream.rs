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
//
// struct Upstreams {
//     upstreams: Vec<Upstream>,
//     strategy: Box<dyn BalancerStrategy>,
// }
//
// impl Upstreams {
//     fn new(destinations: Vec<&str>) -> Self {
//         let upstreams = destinations.iter().map(|d| Upstream::new(d)).collect();
//         let strategy = Box::new(RoundRobinBalancer::new());
//         Upstreams { upstreams, strategy }
//     }
//
//     fn get_next_upstream(&mut self) -> Option<&mut Upstream> {
//         self.strategy.get_next_upstream(&mut self.upstreams)
//     }
// }
//
// trait BalancerStrategy {
//     fn get_next_upstream(&mut self, upstreams: &mut Vec<Upstream>) -> Option<&mut Upstream>;
// }
//
// struct RoundRobinBalancer {
//     current_index: usize,
// }
//
// impl RoundRobinBalancer {
//     fn new() -> Self {
//         RoundRobinBalancer { current_index: 0 }
//     }
// }
//
// impl BalancerStrategy for RoundRobinBalancer {
//     fn get_next_upstream(&mut self, upstreams: &mut Vec<Upstream>) -> Option<&mut Upstream> {
//         let upstream = upstreams.get_mut(self.current_index % upstreams.len())?;
//         self.current_index += 1;
//         Some(upstream)
//     }
// }
