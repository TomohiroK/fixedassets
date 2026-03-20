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

Basic認証はVercel環境変数で管理:

| 変数名 | 説明 |
|--------|------|
| `BASIC_AUTH_USER` | Basic認証ユーザー名 |
| `BASIC_AUTH_PASS` | Basic認証パスワード |

> 環境変数未設定の場合、Basic認証はスキップされます。

## Demo

This application is currently in **demo mode**. On first launch, the following demo accounts are automatically created:

| Name | Email | Password |
|------|-------|----------|
| Demo User | demo@example.com | Demo1234 |
| Admin | admin@example.com | Admin1234 |
| 田中太郎 | tanaka@example.com | Tanaka1234 |

Admin Panel: `/admin` (admin@example.com のパスワードが必要)

> Demo accounts are seeded only when no accounts exist in localStorage. Once you sign up your own account, the seed is skipped.

> ⚠️ **旧バージョンからの移行時**: 旧パスワード（`demo123` 等）でログインできない場合は、ブラウザの DevTools → Console で `localStorage.removeItem('fa_users')` を実行してリロードしてください。新パスワードでデモアカウントが再作成されます。

### Security Features

- **Password hashing** — SHA-256 + random salt (plaintext passwords are never stored)
- **Rate limiting** — 5回のログイン失敗で15分間ロックアウト
- **Session timeout** — 30分間の非操作でセッション失効
- **CSP headers** — Content-Security-Policy, X-Frame-Options, HSTS等を設定
- **Import validation** — ファイルサイズ制限(5MB)、値の範囲チェック、件数制限(10,000件)
- **Password strength** — 8文字以上、大文字・小文字・数字を要求

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

# Set Basic Auth credentials (in Vercel dashboard or CLI)
vercel env add BASIC_AUTH_USER
vercel env add BASIC_AUTH_PASS

# Deploy (build + auth middleware + deploy)
./scripts/deploy.sh
```

The deploy script (`scripts/deploy.sh`) does:
1. `trunk build --release` — WASM production build
2. Prepares Vercel Build Output API structure with Edge Middleware (Basic Auth via env vars)
3. `vercel deploy --prebuilt --prod` — deploys to production

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
├── docs/
│   └── rules/          # Country-specific depreciation rules
└── src/
    ├── main.rs         # Entry point
    ├── app.rs          # Root component
    ├── router.rs       # Client-side routing
    ├── auth.rs         # Authentication (SHA-256 hashed, rate-limited)
    ├── i18n.rs         # Internationalization
    ├── models/         # Data models (Asset, Depreciation, CountryRules)
    ├── stores/         # IndexedDB CRUD operations
    ├── components/     # UI components
    └── pages/          # Page components
```

## i18n

Supports English and Japanese. Language can be switched from the header or settings page. Translation files are in `locales/`.
