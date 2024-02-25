use actix_web::{web, Result};

pub async fn get_statement(path: web::Path<u32>) -> Result<String> {
    let id = path.into_inner();

    Ok(format!("Hello world again! {}", id))
}
