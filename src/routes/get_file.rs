extern crate percent_encoding;

use crate::config::Config;
use crate::server::Route;
use crate::utils::is_valid_file;

use log::warn;
use percent_encoding::percent_decode;
use simple_server::{Method, Request, ResponseBuilder, ResponseResult, StatusCode};
use std::collections::HashMap;
use std::fs;

pub fn handler(
  request: Request<Vec<u8>>,
  mut response: ResponseBuilder,
  config: &Config,
) -> ResponseResult {
  let parsed_url: Vec<u8> = percent_decode(request.uri().to_string().as_bytes()).collect();
  let string = String::from_utf8(parsed_url).expect("BUG: Cannot parse file name from URL");
  let mut url_split = string.splitn(2, "?");
  let _request_path = url_split.next();
  let query_str = url_split.next().expect("BUG: not query string was provide");
  let mut query_params = HashMap::new();
  for query_param_str in query_str.split("&") {
    let mut query_param_str_split = query_param_str.splitn(2, "=");
    let query_param_name = query_param_str_split
      .next()
      .expect("BUG: Invalid query param");
    let query_param_value = query_param_str_split
      .next()
      .expect("BUG: Invalid query param");
    query_params.insert(query_param_name, query_param_value);
  }
  let file_path = query_params["file"].to_string();

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
