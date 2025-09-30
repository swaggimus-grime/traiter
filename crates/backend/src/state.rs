use std::collections::HashMap;
use std::sync::Arc;
use api::ProviderType;
use data::providers::{Provider, Yahoo};
use crate::config::BackendConfig;

pub type SafeProvider = Arc<Box<dyn Provider + Send + Sync>>;

pub struct BackendState {
    pub config: BackendConfig,
    providers: HashMap<ProviderType, SafeProvider>,
}

impl BackendState {
    pub fn new() -> anyhow::Result<Self> {
        let config = BackendConfig::load().expect("Failed to load config");
        let mut providers: HashMap<ProviderType, SafeProvider> = HashMap::new();
        providers.insert(ProviderType::Yahoo, Arc::new(Box::new(Yahoo::new()?)));

        Ok(Self {
            config,
            providers,
        })
    }
    
    pub fn get_provider(&self, provider: ProviderType) -> Option<&SafeProvider> {
        self.providers.get(&provider)
    }
}