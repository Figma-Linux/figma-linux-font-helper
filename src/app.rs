use super::config::Config;
use super::routes::*;
use super::server;

pub fn init() {
  server::Server::new(Config::new())
    .add_route(get_files::init())
    .add_route(get_file::init())
    .start();
}
