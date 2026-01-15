use super::method::Method;
use std::convert::TryFrom;

pub struct Request {
    pub path: String,
    pub query_string: Option<String>,
    pub method: Method,
}

impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        unimplemented!()
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let string = String::from("abcd");
        string.encrypt();
        unimplemented!()
    }
}

trait Encrypt {
    fn encrypt(&self) -> Self;
}

impl Encrypt for String {
    fn encrypt(&self) -> Self {
        unimplemented!()
    }
}

impl Encrypt for &[u8] {
    fn encrypt(&self) -> Self {
        unimplemented!()
    }
}