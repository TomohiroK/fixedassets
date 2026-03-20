use leptos::prelude::*;
use crate::i18n::use_i18n;
use crate::models::company::AseanCountry;

/// Country-specific SEO landing page data
#[derive(Clone, Debug)]
pub struct CountryPageData {
    pub country: AseanCountry,
    pub slug: &'static str,
    pub headline_en: &'static str,
    pub headline_ja: &'static str,
    pub subheadline_en: &'static str,
    pub subheadline_ja: &'static str,
    pub features_en: Vec<&'static str>,
    pub features_ja: Vec<&'static str>,
    pub rules_summary_en: &'static str,
    pub rules_summary_ja: &'static str,
}

pub fn all_country_pages() -> Vec<CountryPageData> {
    vec![
        CountryPageData {
            country: AseanCountry::Japan,
            slug: "japan",
            headline_en: "Fixed Asset Management for Japan",
            headline_ja: "日本向け固定資産管理",
            subheadline_en: "Compliant with Japanese tax law depreciation rules. 200% declining balance with guarantee amount, straight-line for buildings.",
            subheadline_ja: "日本の税法に準拠した減価償却計算。200%定率法の保証率テーブル内蔵、建物は定額法。",
            features_en: vec![
                "200% DB with guarantee amount table (耐用年数2〜50年)",
                "SL for buildings/structures (post-2016 rules)",
                "Memorandum value ¥1 (備忘価額)",
                "16 asset categories per Japanese tax code",
            ],
            features_ja: vec![
                "200%定率法 + 保証率テーブル（耐用年数2〜50年）",
                "建物・構築物は定額法（2016年以降）",
                "備忘価額1円まで償却",
                "法定16分類の固定資産カテゴリ",
            ],
            rules_summary_en: "Japan uses straight-line (定額法) and 200% declining balance (定率法). Buildings acquired after 2016 must use straight-line only. Declining balance includes a guarantee amount switch to revised straight-line.",
            rules_summary_ja: "日本は定額法と200%定率法を採用。2016年以降取得の建物・構築物は定額法のみ。定率法は保証額を下回った時点で改定償却率による定額法に切替。",
        },
        CountryPageData {
            country: AseanCountry::Singapore,
            slug: "singapore",
            headline_en: "Capital Allowance Management for Singapore",
            headline_ja: "シンガポール向けキャピタルアローワンス管理",
            subheadline_en: "IRAS-compliant capital allowance calculation. Initial allowance + annual allowance system with Sixth Schedule working life.",
            subheadline_ja: "IRAS準拠のキャピタルアローワンス計算。初年度控除＋年次控除、第6附則の耐用年数。",
            features_en: vec![
                "Initial Allowance (IA) 20% + Annual Allowance (AA)",
                "Sixth Schedule prescribed working life",
                "3-year / 1-year write-off elections",
                "Low-value asset write-off (≤ SGD 5,000)",
            ],
            features_ja: vec![
                "初年度控除(IA) 20% + 年次控除(AA)",
                "第6附則に基づく法定耐用年数",
                "3年/1年一括償却の選択",
                "少額資産の即時償却（SGD 5,000以下）",
            ],
            rules_summary_en: "Singapore uses Capital Allowances (not depreciation). Year 1 gets 20% IA plus first AA. Full cost is written off with no salvage. Buildings generally do not qualify.",
            rules_summary_ja: "シンガポールは減価償却ではなくキャピタルアローワンスを採用。初年度はIA 20%＋AA。残存価額なしで全額控除。建物は原則対象外。",
        },
        CountryPageData {
            country: AseanCountry::Malaysia,
            slug: "malaysia",
            headline_en: "Capital Allowance Management for Malaysia",
            headline_ja: "マレーシア向けキャピタルアローワンス管理",
            subheadline_en: "LHDN-compliant capital allowance with category-specific IA and AA rates. ICT equipment gets accelerated 40% AA.",
            subheadline_ja: "LHDN準拠のキャピタルアローワンス。カテゴリ別のIA・AAレート、ICT機器は加速償却40%。",
            features_en: vec![
                "IA 20% + category-specific AA rates",
                "ICT/computers: 40% accelerated AA",
                "Heavy machinery & vehicles: 20% AA",
                "Vehicle cost cap (MYR 50,000/100,000)",
            ],
            features_ja: vec![
                "IA 20% + カテゴリ別AAレート",
                "ICT/コンピュータ: 加速AA 40%",
                "重機・車両: AA 20%",
                "車両取得費上限（MYR 50,000/100,000）",
            ],
            rules_summary_en: "Malaysia uses Capital Allowances with 20% initial allowance and category-based annual allowance rates. Full cost is written off. Buildings qualify only for Industrial Building Allowance at 3%.",
            rules_summary_ja: "マレーシアは初年度控除20%＋カテゴリ別年次控除のキャピタルアローワンス。全額控除、建物は工業建物控除3%のみ。",
        },
        CountryPageData {
            country: AseanCountry::Thailand,
            slug: "thailand",
            headline_en: "Fixed Asset Depreciation for Thailand",
            headline_ja: "タイ向け固定資産減価償却",
            subheadline_en: "Compliant with Thai Revenue Code. Maximum depreciation rates with vehicle cost cap at THB 1,000,000.",
            subheadline_ja: "タイ歳入法に準拠。法定最大償却率、車両取得費上限100万バーツ。",
            features_en: vec![
                "SL, DB, and SYD methods available",
                "Statutory maximum rates per category",
                "Vehicle cost cap: THB 1,000,000",
                "SME accelerated depreciation available",
            ],
            features_ja: vec![
                "定額法・定率法・級数法に対応",
                "カテゴリ別法定最大償却率",
                "車両取得費上限: 100万バーツ",
                "中小企業向け加速償却",
            ],
            rules_summary_en: "Thailand allows straight-line, declining balance, or sum-of-years-digits. The tax code prescribes maximum rates. Passenger vehicles are limited to first THB 1M of cost.",
            rules_summary_ja: "タイは定額法・定率法・級数法を選択可能。税法は最大償却率を規定。乗用車は取得原価100万バーツまでが対象。",
        },
        CountryPageData {
            country: AseanCountry::Indonesia,
            slug: "indonesia",
            headline_en: "Fixed Asset Depreciation for Indonesia",
            headline_ja: "インドネシア向け固定資産減価償却",
            subheadline_en: "PMK-72/2023 compliant 4-group depreciation system with prescribed SL and DB rates.",
            subheadline_ja: "PMK-72/2023準拠の4グループ制、法定定額法・定率法レート。",
            features_en: vec![
                "4-group system (4, 8, 16, 20 years)",
                "Group-based SL and DB rates",
                "Buildings: SL only (5% or 10%)",
                "Remaining DB book value expensed in final year",
            ],
            features_ja: vec![
                "4グループ制（4, 8, 16, 20年）",
                "グループ別の定額法・定率法レート",
                "建物: 定額法のみ（5%または10%）",
                "定率法の最終年に残簿価を一括費用化",
            ],
            rules_summary_en: "Indonesia uses a 4-group system with fixed rates per group. Non-building assets in Group 1-4, buildings at 5% (permanent) or 10% (non-permanent). DB remaining value is expensed in the final year.",
            rules_summary_ja: "インドネシアは4グループ制でグループ別固定レート。建物は定額法のみ5%/10%。定率法は最終年に残額を一括費用化。",
        },
        CountryPageData {
            country: AseanCountry::Philippines,
            slug: "philippines",
            headline_en: "Fixed Asset Depreciation for Philippines",
            headline_ja: "フィリピン向け固定資産減価償却",
            subheadline_en: "Flexible depreciation with no prescribed useful life. SL, DB, and SYD methods available.",
            subheadline_ja: "法定耐用年数なしの柔軟な償却。定額法・定率法・級数法に対応。",
            features_en: vec![
                "No prescribed useful life (most flexible)",
                "SL, DB, and SYD available",
                "Taxpayer determines reasonable life",
                "Salvage value: as agreed or 10%",
            ],
            features_ja: vec![
                "法定耐用年数なし（最も柔軟）",
                "定額法・定率法・級数法に対応",
                "合理的な耐用年数を納税者が決定",
                "残存価額: 合意額または10%",
            ],
            rules_summary_en: "The Philippines has the most flexible system—no prescribed useful life. Taxpayers choose a reasonable life and any generally accepted depreciation method.",
            rules_summary_ja: "フィリピンは最も柔軟な制度で法定耐用年数なし。納税者が合理的な耐用年数と一般に認められた償却方法を選択。",
        },
        CountryPageData {
            country: AseanCountry::Vietnam,
            slug: "vietnam",
            headline_en: "Fixed Asset Depreciation for Vietnam",
            headline_ja: "ベトナム向け固定資産減価償却",
            subheadline_en: "Circular 45/2013/TT-BTC compliant. SL, DB, and units of production methods with min-max useful life ranges.",
            subheadline_ja: "通達45/2013/TT-BTC準拠。定額法・定率法・生産高比例法、耐用年数範囲制。",
            features_en: vec![
                "SL, DB, and Units of Production methods",
                "Min-max useful life ranges per category",
                "DB: 150% (≤4yr) or 200% (>4yr) rate",
                "Salvage value: 0",
            ],
            features_ja: vec![
                "定額法・定率法・生産高比例法",
                "カテゴリ別の最短〜最長耐用年数",
                "定率法: 150%（4年以下）/200%（4年超）",
                "残存価額: 0",
            ],
            rules_summary_en: "Vietnam uses Circular 45 with min-max useful life ranges. DB rate is 150% of SL rate for assets ≤4 years, 200% for >4 years. Units of production is also allowed.",
            rules_summary_ja: "ベトナムは通達45で最短〜最長の耐用年数範囲を規定。定率法は4年以下で150%、4年超で200%。生産高比例法も可。",
        },
        CountryPageData {
            country: AseanCountry::Myanmar,
            slug: "myanmar",
            headline_en: "Fixed Asset Depreciation for Myanmar",
            headline_ja: "ミャンマー向け固定資産減価償却",
            subheadline_en: "Notification 19/2016 compliant. Straight-line method with prescribed rates.",
            subheadline_ja: "通知19/2016準拠。法定レートによる定額法。",
            features_en: vec![
                "Straight-line method only",
                "Prescribed rates per asset type",
                "Buildings: 2.5% (40 years)",
                "Vehicles: 20% (5 years)",
            ],
            features_ja: vec![
                "定額法のみ",
                "資産種別別の法定レート",
                "建物: 2.5%（40年）",
                "車両: 20%（5年）",
            ],
            rules_summary_en: "Myanmar uses straight-line depreciation only with prescribed rates per asset type from Notification 19/2016. No declining balance method is available.",
            rules_summary_ja: "ミャンマーは通知19/2016に基づく定額法のみ。資産種別ごとに法定レートが規定。定率法は不可。",
        },
        CountryPageData {
            country: AseanCountry::Cambodia,
            slug: "cambodia",
            headline_en: "Fixed Asset Depreciation for Cambodia",
            headline_ja: "カンボジア向け固定資産減価償却",
            subheadline_en: "4-class system with SL for buildings and pooled declining balance for movable assets.",
            subheadline_ja: "4クラス制。建物は定額法、動産はプール方式の定率法。",
            features_en: vec![
                "4-class system (5, 10, 20 years + buildings)",
                "Buildings: SL at 5% (20 years)",
                "Movable assets: pooled declining balance",
                "Class 2-4: DB at 40%, 20%, 10%",
            ],
            features_ja: vec![
                "4クラス制（5, 10, 20年＋建物）",
                "建物: 定額法5%（20年）",
                "動産: プール方式の定率法",
                "クラス2-4: 定率法40%, 20%, 10%",
            ],
            rules_summary_en: "Cambodia uses a 4-class system. Buildings (Class 1) use straight-line at 5%. Classes 2-4 use pooled declining balance at 40%, 20%, and 10% respectively.",
            rules_summary_ja: "カンボジアは4クラス制。建物（クラス1）は定額法5%。クラス2〜4はプール方式の定率法（40%, 20%, 10%）。",
        },
        CountryPageData {
            country: AseanCountry::Laos,
            slug: "laos",
            headline_en: "Fixed Asset Depreciation for Laos",
            headline_ja: "ラオス向け固定資産減価償却",
            subheadline_en: "SL and DDB methods with prescribed rates for each asset category.",
            subheadline_ja: "資産カテゴリ別の法定レートによる定額法・倍額定率法。",
            features_en: vec![
                "SL and DDB (double declining balance)",
                "Prescribed rates per category",
                "Buildings: 5% (20 years)",
                "Units of production also accepted",
            ],
            features_ja: vec![
                "定額法・倍額定率法(DDB)",
                "カテゴリ別法定レート",
                "建物: 5%（20年）",
                "生産高比例法も可",
            ],
            rules_summary_en: "Laos allows straight-line, double declining balance, or units of production. Prescribed rates vary by asset category. DDB is the typical DB method used.",
            rules_summary_ja: "ラオスは定額法・倍額定率法・生産高比例法を選択可能。カテゴリ別に法定レートが規定。",
        },
        CountryPageData {
            country: AseanCountry::Brunei,
            slug: "brunei",
            headline_en: "Fixed Asset Management for Brunei",
            headline_ja: "ブルネイ向け固定資産管理",
            subheadline_en: "Capital allowance system similar to Singapore and Malaysia.",
            subheadline_ja: "シンガポール・マレーシアに類似のキャピタルアローワンス制度。",
            features_en: vec![
                "Capital allowance system",
                "No corporate income tax (unique advantage)",
                "SL-based allowances",
                "Simplified asset tracking",
            ],
            features_ja: vec![
                "キャピタルアローワンス制度",
                "法人所得税なし（独自の利点）",
                "定額法ベースの控除",
                "簡易的な資産管理",
            ],
            rules_summary_en: "Brunei has no corporate income tax for most businesses, but capital allowances apply for petroleum sector. Asset tracking is primarily for management purposes.",
            rules_summary_ja: "ブルネイは大半の事業で法人所得税なし。石油関連にキャピタルアローワンス適用。資産管理は主に経営目的。",
        },
    ]
}

