use std::env;

#[derive(Debug, Clone)]
pub struct BaseConfig {
    pub log_level: String,
}

impl BaseConfig {
    pub fn load() -> anyhow::Result<Self> {
        dotenvy::from_path(".env.public")?;

        Ok(BaseConfig {
            log_level: env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
        })
    }
}