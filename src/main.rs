mod app;
mod config;
mod log;
mod routes;
mod server;

fn main() {
  log::init();
  app::init();
}
