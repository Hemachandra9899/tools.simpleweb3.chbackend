use actix_web::web::ServiceConfig;

pub mod routes;

// only register routes here
pub fn configure(cfg: &mut ServiceConfig) {
    routes::init(cfg);
}