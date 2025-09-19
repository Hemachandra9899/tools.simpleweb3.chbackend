use crate::config::Config;
use actix_web::web;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub app_name: String,
    pub data_path: PathBuf,
    pub file_lock: Arc<RwLock<()>>,
}

impl AppState {
    pub async fn init(cfg: &Config) -> anyhow::Result<web::Data<Self>> {
        Ok(web::Data::new(Self {
            app_name: cfg.app_name.clone(),
            data_path: PathBuf::from(&cfg.data_path),
            file_lock: Arc::new(RwLock::new(())),
        }))
    }
}
