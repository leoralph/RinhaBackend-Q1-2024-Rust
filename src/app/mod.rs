use actix_web::{App, HttpServer};
use routes::routes;
use std::io::Result;

mod controllers;
mod routes;

pub async fn server() -> Result<()> {
    let mut server = HttpServer::new(|| App::new().configure(routes));

    server = server.bind(("127.0.0.1", 8080))?;

    server.run().await
}
