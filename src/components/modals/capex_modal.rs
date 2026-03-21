use leptos::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::i18n::use_i18n;
use crate::models::asset::{Asset, CapExRecord};
use crate::models::depreciation;
use crate::models::company::CompanySetup;
use crate::stores::asset_store;
use crate::components::common::format_currency;

/// Modal for recording a capital expenditure on an asset
#[component]
pub fn CapExModal(
    show: RwSignal<bool>,
    asset: Asset,
    on_recorded: Callback<()>,
) -> impl IntoView {
    let i18n = use_i18n();

    let symbol = CompanySetup::load()
        .and_then(|s| s.currency().map(|c| c.symbol().to_string()))
        .unwrap_or_else(|| "$".to_string());

    // Calculate current book value
    let prior_months = asset.prior_months_total();
    let prior_years_full = prior_months / 12;
    let own_years = if let Some(acq) = asset.acquisition_date_parsed() {
        let today = chrono::Utc::now().date_naive();
        let days = (today - acq).num_days();
        (days as f64 / 365.25) as u32
    } else {
        0
    };
    let total_years_elapsed = prior_years_full + own_years;
    let current_bv = depreciation::current_book_value(&asset, total_years_elapsed);
    let total_cost = asset.total_cost();
    let existing_capex = asset.total_capex();

    let bv_str = format_currency(&current_bv);
    let cost_str = format_currency(&total_cost);

    let today_str = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let capex_date = RwSignal::new(today_str);
    let capex_amount = RwSignal::new(String::new());
    let capex_description = RwSignal::new(String::new());
    let is_saving = RwSignal::new(false);

    let parsed_amount = move || {
        Decimal::from_str(&capex_amount.get()).unwrap_or(Decimal::ZERO)
    };

    let is_valid = move || {
        let amt = parsed_amount();
        amt > Decimal::ZERO && !capex_date.get().is_empty()
    };

    let asset_for_save = asset.clone();

    view! {
        <div
            class=move || if show.get() {
                "fixed inset-0 z-50 flex items-end justify-center bg-black/40 backdrop-blur-sm animate-fade-in"
            } else {
                "hidden"
            }
            on:click=move |e| {
                use wasm_bindgen::JsCast;
                if let Some(target) = e.target() {
                    if let Some(el) = target.dyn_ref::<web_sys::HtmlElement>() {
                        if el.class_list().contains("fixed") {
                            show.set(false);
                        }
                    }
                }
            }
        >
            <div class="bg-white rounded-t-2xl shadow-2xl w-full max-w-lg max-h-[85vh] overflow-y-auto pb-24 animate-scale-in">
                // Header
                <div class="sticky top-0 bg-white border-b border-gray-100 px-4 py-3 flex items-center justify-between z-10">
                    <h2 class="text-base font-bold text-gray-900 flex items-center gap-2">
                        <svg class="w-5 h-5 text-teal-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z"/>
                        </svg>
                        {move || i18n.t("asset.capex_title")}
                    </h2>
                    <button
                        class="text-gray-400 p-1"
                        on:click=move |_| show.set(false)
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                        </svg>
                    </button>
                </div>

                <div class="px-4 py-4 space-y-4">
                    // Description
                    <p class="text-xs text-gray-500">{move || i18n.t("asset.capex_desc")}</p>

                    // Current asset info
                    <div class="bg-gray-50 rounded-lg p-3 space-y-1">
                        <div class="flex justify-between text-xs">
                            <span class="text-gray-500">{move || i18n.t("asset.capex_current_cost")}</span>
                            <span class="font-bold text-gray-900">{cost_str.clone()}</span>
                        </div>
                        <div class="flex justify-between text-xs">
                            <span class="text-gray-500">{move || i18n.t("asset.book_value")}</span>
                            <span class="font-bold text-blue-600">{bv_str.clone()}</span>
                        </div>
                        {if existing_capex > Decimal::ZERO {
                            let ex_str = format_currency(&existing_capex);
                            Some(view! {
                                <div class="flex justify-between text-xs">
                                    <span class="text-gray-500">{move || i18n.t("asset.capex_existing")}</span>
                                    <span class="font-medium text-teal-600">"+" {ex_str}</span>
                                </div>
                            })
                        } else {
                            None
                        }}
                    </div>

                    // CapEx amount (prominent)
                    <div>
                        <label class="block text-xs font-medium text-gray-700 mb-1">
                            {move || i18n.t("asset.capex_amount")}
                        </label>
                        <div class="relative">
                            <span class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 text-sm font-medium">
                                {symbol.clone()}
                            </span>
                            <input
                                type="number"
                                class="input-field pl-8 text-lg font-bold"
                                min="0"
                                step="1"
                                placeholder="0"
                                prop:value=move || capex_amount.get()
                                on:input=move |ev| {
                                    capex_amount.set(event_target_value(&ev));
                                }
                            />
                        </div>
                    </div>

                    // Live preview: new total cost after CapEx
                    {move || {
                        let amt = parsed_amount();
                        if amt > Decimal::ZERO {
                            let new_total = total_cost + amt;
                            let new_total_str = format_currency(&new_total);
                            let amt_str = format_currency(&amt);
                            Some(view! {
                                <div class="bg-teal-50 border border-teal-200 rounded-lg p-3 space-y-1">
                                    <div class="flex justify-between text-xs">
                                        <span class="text-teal-600">{move || i18n.t("asset.capex_addition")}</span>
                                        <span class="font-bold text-teal-700">"+" {amt_str.clone()}</span>
                                    </div>
                                    <div class="flex justify-between text-xs border-t border-teal-200 pt-1">
                                        <span class="text-teal-600">{move || i18n.t("asset.capex_new_total")}</span>
                                        <span class="font-bold text-teal-800">{new_total_str.clone()}</span>
                                    </div>
                                </div>
                            })
                        } else {
                            None
                        }
                    }}

                    // CapEx date
                    <div>
                        <label class="block text-xs font-medium text-gray-700 mb-1">
                            {move || i18n.t("asset.capex_date")}
                        </label>
                        <input
                            type="date"
                            class="input-field"
                            prop:value=move || capex_date.get()
                            on:input=move |ev| {
                                capex_date.set(event_target_value(&ev));
                            }
                        />
                    </div>

                    // Description
                    <div>
                        <label class="block text-xs font-medium text-gray-700 mb-1">
                            {move || i18n.t("asset.capex_detail")}
                        </label>
                        <textarea
                            class="input-field min-h-[60px]"
                            placeholder=move || i18n.t("asset.capex_detail_placeholder")
                            prop:value=move || capex_description.get()
                            on:input=move |ev| {
                                capex_description.set(event_target_value(&ev));
                            }
                        ></textarea>
                    </div>

                    // Info hint
                    <div class="bg-blue-50 border border-blue-200 rounded-lg p-3 flex items-start gap-2">
                        <svg class="w-4 h-4 text-blue-500 shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                        </svg>
                        <p class="text-xs text-blue-700">{move || i18n.t("asset.capex_hint")}</p>
                    </div>

                    // Record button
                    <button
                        class=move || format!(
                            "w-full py-3 rounded-lg font-medium text-sm flex items-center justify-center gap-2 {}",
                            if is_valid() && !is_saving.get() {
                                "bg-teal-600 text-white active:bg-teal-700"
                            } else {
                                "bg-gray-200 text-gray-400"
                            }
                        )
                        disabled=move || !is_valid() || is_saving.get()
                        on:click={
                            let asset = asset_for_save.clone();
                            move |_| {
                                let amt = parsed_amount();
                                if amt <= Decimal::ZERO { return; }

                                is_saving.set(true);
                                let mut updated = asset.clone();
                                updated.capex_records.push(CapExRecord {
                                    date: capex_date.get_untracked(),
                                    amount: amt,
                                    description: capex_description.get_untracked(),
                                });
                                updated.updated_at = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();

                                leptos::task::spawn_local(async move {
                                    match asset_store::save_asset(&updated).await {
                                        Ok(()) => {
                                            on_recorded.run(());
                                        }
                                        Err(e) => {
                                            log::error!("CapEx save error: {}", e);
                                            is_saving.set(false);
                                        }
                                    }
                                });
                            }
                        }
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z"/>
                        </svg>
                        {move || i18n.t("asset.capex_execute")}
                    </button>
                </div>
            </div>
        </div>
    }
}
