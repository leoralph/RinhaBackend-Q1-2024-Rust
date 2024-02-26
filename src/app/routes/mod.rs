use actix_web::web;

use statement::routes as statement_routes;
use transaction::routes as transaction_routes;

mod statement;
mod transaction;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(statement_routes)
        .configure(transaction_routes);
}
