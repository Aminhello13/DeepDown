// DeepDown Core Engine v0.1
// DarXone Group

mod registry;
mod search;
mod download;
mod health;

use clap::Parser;
use log::info;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "deepx")]
#[command(about = "DeepDown - Universal Download Source Aggregator")]
struct Cli {
    /// Search query
    #[arg(short, long)]
    query: Option<String>,

    /// Path to registry directory
    #[arg(short, long, default_value = "../registry")]
    registry: String,

    /// Download from URL
    #[arg(short, long)]
    download: Option<String>,

    /// Output directory
    #[arg(short, long, default_value = ".")]
    output: String,

    /// Number of parallel requests
    #[arg(short, long, default_value_t = 8)]
    parallel: usize,

    /// Health check only (validate all sources)
    #[arg(long)]
    health_check: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    // Step 1: Load registry
    let reg = registry::load(&cli.registry)?;
    info!("Registry loaded: {} sites", reg.sites.len());

    // Health check mode
    if cli.health_check {
        health::check_all(&reg.sites, &cli.registry).await?;
        return Ok(());
    }

    // Download mode
    if let Some(url) = cli.download {
        download::start(&url, &cli.output).await?;
        return Ok(());
    }

    // Search mode
    if let Some(query) = cli.query {
        let results = search::parallel(&reg.sites, &query, cli.parallel).await?;
        println!("\n[DeepDown] Results for: {}", query);
        println!("=======================================");
        if results.is_empty() {
            println!("  No results found in registry.");
            println!("  Contribute: https://github.com/darxone/site-registry");
        } else {
            println!("  Found {} links\n", results.len());
            for (idx, result) in results.iter().enumerate() {
                println!("  {}. [{}] {}", idx + 1, result.source, result.name);
                println!("     URL: {}", result.url);
                if let Some(ref size) = result.size {
                    println!("     Size: {}", size);
                }
                println!();
            }
        }
        return Ok(());
    }

    println!("DeepDown v0.1 - Use --query 'keyword' to search");
    Ok(())
}
