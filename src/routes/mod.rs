use actix_web::web;

use crate::routes::statement::routes as statement_routes;
use crate::routes::transaction::routes as transaction_routes;

mod statement;
mod transaction;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(statement_routes)
        .configure(transaction_routes);
}
