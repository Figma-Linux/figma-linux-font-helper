use super::config::Config;
use super::server;

pub fn init() {
  server::init(Config::new());
}
