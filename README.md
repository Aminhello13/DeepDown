# DeepDown — Universal Download Source Aggregator

> **Search. Discover. Download.** One engine, hundreds of sources, zero ads.

<p align="center">
  <b>🇬🇧 English</b> |
  <a href="#%D9%81%D8%A7%D8%B1%D8%B3%DB%8C">🇮🇷 فارسی</a> |
  <a href="#español">🇪🇸 Español</a> |
  <a href="#français">🇫🇷 Français</a> |
  <a href="#中文">🇨🇳 中文</a>
</p>

---

## 🇬🇧 English

### What is DeepDown?

DeepDown is an **open-source download engine** that aggregates downloadable content from hundreds of source sites worldwide. Instead of browsing cluttered, ad-heavy download sites, users search once inside DeepDown and get **direct download links** from a community-maintained registry.

### The Problem We Solve

| Pain Point | Our Solution |
|------------|-------------|
| Hours wasted searching dozens of sites | **One search, all sources** |
| Malware, fake download buttons, aggressive ads | **Direct links only — zero ads** |
| Slow internet? Downloads fail and restart | **Resume-capable, segmented downloads down to 200 KB/s** |
| Some sites blocked in your country | **User IP routing — you access sites directly** |
| Can't find rare/old files | **Aggregation across global registrants** |

### How It Works

```
User search "Kali Linux latest"
        │
        ▼
 ┌──────────────────────────────────┐
 │  Registered Sources (YAML)       │
 │  ├ Kali Official   ── search     │
 │  ├ Ubuntu Release  ── search     │
 │  ├ SourceForge     ── search     │
 │  ├ Internet Archive── search     │
 │  └ ...200+ sources               │
 └──────────────────────────────────┘
        │
        ▼
 Parallel HTTP queries (each with user's own IP)
        │
        ▼
 Direct download URL extraction (regex / XPath / CSS)
        │
        ▼
 User picks version → Download Manager (resume + segmented)
```

### Registry Structure

Each site is defined in YAML — simple, human-readable, easy to contribute:

```yaml
name: Kali Linux Official
base_url: https://cdimage.kali.org
category: linux-distro
region: global
search:
  path: /kali-latest/
  method: GET
extractor:
  type: regex
  pattern: 'href="(kali-linux-[\\d.]+-installer-[^\"]+\\.iso)"'
```

→ **[Full Schema](registry/schema.yml)**

---

## 🇮🇷 فارسی

### DeepDown چیست؟

**DeepDown یک موتور جستجوی دانلود متن‌باز** است که محتوای قابل دانلود را از صدها سایت جهانی جمع‌آوری می‌کند. کاربر به جای گشتن در ده‌ها سایت پر از تبلیغات، یک جستجو در DeepDown انجام می‌دهد و **لینک دانلود مستقیم** دریافت می‌کند.

### چرا DeepDown؟

- نیازی به ساعت‌ها گشتن در سایت‌های مختلف مثل سافت‌98، یاس دانلود و... نیست
- لینک مستقیم — بدون تبلیغ، بدون صفحات واسطه
- قابل استفاده در شرایط اینترنت ضعیف (حتی ۲۰۰KB/s)
- تمام لینک‌ها از IP کاربر مستقیم استخراج می‌شود — از فیلترینگ عبور می‌کند
- یکی از بخش‌های پروژه **دارکسوان (DarXone Group)**

### مدل تجاری

| طرح | قیمت | ظرفیت |
|-----|------|--------|
| آزمایشی ۷ روزه | رایگان | ۱۵ جستجو + ۳۰ دانلود |
| ماهانه | $1.99 | نامحدود |
| پرداخت‌به‌ازای‌دانلود | $0.99 | ۱۰۰ دانلود |

---

## 🇪🇸 Español

### ¿Qué es DeepDown?

DeepDown es un **motor de búsqueda de descargas de código abierto**que agrega contenido descargable de cientos de fuentes mundiales. El usuario busca lo que necesita y obtiene **enlaces de descarga directos** sin anuncios, sin páginas intermedias.

**DarXone Group** — herramientas para la comunidad cibernética global.

---

## 🇫🇷 Français

### Qu'est-ce que DeepDown ?

DeepDown est un **moteur de recherche de téléchargement open source** qui agrège le contenu téléchargeable de centaines de sources. Une seule recherche et vous obtenez des **liens de téléchargement directs**, sans publicité.

Inclus dans la suite **DarXone Group** — outils pour la communauté cyber.

---

## 🇨🇳 中文

### DeepDown 是什么？

DeepDown 是一个**开源下载聚合引擎**，从全球数百个资源站聚合可下载内容。用户只需搜索一次，即可获得**直接下载链接**——无广告，无跳转，无混乱。

来自 **DarXone Group** — 为全球网络社区打造的免费工具。

---

## Tech Stack

| Layer | Technology | Language |
|-------|------------|----------|
| Core Engine | Rust (Tokio, reqwest, scraper) | Rust |
| Desktop UI | Tauri v2 + React + Tailwind | TypeScript |
| Mobile | React Native (Android first) | TypeScript |
| Registry | YAML on GitHub | YAML |
| Theme | Deep Purple-Gray (#7C3AED → #1a1b2f) | CSS |

## The DarXone Group

DeepDown is the first public product from **DarXone Group**, a collective building tools for the global cyber community — open, free, and without government interference.

- **Products:** DeepDown, MTProto Proxy Worker, upcoming: DeepVPN, DeepChat
- **Philosophy:** Distributed & permissionless. No centralized servers, no data collection
- **Business:** Freemium core, premium features for sustainable development

---

## Roadmap

| Phase | Date | Milestone |
|-------|------|-----------|
| v0.1 Alpha | Released | Core engine + 5-source registry |
| v0.5 Beta | Q4 2026 | 200+ source registry, Windows CLI |
| v1.0 Desktop | Q1 2027 | Tauri GUI (Win/Linux/macOS) |
| v1.0 Mobile | Q2 2027 | Android release |
| v2.0 | Q4 2027 | Smart discovery (crawl new sources), crowd-sourced ratings |

---

## Contribute

→ [SECURITY.md](SECURITY.md)  
→ [CONTRIBUTING.md](CONTRIBUTING.md)  
→ [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)

**No private user data collected. No dependency on centralized servers. No misuse of freedom.**

---

<p align="center"><b>DeepDown</b> — a DarXone Group product. Built for the cyber community. Forever free.</p>