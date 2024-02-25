use crate::server::server;

mod routes;
mod server;
mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}
