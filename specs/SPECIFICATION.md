# DeepDown — Specification v1.0

## Goals & Non-Goals

### Goals:
- Universal download link aggregation from registered sources
- Client-side extraction — no server proxy needed
- Resume & segmented download for poor connections
- Cross-platform (desktop + mobile)
- Crowdsourced source registry

### Non-Goals:
- Does NOT store, upload, or redistribute files
- Does NOT copy content — only extracts URLs from public pages
- Does NOT replace browser download managers
- Does NOT crawl sites generically — only registered sources

## Feature Mapping

| Feature | v0.1 (Alpha) | v1.0 (Desktop) | v1.0 (Mobile) | v2.0 |
|---------|-------------|----------------|---------------|------|
| Registry loading | YAML local | YAML + GitHub fetch | same | auto-update |
| Search (parallel) | 5 sources | 200+ sources | same | 500+ |
| Download (resume) | ✅ | ✅ + segmented | same | HLS/MPEG-DASH |
| Health check | Manual | Daily cron | same | Auto-report |
| Proxy support | ❌ | SOCKS5/HTTP | same | VPN auto-detect |
| License/Trial | ❌ | 7-day free + $1.99/m | same | TON payment |
| Cross-platform | Cargo build | CI builds | Expo | CI builds |
| Dark/light theme | ✅ Purple-Gray | same | same | 5 themes |
| Extractor types | Regex | Regex + XPath + CSS | same | ML auto-detect |
| Regional filtering | ❌ | ❌ | ✅ | Global CDN |

## Architecture Principles

1. **Peer-to-search:** Each user connects directly to source sites with own IP
2. **YAML-first:** Registry is human-editible YAML, stored on GitHub
3. **Offline resilience:** Last known registry cache works for 7 days
4. **Zero analytics:** No user data collected. Stats are optional and opt-in
5. **Minimal binaries:** <2MB desktop app, <5MB mobile app (React Native)