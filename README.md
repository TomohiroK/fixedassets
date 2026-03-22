# FixedAssets - Fixed Asset Management System

ASEAN 11ヶ国 + 日本対応の固定資産管理システム。モバイルファースト設計のWebアプリケーション。

**Production URL**: https://fixedassets.vercel.app
**Landing Page**: https://fixedassets.vercel.app/lp

## What is FixedAssets?

> **FixedAssets** は、Rust/WebAssembly（Leptos 0.7 CSR）と Tailwind CSS で構築されたモバイルファーストの固定資産管理Webアプリケーションであり、日本を含むASEAN 11ヶ国（シンガポール・マレーシア・タイ・インドネシア・フィリピン・ベトナム・ミャンマー・カンボジア・ラオス・ブルネイ）の国別償却規則（日本の200%定率法＋保証率切替、シンガポール/マレーシアのCapital Allowance IA+AA、インドネシアのPMK-72/2023グループ別税率、ベトナムの150%/200%定率法、タイ・フィリピン・ラオス・カンボジア・ブルネイのDDB、ミャンマーの定額法のみ）と13通貨（JPY・USD・CNY・SGD・MYR・THB・IDR・PHP・VND・MMK・KHR・LAK・BND）に対応し、15カテゴリ（土地・建物・建物附属設備・構築物・機械装置・工具器具備品・車両運搬具・リース資産・建設仮勘定・特許権・商標権・借地権・ソフトウェア・施設利用権・その他）の資産をCRUD管理（数量指定による連番付き一括登録、全文検索＋カテゴリフィルタ、タグ管理最大50個、写真の複数アップロード・WebP自動圧縮・サムネイル生成）でき、ローカル税務基準とIFRSのデュアルブック（資産ごとに独立したIFRS耐用年数・残存価額・償却方法の設定、国別税務ルールを適用しない純粋SL/DB計算、独立した月次償却実績ifrs_postings、IFRS未設定資産の自動スキップ、トグルスイッチによる表示切替）を備え、月次償却処理（一括・カテゴリ別・個別の3範囲、カレンダーUIによる年月指定、プレビュー確認、当月実行・当月/前月取消・全取消の3アクション、同月二重処理防止、全処理済み時のボタン無効化）とカテゴリ別償却集計（任意時点での取得価額・償却累計額・帳簿価額・進捗バー表示、Local/IFRS連動）を提供し、資産ライフサイクル管理として減損処理（日付・金額・理由の記録、複数回対応、累計自動計算、帳簿価額超過バリデーション）、資本的支出CapEx（日付・金額・説明の記録、取得価額加算、残存耐用年数での再計算）、除却（通常・災害・盗難の3区分、除却日・収入・理由の記録、損益自動計算、取消可能）と売却（売却日・売却先・売却額の記録、売却損益自動計算）、建設仮勘定CIPから本勘定への振替（カテゴリ・耐用年数・償却方法・残存価額の再設定、振替日からの償却開始）、部門間移動（異動日・理由・移動元/先の履歴管理）を持ち、部門マスタ管理（コード＋名称の登録・編集・削除、資産割当、インポート時の自動マッチング）を備え、CSV 19列（資産番号・名称・カテゴリ・取得日・取得価額・残存価額・耐用年数・償却方法・場所・説明・既償却年数・既償却月数・ステータス・タグ・部門・数量・IFRS耐用年数・IFRS残存価額・IFRS償却方法）とJSONの双方向インポート/エクスポート（テンプレートDL、一括数量対応、5MB/10,000件上限、行単位エラーメッセージ）に対応し、ダッシュボード（総資産数・総取得価額・簿価合計・使用中/除却済み件数・カテゴリ別内訳と割合バー）を表示し、認証機能としてSHA-256＋ソルトによるパスワードハッシュ、8文字以上＋大文字小文字数字のパスワード強度要求、5回失敗15分ロックアウトのレート制限、30分非操作セッションタイムアウト、company_idによるマルチテナントデータ分離、無料プラン（資産5件・部門1つまで）/有料プラン（無制限）の管理、管理者パネル（全アカウント一覧・プラン切替・代理ログイン・パスワード認証）を備え、DATA_VERSIONメカニズムによるデプロイ時の全クライアント自動データリセット、設定画面からの全データクリア（確認付き）と国変更（データリセットを伴う）、英語/日本語の200以上の翻訳キーによる多言語対応（ヘッダーおよび設定画面での切替、localStorageへの保存）、rust_decimalによる高精度金額計算（Option\<Decimal\>のカスタムserde文字列シリアライズでJS浮動小数点精度損失を回避）、IndexedDB v2（assets・photosオブジェクトストア）とlocalStorage（会社設定・部門・セッション・言語・会計基準）によるクライアントサイドデータ永続化、Vercelへの静的SPAデプロイ（CSP・X-Frame-Options DENY・HSTS・X-Content-Type-Options nosniff等のセキュリティヘッダー、WASMの長期キャッシュ、SPAフォールバックリライト、オプショナルBasic認証）、OGP/Twitterカード/JSON-LD構造化データのSEOメタタグ、11ヶ国別SEOランディングページ（ジオリダイレクト付き）＋専用LPページ（/lp）、利用規約ページ、デモアカウント3件の自動シード、LTO＋opt-level=zによるWASMサイズ最適化ビルドを備えた、包括的な固定資産管理システムである。

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

