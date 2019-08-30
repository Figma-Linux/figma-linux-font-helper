use simple_server::{Request, ResponseBuilder, ResponseResult, StatusCode};

pub fn handler(request: Request<Vec<u8>>, mut response: ResponseBuilder) ->  ResponseResult {
    response.status(StatusCode::NOT_FOUND);
    Ok(response.body("".as_bytes().to_vec())?)
}
