use simple_server::{Request, ResponseBuilder, ResponseResult, StatusCode};
use log::info;

pub fn handler(request: Request<Vec<u8>>, mut response: ResponseBuilder) ->  ResponseResult {
    info!("Not found {}", request.uri());

    response.status(StatusCode::NOT_FOUND);
    Ok(response.body("".as_bytes().to_vec())?)
}