/// Get country data by slug
pub fn get_country_by_slug(slug: &str) -> Option<CountryPageData> {
    all_country_pages().into_iter().find(|c| c.slug == slug)
}

/// Country-specific landing page component
#[component]
pub fn CountryLandingPage(
    #[prop(into)] country_slug: String,
) -> impl IntoView {
    let i18n = use_i18n();
    let data = get_country_by_slug(&country_slug);

    match data {
        Some(page) => {
            let flag = page.country.flag().to_string();
            let country_name_en = page.country.name_en().to_string();
            let country_name_ja = page.country.name_ja().to_string();
            let headline_en = page.headline_en.to_string();
            let headline_ja = page.headline_ja.to_string();
            let sub_en = page.subheadline_en.to_string();
            let sub_ja = page.subheadline_ja.to_string();
            let features_en = page.features_en.iter().map(|s| s.to_string()).collect::<Vec<_>>();
            let features_ja = page.features_ja.iter().map(|s| s.to_string()).collect::<Vec<_>>();
            let rules_en = page.rules_summary_en.to_string();
            let rules_ja = page.rules_summary_ja.to_string();

            // All other country links for SEO internal linking
            let other_countries: Vec<(&'static str, String, String)> = all_country_pages()
                .into_iter()
                .filter(|c| c.slug != country_slug)
                .map(|c| {
                    let flag = c.country.flag().to_string();
                    let name = c.country.name_en().to_string();
                    (c.slug, flag, name)
                })
                .collect();

            // A/B test hero image
            let bg_image = if js_sys::Math::random() < 0.5 {
                "images/hero-1.webp"
            } else {
                "images/hero-2.webp"
            };
            let bg_style = format!(
                "background-image: url('/{}'); background-size: cover; background-position: center;",
                bg_image
            );

            view! {
                <div class="min-h-screen">
                    // Hero section with background
                    <div class="relative text-white" style=bg_style>
                        <div class="absolute inset-0 bg-black/55"></div>
                        <div class="relative z-10 px-6 py-8">
                            <div class="max-w-lg mx-auto">
                                // Header
                                <div class="flex items-center justify-between mb-8">
                                    <a href="/welcome" class="text-2xl font-bold tracking-tight drop-shadow-lg">{move || i18n.t("app.title")}</a>
                                    <button
                                        class="text-sm bg-white/20 backdrop-blur-sm px-3 py-1.5 rounded-lg"
                                        on:click=move |_| {
                                            let current = i18n.current_locale();
                                            let next = if current == "en" { "ja" } else { "en" };
                                            i18n.set_locale(next);
                                        }
                                    >
                                        {move || if i18n.current_locale() == "en" { "日本語" } else { "English" }}
                                    </button>
                                </div>

                                // Country flag + headline
                                <div class="text-4xl mb-3">{flag}</div>
                                <h1 class="text-2xl font-bold leading-tight mb-2 drop-shadow-lg">
                                    {let h_en = headline_en.clone(); let h_ja = headline_ja.clone();
                                     move || if i18n.current_locale() == "en" { h_en.clone() } else { h_ja.clone() }}
                                </h1>
                                <p class="text-white/80 text-sm leading-relaxed mb-6">
                                    {let s_en = sub_en.clone(); let s_ja = sub_ja.clone();
                                     move || if i18n.current_locale() == "en" { s_en.clone() } else { s_ja.clone() }}
                                </p>

                                // CTA
                                <div class="flex gap-3">
                                    <a href="/signup" class="flex-1 py-3 bg-blue-600 hover:bg-blue-500 text-white rounded-xl font-bold text-center shadow-lg active:bg-blue-700 transition-colors">
                                        {move || i18n.t("landing.get_started")}
                                    </a>
                                    <a href="/login" class="flex-1 py-3 bg-white/15 backdrop-blur-sm text-white rounded-xl font-bold text-center active:bg-white/25 transition-colors border border-white/30">
                                        {move || i18n.t("landing.sign_in")}
                                    </a>
                                </div>
                            </div>
                        </div>
                    </div>

                    // Features section (white bg)
                    <div class="bg-white px-6 py-8">
                        <div class="max-w-lg mx-auto">
                            <h2 class="text-lg font-bold text-gray-900 mb-4">
                                {let cn_en = country_name_en.clone(); let cn_ja = country_name_ja.clone();
                                 move || if i18n.current_locale() == "en" {
                                    format!("Key Features for {}", cn_en)
                                 } else {
                                    format!("{}向け主要機能", cn_ja)
                                 }}
                            </h2>
                            <ul class="space-y-3">
                                {features_en.iter().zip(features_ja.iter()).map(|(en, ja)| {
                                    let en = en.clone();
                                    let ja = ja.clone();
                                    view! {
                                        <li class="flex items-start gap-3">
                                            <svg class="w-5 h-5 text-blue-600 shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                            </svg>
                                            <span class="text-sm text-gray-700">
                                                {move || if i18n.current_locale() == "en" { en.clone() } else { ja.clone() }}
                                            </span>
                                        </li>
                                    }
                                }).collect::<Vec<_>>()}
                            </ul>
                        </div>
                    </div>

                    // Depreciation rules summary
                    <div class="bg-gray-50 px-6 py-8">
                        <div class="max-w-lg mx-auto">
                            <h2 class="text-lg font-bold text-gray-900 mb-3">
                                {move || if i18n.current_locale() == "en" { "Depreciation Rules" } else { "減価償却ルール" }}
                            </h2>
                            <div class="bg-white rounded-xl p-4 border border-gray-200">
                                <p class="text-sm text-gray-600 leading-relaxed">
                                    {let r_en = rules_en.clone(); let r_ja = rules_ja.clone();
                                     move || if i18n.current_locale() == "en" { r_en.clone() } else { r_ja.clone() }}
                                </p>
                            </div>
                        </div>
                    </div>

                    // Other countries (internal linking for SEO)
                    <div class="bg-white px-6 py-8 border-t border-gray-100">
                        <div class="max-w-lg mx-auto">
                            <h3 class="text-sm font-semibold text-gray-500 uppercase tracking-wide mb-3">
                                {move || if i18n.current_locale() == "en" { "Other Countries" } else { "他の国" }}
                            </h3>
                            <div class="grid grid-cols-3 gap-2">
                                {other_countries.iter().map(|(slug, flag, name)| {
                                    let href = format!("/{}", slug);
                                    let label = format!("{} {}", flag, name);
                                    view! {
                                        <a href=href class="text-xs text-blue-600 hover:text-blue-800 py-1.5 px-2 bg-gray-50 rounded-lg text-center truncate">
                                            {label}
                                        </a>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    </div>

                    // Footer
                    <div class="bg-gray-900 text-gray-400 px-6 py-6 text-center text-xs space-y-1">
                        <p><a href="/terms" class="underline hover:text-gray-300">"Terms of Service"</a></p>
                        <p>{move || i18n.t("landing.footer")}</p>
                    </div>
                </div>
            }.into_any()
        }
        None => {
            // Fallback: redirect to /welcome
            view! {
                <div class="min-h-screen flex items-center justify-center">
                    <div class="text-center">
                        <p class="text-gray-500 mb-4">"Country not found"</p>
                        <a href="/welcome" class="text-blue-600">"Go to home"</a>
                    </div>
                </div>
            }.into_any()
        }
    }
}

/// Auto-detect component: reads geo info from Vercel headers via meta tag
/// and redirects to appropriate country page
#[component]
pub fn GeoRedirectLanding() -> impl IntoView {
    let i18n = use_i18n();

    // Try to get country from Vercel's x-vercel-ip-country header
    // We inject this via Edge Middleware as a meta tag
    let detected_country = detect_country_from_meta();

    if let Some(slug) = detected_country {
        // Render the country-specific page directly (no redirect for SEO)
        view! {
            <CountryLandingPage country_slug=slug.to_string() />
        }.into_any()
    } else {
        // Default landing (same as original)
        let bg_image = if js_sys::Math::random() < 0.5 {
            "images/hero-1.webp"
        } else {
            "images/hero-2.webp"
        };

        let bg_style = format!(
            "background-image: url('{}'); background-size: cover; background-position: center;",
            bg_image
        );

        view! {
            <div class="min-h-screen text-white relative" style=bg_style>
                <div class="absolute inset-0 bg-black/50"></div>
                <div class="relative z-10 min-h-screen flex flex-col">
                    <div class="px-6 pt-6">
                        <div class="flex items-center justify-between max-w-lg mx-auto">
                            <h1 class="text-2xl font-bold tracking-tight drop-shadow-lg">{move || i18n.t("app.title")}</h1>
                            <button
                                class="text-sm bg-white/20 backdrop-blur-sm px-3 py-1.5 rounded-lg"
                                on:click=move |_| {
                                    let current = i18n.current_locale();
                                    let next = if current == "en" { "ja" } else { "en" };
                                    i18n.set_locale(next);
                                }
                            >
                                {move || if i18n.current_locale() == "en" { "日本語" } else { "English" }}
                            </button>
                        </div>
                    </div>
                    <div class="flex-1"></div>
                    <div class="px-6 pb-8">
                        <div class="max-w-lg mx-auto">
                            <h2 class="text-3xl font-bold leading-tight mb-2 drop-shadow-lg">
                                {move || i18n.t("landing.headline")}
                            </h2>
                            <p class="text-white/80 text-sm leading-relaxed mb-6 drop-shadow">
                                {move || i18n.t("landing.subheadline")}
                            </p>
                            // Country grid for SEO
                            <div class="grid grid-cols-4 gap-2 mb-6">
                                {all_country_pages().into_iter().map(|c| {
                                    let href = format!("/{}", c.slug);
                                    let flag = c.country.flag().to_string();
                                    let code = c.country.code().to_string();
                                    view! {
                                        <a href=href class="flex flex-col items-center gap-1 bg-white/10 backdrop-blur-sm rounded-lg py-2 px-1 active:bg-white/20 transition-colors">
                                            <span class="text-lg">{flag}</span>
                                            <span class="text-[10px] font-medium">{code}</span>
                                        </a>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                            <div class="space-y-3">
                                <a href="/signup" class="block w-full py-4 bg-blue-600 hover:bg-blue-500 text-white rounded-xl font-bold text-lg text-center shadow-lg active:bg-blue-700 transition-colors">
                                    {move || i18n.t("landing.get_started")}
                                </a>
                                <a href="/login" class="block w-full py-4 bg-white/15 backdrop-blur-sm text-white rounded-xl font-bold text-lg text-center active:bg-white/25 transition-colors border border-white/30">
                                    {move || i18n.t("landing.sign_in")}
                                </a>
                            </div>
                            <div class="text-center mt-6 text-white/50 text-xs space-y-1">
                                <p><a href="/terms" class="underline hover:text-white/70">"Terms of Service"</a></p>
                                <p>{move || i18n.t("landing.footer")}</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }.into_any()
    }
}

/// Detect country from meta tag injected by Vercel Edge Middleware
fn detect_country_from_meta() -> Option<&'static str> {
    let window = web_sys::window()?;
    let document = window.document()?;

    // Look for <meta name="x-vercel-ip-country" content="JP">
    let meta = document.query_selector("meta[name='x-vercel-ip-country']").ok()??;
    let country_code = meta.get_attribute("content")?;

    match country_code.as_str() {
        "JP" => Some("japan"),
        "SG" => Some("singapore"),
        "MY" => Some("malaysia"),
        "TH" => Some("thailand"),
        "ID" => Some("indonesia"),
        "PH" => Some("philippines"),
        "VN" => Some("vietnam"),
        "MM" => Some("myanmar"),
        "KH" => Some("cambodia"),
        "LA" => Some("laos"),
        "BN" => Some("brunei"),
        _ => None,
    }
}
