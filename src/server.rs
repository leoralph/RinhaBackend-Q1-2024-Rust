use crate::routes::routes;
use actix_web::{App, HttpServer};
use std::io::Result;

pub async fn server() -> Result<()> {
    let mut server = HttpServer::new(|| App::new().configure(routes));

    server = server.bind(("127.0.0.1", 8080))?;

    server.run().await
}
