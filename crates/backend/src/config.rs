use std::env;
use config::BaseConfig;

#[derive(Debug, Clone)]
pub struct BackendConfig {
    pub base: BaseConfig,
    pub port: u16,
}

impl BackendConfig {
    pub fn load() -> anyhow::Result<Self> {
        dotenvy::from_path("crates/backend/.env.public")?;

        let port: u16 = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("PORT must be a number");

        Ok(Self {
            base: BaseConfig::load()?,
            port
        })
    }
}
