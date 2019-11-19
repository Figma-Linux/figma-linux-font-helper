extern crate percent_encoding;

use crate::config::Config;
use crate::server::Route;
use crate::utils::is_valid_file;

use log::warn;
use percent_encoding::percent_decode;
use simple_server::{Method, Request, ResponseBuilder, ResponseResult, StatusCode};
use std::fs;

pub fn handler(
  request: Request<Vec<u8>>,
  mut response: ResponseBuilder,
  config: &Config,
) -> ResponseResult {
  let parsed_url: Vec<u8> = percent_decode(request.uri().to_string().as_bytes()).collect();
  let string = String::from_utf8(parsed_url).expect("BUG: Cannot parse file name from URL");
  let file_path = string[22..string.len()].to_string();

  if !is_valid_file(&config, &file_path) {
    warn!("Incorrect file: {}", &file_path);
    return Ok(
      response.status(StatusCode::BAD_REQUEST).body(
        format!("Incorrect file: {}", &file_path)
          .as_bytes()
          .to_vec(),
      )?,
    );
  }

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
