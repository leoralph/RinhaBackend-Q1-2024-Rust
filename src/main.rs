use crate::app::server;

mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}
