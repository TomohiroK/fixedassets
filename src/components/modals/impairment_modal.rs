use leptos::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::i18n::use_i18n;
use crate::models::asset::{Asset, ImpairmentRecord};
use crate::models::depreciation;
use crate::models::company::CompanySetup;
use crate::stores::asset_store;
use crate::components::common::format_currency;

/// Modal for recording an impairment loss on an asset
#[component]
pub fn ImpairmentModal(
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
    let bv_str = format_currency(&current_bv);

    let today_str = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let impairment_date = RwSignal::new(today_str);
    let impairment_amount = RwSignal::new(String::new());
    let impairment_reason = RwSignal::new(String::new());
    let is_saving = RwSignal::new(false);

    // Parse amount and calculate new book value reactively
    let parsed_amount = move || {
        Decimal::from_str(&impairment_amount.get()).unwrap_or(Decimal::ZERO)
    };

    let new_bv = move || {
        let amt = parsed_amount();
        let new = current_bv - amt;
        if new < Decimal::ZERO { Decimal::ZERO } else { new }
    };

    let is_valid = move || {
        let amt = parsed_amount();
        amt > Decimal::ZERO && amt <= current_bv && !impairment_date.get().is_empty()
    };

    let asset_for_save = asset.clone();
    let symbol_for_display = symbol.clone();

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
                        <svg class="w-5 h-5 text-purple-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 17h8m0 0V9m0 8l-8-8-4 4-6-6"/>
                        </svg>
                        {move || i18n.t("asset.impairment_title")}
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
                    <p class="text-xs text-gray-500">{move || i18n.t("asset.impairment_desc")}</p>

                    // Current book value display
                    <div class="bg-gray-50 rounded-lg p-3">
                        <p class="text-xs text-gray-500">{move || i18n.t("asset.book_value")}</p>
                        <p class="text-xl font-bold text-gray-900 mt-0.5">{bv_str.clone()}</p>
                    </div>

                    // Impairment amount (prominent)
                    <div>
                        <label class="block text-xs font-medium text-gray-700 mb-1">
                            {move || i18n.t("asset.impairment_amount")}
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
                                prop:value=move || impairment_amount.get()
                                on:input=move |ev| {
                                    impairment_amount.set(event_target_value(&ev));
                                }
                            />
                        </div>
                        // Validation: amount exceeds book value
                        {move || {
                            let amt = parsed_amount();
                            if amt > current_bv && amt > Decimal::ZERO {
                                Some(view! {
                                    <p class="text-xs text-red-500 mt-1">{move || i18n.t("asset.impairment_exceeds")}</p>
                                })
                            } else {
                                None
                            }
                        }}
                    </div>

                    // New book value after impairment (live calculation)
                    {move || {
                        let amt = parsed_amount();
                        if amt > Decimal::ZERO && amt <= current_bv {
                            let new_val = new_bv();
                            let new_str = format_currency(&new_val);
                            let loss_str = format_currency(&amt);
                            Some(view! {
                                <div class="bg-purple-50 border border-purple-200 rounded-lg p-3 space-y-1">
                                    <div class="flex justify-between text-xs">
                                        <span class="text-purple-600">{move || i18n.t("asset.impairment_loss")}</span>
                                        <span class="font-bold text-red-600">"-" {loss_str.clone()}</span>
                                    </div>
                                    <div class="flex justify-between text-xs border-t border-purple-200 pt-1">
                                        <span class="text-purple-600">{move || i18n.t("asset.impairment_new_bv")}</span>
                                        <span class="font-bold text-purple-800">{new_str.clone()}</span>
                                    </div>
                                </div>
                            })
                        } else {
                            None
                        }
                    }}

                    // Impairment date
                    <div>
                        <label class="block text-xs font-medium text-gray-700 mb-1">
                            {move || i18n.t("asset.impairment_date")}
                        </label>
                        <input
                            type="date"
                            class="input-field"
                            prop:value=move || impairment_date.get()
                            on:input=move |ev| {
                                impairment_date.set(event_target_value(&ev));
                            }
                        />
                    </div>

                    // Reason
                    <div>
                        <label class="block text-xs font-medium text-gray-700 mb-1">
                            {move || i18n.t("asset.impairment_reason")}
                        </label>
                        <textarea
                            class="input-field min-h-[60px]"
                            placeholder=move || i18n.t("asset.impairment_reason_placeholder")
                            prop:value=move || impairment_reason.get()
                            on:input=move |ev| {
                                impairment_reason.set(event_target_value(&ev));
                            }
                        ></textarea>
                    </div>

                    // Record button
                    <button
                        class=move || format!(
                            "w-full py-3 rounded-lg font-medium text-sm flex items-center justify-center gap-2 {}",
                            if is_valid() && !is_saving.get() {
                                "bg-purple-600 text-white active:bg-purple-700"
                            } else {
                                "bg-gray-200 text-gray-400"
                            }
                        )
                        disabled=move || !is_valid() || is_saving.get()
                        on:click={
                            let asset = asset_for_save.clone();
                            move |_| {
                                let amt = parsed_amount();
                                if amt <= Decimal::ZERO || amt > current_bv { return; }

                                is_saving.set(true);
                                let mut updated = asset.clone();
                                updated.impairments.push(ImpairmentRecord {
                                    date: impairment_date.get_untracked(),
                                    amount: amt,
                                    reason: impairment_reason.get_untracked(),
                                });
                                updated.updated_at = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();

                                leptos::task::spawn_local(async move {
                                    match asset_store::save_asset(&updated).await {
                                        Ok(()) => {
                                            on_recorded.run(());
                                        }
                                        Err(e) => {
                                            log::error!("Impairment save error: {}", e);
                                            is_saving.set(false);
                                        }
                                    }
                                });
                            }
                        }
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 17h8m0 0V9m0 8l-8-8-4 4-6-6"/>
                        </svg>
                        {move || i18n.t("asset.impairment_execute")}
                    </button>
                </div>
            </div>
        </div>
    }
}
