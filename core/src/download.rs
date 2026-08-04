// Download Manager - resume-capable, segmented downloads

use anyhow::{Context, Result};
use log::{info, warn};
use std::path::Path;
use tokio::io::AsyncWriteExt;

/// Start a download from a URL to a local file
pub async fn start(url: &str, output_dir: &str) -> Result<()> {
    info!("Starting download: {}", url);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .user_agent("DeepDown/0.1 (DarXone)")
        .build()?;

    // Get filename from URL
    let filename = url
        .split('/')
        .last()
        .unwrap_or("deepx_download.bin")
        .split('?')
        .next()
        .unwrap_or("deepx_download.bin");

    let output_path = Path::new(output_dir).join(filename);
    info!("Output: {:?}", output_path);

    // Check for partial download (resume support)
    let mut resume_from: u64 = 0;
    if output_path.exists() {
        resume_from = std::fs::metadata(&output_path)?.len();
        info!("Partial download found: {} bytes, resuming...", resume_from);
    }

    // Build request with range header for resume
    let mut req = client.get(url);
    if resume_from > 0 {
        req = req.header("Range", format!("bytes={}-", resume_from));
    }

    let resp = req.send().await.context("Failed to connect")?;

    if !resp.status().is_success() && resp.status().as_u16() != 206 {
        anyhow::bail!("HTTP {} - download failed", resp.status());
    }

    // Get total size
    let total_size = resp
        .content_length()
        .map(|l| l + resume_from)
        .unwrap_or(0);

    info!("Total size: {} bytes", total_size);

    // Open file for append (resume) or create new
    let mut file = if resume_from > 0 {
        tokio::fs::OpenOptions::new()
            .append(true)
            .open(&output_path)
            .await?
    } else {
        tokio::fs::File::create(&output_path).await?
    };

    // Stream body to file
    let mut downloaded = resume_from;
    let mut stream = resp.bytes_stream();

    use futures_util::StreamExt;
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;

        // Progress
        if total_size > 0 {
            let pct = (downloaded as f64 / total_size as f64) * 100.0;
            print!("\r  Progress: {:.1}% ({}/{} bytes)", pct, downloaded, total_size);
        } else {
            print!("\r  Downloaded: {} bytes", downloaded);
        }
    }
    println!();

    file.flush().await?;
    info!("Download complete: {:?}", output_path);
    println!("\n  [DONE] Saved to: {:?}", output_path);

    Ok(())
}

/// Download with segments (parallel chunks) - for large files
pub async fn segmented(url: &str, output_dir: &str, segments: usize) -> Result<()> {
    info!("Segmented download: {} ({} segments)", url, segments);

    let client = reqwest::Client::builder()
        .user_agent("DeepDown/0.1 (DarXone)")
        .build()?;

    // First, get file size with HEAD request
    let resp = client.head(url).send().await?;
    let total_size = resp
        .content_length()
        .ok_or_else(|| anyhow::anyhow!("Server did not return content-length"))?;

    let filename = url.split('/').last().unwrap_or("download.bin").split('?').next().unwrap_or("download.bin");
    let output_path = Path::new(output_dir).join(filename);

    // Calculate segment sizes
    let segment_size = total_size / segments as u64;
    let mut tasks = Vec::new();

    for i in 0..segments {
        let start = i as u64 * segment_size;
        let end = if i == segments - 1 {
            total_size - 1
        } else {
            (i as u64 + 1) * segment_size - 1
        };
        let url = url.to_string();
        let client = client.clone();
        let seg_path = format!("{}.part{}", output_path.display(), i);
        tasks.push(tokio::spawn(async move {
            download_segment(&client, &url, start, end, &seg_path).await
        }));
    }

    // Wait for all segments
    for task in tasks {
        task.await??;
    }

    // Merge segments
    let mut final_file = tokio::fs::File::create(&output_path).await?;
    for i in 0..segments {
        let seg_path = format!("{}.part{}", output_path.display(), i);
        let seg_data = tokio::fs::read(&seg_path).await?;
        final_file.write_all(&seg_data).await?;
        tokio::fs::remove_file(&seg_path).await.ok();
    }
    final_file.flush().await?;

    info!("Segmented download complete: {:?}", output_path);
    println!("\n  [DONE] Saved to: {:?}", output_path);
    Ok(())
}

async fn download_segment(
    client: &reqwest::Client,
    url: &str,
    start: u64,
    end: u64,
    output_path: &str,
) -> Result<()> {
    let resp = client
        .get(url)
        .header("Range", format!("bytes={}-{}", start, end))
        .send()
        .await?;

    if !resp.status().is_success() && resp.status().as_u16() != 206 {
        anyhow::bail!("Segment download failed: HTTP {}", resp.status());
    }

    let bytes = resp.bytes().await?;
    tokio::fs::write(output_path, &bytes).await?;
    Ok(())
}
