use actix_web::web;

use crate::controllers::statement_controller;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/clientes/{id}/extrato",
        web::get().to(statement_controller::get_statement),
    );
}
