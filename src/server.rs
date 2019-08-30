
extern crate simple_server;
extern crate libfonthelper;

use simple_server::{Method, Server};
use super::config::Config;
use super::routes::*;

pub fn init(config: Config) {
    let server = Server::new(|request, mut response| {
        println!("Request received. {} {}", request.method(), request.uri());

        match (request.method(), request.uri().path()) {
            (&Method::GET, "/figma/font-files") => get_files::handler(request, response),
            (&Method::GET, "/figma/font-file") => get_file::handler(request, response),
            (_, _) => any::handler(request, response),
        }
    });

    server.listen(&config.host, &config.port);
}
