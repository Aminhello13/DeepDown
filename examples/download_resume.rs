// DeepDown Example: Resumable Download Manager (simplified)
// Shows the HTTP Range-based resume algorithm.
// Production version includes: segmentation, checksums, rate limiting.

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

/// Resume-capable download: continues from where it stopped
fn download_with_resume(url: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Path::new(output_path);

    // Check if partial download already exists
    let resume_from: u64 = if output.exists() {
        let metadata = fs::metadata(output)?;
        let existing_size = metadata.len();

        // Only attempt resume if file is large enough to contain valid data
        if existing_size > 1024 {
            println!("→ Resuming existing download ({} bytes saved)", existing_size);
            existing_size
        } else {
            // Tiny file — just redownload
            0
        }
    } else {
        0
    };

    // Build HTTP client with resume range
    let client = reqwest::blocking::Client::new();
    let mut request = client.get(url);
    if resume_from > 0 {
        request = request.header("Range", format!("bytes={}-", resume_from));
    }

    let mut response = request.send()?;

    // Get total file size
    let total_size = response.content_length().unwrap_or(0);
    let effective_size = total_size + resume_from;

    println!("→ Total size: {} bytes", effective_size);

    // Open file in append mode for resume
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output)?;

    // Read body in chunks
    let mut buffer = [0u8; 8192]; // 8KB buffer
    let mut downloaded = resume_from;

    while let &bytes_read = response.copy_to(&mut buffer)? {
        if bytes_read == 0 {
            break; // stream ended
        }
        file.write_all(&buffer[..bytes_read])?;
        downloaded += bytes_read as u64;

        // Progress display (every 0.25MB)
        if downloaded % (256 * 1024) == 0 {
            let pct = (downloaded as f64 / effective_size as f64) * 100.0;
            log::info!("  {:.1}% — {}/{} bytes", pct, downloaded, effective_size);
        }
    }

    log::info!("download complete: {} bytes", downloaded);
    Ok(())