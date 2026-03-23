use leptos::prelude::*;
use rust_decimal::Decimal;
use crate::i18n::use_i18n;
use crate::models::asset::{Asset, Category, DepreciationPosting};
use crate::models::depreciation;
use crate::models::accounting_standard::use_accounting_standard;
use crate::stores::asset_store;
use crate::components::common::{use_confirm, ConfirmStyle, format_currency, StandardToggle};

#[component]
pub fn DepreciationPage() -> impl IntoView {
    let i18n = use_i18n();
    let confirm = use_confirm();
    let standard = use_accounting_standard();

    let now = chrono::Utc::now();
    let current_year = now.format("%Y").to_string().parse::<u32>().unwrap_or(2026);
    let current_month = now.format("%m").to_string().parse::<u32>().unwrap_or(1);
    let current_day = now.format("%d").to_string().parse::<u32>().unwrap_or(1);

    let sel_year = RwSignal::new(current_year);
    let sel_month = RwSignal::new(current_month);
    // Scope: 0=all, 1=by category, 2=individual
    let scope = RwSignal::new(0u32);
    let sel_category = RwSignal::new(0usize);
    let sel_asset_id = RwSignal::new(String::new());

    let refresh = RwSignal::new(0u32);
    let status_msg = RwSignal::new(Option::<(String, bool)>::None);
    let is_processing = RwSignal::new(false);

    // Tab: 0=Processing, 1=Summary
    let active_tab = RwSignal::new(0u32);

    // Load all assets reactively
    let assets_resource = LocalResource::new(move || {
        refresh.get();
        async move {
            asset_store::get_all_assets().await.unwrap_or_default()
        }
    });

    // Filter assets by scope
    let filtered_assets = move || -> Vec<Asset> {
        let all = match assets_resource.get() {
            Some(a) => (*a).clone(),
            None => return vec![],
        };
        let s = scope.get();
        match s {
            1 => {
                let cat = Category::from_index(sel_category.get());
                all.into_iter().filter(|a| a.category == cat).collect()
            }
            2 => {
                let id = sel_asset_id.get();
                if id.is_empty() {
                    vec![]
                } else {
                    all.into_iter().filter(|a| a.id == id).collect()
                }
            }
            _ => all,
        }
    };

    // Check if selected month has any postings in filtered assets (standard-aware)
    let selected_month_has_postings = move || -> bool {
        let year = sel_year.get();
        let month = sel_month.get();
        let is_ifrs = standard.is_ifrs();
        let assets = filtered_assets();
        assets.iter().any(|a| {
            if is_ifrs { a.has_ifrs_posting(year, month) } else { a.has_posting(year, month) }
        })
    };

    // Preview: compute targets and amounts (standard-aware)
    // status: 0=new, 1=already posted, 2=skipped (IFRS not configured)
    let preview = move || -> (Vec<(String, String, Decimal, u8)>, Decimal) {
        let year = sel_year.get();
        let month = sel_month.get();
        let is_ifrs = standard.is_ifrs();
        let assets = filtered_assets();
        let mut items = Vec::new();
        let mut total = Decimal::ZERO;

        for asset in &assets {
            if !depreciation::is_postable(asset) {
                continue;
            }
            let label = if asset.asset_number.is_empty() {
                asset.name.clone()
            } else {
                format!("{} {}", asset.asset_number, asset.name)
            };

            // In IFRS mode, skip assets without IFRS settings
            if is_ifrs && !asset.has_ifrs_settings() {
                items.push((asset.id.clone(), label, Decimal::ZERO, 2u8));
                continue;
            }

            let already = if is_ifrs { asset.has_ifrs_posting(year, month) } else { asset.has_posting(year, month) };
            let postings_ref = if is_ifrs { &asset.ifrs_postings } else { &asset.postings };
            let amount = if already {
                postings_ref.iter()
                    .find(|p| p.year == year && p.month == month)
                    .map(|p| p.amount)
                    .unwrap_or(Decimal::ZERO)
            } else {
                if is_ifrs {
                    depreciation::ifrs_monthly_depreciation(asset, year, month)
                } else {
                    depreciation::monthly_depreciation(asset, year, month)
                }
            };
            total += amount;
            let status = if already { 1u8 } else { 0u8 };
            items.push((asset.id.clone(), label, amount, status));
        }
        (items, total)
    };

    // Action: Process current month
    let do_process = move || {
        let year = sel_year.get();
        let month = sel_month.get();
        let (items, _) = preview();
        let new_count = items.iter().filter(|(_, _, _, status)| *status == 0).count();
        if new_count == 0 {
            status_msg.set(Some((i18n.t("dep_post.no_targets_to_process"), false)));
            return;
        }

        let confirm = confirm.clone();
        let msg = format!("{}{}", new_count, i18n.t("dep_post.confirm_process"));
        let ok_label = i18n.t("dep_post.action_process");
        let cancel_label = i18n.t("asset.cancel");
        confirm.show(
            &msg,
            ConfirmStyle::Info,
            &ok_label,
            &cancel_label,
            move || {
                is_processing.set(true);
                let is_ifrs = standard.is_ifrs();
                leptos::task::spawn_local(async move {
                    let assets = asset_store::get_all_assets().await.unwrap_or_default();
                    let filtered_ids: Vec<String> = {
                        let s = scope.get_untracked();
                        match s {
                            1 => {
                                let cat = Category::from_index(sel_category.get_untracked());
                                assets.iter().filter(|a| a.category == cat).map(|a| a.id.clone()).collect()
                            }
                            2 => {
                                let id = sel_asset_id.get_untracked();
                                vec![id]
                            }
                            _ => assets.iter().map(|a| a.id.clone()).collect(),
                        }
                    };

                    let mut modified_assets: Vec<crate::models::asset::Asset> = Vec::new();
                    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
                    for mut asset in assets {
                        if !filtered_ids.contains(&asset.id) {
                            continue;
                        }
                        if !depreciation::is_postable(&asset) {
                            continue;
                        }
                        // In IFRS mode, skip assets without IFRS settings
                        if is_ifrs && !asset.has_ifrs_settings() {
                            continue;
                        }
                        let already = if is_ifrs { asset.has_ifrs_posting(year, month) } else { asset.has_posting(year, month) };
                        if already {
                            continue;
                        }
                        let amount = if is_ifrs {
                            depreciation::ifrs_monthly_depreciation(&asset, year, month)
                        } else {
                            depreciation::monthly_depreciation(&asset, year, month)
                        };
                        if amount == Decimal::ZERO {
                            continue;
                        }
                        let posting = DepreciationPosting {
                            year,
                            month,
                            amount,
                            posted_at: now.clone(),
                        };
                        if is_ifrs {
                            asset.ifrs_postings.push(posting);
                        } else {
                            asset.postings.push(posting);
                        }
                        asset.updated_at = now.clone();
                        modified_assets.push(asset);
                    }
                    let count = modified_assets.len() as u32;
                    let _ = asset_store::batch_save_assets(&modified_assets).await;
                    is_processing.set(false);
                    status_msg.set(Some((format!("{}{}", count, i18n.t("dep_post.success_process")), true)));
                    refresh.update(|v| *v += 1);
                });
            },
        );
    };

    // Action: Cancel month
    let do_cancel_month = move || {
        let year = sel_year.get();
        let month = sel_month.get();

        let has_current = selected_month_has_postings();
        let (t_year, t_month) = if has_current {
            (year, month)
        } else if month == 1 {
            (year - 1, 12u32)
        } else {
            (year, month - 1)
        };

        let confirm = confirm.clone();
        let msg = format!("{}/{}{}", t_year, t_month, i18n.t("dep_post.confirm_cancel_month"));
        let ok_label = if has_current {
            i18n.t("dep_post.action_cancel_current")
        } else {
            i18n.t("dep_post.action_cancel_month")
        };
        let cancel_label = i18n.t("asset.cancel");
        confirm.show(
            &msg,
            ConfirmStyle::Warning,
            &ok_label,
            &cancel_label,
            move || {
                is_processing.set(true);
                let is_ifrs = standard.is_ifrs();
                leptos::task::spawn_local(async move {
                    let assets = asset_store::get_all_assets().await.unwrap_or_default();
                    let filtered_ids: Vec<String> = {
                        let s = scope.get_untracked();
                        match s {
                            1 => {
                                let cat = Category::from_index(sel_category.get_untracked());
                                assets.iter().filter(|a| a.category == cat).map(|a| a.id.clone()).collect()
                            }
                            2 => {
                                let id = sel_asset_id.get_untracked();
                                vec![id]
                            }
                            _ => assets.iter().map(|a| a.id.clone()).collect(),
                        }
                    };

                    let mut modified_assets: Vec<crate::models::asset::Asset> = Vec::new();
                    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
                    for mut asset in assets {
                        if !filtered_ids.contains(&asset.id) {
                            continue;
                        }
                        let postings = if is_ifrs { &mut asset.ifrs_postings } else { &mut asset.postings };
                        let before = postings.len();
                        postings.retain(|p| !(p.year == t_year && p.month == t_month));
                        if postings.len() < before {
                            asset.updated_at = now.clone();
                            modified_assets.push(asset);
                        }
                    }
                    let count = modified_assets.len() as u32;
                    let _ = asset_store::batch_save_assets(&modified_assets).await;
                    is_processing.set(false);
                    status_msg.set(Some((format!("{}{}", count, i18n.t("dep_post.success_cancel")), true)));
                    refresh.update(|v| *v += 1);
                });
            },
        );
    };

    // Action: Cancel all
    let do_cancel_all = move || {
        let confirm = confirm.clone();
        let msg = i18n.t("dep_post.confirm_cancel_all");
        let ok_label = i18n.t("dep_post.action_cancel_all");
        let cancel_label = i18n.t("asset.cancel");
        confirm.show(
            &msg,
            ConfirmStyle::Danger,
            &ok_label,
            &cancel_label,
            move || {
                is_processing.set(true);
                let is_ifrs = standard.is_ifrs();
                leptos::task::spawn_local(async move {
                    let assets = asset_store::get_all_assets().await.unwrap_or_default();
                    let filtered_ids: Vec<String> = {
                        let s = scope.get_untracked();
                        match s {
                            1 => {
                                let cat = Category::from_index(sel_category.get_untracked());
                                assets.iter().filter(|a| a.category == cat).map(|a| a.id.clone()).collect()
                            }
                            2 => {
                                let id = sel_asset_id.get_untracked();
                                vec![id]
                            }
                            _ => assets.iter().map(|a| a.id.clone()).collect(),
                        }
                    };

                    let mut modified_assets: Vec<crate::models::asset::Asset> = Vec::new();
                    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
                    for mut asset in assets {
                        if !filtered_ids.contains(&asset.id) {
                            continue;
                        }
                        let postings = if is_ifrs { &mut asset.ifrs_postings } else { &mut asset.postings };
                        if !postings.is_empty() {
                            postings.clear();
                            asset.updated_at = now.clone();
                            modified_assets.push(asset);
                        }
                    }
                    let count = modified_assets.len() as u32;
                    let _ = asset_store::batch_save_assets(&modified_assets).await;
                    is_processing.set(false);
                    status_msg.set(Some((format!("{}{}", count, i18n.t("dep_post.success_cancel")), true)));
                    refresh.update(|v| *v += 1);
                });
            },
        );
    };

    // Check if there are unposted assets to process (status=0 means new/unposted)
    let has_unposted = move || -> bool {
        let (items, _) = preview();
        items.iter().any(|(_, _, _, status)| *status == 0)
    };

    // Cancel button label (dynamic)
    let cancel_month_label = move || -> String {
        if selected_month_has_postings() {
            i18n.t("dep_post.action_cancel_current")
        } else {
            i18n.t("dep_post.action_cancel_month")
        }
    };

    // ========================
    // Summary: category-wise accumulated depreciation up to selected month
    // ========================
    let category_summary = move || -> Vec<(Category, String, u32, Decimal, Decimal, Decimal)> {
        let all = match assets_resource.get() {
            Some(a) => (*a).clone(),
            None => return vec![],
        };
        let target_ym = sel_year.get() * 12 + sel_month.get();
        let is_ifrs = standard.is_ifrs();

        let categories = Category::all();
        let mut result = Vec::new();

        for cat in categories {
            let cat_assets: Vec<&Asset> = all.iter()
                .filter(|a| a.category == cat && depreciation::is_postable(a))
                .collect();

            if cat_assets.is_empty() {
                continue;
            }

            let asset_count = cat_assets.len() as u32;
            let mut total_cost = Decimal::ZERO;
            let mut total_acc_dep = Decimal::ZERO;

            for asset in &cat_assets {
                total_cost += asset.total_cost();
                let postings = if is_ifrs { &asset.ifrs_postings } else { &asset.postings };
                let acc: Decimal = postings.iter()
                    .filter(|p| p.year * 12 + p.month <= target_ym)
                    .map(|p| p.amount)
                    .sum();
                total_acc_dep += acc;
            }

            let book_value = total_cost - total_acc_dep;
            let cat_key = cat.i18n_key().to_string();
            result.push((cat, cat_key, asset_count, total_cost, total_acc_dep, book_value));
        }

        result
    };

    // Grand total for summary
    let summary_totals = move || -> (u32, Decimal, Decimal, Decimal) {
        let summary = category_summary();
        let mut count = 0u32;
        let mut cost = Decimal::ZERO;
        let mut dep = Decimal::ZERO;
        let mut bv = Decimal::ZERO;
        for (_, _, c, tc, td, tbv) in &summary {
            count += c;
            cost += tc;
            dep += td;
            bv += tbv;
        }
        (count, cost, dep, bv)
    };

    view! {
        <div class="page-container pb-32">
            <div class="flex items-center justify-between mb-4">
                <h2 class="page-title flex items-center gap-2 mb-0">
                    <svg class="w-6 h-6 text-emerald-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z"/>
                    </svg>
                    {move || i18n.t("dep_post.title")}
                </h2>
                <StandardToggle />
            </div>

            // Tab switcher: Processing / Summary
            <div class="grid grid-cols-2 gap-1 bg-gray-100 rounded-lg p-1 mb-4">
                <button
                    class=move || if active_tab.get() == 0 {
                        "py-2.5 text-sm font-bold text-white bg-emerald-600 rounded-md shadow-sm"
                    } else {
                        "py-2.5 text-sm font-medium text-gray-600"
                    }
                    on:click=move |_| { active_tab.set(0); status_msg.set(None); }
                >
                    {move || i18n.t("dep_post.tab_processing")}
                </button>
                <button
                    class=move || if active_tab.get() == 1 {
                        "py-2.5 text-sm font-bold text-white bg-emerald-600 rounded-md shadow-sm"
                    } else {
                        "py-2.5 text-sm font-medium text-gray-600"
                    }
                    on:click=move |_| { active_tab.set(1); status_msg.set(None); }
                >
                    {move || i18n.t("dep_post.tab_summary")}
                </button>
            </div>

            // Status message
            {move || status_msg.get().map(|(msg, ok)| {
                let cls = if ok {
                    "mb-4 p-3 bg-emerald-50 border border-emerald-200 rounded-lg text-sm text-emerald-800 flex items-center gap-2"
                } else {
                    "mb-4 p-3 bg-red-50 border border-red-200 rounded-lg text-sm text-red-800 flex items-center gap-2"
                };
                view! {
                    <div class=cls>
                        <svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                        </svg>
                        {msg}
                    </div>
                }
            })}

            // Visual Calendar Card (shared between both tabs)
            <div class="card mb-4">
                <div class="flex items-center justify-between mb-3">
                    <div class="flex items-center gap-2">
                        <div class="w-8 h-8 rounded-full bg-emerald-600 text-white flex items-center justify-center text-sm font-bold">
                            {current_day}
                        </div>
                        <div>
                            <p class="text-xs text-gray-500">{move || i18n.t("dep_post.today")}</p>
                            <p class="text-sm font-bold text-gray-800">
                                {format!("{}/{:02}/{:02}", current_year, current_month, current_day)}
                            </p>
                        </div>
                    </div>
                    <div class="text-right">
                        <p class="text-xs text-gray-500">
                            {move || if active_tab.get() == 0 {
                                i18n.t("dep_post.processing_month")
                            } else {
                                i18n.t("dep_post.snapshot_date")
                            }}
                        </p>
                        <p class="text-lg font-bold text-emerald-700">
                            {move || format!("{}/{:02}", sel_year.get(), sel_month.get())}
                        </p>
                    </div>
                </div>

                // Year selector row
                <div class="flex items-center justify-center gap-2 mb-2">
                    <button
                        class="w-7 h-7 flex items-center justify-center rounded-full hover:bg-gray-100 text-gray-500"
                        on:click=move |_| { sel_year.update(|y| *y -= 1); status_msg.set(None); refresh.update(|v| *v += 1); }
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                        </svg>
                    </button>
                    <span class="text-sm font-bold text-gray-800 w-16 text-center">{move || sel_year.get().to_string()}</span>
                    <button
                        class="w-7 h-7 flex items-center justify-center rounded-full hover:bg-gray-100 text-gray-500"
                        on:click=move |_| { sel_year.update(|y| *y += 1); status_msg.set(None); refresh.update(|v| *v += 1); }
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                        </svg>
                    </button>
                </div>

                // Mini month grid (4x3)
                <div class="grid grid-cols-4 gap-1.5">
                    {(1u32..=12).map(|m| {
                        let month_label = format!("{:02}", m);
                        view! {
                            <button
                                class=move || {
                                    let selected = sel_month.get() == m;
                                    let is_current = m == current_month && sel_year.get() == current_year;
                                    if selected {
                                        "relative py-2 rounded-lg text-xs font-bold text-white bg-emerald-600 shadow-sm"
                                    } else if is_current {
                                        "relative py-2 rounded-lg text-xs font-bold text-emerald-700 bg-emerald-50 border-2 border-emerald-300"
                                    } else {
                                        "relative py-2 rounded-lg text-xs font-medium text-gray-600 hover:bg-gray-100"
                                    }
                                }
                                on:click=move |_| { sel_month.set(m); status_msg.set(None); refresh.update(|v| *v += 1); }
                            >
                                {month_label}
                                {move || {
                                    let is_current = m == current_month && sel_year.get() == current_year;
                                    let selected = sel_month.get() == m;
                                    if is_current && !selected {
                                        Some(view! {
                                            <span class="absolute bottom-0.5 left-1/2 -translate-x-1/2 w-1 h-1 rounded-full bg-emerald-500"></span>
                                        })
                                    } else if is_current && selected {
                                        Some(view! {
                                            <span class="absolute bottom-0.5 left-1/2 -translate-x-1/2 w-1 h-1 rounded-full bg-white"></span>
                                        })
                                    } else {
                                        None
                                    }
                                }}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>

            // ====== TAB 0: Processing ======
            {move || if active_tab.get() == 0 {
                view! {
                    <div>
                        // Scope selector
                        <div class="card mb-4">
                            <h3 class="text-sm font-semibold text-gray-700 mb-2">{move || i18n.t("dep_post.scope")}</h3>
                            <div class="grid grid-cols-3 gap-1 bg-gray-100 rounded-lg p-1 mb-3">
                                <button
                                    class=move || if scope.get() == 0 { "py-2 text-xs font-bold text-white bg-emerald-600 rounded-md" } else { "py-2 text-xs font-medium text-gray-600" }
                                    on:click=move |_| { scope.set(0); status_msg.set(None); refresh.update(|v| *v += 1); }
                                >{move || i18n.t("dep_post.scope_all")}</button>
                                <button
                                    class=move || if scope.get() == 1 { "py-2 text-xs font-bold text-white bg-emerald-600 rounded-md" } else { "py-2 text-xs font-medium text-gray-600" }
                                    on:click=move |_| { scope.set(1); status_msg.set(None); refresh.update(|v| *v += 1); }
                                >{move || i18n.t("dep_post.scope_category")}</button>
                                <button
                                    class=move || if scope.get() == 2 { "py-2 text-xs font-bold text-white bg-emerald-600 rounded-md" } else { "py-2 text-xs font-medium text-gray-600" }
                                    on:click=move |_| { scope.set(2); status_msg.set(None); refresh.update(|v| *v += 1); }
                                >{move || i18n.t("dep_post.scope_individual")}</button>
                            </div>

                            {move || if scope.get() == 1 {
                                view! {
                                    <select
                                        class="input-field"
                                        on:change=move |ev| {
                                            let v: usize = event_target_value(&ev).parse().unwrap_or(0);
                                            sel_category.set(v);
                                        }
                                    >
                                        {Category::all().into_iter().enumerate().map(|(idx, cat)| {
                                            let key = cat.i18n_key().to_string();
                                            view! {
                                                <option value=idx.to_string()>{move || i18n.t(&key)}</option>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </select>
                                }.into_any()
                            } else if scope.get() == 2 {
                                view! {
                                    <Suspense fallback=|| ()>
                                        {move || assets_resource.get().map(|all_assets| {
                                            let postable: Vec<_> = all_assets.iter()
                                                .filter(|a| depreciation::is_postable(a))
                                                .collect();
                                            view! {
                                                <select
                                                    class="input-field"
                                                    on:change=move |ev| {
                                                        sel_asset_id.set(event_target_value(&ev));
                                                    }
                                                >
                                                    <option value="">{move || i18n.t("dep_post.select_asset")}</option>
                                                    {postable.into_iter().map(|a| {
                                                        let id = a.id.clone();
                                                        let label = if a.asset_number.is_empty() {
                                                            a.name.clone()
                                                        } else {
                                                            format!("{} {}", a.asset_number, a.name)
                                                        };
                                                        view! {
                                                            <option value=id>{label}</option>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </select>
                                            }
                                        })}
                                    </Suspense>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }}
                        </div>

                        // Preview
                        <div class="card mb-4">
                            <h3 class="text-sm font-semibold text-gray-700 mb-3">{move || i18n.t("dep_post.preview")}</h3>
                            <Suspense fallback=move || view! { <p class="text-sm text-gray-400">{move || i18n.t("common.loading")}</p> }>
                                {move || {
                                    let _ = assets_resource.get();
                                    let (items, total) = preview();
                                    // status: 0=new, 1=posted, 2=ifrs_not_configured
                                    let new_count = items.iter().filter(|(_, _, _, s)| *s == 0).count();

                                    if items.is_empty() {
                                        view! {
                                            <p class="text-sm text-gray-400 text-center py-4">{move || i18n.t("dep_post.no_targets")}</p>
                                        }.into_any()
                                    } else {
                                        let processable_count = items.iter().filter(|(_, _, _, s)| *s != 2).count();
                                        let posted_count = items.iter().filter(|(_, _, _, s)| *s == 1).count();
                                        let not_configured = items.iter().filter(|(_, _, _, s)| *s == 2).count();
                                        view! {
                                            <div>
                                                <div class="grid grid-cols-2 gap-3 mb-3">
                                                    <div class="bg-emerald-50 rounded-lg p-3 text-center">
                                                        <p class="text-xs text-emerald-600">{move || i18n.t("dep_post.target_count")}</p>
                                                        <p class="text-lg font-bold text-emerald-800">{processable_count}</p>
                                                        {if posted_count != 0 {
                                                            view! {
                                                                <p class="text-[10px] text-gray-400 mt-0.5">
                                                                    {format!("({} {})", posted_count, i18n.t("dep_post.already_posted"))}
                                                                </p>
                                                            }.into_any()
                                                        } else {
                                                            view! { <span></span> }.into_any()
                                                        }}
                                                    </div>
                                                    <div class="bg-emerald-50 rounded-lg p-3 text-center">
                                                        <p class="text-xs text-emerald-600">{move || i18n.t("dep_post.total_amount")}</p>
                                                        <p class="text-lg font-bold text-emerald-800">{format_currency(&total)}</p>
                                                    </div>
                                                </div>

                                                <div class="max-h-60 overflow-y-auto space-y-1">
                                                    {items.into_iter().map(|(_id, label, amount, status)| {
                                                        let is_not_configured = status == 2;
                                                        let is_posted = status == 1;
                                                        let amount_cls = if is_posted {
                                                            "text-sm text-gray-400"
                                                        } else if is_not_configured {
                                                            "text-sm text-gray-300"
                                                        } else {
                                                            "text-sm font-medium text-gray-900"
                                                        };
                                                        let row_cls = if is_not_configured {
                                                            "flex items-center justify-between py-1.5 px-2 rounded bg-gray-50 opacity-50"
                                                        } else {
                                                            "flex items-center justify-between py-1.5 px-2 rounded hover:bg-gray-50"
                                                        };
                                                        view! {
                                                            <div class=row_cls>
                                                                <span class="text-xs text-gray-700 truncate flex-1 mr-2">{label}</span>
                                                                <div class="flex items-center gap-2 shrink-0">
                                                                    {if !is_not_configured {
                                                                        view! { <span class=amount_cls>{format_currency(&amount)}</span> }.into_any()
                                                                    } else {
                                                                        view! { <span></span> }.into_any()
                                                                    }}
                                                                    {if is_posted {
                                                                        view! { <span class="text-[10px] px-1.5 py-0.5 bg-gray-200 text-gray-500 rounded">{i18n.t("dep_post.already_posted")}</span> }.into_any()
                                                                    } else if is_not_configured {
                                                                        view! { <span class="text-[10px] px-1.5 py-0.5 bg-orange-100 text-orange-600 rounded">{i18n.t("dep_post.ifrs_not_set")}</span> }.into_any()
                                                                    } else {
                                                                        view! { <span></span> }.into_any()
                                                                    }}
                                                                </div>
                                                            </div>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            </div>
                                        }.into_any()
                                    }
                                }}
                            </Suspense>
                        </div>
                    </div>
                }.into_any()
            } else {
                // ====== TAB 1: Summary (Category-wise accumulated depreciation) ======
                view! {
                    <div>
                        <Suspense fallback=move || view! { <p class="text-sm text-gray-400">{move || i18n.t("common.loading")}</p> }>
                            {move || {
                                let _ = assets_resource.get();
                                let summary = category_summary();
                                let (total_count, total_cost, total_dep, total_bv) = summary_totals();

                                if summary.is_empty() {
                                    view! {
                                        <div class="card">
                                            <p class="text-sm text-gray-400 text-center py-6">{move || i18n.t("dep_post.no_targets")}</p>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div>
                                            // Grand total card
                                            <div class="card mb-4">
                                                <h3 class="text-sm font-semibold text-gray-700 mb-3">
                                                    {move || format!("{}/{:02} {}", sel_year.get(), sel_month.get(), i18n.t("dep_post.summary_as_of"))}
                                                </h3>
                                                <div class="grid grid-cols-3 gap-2">
                                                    <div class="bg-blue-50 rounded-lg p-3 text-center">
                                                        <p class="text-[10px] text-blue-600 mb-1">{move || i18n.t("dep_post.summary_cost")}</p>
                                                        <p class="text-sm font-bold text-blue-800">{format_currency(&total_cost)}</p>
                                                    </div>
                                                    <div class="bg-amber-50 rounded-lg p-3 text-center">
                                                        <p class="text-[10px] text-amber-600 mb-1">{move || i18n.t("dep_post.summary_acc_dep")}</p>
                                                        <p class="text-sm font-bold text-amber-800">{format_currency(&total_dep)}</p>
                                                    </div>
                                                    <div class="bg-emerald-50 rounded-lg p-3 text-center">
                                                        <p class="text-[10px] text-emerald-600 mb-1">{move || i18n.t("dep_post.summary_book_value")}</p>
                                                        <p class="text-sm font-bold text-emerald-800">{format_currency(&total_bv)}</p>
                                                    </div>
                                                </div>
                                                <p class="text-[10px] text-gray-400 text-right mt-2">
                                                    {move || format!("{} {}", total_count, i18n.t("dep_post.summary_assets_unit"))}
                                                </p>
                                            </div>

                                            // Category breakdown
                                            <div class="space-y-2">
                                                {summary.into_iter().map(|(_, cat_key, count, cost, dep, bv)| {
                                                    let dep_ratio = if cost > Decimal::ZERO {
                                                        let pct = (dep * Decimal::from(100) / cost).round_dp(1);
                                                        format!("{}%", pct)
                                                    } else {
                                                        "0%".to_string()
                                                    };
                                                    let bar_width = if cost > Decimal::ZERO {
                                                        let pct = (dep * Decimal::from(100) / cost).round_dp(0);
                                                        let w: u32 = pct.to_string().parse().unwrap_or(0);
                                                        let w_clamped = if w > 100 { 100 } else { w };
                                                        format!("{}%", w_clamped)
                                                    } else {
                                                        "0%".to_string()
                                                    };
                                                    view! {
                                                        <div class="card">
                                                            <div class="flex items-center justify-between mb-2">
                                                                <div class="flex items-center gap-2">
                                                                    <h4 class="text-sm font-semibold text-gray-800">{move || i18n.t(&cat_key)}</h4>
                                                                    <span class="text-[10px] bg-gray-100 text-gray-500 px-1.5 py-0.5 rounded">
                                                                        {format!("{}{}", count, i18n.t("dep_post.summary_assets_unit"))}
                                                                    </span>
                                                                </div>
                                                                <span class="text-xs font-bold text-amber-600">{dep_ratio}</span>
                                                            </div>
                                                            // Progress bar
                                                            <div class="w-full h-2 bg-gray-100 rounded-full mb-3 overflow-hidden">
                                                                <div class="h-full bg-amber-400 rounded-full transition-all" style=format!("width: {}", bar_width)></div>
                                                            </div>
                                                            <div class="grid grid-cols-3 gap-2 text-center">
                                                                <div>
                                                                    <p class="text-[10px] text-gray-400">{move || i18n.t("dep_post.summary_cost")}</p>
                                                                    <p class="text-xs font-medium text-gray-800">{format_currency(&cost)}</p>
                                                                </div>
                                                                <div>
                                                                    <p class="text-[10px] text-gray-400">{move || i18n.t("dep_post.summary_acc_dep")}</p>
                                                                    <p class="text-xs font-medium text-amber-700">{format_currency(&dep)}</p>
                                                                </div>
                                                                <div>
                                                                    <p class="text-[10px] text-gray-400">{move || i18n.t("dep_post.summary_book_value")}</p>
                                                                    <p class="text-xs font-medium text-emerald-700">{format_currency(&bv)}</p>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        </div>
                                    }.into_any()
                                }
                            }}
                        </Suspense>
                    </div>
                }.into_any()
            }}

            // Action buttons (only on Processing tab, fixed at bottom)
            {move || if active_tab.get() == 0 {
                Some(view! {
                    <div class="fixed bottom-16 left-0 right-0 bg-white border-t border-gray-200 px-4 py-3 z-40 max-w-lg mx-auto space-y-2">
                        <button
                            class="w-full py-3 rounded-lg font-medium text-sm flex items-center justify-center gap-2 bg-emerald-600 text-white active:bg-emerald-700 disabled:opacity-40"
                            disabled=move || is_processing.get() || !has_unposted()
                            on:click=move |_| do_process()
                        >
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                            </svg>
                            {move || i18n.t("dep_post.action_process")}
                        </button>
                        <div class="grid grid-cols-2 gap-2">
                            <button
                                class="py-2.5 rounded-lg font-medium text-xs flex items-center justify-center gap-1 border border-amber-300 text-amber-700 bg-amber-50 active:bg-amber-100 disabled:opacity-40"
                                disabled=move || is_processing.get()
                                on:click=move |_| do_cancel_month()
                            >
                                <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6"/>
                                </svg>
                                {cancel_month_label}
                            </button>
                            <button
                                class="py-2.5 rounded-lg font-medium text-xs flex items-center justify-center gap-1 border border-red-300 text-red-700 bg-red-50 active:bg-red-100 disabled:opacity-40"
                                disabled=move || is_processing.get()
                                on:click=move |_| do_cancel_all()
                            >
                                <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
                                </svg>
                                {move || i18n.t("dep_post.action_cancel_all")}
                            </button>
                        </div>
                    </div>
                })
            } else {
                None
            }}
        </div>
    }
}