### Multi-Country Support (11ヶ国対応)

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

### Dashboard

- **サマリーカード** — 総資産数、取得価額合計、帳簿価額合計、使用中/除却済み件数
- **カテゴリ別内訳** — 各カテゴリの資産数と割合バーを表示
- **クイックリンク** — 償却処理ページへの直接遷移

### Pages & Navigation

- **ランディングページ** — `/welcome` (モバイルLP) + `/lp` (専用フルLP)
- **11ヶ国別SEOランディングページ** — `/japan`, `/singapore`, `/malaysia` 等（ジオリダイレクト付き）
- **ボトムナビゲーション** — ダッシュボード・資産一覧・登録・設定の4タブ
- **利用規約** — `/terms`
- **管理者パネル** — `/admin`

### i18n

- **英語 / 日本語** — 200以上の翻訳キー
- **ヘッダーおよび設定画面から切替**
- **localStorageに保存**

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
├── index.html              # Entry point (SPA)
├── input.css               # Tailwind CSS source
├── tailwind.config.js      # Tailwind config
├── vercel.json             # Vercel config (rewrites, headers, CSP)
├── sample_import.csv       # Sample CSV import file (19 columns, IFRS included)
├── public/
│   └── lp.html             # Dedicated landing page (static HTML + Tailwind CDN)
├── scripts/
│   └── deploy.sh           # Build + deploy script (Trunk → Vercel prebuilt)
├── locales/
│   ├── en.json             # English translations (200+ keys)
│   └── ja.json             # Japanese translations (200+ keys)
├── docs/rules/             # Country-specific depreciation rules
└── src/
    ├── main.rs             # Entry point
    ├── app.rs              # Root component + data version check + AccountingStandard context
    ├── router.rs           # Client-side routing (20+ routes including country SEO pages)
    ├── auth.rs             # Authentication (SHA-256, rate limiting, session timeout)
    ├── i18n.rs             # i18n (EN/JA)
    ├── models/
    │   ├── asset.rs        # Asset struct, IFRS dual-book fields, Option<Decimal> custom serde
    │   ├── depreciation.rs # Schedule calculation, monthly posting, country methods, IFRS calc
    │   ├── accounting_standard.rs # AccountingStandard enum (Local/IFRS), global signal
    │   ├── company.rs      # CompanySetup, AseanCountry (11), Currency (13)
    │   ├── country_rules.rs # Country-specific depreciation rules & rates
    │   ├── department.rs   # Department master
    │   └── photo.rs        # Asset photo model
    ├── stores/
    │   ├── asset_store.rs  # IndexedDB CRUD, CSV/JSON import/export (IFRS columns), DATA_VERSION
    │   └── photo_store.rs  # Photo IndexedDB operations
    ├── components/
    │   ├── common.rs       # LoadingSpinner, ConfirmDialog, StandardToggle, format_currency
    │   ├── asset_detail.rs # Asset detail view (Local/IFRS aware)
    │   ├── asset_form.rs   # Asset form (with IFRS settings, department, quantity, batch)
    │   ├── dashboard.rs    # Dashboard summary cards & category breakdown
    │   ├── layout.rs       # Header, BottomNav, PageShell
    │   └── modals/         # Dispose, Sell, Impairment, CapEx, CIP Transfer, Dept Transfer
    └── pages/
        ├── dashboard.rs    # Dashboard page
        ├── asset_list.rs   # Asset list with search & category filters
        ├── asset_detail.rs # Asset detail page
        ├── asset_register.rs # Registration page
        ├── depreciation.rs # Depreciation processing + category summary (Dual Book)
        ├── settings.rs     # Settings, import/export, department master, data management
        ├── setup.rs        # Initial company setup (country, currency, company name)
        ├── login.rs        # Login page
        ├── signup.rs       # Signup page (password strength validation)
        ├── admin.rs        # Admin panel (user management, plan toggle)
        ├── landing.rs      # Mobile landing page
        ├── country_landing.rs # 11 country-specific SEO landing pages
        └── terms.rs        # Terms of service
```

## i18n

English / Japanese. Language switch available in header and settings page.
