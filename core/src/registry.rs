// Registry Loader
// Loads YAML-based site registries from local path or GitHub

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Registry holding all known sites
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Registry {
    pub sites: Vec<SiteEntry>,
}

/// Single site entry from YAML registry
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SiteEntry {
    pub name: String,
    pub base_url: String,
    pub category: Option<String>,
    pub region: Option<String>,
    pub language: Option<String>,
    pub search: SiteSearch,
    pub extractor: Option<Extractor>,
    pub health: Option<HealthStatus>,
    pub vpn_required: Option<bool>,
    pub requires_api_key: Option<bool>,
    pub created_by: Option<String>,
    pub version: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SiteSearch {
    pub path: String,
    pub method: Option<String>,
    pub param: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Extractor {
    pub pattern: Option<String>,
    #[serde(rename = "type")]
    pub extractor_type: Option<String>,
    pub selector: Option<String>,
    pub fallback: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HealthStatus {
    pub last_checked: Option<String>,
    pub status: Option<String>,
}

/// Load registry from local YAML directory
pub fn load<P: AsRef<Path>>(registry_path: P) -> Result<Registry> {
    let dir = registry_path.as_ref().join("sites");
    let mut sites = Vec::new();

    fn collect_yml(dir: &Path, sites: &mut Vec<SiteEntry>) -> Result<()> {
        if !dir.exists() {
            anyhow::bail!("Registry path not found: {:?}", dir);
        }
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                collect_yml(&path, sites)?;
            } else if path.extension().map_or(false, |e| e == "yml" || e == "yaml") {
                let yaml = fs::read_to_string(&path)
                    .with_context(|| format!("Cannot read: {:?}", path))?;
                let site: SiteEntry = serde_yaml::from_str(&yaml)
                    .with_context(|| format!("Invalid YAML: {:?}", path))?;
                sites.push(site);
            }
        }
        Ok(())
    }

    collect_yml(&dir, &mut sites)?;
    Ok(Registry { sites })
}

/// Load from GitHub as fallback
pub async fn load_remote(url: &str) -> Result<Registry> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?.text().await?;
    Ok(serde_yaml::from_str(&response)?)
}
