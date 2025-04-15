use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SiteConfig {
    pub title: String,
    #[serde(default = "default_author")]
    pub author: String,
    #[serde(default = "default_content_dir")]
    pub content_dir: String,
    #[serde(default = "default_build_dir")]
    pub build_dir: String,
    #[serde(default = "default_templates")]
    pub templates: String,
    #[serde(default = "default_base_url")]
    pub base_url: String,
}

/// Loads the specified file as a SiteConfig struct
pub fn load_site_config<P: AsRef<Path>>(path: P) -> Result<SiteConfig, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let config: SiteConfig = toml::from_str(&content)?;
    Ok(config)
}

fn default_author() -> String {
    "Author Name".into()
}

fn default_content_dir() -> String {
    "content".into()
}

fn default_build_dir() -> String {
    "dist".into()
}

fn default_templates() -> String {
    "templates".into()
}

fn default_base_url() -> String {
    "/".into()
}
