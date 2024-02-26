use actix_web::web;

use crate::app::controllers::transaction_controller;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/clientes/{id}/transacoes",
        web::post().to(transaction_controller::store_transaction),
    );
}
