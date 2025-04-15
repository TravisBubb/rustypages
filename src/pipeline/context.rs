use crate::config::SiteConfig;

/// Represents the context/state that gets passed between the pipeline stages
pub struct Context {
    pub config: Option<SiteConfig>,
}

impl Context {
    /// Initializes a new Context
    pub fn new() -> Self {
        Self {
            config: None,
        }
    }

    /// Safe access to config. returns a reference to the config if loaded
    pub fn config(&self) -> Option<&SiteConfig> {
        self.config.as_ref()
    }

    /// Force access to config, panics if not loaded
    pub fn config_or_panic(&self) -> &SiteConfig {
        self.config.as_ref().expect("Config is not loaded")
    }
}
