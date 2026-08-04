// Search Engine - parallel search across registered sites

use crate::registry::SiteEntry;
use anyhow::Result;
use log::{debug, warn};
use regex::Regex;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub name: String,
    pub url: String,
    pub size: Option<String>,
    pub source: String,
}

/// Execute query against all registered sites in parallel
pub async fn parallel(sites: &[SiteEntry], query: &str, max_parallel: usize) -> Result<Vec<SearchResult>> {
    debug!("Searching '{}' across {} site(s)", query, sites.len());

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .user_agent("DeepDown/0.1 (DarXone)")
        .build()?;

    let mut tasks = Vec::new();

    for site in sites {
        let site = site.clone();
        let client = client.clone();
        let query = query.to_string();
        tasks.push(tokio::spawn(async move {
            search_site(&client, &site, &query).await
        }));
    }

    let mut results = Vec::new();
    for task in tasks {
        match task.await {
            Ok(Ok(hits)) => {
                if !hits.is_empty() {
                    debug!("{} - {} hits", hits[0].source, hits.len());
                }
                results.extend(hits);
            }
            Ok(Err(e)) => debug!("site error: {}", e),
            Err(e) => debug!("task join error: {}", e),
        }
    }

    Ok(results)
}

/// Search a single site
async fn search_site(client: &reqwest::Client, site: &SiteEntry, query: &str) -> Result<Vec<SearchResult>> {
    let url = build_search_url(site, query);
    debug!("[GET] {} ({})", url, site.name);

    let resp = client.get(&url).send().await?;
    if !resp.status().is_success() {
        return Ok(Vec::new());
    }

    let body = resp.text().await?;
    let extracted = extract_urls(site, &body);

    let results: Vec<SearchResult> = extracted
        .into_iter()
        .map(|u| SearchResult {
            name: site.name.clone(),
            url: u,
            size: None,
            source: site.base_url.clone(),
        })
        .collect();

    Ok(results)
}

/// Build the search URL from site definition
fn build_search_url(site: &SiteEntry, query: &str) -> String {
    let path = &site.search.path;
    if path.contains("{query}") {
        let encoded = url_encode(query);
        return format!("{}{}", site.base_url, path.replace("{query}", &encoded));
    }
    // If no {query} placeholder, it's a listing page (like kali-latest/)
    // We fetch the page and extract all matching links
    format!("{}{}", site.base_url, path)
}

/// Simple URL encoder
fn url_encode(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => result.push(c),
            _ => {
                for byte in c.to_string().as_bytes() {
                    result.push_str(&format!("%{:02X}", byte));
                }
            }
        }
    }
    result
}

/// Extract download URLs from HTML body using regex or CSS/XPath
fn extract_urls(site: &SiteEntry, body: &str) -> Vec<String> {
    let mut urls = Vec::new();

    if let Some(ref extractor) = site.extractor {
        let ext_type = extractor.extractor_type.as_deref().unwrap_or("regex");

        match ext_type {
            "regex" => {
                if let Some(ref pattern) = extractor.pattern {
                    if let Ok(re) = Regex::new(pattern) {
                        for cap in re.captures_iter(body) {
                            let url = cap.get(1).map_or("", |m| m.as_str()).to_string();
                            if !url.is_empty() && !urls.contains(&url) {
                                urls.push(url);
                            }
                        }
                    }
                }
            }
            "xpath" | "css" => {
                use scraper::{Html, Selector};
                let document = Html::parse_document(body);
                let css_sel = extractor.selector.as_deref().unwrap_or("a[href]");
                if let Ok(selector) = Selector::parse(css_sel) {
                    for element in document.select(&selector) {
                        if let Some(href) = element.value().attr("href") {
                            let full_url = if href.starts_with("http") {
                                href.to_string()
                            } else {
                                format!("{}/{}", site.base_url.trim_end_matches('/'), href.trim_start_matches('/'))
                            };
                            if !urls.contains(&full_url) {
                                urls.push(full_url);
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        // Fallback pattern
        if urls.is_empty() {
            if let Some(ref fallback) = extractor.fallback {
                if let Ok(re) = Regex::new(fallback) {
                    for cap in re.captures_iter(body) {
                        let url = cap.get(1).map_or("", |m| m.as_str()).to_string();
                        if !url.is_empty() {
                            urls.push(url);
                        }
                    }
                }
            }
        }
    }

    urls
}
