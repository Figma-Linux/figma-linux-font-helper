mod app;
mod args;
mod config;
mod log;
mod routes;
mod server;

fn main() {
  args::init();
  log::init();
  app::init();
}
