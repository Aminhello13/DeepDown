# DeepDown — Universal Download Source Aggregator

> **Search. Discover. Download.** One app, hundreds of sources, zero ads.

DeepDown is an open-source download engine that aggregates downloadable content from hundreds of source sites worldwide. Instead of browsing cluttered, ad-heavy download sites, users search for what they need inside DeepDown and get **direct download links** from registries, managed by the community.

---

## How It Works

```
User Input Query → Core Engine → Parallel Searches in Registry → Extract Direct Links → Download Manager
```

| Step | Description |
|------|-------------|
| 1. User Search | Type a query (e.g. "Kali Linux latest") |
| 2. Engine reads | Fetches registry from `github.com/darxone/site-registry` |
| 3. Parallel HTTP | Simultaneously queries each matching site structure |
| 4. Extraction | Regex/XPath extracts direct download URLs |
| 5. Download | Get direct link, resume-capable download manager |

## Registry Format

Each site is defined as a YAML file under `registry/sites/`:

```yaml
name: Kali Linux Official
base_url: https://cdimage.kali.org
category: linux-distro
region: global
search:
  path: /kali-{version}/
  query_param: q
extractor:
  type: regex
  pattern: 'href="(kali-linux-[\d.]+-installer-.*?\.iso)"'
```

## Tech Stack

| Component | Technology |
|-----------|------------|
| Core Engine | Rust (Tokio, reqwest, rayon) |
| Desktop UI | Tauri v2 + React + Tailwind CSS |
| Mobile | React Native (Android first) |
| Registry | GitHub (YAML-based) |
| Theme | Purple-Gray (#7C3AED + #1a1b2f) |

## License

DeepDown — trial-based (7 days free), $1.99/month. Project source managed under DarXone Group.
