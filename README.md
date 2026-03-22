# FixedAssets - Fixed Asset Management System

ASEAN 11ヶ国 + 日本対応の固定資産管理システム。モバイルファースト設計のWebアプリケーション。

**Production URL**: https://fixedassets.vercel.app

## Tech Stack

- **Rust / WebAssembly** — Leptos 0.7 CSR (Client-Side Rendering)
- **Tailwind CSS** — mobile-first responsive design
- **IndexedDB v2** — client-side asset & photo storage
- **localStorage** — company setup, departments, user sessions
- **Trunk** — WASM build tool
- **Vercel** — static hosting with Edge Middleware (Basic Auth)

## Features

### Asset Management

- **CRUD操作** — 資産の登録・閲覧・編集・削除
- **一括登録** — 数量指定で同一資産を連番付きで一括作成（例: FA-001〜FA-005）
- **15カテゴリ** — 土地、建物、建物附属設備、構築物、機械装置、工具器具備品、車両運搬具、リース資産、建設仮勘定、特許権、商標権、借地権、ソフトウェア、施設利用権、その他
- **写真管理** — 複数写真のアップロード、WebP自動圧縮、サムネイル生成
- **検索・フィルタ** — 名称、資産番号、場所、タグによる全文検索 + カテゴリフィルタ
- **タグ管理** — 資産へのタグ付け（最大50個）

### Depreciation (減価償却)

#### 対応する償却方法

| 方法 | 対応国 |
|------|--------|
| 定額法 (Straight-Line) | 全国共通 |
| 200%定率法 (200% DB + 保証率切替) | 日本 |
| 200%定率法 (Double Declining Balance) | タイ、フィリピン、ラオス、カンボジア |
| 150%/200%定率法 | ベトナム（耐用年数≤4年: 150%、>4年: 200%） |
| グループ別償却率 (PMK-72/2023) | インドネシア |
| Capital Allowance (IA + AA) | シンガポール、マレーシア |
| 定額法のみ | ミャンマー |

#### IFRS デュアルブック

- **ローカル/IFRS切替** — トグルスイッチで税務基準とIFRS基準の表示を切り替え
- **IFRS個別パラメータ** — 資産ごとにIFRS用の耐用年数・残存価額・償却方法を設定可能
- **IFRS未設定の自動スキップ** — IFRS設定がない資産はIFRSモードの償却処理でスキップ（ステータス表示）
- **独立した償却実績** — ローカルとIFRSで別々の月次償却履歴を管理
- **IFRS償却計算** — 国別税務ルールを適用しない純粋なSL/DB計算
- **資産詳細連動** — 詳細画面の帳簿価額・償却累計額・スケジュールもトグルで切り替え

#### 月次償却処理

- **3つの処理範囲** — 一括処理、カテゴリ別、個別資産
- **カレンダーUI** — 年月選択で処理対象月を指定
- **プレビュー** — 処理前に対象資産数・金額を確認
- **3つのアクション** — 当月実行、当月/前月取消、全取消
- **重複防止** — 同月の二重処理を自動検出
- **処理済み判定** — 全資産処理済みの場合は実行ボタン無効化

#### 償却集計（サマリー）

- **カテゴリ別集計** — 取得価額・償却累計額・帳簿価額をカテゴリごとに表示
- **時点指定** — カレンダーで任意の年月を選択し、その時点での累計額を表示
- **進捗バー** — カテゴリごとの償却進捗率を視覚的に表示
- **Local/IFRS対応** — トグルに連動してそれぞれの集計を表示

### Asset Lifecycle (資産ライフサイクル)

- **減損処理** — 個別の減損イベントを記録（日付・金額・理由）、累計減損額を自動計算
- **資本的支出 (CapEx)** — 改良・追加投資の記録、取得価額に加算して償却基礎に反映
- **除却・売却** — 除却（通常/災害/盗難）と売却を区分管理、売却収入・理由の記録
- **建設仮勘定 (CIP)** — 建設中資産の管理、完成時に本勘定へ振替
- **部門間移動** — 資産の配置転換履歴を管理（異動日・理由・移動元/先部門）

### Department Management (部門管理)

- **部門マスタ** — 部門コード・部門名の登録・編集・削除
- **資産割当** — 資産登録時に部門を選択
- **インポート連携** — CSV/JSONインポート時に部門コード/名称で自動マッチング

### Import / Export

| 形式 | インポート | エクスポート | テンプレート |
|------|:---:|:---:|:---:|
| CSV (19列) | ✅ | ✅ | ✅ |
| JSON | ✅ | ✅ | ✅ |

- **CSV 19列**: 資産番号, 名称, カテゴリ, 取得日, 取得価額, 残存価額, 耐用年数, 償却方法, 場所, 説明, 既償却年数, 既償却月数, ステータス, タグ, 部門, 数量, IFRS耐用年数, IFRS残存価額, IFRS償却方法
- **IFRS列は任意** — 空欄ならローカルのみ、値を入力すればIFRS設定も一括登録
- **一括数量対応** — CSV1行で数量指定 → 連番付き複数資産を自動生成
- **バリデーション** — ファイルサイズ上限5MB、最大10,000件、行単位のエラーメッセージ

### Multi-Country Support (12ヶ国対応)

