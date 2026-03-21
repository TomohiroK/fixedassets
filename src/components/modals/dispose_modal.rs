use leptos::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::i18n::use_i18n;
use crate::stores::asset_store;
use crate::models::asset::{Asset, AssetStatus};
use crate::models::depreciation;
use crate::models::company::CompanySetup;

/// Disposal modal with form
#[component]
pub fn DisposeModal(
    show: RwSignal<bool>,
    asset: Asset,
    on_disposed: Callback<()>,
) -> impl IntoView {
    let i18n = use_i18n();
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let disposal_date = RwSignal::new(today);
    let disposal_proceeds = RwSignal::new("0".to_string());
    let disposal_reason = RwSignal::new(String::new());
    let is_saving = RwSignal::new(false);

    // Calculate current book value for display
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

    view! {
        <div
            class=move || if show.get() {
                "fixed inset-0 z-50 flex items-end sm:items-center justify-center bg-black/50"
            } else {
                "hidden"
            }
            on:click=move |e| {
                // Close on backdrop click
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
                        <svg class="w-5 h-5 text-orange-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636"/>
                        </svg>
                        {move || i18n.t("asset.dispose_title")}
                    </h3>
                    <button
                        class="p-1 rounded-full hover:bg-gray-100"
                        on:click=move |_| show.set(false)
                    >
                        <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                        </svg>
                    </button>
                </div>

                // Current book value info
                <div class="bg-gray-50 rounded-lg p-3 mb-4">
                    <p class="text-xs text-gray-500">{move || i18n.t("asset.disposal_book_value")}</p>
                    <p class="text-lg font-bold text-gray-900">{format_bv.clone()}</p>
                </div>

                // Form
                <div class="space-y-4">
                    // Disposal date
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">
                            {move || i18n.t("asset.disposal_date")}
                        </label>
                        <input
                            type="date"
                            class="w-full px-3 py-2.5 border border-gray-200 rounded-lg text-sm"
                            prop:value=move || disposal_date.get()
                            on:input=move |e| {
                                use wasm_bindgen::JsCast;
                                let val = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>().value();
                                disposal_date.set(val);
                            }
                        />
                    </div>

                    // Disposal proceeds
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">
                            {move || i18n.t("asset.disposal_proceeds")}
                        </label>
                        <input
                            type="number"
                            min="0"
                            step="1"
                            class="w-full px-3 py-2.5 border border-gray-200 rounded-lg text-sm"
                            placeholder="0"
                            prop:value=move || disposal_proceeds.get()
                            on:input=move |e| {
                                use wasm_bindgen::JsCast;
                                let val = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>().value();
                                disposal_proceeds.set(val);
                            }
                        />
                        <p class="text-xs text-gray-400 mt-1">{move || i18n.t("asset.disposal_proceeds_hint")}</p>
                    </div>

                    // Disposal reason
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">
                            {move || i18n.t("asset.disposal_reason")}
                        </label>
                        <input
                            type="text"
                            class="w-full px-3 py-2.5 border border-gray-200 rounded-lg text-sm"
                            placeholder=move || i18n.t("asset.disposal_reason_placeholder")
                            prop:value=move || disposal_reason.get()
                            on:input=move |e| {
                                use wasm_bindgen::JsCast;
                                let val = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>().value();
                                disposal_reason.set(val);
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
                        class="flex-1 py-3 text-white font-medium bg-orange-500 rounded-lg active:bg-orange-600 disabled:opacity-50"
                        disabled=move || is_saving.get()
                        on:click={
                            let mut asset = asset.clone();
                            move |_| {
                                is_saving.set(true);
                                let date = disposal_date.get();
                                let proceeds = Decimal::from_str(&disposal_proceeds.get()).unwrap_or(Decimal::ZERO);
                                let reason = disposal_reason.get();

                                asset.status = AssetStatus::Disposed;
                                asset.disposal_type = Some("disposal".to_string());
                                asset.disposal_date = Some(date);
                                asset.disposal_proceeds = Some(proceeds);
                                asset.disposal_reason = if reason.is_empty() { None } else { Some(reason) };
                                asset.updated_at = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();

                                let a = asset.clone();
                                let on_done = on_disposed.clone();
                                leptos::task::spawn_local(async move {
                                    match asset_store::save_asset(&a).await {
                                        Ok(()) => {
                                            on_done.run(());
                                        }
                                        Err(e) => {
                                            log::error!("Dispose error: {}", e);
                                            is_saving.set(false);
                                        }
                                    }
                                });
                            }
                        }
                    >
                        {move || i18n.t("asset.dispose")}
                    </button>
                </div>
            </div>
        </div>
    }
}
