# DeepDown Architecture

## Overview

DeepDown is a **peer-to-search** download engine — no central server processes search queries. Each user's client connects **directly** to source sites using their own IP address.

```
┌────────────────────────────────────────────────┐
│                  DeepDown Desktop/Mobile         │
│  ┌──────────┐  ┌──────────┐  ┌──────────────┐ │
│  │ Registry │  │ Search   │  │ Download Mgr   │ │
│  │ Loader   │  │ Engine   │  │ (resume/seg)   │ │
│  └────┬─────┘  └────┬─────┘  └──────┬───────┘ │
│       │              │               │          │
└───────┼──────────────┼───────────────┼──────────┘
        │              │               │
        ▼              ▼               ▼
   GitHub API    Source Sites    Local Filesystem
   (YAML files)  (Direct HTTP)   (Downloads)
```
## Smart Ranking Algorithm (Non-Destructive)

Unlike traditional aggregators that strictly filter out results based on geo-blocking, DeepDown uses a **Scoring and Ranking system**:
1. **Local-First Priority (+100 pts):** Sources hosted in the user's detected region are bubbled to the top for maximum download speed and to bypass strict international throttling.
2. **Global Availability (+50 pts):** Global, unmetered sources rank second.
3. **Freemium Sorting (+30/-10 pts):** Completely free sources get a boost. Sub/Paid sources are penalized but **never hidden**.

This ensures users in highly restricted network environments (like IR, CN) instantly see the most accessible links, while still having full access to global paid/premium alternatives.

## Core Modules
### 1. Registry Loader (`registry.rs`)

### 1. Registry Loader (`registry.rs`)
Loads YAML-defined source sites from:
- Local cache (`~/.deepdown/registry/`)
- GitHub repository (`github.com/Aminhello13/DeepDown/registry/`)

Protocol: HTTPS with ETag caching → only re-downloads when registry changes.

### 2. Search Engine (`search.rs`)
Given a query:
1. Filters eligible sites (category, region, language)
2. Executes parallel HTTP requests using Tokio async runtime
3. Extracts download links using site-specific patterns (regex, XPath, or CSS selectors)
4. Returns structured results sorted by relevance & download metrics

Optimization:
- Connection pooling per domain (keep-alive)
- Timeout per-site: 15s default, 30s for slow regions
- Results streamed to UI as they arrive (not batch)

### 3. Download Manager (`download.rs`)
- **Resume:** Range-based (HTTP Range headers)
- **Segmented:** Parallel chunks (4-8 segments)
- **Low-bandwidth mode:** 8KB chunk size, retry×3, timeout 60s
- **Integrity:** Checksum validation (SHA256)

### 4. Health Checker (`health.rs`)
Daily cron:
- HTTP HEAD request to each source base_url
- Tracks: alive/redirected/dead
- Reports to registry maintainers

### 5. License Manager (v1.1+)
- Encryption: AES-256-GCM
- Key binding: Hardware ID (MAC + CPU serial)
- Activation: JWT-based offline ticket via Telegram bot

## Data Flow

```
1. App starts → load registry cache (or fetch GitHub)
2. User types query → filter sites → parallel HTTP to each
3. HTML/JSON responses → extract (regex/XPath/CSS)
4. Results displayed → user clicks download
5. Download → resume check → segments → merge → verify
6. Peer metrics → (anon, opt-in) success rate, speed, site quality
```

## Security Model

**No centralized server.** The only "backend" is:
1. GitHub API (read-only public repo)
2. User's own IP for HTTP requests
3. License server: keyGen server (minimal, low-traffic)

**Privacy:** No user data collected. Source EXTRA parameters are**unsigned.**

## Repository Split

| Repo | Visibility | Contents |
|------|------------|----------|
| `Aminhello13/DeepDown` | **Public** | Registry files, docs, examples |
| DarXone-private | **Private** | Core engine, build systems, license code |