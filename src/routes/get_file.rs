use super::super::config::Config;
use super::super::server::Route;
use log::warn;
use simple_server::{Method, Request, ResponseBuilder, ResponseResult, StatusCode};
use std::fs;

pub fn handler(
  request: Request<Vec<u8>>,
  mut response: ResponseBuilder,
  _: &Config,
) -> ResponseResult {
  let v: Vec<String> = request
    .uri()
    .to_string()
    .split("%2F")
    .map(|s| s.to_string())
    .collect();
  let string = v.join("/");
  let file_path = string[22..string.len()].to_string();

  match fs::read(file_path) {
    Err(err) => {
      warn!("Parse uri failed: {}", err);
      return Ok(
        response
          .status(StatusCode::INTERNAL_SERVER_ERROR)
          .body(format!("Failed to read file, {}", err).as_bytes().to_vec())?,
      );
    }
    Ok(file) => {
      return Ok(
        response
          .header("Access-Control-Allow-Origin", "https://www.figma.com")
          .header("Content-Type", "application/octet-stream")
          .body(file.to_vec())?,
      );
    }
  };
}

pub fn init() -> Route {
  Route {
    method: Method::GET,
    path: String::from("/figma/font-file"),
    handler: Box::new(handler),
  }
}
