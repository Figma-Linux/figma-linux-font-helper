mod app;
mod log;
mod routes;
mod config;
mod server;

fn main() {
    log::init();
    app::init();
}
