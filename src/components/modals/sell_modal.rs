use leptos::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::i18n::use_i18n;
use crate::stores::asset_store;
use crate::models::asset::{Asset, AssetStatus};
use crate::models::depreciation;
use crate::models::company::CompanySetup;

/// Sell modal — record asset sale
#[component]
pub fn SellModal(
    show: RwSignal<bool>,
    asset: Asset,
    on_sold: Callback<()>,
) -> impl IntoView {
    let i18n = use_i18n();
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let sell_date = RwSignal::new(today);
    let sell_price = RwSignal::new(String::new());
    let buyer = RwSignal::new(String::new());
    let is_saving = RwSignal::new(false);

    let current_bv = {
        let years = asset.useful_life + asset.prior_depreciation_years;
        let acc = depreciation::accumulated_depreciation(&asset, years);
        let bv = asset.cost - acc;
        if bv < Decimal::ZERO { Decimal::ZERO } else { bv }
    };

    let format_bv = {
        let setup = CompanySetup::load();
        let symbol = setup.as_ref()
            .and_then(|s| s.currency().map(|c| c.symbol().to_string()))
            .unwrap_or_else(|| "$".to_string());
        format!("{}{}", symbol, current_bv.round_dp(0))
    };

    // Live gain/loss calculation
    let gain_loss_view = move || {
        let price = Decimal::from_str(&sell_price.get()).unwrap_or(Decimal::ZERO);
        let gl = price - current_bv;
        let setup = CompanySetup::load();
        let symbol = setup.as_ref()
            .and_then(|s| s.currency().map(|c| c.symbol().to_string()))
            .unwrap_or_else(|| "$".to_string());

        if gl > Decimal::ZERO {
            let label = i18n.t("asset.sale_gain");
            view! {
                <span class="text-green-600 font-semibold text-sm">
                    {format!("{}: {}{}", label, symbol, gl.round_dp(0))}
                </span>
            }.into_any()
        } else if gl < Decimal::ZERO {
            let label = i18n.t("asset.sale_loss");
            view! {
                <span class="text-red-600 font-semibold text-sm">
                    {format!("{}: -{}{}", label, symbol, (-gl).round_dp(0))}
                </span>
            }.into_any()
        } else {
            view! { <span class="text-gray-400 text-sm">"±0"</span> }.into_any()
        }
    };

    view! {
        <div
            class=move || if show.get() {
                "fixed inset-0 z-50 flex items-end sm:items-center justify-center bg-black/50"
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
            <div class="bg-white w-full sm:max-w-md rounded-t-2xl sm:rounded-2xl shadow-2xl p-5 pb-24 max-h-[85vh] overflow-y-auto">
                // Header
                <div class="flex items-center justify-between mb-4">
                    <h3 class="text-lg font-bold text-gray-900 flex items-center gap-2">
                        <svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                        </svg>
                        {move || i18n.t("asset.sell_title")}
                    </h3>
                    <button class="p-1 rounded-full hover:bg-gray-100" on:click=move |_| show.set(false)>
                        <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                        </svg>
                    </button>
                </div>

                // Current book value + live gain/loss
                <div class="bg-gray-50 rounded-lg p-3 mb-4">
                    <div class="flex justify-between items-start">
                        <div>
                            <p class="text-xs text-gray-500">{move || i18n.t("asset.disposal_book_value")}</p>
                            <p class="text-lg font-bold text-gray-900">{format_bv.clone()}</p>
                        </div>
                        <div class="text-right mt-1">
                            {gain_loss_view}
                        </div>
                    </div>
                </div>

                // Form
                <div class="space-y-4">
                    // Sale price (prominent)
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">
                            {move || i18n.t("asset.sell_price")}
                        </label>
                        <input
                            type="number" min="0" step="1"
                            class="w-full px-3 py-3 border-2 border-emerald-200 rounded-lg text-lg font-semibold focus:border-emerald-400 focus:ring-1 focus:ring-emerald-400 outline-none"
                            placeholder="0"
                            prop:value=move || sell_price.get()
                            on:input=move |e| {
                                use wasm_bindgen::JsCast;
                                let val = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>().value();
                                sell_price.set(val);
                            }
                        />
                    </div>
                    // Sale date
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">
                            {move || i18n.t("asset.sell_date")}
                        </label>
                        <input
                            type="date"
                            class="w-full px-3 py-2.5 border border-gray-200 rounded-lg text-sm"
                            prop:value=move || sell_date.get()
                            on:input=move |e| {
                                use wasm_bindgen::JsCast;
                                let val = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>().value();
                                sell_date.set(val);
                            }
                        />
                    </div>
                    // Buyer
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">
                            {move || i18n.t("asset.sell_buyer")}
                        </label>
                        <input
                            type="text"
                            class="w-full px-3 py-2.5 border border-gray-200 rounded-lg text-sm"
                            placeholder=move || i18n.t("asset.sell_buyer_placeholder")
                            prop:value=move || buyer.get()
                            on:input=move |e| {
                                use wasm_bindgen::JsCast;
                                let val = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>().value();
                                buyer.set(val);
                            }
                        />
                    </div>
                </div>

                // Buttons
                <div class="flex gap-3 mt-6">
                    <button
                        class="flex-1 py-3 text-gray-600 font-medium border border-gray-200 rounded-lg active:bg-gray-50"
                        on:click=move |_| show.set(false)
                    >
                        {move || i18n.t("asset.cancel")}
                    </button>
                    <button
                        class="flex-1 py-3 text-white font-medium bg-emerald-500 rounded-lg active:bg-emerald-600 disabled:opacity-50"
                        disabled=move || is_saving.get() || sell_price.get().is_empty()
                        on:click={
                            let mut asset = asset.clone();
                            move |_| {
                                is_saving.set(true);
                                let date = sell_date.get();
                                let price = Decimal::from_str(&sell_price.get()).unwrap_or(Decimal::ZERO);
                                let buyer_name = buyer.get();

                                asset.status = AssetStatus::Disposed;
                                asset.disposal_type = Some("sale".to_string());
                                asset.disposal_date = Some(date);
                                asset.disposal_proceeds = Some(price);
                                asset.disposal_reason = if buyer_name.is_empty() { None } else { Some(buyer_name) };
                                asset.updated_at = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();

                                let a = asset.clone();
                                let on_done = on_sold.clone();
                                leptos::task::spawn_local(async move {
                                    match asset_store::save_asset(&a).await {
                                        Ok(()) => { on_done.run(()); }
                                        Err(e) => {
                                            log::error!("Sale error: {}", e);
                                            is_saving.set(false);
                                        }
                                    }
                                });
                            }
                        }
                    >
                        {move || i18n.t("asset.sell_execute")}
                    </button>
                </div>
            </div>
        </div>
    }
}
