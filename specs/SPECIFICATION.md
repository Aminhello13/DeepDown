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

## Licensing & Anti-Crack System (TON Integration)

To maintain sustainability while respecting user privacy, DeepDown employs a **database-free, crypto-native licensing model**:

1. **Hardware Binding & Trial Enforcement (HWID):** Upon first launch, the core Rust engine generates a unique anonymous hardware hash (combining MAC address, CPU ID, and Motherboard Serial) (e.g., `DX-8A9F-2B1C`). This HWID is used locally to encrypt a hidden registry/state file that enforces the 7-day trial. Because it is tied to immutable hardware, simply uninstalling and reinstalling the app (or clearing caches) will *not* reset the trial.
2. **TON Blockchain Payment:** The user pays the $1.99 monthly fee via the TON network (Telegram Wallet/Tonkeeper) attaching their HWID as a memo.
3. **Offline JWT Token:** Once the blockchain confirms the transaction, an automated bot generates an AES-256 signed JWT token.
4. **Activation:** The user enters the token into the app. The core engine verifies the signature and matches the hardware ID.

### Why this is Anti-Crack?
- The license token is mathematically bound to the specific machine. Sharing the token with another user will fail the local HWID verification.
- The trial state is encrypted using the HWID as the key. Modifying the state file corrupts it, instantly expiring the trial.
- The verification logic is compiled directly into the Rust binary, stripped of symbols, and utilizing LTO (Link Time Optimization), making reverse engineering and patching extremely difficult.
- No centralized authentication servers are constantly pinged, meaning there are no auth endpoints for crackers to spoof or block via `hosts` files.

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