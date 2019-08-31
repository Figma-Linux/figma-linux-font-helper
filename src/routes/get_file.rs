use simple_server::{Request, ResponseBuilder, ResponseResult, StatusCode};
use log::warn;
use std::fs;

pub fn handler(request: Request<Vec<u8>>, mut response: ResponseBuilder) ->  ResponseResult {
    let v: Vec<String> = request.uri().to_string().split("%2F").map(|s| s.to_string()).collect();
    let string = v.join("/");
    let file_path = string[22..string.len()].to_string();

    match fs::read(file_path) {
        Err(err) => {
            warn!("Parse uri failed: {}", err);
            return Ok(response
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("Failed to read file, {}", err).as_bytes().to_vec())?
            );
        },
        Ok(file) => {
            return Ok(response
                .header("Access-Control-Allow-Origin", "https://www.figma.com")
                .header("Content-Type", "application/octet-stream")
                .body(file.to_vec())?
            );
        }
    };
}
