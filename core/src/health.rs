// Health Check - verify all registered sites are alive
// Runs daily via cron to detect dead/moved sites

use crate::registry::SiteEntry;
use anyhow::Result;
use log::{info, warn};
use std::time::Duration;

/// Check all sites in registry
pub async fn check_all(sites: &[SiteEntry], registry_path: &str) -> Result<()> {
    println!("[Health Check] Checking {} sites...\n", sites.len());

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("DeepDown/0.1 (DarXone)")
        .build()?;

    let mut alive = 0;
    let mut dead = 0;
    let mut redirected = 0;

    for site in sites {
        match check_site(&client, site).await {
            SiteStatus::Alive => {
                println!("  [OK]   {} - {}", site.name, site.base_url);
                alive += 1;
            }
            SiteStatus::Redirected(url) => {
                warn!("  [REDIR] {} -> {}", site.name, url);
                redirected += 1;
            }
            SiteStatus::Dead(reason) => {
                warn!("  [FAIL] {} - {}", site.name, reason);
                dead += 1;
            }
        }
    }

    println!("\n--- Summary ---");
    println!("  Alive:     {}", alive);
    println!("  Redirected: {}", redirected);
    println!("  Dead:      {}", dead);
    println!("  Total:     {}", sites.len());

    Ok(())
}

enum SiteStatus {
    Alive,
    Redirected(String),
    Dead(String),
}

async fn check_site(client: &reqwest::Client, site: &SiteEntry) -> SiteStatus {
    let url = &site.base_url;
    match client.get(url).send().await {
        Ok(resp) => {
            let status = resp.status().as_u16();
            if status >= 200 && status < 300 {
                SiteStatus::Alive
            } else if status >= 300 && status < 400 {
                let location = resp
                    .headers()
                    .get("location")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("unknown")
                    .to_string();
                SiteStatus::Redirected(location)
            } else {
                SiteStatus::Dead(format!("HTTP {}", status))
            }
        }
        Err(e) => SiteStatus::Dead(e.to_string()),
    }
}
