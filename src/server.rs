use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;


pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // if stream > 1024 Bytes, this would fail but 1024 is enough for us in this case.
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(_) => {},
                                Err(e) => println!("Failed to parse the request: {}", e),
                            }
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}