pub struct Config {
    pub host: String,
    pub port: u16,
    pub app_name: String,
    pub data_path: String,
}

impl Config {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            port: std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(3000),
            app_name: std::env::var("APP_NAME").unwrap_or_else(|_| "json-backend".into()),
            data_path: std::env::var("DATA_PATH").unwrap_or_else(|_| "data.json".into()),
        })
    }
}
