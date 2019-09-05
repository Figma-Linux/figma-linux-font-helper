use super::super::config::Config;
use super::super::server::Route;
use libfonthelper::Fonts;
use log::warn;
use simple_server::{Method, Request, ResponseBuilder, ResponseResult, StatusCode};

fn handler(_: Request<Vec<u8>>, mut response: ResponseBuilder, config: &Config) -> ResponseResult {
  match Fonts::new(&config.directories) {
    Err(err) => {
      warn!("Cannot get fonts, ERROR: {}", err);
      Ok(
        response
          .header("Access-Control-Allow-Origin", "https://www.figma.com")
          .header("Content-Type", "text/plain")
          .status(StatusCode::INTERNAL_SERVER_ERROR)
          .body("Failed get fonts".as_bytes().to_vec())?,
      )
    }
    Ok(fonts) => {
      let mut json = "{\"version\": 4,\"fontFiles\":".to_string();
      json.push_str(&fonts.to_json());
      json.push_str("}");

      Ok(
        response
          .header("Access-Control-Allow-Origin", "https://www.figma.com")
          .header("Content-Type", "application/json")
          .header("Content-Length", json.bytes().len())
          .body(json.as_bytes().to_vec())?,
      )
    }
  }
}

pub fn init() -> Route {
  Route {
    method: Method::GET,
    path: String::from("/figma/font-files"),
    handler: Box::new(handler),
  }
}
