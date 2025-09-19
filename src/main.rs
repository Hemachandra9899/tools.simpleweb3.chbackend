mod config;
mod errors;
mod state;
mod telemetry;

// declare these so submodules compile
mod http;
mod models;
mod services;

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use config::Config;
use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let cfg = Config::from_env().expect("failed to load config");
    telemetry::init(&cfg);

    let app_state = AppState::init(&cfg).await.expect("failed to init state");

    let bind = (cfg.host.as_str(), cfg.port);
    tracing::info!(?bind, "starting HTTP server");

    HttpServer::new(move || {
        App::new()
            // middleware must be on App, not ServiceConfig
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            // shared state
            .app_data(app_state.clone())
            // only routes go into configure()
            .configure(http::configure)
    })
    .bind(bind)?
    .run()
    .await
}
