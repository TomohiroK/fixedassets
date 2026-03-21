use leptos::prelude::*;
use rust_decimal::Decimal;
use crate::i18n::use_i18n;
use crate::models::asset::CapExRecord;
use crate::components::common::format_currency;

/// Section showing capital expenditure history on the asset detail page
#[component]
pub fn CapExInfoSection(
    capex_records: Vec<CapExRecord>,
    total_capex: Decimal,
    original_cost: Decimal,
) -> impl IntoView {
    let i18n = use_i18n();
    let total_str = format_currency(&total_capex);
    let total_cost = original_cost + total_capex;
    let total_cost_str = format_currency(&total_cost);
    let original_str = format_currency(&original_cost);
    let count = capex_records.len();

    view! {
        <div class="mt-4 border rounded-xl p-4 bg-teal-50 border-teal-200">
            <h3 class="text-sm font-semibold mb-3 flex items-center gap-2 text-teal-800">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z"/>
                </svg>
                {move || i18n.t("asset.capex_info")}
                <span class="text-xs font-normal text-teal-500">
                    "(" {count} {if count == 1 { " record" } else { " records" }} ")"
                </span>
            </h3>
            <div class="space-y-2 text-sm">
                // Summary: Original + CapEx = Total
                <div class="bg-white/60 rounded-lg p-2.5 space-y-1">
                    <div class="flex justify-between text-xs">
                        <span class="text-gray-600">{move || i18n.t("asset.capex_original_cost")}</span>
                        <span class="font-medium text-gray-900">{original_str.clone()}</span>
                    </div>
                    <div class="flex justify-between text-xs">
                        <span class="text-teal-600 font-medium">{move || i18n.t("asset.capex_total")}</span>
                        <span class="font-bold text-teal-700">"+" {total_str.clone()}</span>
                    </div>
                    <div class="flex justify-between text-xs border-t border-teal-200 pt-1">
                        <span class="text-gray-800 font-medium">{move || i18n.t("asset.capex_new_total")}</span>
                        <span class="font-bold text-gray-900">{total_cost_str.clone()}</span>
                    </div>
                </div>

                // Individual records
                <div class="space-y-1.5">
                    {capex_records.into_iter().enumerate().map(|(i, record)| {
                        let amt_str = format_currency(&record.amount);
                        let date = record.date.clone();
                        let desc = record.description.clone();
                        let has_desc = !desc.is_empty();
                        view! {
                            <div class="bg-white/60 rounded-lg p-2.5">
                                <div class="flex justify-between items-center">
                                    <span class="text-xs text-teal-600 font-medium">"#" {i + 1} " - " {date}</span>
                                    <span class="text-xs font-bold text-teal-700">"+" {amt_str}</span>
                                </div>
                                {if has_desc {
                                    Some(view! {
                                        <p class="text-xs text-gray-500 mt-1">{desc}</p>
                                    })
                                } else {
                                    None
                                }}
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
