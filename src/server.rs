use std::{io::Read, net::TcpListener};
use std::io::Read;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Server running on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    stream.read(&mut buffer);
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                    continue;
                }
            }
        }
    }
}
