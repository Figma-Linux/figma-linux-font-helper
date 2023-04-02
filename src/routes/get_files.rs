use crate::config::Config;
use crate::server::Route;

use libfonthelper::FontsHelper;
use simple_server::{Method, Request, ResponseBuilder, ResponseResult};

fn handler(_: Request<Vec<u8>>, mut response: ResponseBuilder, config: &Config) -> ResponseResult {
  let fonts = FontsHelper::new(&config.app.font_dirs);

  let mut json = "{\"version\": 4,\"fontFiles\":".to_string();
  json.push_str("{");

  for font in fonts {
    if font.entries.len() > 0 {
      json.push_str(&format!("\"{}\":[", font.path));
      for num in 0..font.entries.len() {
        json.push_str("{");
        json.push_str(&format!(
          "\"postscript\": \"{}\",",
          font.entries[num].postscript
        ));
        json.push_str(&format!("\"family\": \"{}\",", font.entries[num].family));
        json.push_str(&format!("\"id\": \"{}\",", font.entries[num].id));
        json.push_str(&format!("\"style\": \"{}\",", font.entries[num].style));
        json.push_str(&format!("\"weight\": {},", font.entries[num].weight));
        json.push_str(&format!("\"stretch\": {},", font.entries[num].stretch));
        json.push_str(&format!("\"italic\": {}", font.entries[num].italic));
        json.push_str("},");
      }
      json.pop();
      json.push_str("],");
    }
  }

  json.pop();
  json.push_str("}}");

  Ok(
    response
      .header("Access-Control-Allow-Origin", "https://www.figma.com")
      .header("Content-Type", "application/json")
      .header("Content-Length", json.bytes().len())
      .body(json.as_bytes().to_vec())?,
  )
}

pub fn init() -> Route {
  Route {
    method: Method::GET,
    path: String::from("/figma/font-files"),
    handler: Box::new(handler),
  }
}
