
use super::method::Method;

pub struct Request {
    pub path: String,
    pub query_string: Option<String>,
    pub method: Method,
}