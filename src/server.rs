
extern crate simple_server;
extern crate libfonthelper;

use std::fs;
use simple_server::{Method, Server, StatusCode};
use libfonthelper::Fonts;

pub fn init() {
    let host = "127.0.0.1";
    let port = "18412";

    let server = Server::new(|request, mut response| {
        println!("Request received. {} {}", request.method(), request.uri());

        match (request.method(), request.uri().path()) {
            (&Method::GET, "/figma/font-files") => {
                let dirs = vec![
                    String::from("/usr/share/fonts"),
                    String::from("/home/ruut/.local/share/fonts"),
                ];

                match Fonts::new(&dirs) {
                    Err(err) => {
                        eprintln!("{:?}", err);
                        response.status(StatusCode::INTERNAL_SERVER_ERROR);
                        Ok(response.body("Failed get fonts".as_bytes().to_vec())?)
                    },
                    Ok(fonts) => {
                        let mut json = "{\"version\": 4,\"fontFiles\":".to_string();
                        json.push_str(&fonts.to_json());
                        json.push_str("}");
                        Ok(response
                            .header("Access-Control-Allow-Origin", "https://www.figma.com")
                            .header("Content-Type", "application/json")
                            .header("Content-Length", json.bytes().len())
                            .body(json.as_bytes().to_vec())?)
                    }
                }
            }
            (&Method::GET, "/figma/font-file") => {
                let v: Vec<String> = request.uri().to_string().split("%2F").map(|s| s.to_string()).collect();
                let string = v.join("/");
                let file_path = string[22..string.len()].to_string();
                match fs::read(file_path) {
                    Err(err) => {
                        eprintln!("Parse uri failed: {}", err);
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
            (_, _) => {
                response.status(StatusCode::NOT_FOUND);
                Ok(response.body("".as_bytes().to_vec())?)
            }
        }
    });

    server.listen(host, port);
}
