use actix_web::web;

pub mod data;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(data::get_data).service(data::add_item);
}
