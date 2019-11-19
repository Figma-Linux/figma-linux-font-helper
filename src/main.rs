mod app;
mod args;
mod config;
mod log;
mod routes;
mod server;
mod utils;

fn main() {
  args::init();
  log::init();
  app::init();
}
