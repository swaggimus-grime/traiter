use std::collections::HashMap;
use std::sync::Arc;
use data::providers::{Provider, ProviderType, Yahoo};
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
    
    pub fn provider_from_string(&self, provider: &str) -> Option<&SafeProvider> {
        let ty = ProviderType::from(provider);
        self.providers.get(&ty)
    }
}