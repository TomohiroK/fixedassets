use leptos::prelude::*;
use rust_decimal::Decimal;
use crate::i18n::use_i18n;
use crate::models::asset::ImpairmentRecord;
use crate::components::common::format_currency;

/// Section showing impairment history on the asset detail page
#[component]
pub fn ImpairmentInfoSection(
    impairments: Vec<ImpairmentRecord>,
    total_impairment: Decimal,
) -> impl IntoView {
    let i18n = use_i18n();
    let total_str = format_currency(&total_impairment);
    let count = impairments.len();

    view! {
        <div class="mt-4 border rounded-xl p-4 bg-purple-50 border-purple-200">
            <h3 class="text-sm font-semibold mb-3 flex items-center gap-2 text-purple-800">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 17h8m0 0V9m0 8l-8-8-4 4-6-6"/>
                </svg>
                {move || i18n.t("asset.impairment_info")}
                <span class="text-xs font-normal text-purple-500">
                    "(" {count} {if count == 1 { " record" } else { " records" }} ")"
                </span>
            </h3>
            <div class="space-y-2 text-sm">
                // Total impairment loss
                <div class="flex justify-between items-center">
                    <span class="text-gray-600 font-medium">{move || i18n.t("asset.impairment_total")}</span>
                    <span class="font-bold text-red-600">"-" {total_str.clone()}</span>
                </div>

                // Individual records
                <div class="border-t border-purple-200 pt-2 mt-2 space-y-2">
                    {impairments.into_iter().enumerate().map(|(i, record)| {
                        let amt_str = format_currency(&record.amount);
                        let date = record.date.clone();
                        let reason = record.reason.clone();
                        let has_reason = !reason.is_empty();
                        view! {
                            <div class="bg-white/60 rounded-lg p-2.5">
                                <div class="flex justify-between items-center">
                                    <span class="text-xs text-purple-600 font-medium">"#" {i + 1} " - " {date}</span>
                                    <span class="text-xs font-bold text-red-600">"-" {amt_str}</span>
                                </div>
                                {if has_reason {
                                    Some(view! {
                                        <p class="text-xs text-gray-500 mt-1">{reason}</p>
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
