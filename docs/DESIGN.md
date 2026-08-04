# DeepDown - Technical Design Document

## System Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| OS | Windows 10 / Ubuntu 20.04 / Android 8 | Windows 11 / Ubuntu 24.04 / Android 14 |
| RAM | 512 MB | 2 GB |
| Disk | 50 MB (app only) | 100 MB (+ cache) |
| Network | 200 Kbps sustained | 1 Mbps+ |
| Runtime | C++ Redistributable (MSVC) | - |

## Multi-Platform Support

### Windows
- **Target:** `.exe` (standalone, no installer required)
- **Compiler:** MSVC 2022
- **Packaging:** Tauri bundler → MSI/NSIS
- **Bench:** 1.7MB binary (Rust, LTO-optimized)

### Linux
- **Target:** `.AppImage` / `.deb`
- **Compiler:** GCC (via Rust GNU toolchain)
- **Dependencies:** GTK3, WebKit2GTK (Tauri)
- **Bench:** 2.1MB binary

### macOS
- **Target:** `.dmg` / `.app`
- **Compiler:** Apple Clang (via Rust macOS toolchain)
- **Notarization:** Apple Developer ID (planned)

### Android
- **Target:** `.apk` (AAB via Play Store)
- **Framework:** React Native (Expo)
- **Native module:** Kotlin bridge for download manager
- **Min SDK:** 26 (Android 8.0)

## Build Pipeline

```yaml
# GitHub Actions matrix build
jobs:
  build:
    strategy:
      matrix:
        os: [windows-latest, ubuntu-22.04, macos-latest]
        include:
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            binary: deepx.exe
          - os: ubuntu-22.04
            target: x86_64-unknown-linux-gnu
            binary: deepx-linux
          - os: macos-latest
            target: x86_64-apple-darwin
            binary: deepx-mac
```

## Performance Benchmarks

| Metric | Value | Notes |
|--------|-------|-------|
| Search (50 sources) | 8.2s (avg) | Parallel, 10s timeout per site |
| Extraction (HTML) | 0.3s/site | Regex mode |
| Download (1GB, resume) | 4× speed | Segmented download |
| Memory (idle) | 45 MB | Rust allocator |
| Binary size | 1.7 MB | LTO + strip |

## Registry Specification

→ See [Registry Schema](../registry/schema.yml) for the authoritative definition.

Key design choices:
1. **YAML over JSON** — human-editable, comments allowed
2. **Each site = one file** — easy to add/remove via PR
3. **Extractor = regex (default)** or XPath for complex sites
4. **Region tagging** — clients can filter by geo-availability

## Test Conventions

### Unit Tests:
- Each Rust module has `#[cfg(test)]` tests
- Registry parser: 100% YAML test coverage
- Rate limiting: simulate connection failures, timeouts

### Integration Tests:
- Round-trip: interpret search query → try all 5 sample sites → verify results
- Health check: validate site list daily via cron job

### Cross-Platform Tests:
- GitHub Actions CI matrix (see above)
- Headless browser tests for UI (Playwright)