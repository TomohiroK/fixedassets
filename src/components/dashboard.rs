use leptos::prelude::*;
use rust_decimal::Decimal;
use crate::i18n::use_i18n;
use crate::models::asset::{Asset, AssetStatus, Category};
use crate::models::depreciation;
use crate::components::common::format_currency;

#[component]
pub fn DashboardSummary(assets: Vec<Asset>) -> impl IntoView {
    let i18n = use_i18n();

    let total_count = assets.len();
    let total_value: Decimal = assets.iter().map(|a| a.cost).sum();
    let in_use_count = assets.iter().filter(|a| a.status == AssetStatus::InUse).count();
    let disposed_count = assets.iter().filter(|a| a.status == AssetStatus::Disposed).count();

    let total_book_value: Decimal = assets.iter().map(|a| {
        let years = if let Some(acq) = a.acquisition_date_parsed() {
            let today = chrono::Utc::now().date_naive();
            let days = (today - acq).num_days();
            (days as f64 / 365.25) as u32
        } else {
            0
        };
        depreciation::current_book_value(a, years)
    }).sum();

    // Category breakdown
    let categories = Category::all();
    let cat_counts: Vec<(String, usize)> = categories
        .iter()
        .map(|cat| {
            let count = assets.iter().filter(|a| a.category == *cat).count();
            (cat.i18n_key().to_string(), count)
        })
        .filter(|(_, count)| *count > 0)
        .collect();

    view! {
        <div class="space-y-4">
            // Summary Cards
            <div class="grid grid-cols-2 gap-3">
                <div class="card text-center">
                    <p class="text-xs text-gray-500 mb-1">{move || i18n.t("dashboard.total_assets")}</p>
                    <p class="text-2xl font-bold text-gray-900">{total_count}</p>
                </div>
                <div class="card text-center">
                    <p class="text-xs text-gray-500 mb-1">{move || i18n.t("dashboard.total_value")}</p>
                    <p class="text-lg font-bold text-gray-900">{format_currency(&total_value)}</p>
                </div>
                <div class="card text-center">
                    <p class="text-xs text-gray-500 mb-1">{move || i18n.t("dashboard.net_book_value")}</p>
                    <p class="text-lg font-bold text-blue-600">{format_currency(&total_book_value)}</p>
                </div>
                <div class="card">
                    <div class="flex justify-between items-center mb-1">
                        <span class="text-xs text-gray-500">{move || i18n.t("dashboard.in_use")}</span>
                        <span class="text-sm font-bold text-green-600">{in_use_count}</span>
                    </div>
                    <div class="flex justify-between items-center">
                        <span class="text-xs text-gray-500">{move || i18n.t("dashboard.disposed")}</span>
                        <span class="text-sm font-bold text-red-600">{disposed_count}</span>
                    </div>
                </div>
            </div>

            // Category Breakdown
            {if !cat_counts.is_empty() {
                Some(view! {
                    <div class="card">
                        <h3 class="font-semibold text-gray-900 mb-3">{move || i18n.t("dashboard.by_category")}</h3>
                        <div class="space-y-2">
                            {cat_counts.into_iter().map(|(key, count)| {
                                let pct = if total_count > 0 { (count as f64 / total_count as f64) * 100.0 } else { 0.0 };
                                view! {
                                    <div>
                                        <div class="flex justify-between text-sm mb-1">
                                            <span class="text-gray-600">{move || i18n.t(&key)}</span>
                                            <span class="font-medium">{count}</span>
                                        </div>
                                        <div class="w-full bg-gray-200 rounded-full h-2">
                                            <div
                                                class="bg-blue-600 h-2 rounded-full transition-all duration-500"
                                                style=format!("width: {}%", pct)
                                            ></div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                })
            } else {
                None
            }}
        </div>
    }
}
