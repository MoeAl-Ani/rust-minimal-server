use crate::server::{Server, ServerBuilder};

mod logger;
mod server;
mod dao;
mod prelude;
mod entities;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = ServerBuilder::new().address([0, 0, 0, 0]).port(8080).
        build();
    server.start().await
}



