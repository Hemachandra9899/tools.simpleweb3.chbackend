use std::{path::PathBuf, sync::Arc};
use actix_web::web;
use tokio::sync::RwLock;
use crate::config::Config;

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
