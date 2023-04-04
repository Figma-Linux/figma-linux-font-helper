extern crate libfonthelper;
extern crate log;
extern crate simple_server;

use super::config::Config;
use super::routes::*;
use log::info;
use simple_server::{Method, Request, ResponseBuilder, ResponseResult, StatusCode};
use std::sync::Arc;

pub struct Server {
  config: Config,
  routes: Box<Vec<Route>>,
}

impl Server {
  pub fn new(config: Config) -> Self {
    Server {
      config,
      routes: Box::new(vec![]),
    }
  }

  pub fn add_route(mut self, route: Route) -> Self {
    self.routes.push(route);
    self
  }

  pub fn start(self) {
    let serv = Arc::new(self);
    let s = serv.clone();

    let server = simple_server::Server::new(move |request, mut response| {
      info!("Request received. {} {}", request.method(), request.uri());

      let s = serv.as_ref();
      let routes = Arc::new(s.routes.as_ref());

      if request.method() == Method::OPTIONS {
        return Ok(
          response
            .status(StatusCode::NO_CONTENT)
            .header("Access-Control-Allow-Origin", "https://www.figma.com")
            .header("Access-Control-Allow-Private-Network", "true")
            .header("Content-Type", "application/octet-stream")
            .body("".as_bytes().to_vec())?,
        );
      }

      for route in *routes {
        if route.method == request.method() && route.path == request.uri().path() {
          let handler = &(route.handler).as_ref();
          return Ok(handler(request, response, &s.config).unwrap());
        }
      }

      any::handler(request, response)
    });

    info!("{:?}", &s.config);

    server.listen(&s.config.host, &s.config.port);
  }
}

pub type Handler =
  Box<dyn Fn(Request<Vec<u8>>, ResponseBuilder, &Config) -> ResponseResult + 'static + Send + Sync>;

pub struct Route {
  pub method: Method,
  pub path: String,
  pub handler: Handler,
}
