use std::io::{Error, ErrorKind};
use std::net::Ipv4Addr;
use log::{debug, error, info};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::net::ToSocketAddrs;
use serde::{Serialize, Deserialize};
pub struct Server {
    port: u16,
    address: [u8; 4],
    connection: Option<Connection>
}

impl Server {
    pub async fn start(&self) -> std::io::Result<()> {
        logger::init_logger();
        let address = (Ipv4Addr::from(self.address), self.port);
        let listener = TcpListener::bind(address).await?;
        let addr = listener.local_addr()?;
        info!("Server started, listening on {}:{}", addr.ip().to_string(), addr.port());
        while let Ok((socket, _)) = listener.accept().await {
            info!("Connection Accepted");
            tokio::spawn(async move {
                process_socket(socket).await
            });
        };
        Ok(())
    }
}

async fn process_socket(mut socket: TcpStream) {
    let mut connection = Connection::new(socket);
    loop {
        let frame = connection.read_frame().await;
        match frame {
            Ok(req) => {
                match req {
                    None => {
                        info!("Disconnecting!");
                        connection.flush();
                        break;
                    }
                    Some(req) => {
                        debug!("req: {:?}", req);
                        let response = process_request(&req).await;
                        debug!("res: {:?}", response);
                        connection.write_frame(response).await.unwrap();
                    }
                }
            }
            Err(err) => {
                error!("Error: {}", err);
                break;
            }
        }
    };
}

pub struct ServerBuilder {
    port: u16,
    address: [u8; 4],
}


impl ServerBuilder {
    pub fn new() -> Self {
        ServerBuilder::default()
    }
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    pub fn address(mut self, address: [u8; 4]) -> Self {
        self.address = address;
        self
    }

    pub fn build(self) -> Server {
        Server {
            port: self.port,
            address: self.address,
            connection: None
        }
    }
}

impl Default for ServerBuilder {
    fn default() -> Self {
        ServerBuilder {
            address: [127,0,0,1],
            port: 8080
        }
    }
}

struct Connection {
    stream: TcpStream,
    buffer: Vec<u8>,
    cursor: usize,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            // Allocate the buffer with 4kb of capacity.
            buffer: vec![0; 4096],
            cursor: 0,
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<Request>, Error> {
        loop {
            if let Some(model) = self.parse_frame()? {
                return Ok(Some(model));
            }

            if self.cursor == self.buffer.len() {
                self.buffer.resize(self.cursor * 2, 0);
            }


            let n = self.stream.read(&mut self.buffer[self.cursor..]).await?;

            if 0 == n {
                if self.cursor == 0 {
                    return Ok(None);
                } else {
                    return Err(Error::new(ErrorKind::ConnectionAborted, "Connection reset by the peer!"));
                }
            } else {
                self.cursor += n;
            }
        }
    }
    pub async fn write_frame(&mut self, response: Response) -> Result<(), Error> {
        match serde_json::to_string(&response) {
            Ok(data) => {
                self.stream.write_all(data.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
                self.flush().await;
            }
            Err(err) => {
                error!("Error serializing response: {}", err);
            }
        }
        Ok(())
    }

    pub async fn flush(&mut self) -> Result<(), Error> {
        self.buffer.clear();
        self.buffer.resize(4096, 0);
        self.stream.flush().await;
        self.cursor = 0;
        Ok(())
    }

    fn parse_frame(&mut self) -> Result<Option<Request>, Error> {
        info!("frame: {}", String::from_utf8_lossy(&self.buffer[..]));
        info!("example: {}", "HTTP/1.1 200 OK\r\nFolded-Header: hello\r\n there \r\n\r\n");
        let result: serde_json::Result<Request> = serde_json::from_slice(&self.buffer[..self.cursor]);
        match result {
            Ok(req) => {
                Ok(Some(req))
            }
            err => {
                match err {
                    Ok(_) => {}
                    Err(err) => {
                        error!("Error: {}", err);
                        error!("Error BufSize: {}", self.buffer.len());
                        error!("Error Cursor: {}", self.cursor);
                    }
                }
                Ok(None)
            }
        }
    }
}
use crate::dao::*;
use crate::logger;
use crate::prelude::*;

async fn process_request(req: &Request) -> Response {
    match req.method {
        Command::ADD => {
            add_user_images(&req.body).await.transform().await.into().0.into()
        }
        Command::DELETE => {
            delete_user(&req.body).await.transform().await.into().0.into()
        }
        Command::VERIFY => {
            verify_user(&req.body).await.transform().await.into().0.into()
        }
    }
}