| 国 | 通貨 | 償却方式 |
|----|------|----------|
| 日本 | JPY | 定額法 / 200%DB(保証率) |
| シンガポール | SGD | Capital Allowance (IA+AA) |
| マレーシア | MYR | Capital Allowance (IA+AA) |
| タイ | THB | SL / 200% DDB |
| インドネシア | IDR | グループ別税率 |
| フィリピン | PHP | SL / 200% DDB |
| ベトナム | VND | SL / 150-200% DB |
| ミャンマー | MMK | SL のみ |
| カンボジア | KHR | SL / DDB |
| ラオス | LAK | SL / DDB |
| ブルネイ | BND | SL / DDB |

追加通貨: USD, CNY も選択可能

### Authentication & Multi-Tenancy

- **パスワードハッシュ** — SHA-256 + ランダムソルト（平文保存なし）
- **レート制限** — ログイン5回失敗で15分間ロックアウト
- **セッション管理** — 30分間の非操作で自動タイムアウト
- **CSPヘッダー** — Content-Security-Policy, X-Frame-Options, HSTS
- **パスワード強度** — 8文字以上、大文字・小文字・数字を要求
- **マルチテナント** — company_id による完全なデータ分離
- **プラン管理** — 無料プラン（資産5件まで・部門1つまで）/ 有料プラン（無制限）

### Data Management

- **DATA_VERSION** — アプリバージョンを上げてデプロイするだけで全クライアントのデータを自動リセット
- **データリセット** — 設定画面から全データクリア（確認ダイアログ付き）
- **国変更** — 設定画面から国・通貨を変更（データリセットを伴う）

## Demo Accounts

| Name | Email | Password |
|------|-------|----------|
| Demo User | demo@example.com | Demo1234 |
| Admin | admin@example.com | Admin1234 |
| 田中太郎 | tanaka@example.com | Tanaka1234 |

Admin Panel: `/admin`（admin@example.com のパスワードが必要）

> Demo accounts are seeded only when no accounts exist in localStorage.

## Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk
```

## Local Development

```bash
trunk serve --port 8080
```

Open http://localhost:8080. Trunk watches `src/`, `locales/`, `index.html`, `input.css` for changes.

## Production Build

```bash
trunk build --release
```

Output: `dist/` directory. Release profile applies LTO and `opt-level = "z"`.

## Deploy to Vercel

```bash
# Install Vercel CLI
npm i -g vercel

# Login and link
vercel login
vercel link

# Set Basic Auth (optional)
vercel env add BASIC_AUTH_USER
vercel env add BASIC_AUTH_PASS

# Deploy
./scripts/deploy.sh
```

Basic認証はVercel環境変数で管理。未設定の場合はスキップされます。

## Project Structure

```
fixedassets/
├── Cargo.toml              # Rust dependencies
├── Trunk.toml              # Trunk build config
├── index.html              # Entry point
├── input.css               # Tailwind CSS source
├── tailwind.config.js      # Tailwind config
├── vercel.json             # Vercel config
├── sample_import.csv       # Sample CSV import file (19 columns, IFRS included)
├── locales/
│   ├── en.json             # English translations
│   └── ja.json             # Japanese translations
├── docs/rules/             # Country-specific depreciation rules
└── src/
    ├── main.rs             # Entry point
    ├── app.rs              # Root component + data version check + AccountingStandard context
    ├── router.rs           # Client-side routing (12 routes)
    ├── auth.rs             # Authentication (SHA-256, rate limiting)
    ├── i18n.rs             # i18n (EN/JA)
    ├── models/
    │   ├── asset.rs        # Asset, DepreciationPosting, ImpairmentRecord, CapExRecord, TransferRecord, IFRS fields
    │   ├── depreciation.rs # Schedule calculation, monthly posting, country-specific methods, IFRS calculation
    │   ├── accounting_standard.rs # AccountingStandard enum (Local/IFRS), global signal context
    │   ├── company.rs      # CompanySetup, AseanCountry, Currency
    │   ├── country_rules.rs # Country-specific depreciation rules & rates
    │   └── department.rs   # Department master
    ├── stores/
    │   └── asset_store.rs  # IndexedDB CRUD, CSV/JSON import/export (IFRS columns), data versioning
    ├── components/
    │   ├── common.rs       # Shared UI (LoadingSpinner, ConfirmDialog, StandardToggle, format_currency)
    │   ├── asset_detail.rs # Asset detail view (Local/IFRS aware financial summary & schedule)
    │   ├── asset_form.rs   # Asset registration/edit form (with IFRS settings, department & quantity)
    │   ├── dashboard.rs    # Dashboard summary cards
    │   └── modals/         # Dispose, Sell, Impairment, CapEx, CIP Transfer, Department Transfer modals
    └── pages/
        ├── dashboard.rs    # Dashboard page
        ├── asset_list.rs   # Asset list with search & filters
        ├── asset_detail.rs # Asset detail page
        ├── asset_register.rs # Registration page
        ├── depreciation.rs # Depreciation processing & category summary (Local/IFRS dual-book)
        ├── settings.rs     # Settings, import/export, department master
        ├── setup.rs        # Initial company setup
        ├── login.rs        # Login page
        ├── signup.rs       # Signup page
        ├── admin.rs        # Admin panel
        └── terms.rs        # Terms of service
```

## i18n

English / Japanese. Language switch available in header and settings page.
