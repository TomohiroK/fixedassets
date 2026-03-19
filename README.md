# FixedAssets - Fixed Asset Management System

ASEAN向け固定資産管理システム。モバイルファースト設計。

## Tech Stack

- **Rust / WebAssembly** (Leptos 0.7 CSR)
- **Tailwind CSS** (mobile-first)
- **IndexedDB** (client-side storage)
- **Trunk** (build tool)
- **Vercel** (deployment)

## Access

Production URL: https://fixedassets.vercel.app

Basic認証:

| | |
|---|---|
| **ID** | fx123 |
| **Password** | xckqg |

## Demo

This application is currently in **demo mode**. On first launch, the following demo accounts are automatically created:

| Name | Email | Password |
|------|-------|----------|
| Demo User | demo@example.com | demo123 |
| Admin | admin@example.com | admin123 |
| 田中太郎 | tanaka@example.com | tanaka123 |

You can also access the **Admin Panel** at `/admin` to switch between accounts without a password.

> Demo accounts are seeded only when no accounts exist in localStorage. Once you sign up your own account, the seed is skipped.

## Prerequisites

```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Trunk (WASM build tool)
cargo install trunk
```

## Local Development

```bash
# Start dev server with hot reload
trunk serve --port 8080
```

Open http://localhost:8080 in your browser. Trunk watches `src/`, `locales/`, `index.html`, `input.css` for changes and auto-rebuilds.

## Production Build

```bash
trunk build --release
```

Output is generated in the `dist/` directory. The release profile applies LTO and `opt-level = "z"` for minimal WASM binary size.

## Deploy to Vercel

Deploy script handles Trunk build, Basic Auth middleware setup, and Vercel deployment:

```bash
# Install Vercel CLI (if not installed)
npm i -g vercel

# Login and link project
vercel login
vercel link

# Deploy (build + auth middleware + deploy)
./scripts/deploy.sh
```

The deploy script (`scripts/deploy.sh`) does:
1. `trunk build --release` — WASM production build
2. Prepares Vercel Build Output API structure with Edge Middleware (Basic Auth)
3. `vercel deploy --prebuilt --prod` — deploys to production

### Basic Auth

Basic認証は Edge Middleware (`scripts/deploy.sh` 内で生成) で実装。認証情報を変更する場合はスクリプト内の `user` / `pass` を編集して再デプロイ。

## Project Structure

```
fixedassets/
├── Cargo.toml          # Rust dependencies
├── Trunk.toml          # Trunk build config
├── index.html          # Entry point (Trunk)
├── input.css           # Tailwind CSS source
├── tailwind.config.js  # Tailwind config
├── vercel.json         # Vercel deployment config
├── locales/
│   ├── en.json         # English translations
│   └── ja.json         # Japanese translations
└── src/
    ├── main.rs         # Entry point
    ├── app.rs          # Root component
    ├── router.rs       # Client-side routing
    ├── auth.rs         # Authentication (localStorage)
    ├── i18n.rs         # Internationalization
    ├── models/         # Data models (Asset, Depreciation)
    ├── stores/         # IndexedDB CRUD operations
    ├── components/     # UI components
    └── pages/          # Page components
```

## i18n

Supports English and Japanese. Language can be switched from the header or settings page. Translation files are in `locales/`.
