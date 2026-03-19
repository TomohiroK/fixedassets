# FixedAssets - Fixed Asset Management System

ASEAN向け固定資産管理システム。モバイルファースト設計。

## Tech Stack

- **Rust / WebAssembly** (Leptos 0.7 CSR)
- **Tailwind CSS** (mobile-first)
- **IndexedDB** (client-side storage)
- **Trunk** (build tool)
- **Vercel** (deployment)

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

### Option 1: Vercel CLI

```bash
# Install Vercel CLI
npm i -g vercel

# Build locally first
trunk build --release

# Deploy the dist directory
vercel --prod
```

### Option 2: GitHub Integration

1. Push this repository to GitHub
2. Import the project on [vercel.com/new](https://vercel.com/new)
3. Configure build settings:
   - **Build Command**: `trunk build --release`
   - **Output Directory**: `dist`
   - **Install Command**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && . $HOME/.cargo/env && rustup target add wasm32-unknown-unknown && cargo install trunk`
4. Deploy

> **Note**: Vercel's default build environment does not include Rust. The install command above sets up the full Rust toolchain during build. Build times will be longer on the first deploy (~3-5 min) but are cached afterward.

### Vercel Configuration

`vercel.json` is pre-configured with:
- **SPA rewrites**: All routes redirect to `index.html` for client-side routing
- **WASM headers**: Correct `Content-Type: application/wasm` with immutable cache

### Option 3: GitHub Actions + Vercel

For faster CI builds, create `.github/workflows/deploy.yml`:

```yaml
name: Deploy to Vercel
on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      - name: Install Trunk
        run: cargo install trunk
      - name: Build
        run: trunk build --release
      - name: Deploy to Vercel
        uses: amondnet/vercel-action@v25
        with:
          vercel-token: ${{ secrets.VERCEL_TOKEN }}
          vercel-org-id: ${{ secrets.VERCEL_ORG_ID }}
          vercel-project-id: ${{ secrets.VERCEL_PROJECT_ID }}
          working-directory: ./dist
          vercel-args: --prod
```

Required secrets: `VERCEL_TOKEN`, `VERCEL_ORG_ID`, `VERCEL_PROJECT_ID` (obtain from Vercel dashboard).

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
