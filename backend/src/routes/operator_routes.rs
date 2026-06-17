use actix_web::web;
use crate::handlers::operator_handler::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/operators", web::get().to(get_operators))
    );
}