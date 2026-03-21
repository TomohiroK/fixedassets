use leptos::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::i18n::use_i18n;
use crate::stores::asset_store;
use crate::models::asset::{Asset, Category, DepreciationMethod};
use crate::models::company::CompanySetup;

/// CIP Transfer modal — transfer from ConstructionInProgress to a fixed asset category
#[component]
pub fn CipTransferModal(
    show: RwSignal<bool>,
    asset: Asset,
    on_transferred: Callback<()>,
) -> impl IntoView {
    let i18n = use_i18n();
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

    // Form state — default category to Building (index 1)
    let selected_category = RwSignal::new(1usize);
    let transfer_date = RwSignal::new(today);
    let useful_life = RwSignal::new("10".to_string());
    let salvage_value = RwSignal::new("1".to_string());
    let dep_method = RwSignal::new(0usize); // 0 = SL, 1 = DB
    let is_saving = RwSignal::new(false);

    // Transferable categories (exclude Land, CIP, and non-depreciable)
    let transferable: Vec<(usize, Category)> = Category::all()
        .into_iter()
        .enumerate()
        .filter(|(_, cat)| {
            !matches!(cat, Category::Land | Category::ConstructionInProgress)
        })
        .collect();

    let format_cost = {
        let setup = CompanySetup::load();
        let symbol = setup.as_ref()
            .and_then(|s| s.currency().map(|c| c.symbol().to_string()))
            .unwrap_or_else(|| "$".to_string());
        format!("{}{}", symbol, asset.cost.round_dp(0))
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
                        <svg class="w-5 h-5 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4"/>
                        </svg>
                        {move || i18n.t("asset.transfer_cip_title")}
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

                // Description
                <p class="text-xs text-gray-500 mb-4">{move || i18n.t("asset.transfer_cip_desc")}</p>

                // Current cost info
                <div class="bg-blue-50 rounded-lg p-3 mb-4">
                    <p class="text-xs text-blue-600">{move || i18n.t("asset.cost")}</p>
                    <p class="text-lg font-bold text-gray-900">{format_cost.clone()}</p>
                    <p class="text-xs text-gray-500 mt-0.5">{asset.name.clone()}</p>
                </div>

                // Form fields
                <div class="space-y-4">
                    // Target category
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">
                            {move || i18n.t("asset.transfer_category")}
                        </label>
                        <select
                            class="w-full px-3 py-2.5 border border-gray-200 rounded-lg text-sm bg-white"
                            on:change=move |e| {
                                use wasm_bindgen::JsCast;
                                let val = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>().value();
                                if let Ok(idx) = val.parse::<usize>() {
                                    selected_category.set(idx);
                                }
                            }
                        >
                            {transferable.iter().map(|(idx, cat)| {
                                let key = cat.i18n_key().to_string();
                                let emoji = cat.emoji().to_string();
                                let idx_val = *idx;
                                let selected = idx_val == 1; // Building default
                                view! {
                                    <option value=idx_val.to_string() selected=selected>
                                        {emoji} " " {move || i18n.t(&key)}
                                    </option>
                                }
                            }).collect::<Vec<_>>()}
                        </select>
                    </div>

                    // Transfer date
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">
                            {move || i18n.t("asset.transfer_date")}
                        </label>
                        <input
                            type="date"
                            class="w-full px-3 py-2.5 border border-gray-200 rounded-lg text-sm"
                            prop:value=move || transfer_date.get()
                            on:input=move |e| {
                                use wasm_bindgen::JsCast;
                                let val = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>().value();
                                transfer_date.set(val);
                            }
                        />
                    </div>

                    // Useful life
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">
                            {move || i18n.t("asset.transfer_useful_life")}
                        </label>
                        <input
                            type="number"
                            min="1"
                            max="100"
                            class="w-full px-3 py-2.5 border border-gray-200 rounded-lg text-sm"
                            prop:value=move || useful_life.get()
                            on:input=move |e| {
                                use wasm_bindgen::JsCast;
                                let val = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>().value();
                                useful_life.set(val);
                            }
                        />
                    </div>

                    // Depreciation method
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">
                            {move || i18n.t("asset.transfer_method")}
                        </label>
                        <select
                            class="w-full px-3 py-2.5 border border-gray-200 rounded-lg text-sm bg-white"
                            on:change=move |e| {
                                use wasm_bindgen::JsCast;
                                let val = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>().value();
                                if let Ok(idx) = val.parse::<usize>() {
                                    dep_method.set(idx);
                                }
                            }
                        >
                            {DepreciationMethod::all().into_iter().enumerate().map(|(idx, m)| {
                                let key = m.i18n_key().to_string();
                                view! {
                                    <option value=idx.to_string()>
                                        {move || i18n.t(&key)}
                                    </option>
                                }
                            }).collect::<Vec<_>>()}
                        </select>
                    </div>

                    // Salvage value
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">
                            {move || i18n.t("asset.transfer_salvage")}
                        </label>
                        <input
                            type="number"
                            min="0"
                            step="1"
                            class="w-full px-3 py-2.5 border border-gray-200 rounded-lg text-sm"
                            prop:value=move || salvage_value.get()
                            on:input=move |e| {
                                use wasm_bindgen::JsCast;
                                let val = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>().value();
                                salvage_value.set(val);
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
                        class="flex-1 py-3 text-white font-medium bg-blue-500 rounded-lg active:bg-blue-600 disabled:opacity-50"
                        disabled=move || is_saving.get()
                        on:click={
                            let mut asset = asset.clone();
                            move |_| {
                                is_saving.set(true);
                                let cat = Category::from_index(selected_category.get());
                                let date = transfer_date.get();
                                let life: u32 = useful_life.get().parse().unwrap_or(10);
                                let salvage = Decimal::from_str(&salvage_value.get()).unwrap_or(Decimal::ONE);
                                let method = DepreciationMethod::from_index(dep_method.get());

                                // Update asset: change category, set acquisition date to transfer date, reset depreciation
                                asset.category = cat;
                                asset.acquisition_date = date;
                                asset.useful_life = life;
                                asset.salvage_value = salvage;
                                asset.depreciation_method = method;
                                asset.prior_depreciation_years = 0;
                                asset.prior_depreciation_months = 0;
                                // Add tag to mark it was transferred from CIP
                                if !asset.tags.iter().any(|t| t == "CIP Transfer" || t == "建設仮勘定振替") {
                                    asset.tags.push("CIP Transfer".to_string());
                                }
                                asset.updated_at = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();

                                let a = asset.clone();
                                let on_done = on_transferred.clone();
                                leptos::task::spawn_local(async move {
                                    match asset_store::save_asset(&a).await {
                                        Ok(()) => {
                                            on_done.run(());
                                        }
                                        Err(e) => {
                                            log::error!("Transfer error: {}", e);
                                            is_saving.set(false);
                                        }
                                    }
                                });
                            }
                        }
                    >
                        {move || i18n.t("asset.transfer_execute")}
                    </button>
                </div>
            </div>
        </div>
    }
}
