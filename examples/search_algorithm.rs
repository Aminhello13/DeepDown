// DeepDown Example: Parallel Site Search (simplified)
// This demonstrates the core search algorithm without proprietary code.
// Production version includes: caching, regional routing, smart filtering.

// DeepDown Example: Parallel Site Search (simplified)
// This demonstrates the core search algorithm without proprietary code.
// Production version includes: caching, regional routing, smart filtering.

/// Simplified registry entry for demo purposes
#[derive(Debug, Clone)]
struct SourceSite {
    name: String,
    base_url: String,
    search_path: String,
    extractor_regex: Option<String>,
}

/// Metadata about a download
#[derive(Debug)]
struct DownloadLink {
    name: String,
    url: String,
    size: Option<u64>,
    source: String,
}

/// Core search algorithm (simplified)
fn search_sources(
    query: &str,
    sources: &[SourceSite],
    client: &reqwest::blocking::Client,
) -> Vec<DownloadLink> {
    let mut results = Vec::new();

    // Step 1: For each source, build search URL
    for site in sources {
        let search_url = format!("{}{}", site.base_url, site.search_path.replace("{query}", query));

        // Step 2: Fetch page
        let resp = match client.get(&search_url).send() {
            Ok(r) if r.status().is_success() => r,
            _ => continue,
        };

        let body = resp.text().unwrap_or_default();

        // Step 3: Extract download links using site-specific pattern
        if let Some(ref pattern) = site.extractor_regex {
            let re = regex::Regex::new(pattern).unwrap();
            for cap in re.captures_iter(&body) {
                let url = cap.get(1).unwrap().as_str().to_string();
                results.push(DownloadLink {
                    name: site.name.clone(),
                    url,
                    size: None,
                    source: site.base_url.clone(),
                });
            }
        }
    }

    results
}

fn main() {
    let sources = vec![
        SourceSite {
            name: "Kali Linux Official".into(),
            base_url: "https://cdimage.kali.org".into(),
            search_path: "/kali-latest/".into(),
            extractor_regex: Some(r#"href="(kali-linux-[\d.]+-installer-[^"]+\.iso)""#.into()),
        },
        SourceSite {
            name: "Ubuntu Releases".into(),
            base_url: "https://releases.ubuntu.com".into(),
            search_path: "/".into(),
            extractor_regex: Some(r#"href="(ubuntu-[\d.]+-desktop-[^"]+\.iso)""#.into()),
        },
    ];

    let client = reqwest::blocking::Client::new();
    let results = search_sites("linux", &sources, &client);

    println!("[DeepDown Example] Found {} results:", results.len());
    for link in &results {
        println!("  [{}] {}", link.source, link.url);
    }
}