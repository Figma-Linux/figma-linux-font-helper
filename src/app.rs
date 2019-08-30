use super::config::Config;
use super::server;

pub fn init() {
    let config = Config::new();
    server::init(config);
}
