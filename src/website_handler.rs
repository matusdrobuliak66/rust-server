use super::server::Handler;
use super::http::{Method, Request, Response, StatusCode};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => {
                match request.path() {
                    "/" => Response::new(StatusCode::Ok, Some("<h1>Welcome to the homepage!</h1>".to_string())),
                    "/about" => Response::new(StatusCode::Ok, Some("<h1>About us</h1><p>This is the about page.</p>".to_string())),
                    _ => Response::new(StatusCode::NotFound, None),
                }
            }
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}