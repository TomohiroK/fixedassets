use leptos::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::i18n::use_i18n;
use crate::stores::asset_store;
use crate::models::asset::{Asset, AssetStatus};
use crate::models::depreciation;
use crate::models::company::CompanySetup;

/// Disposal modal with form — supports normal, casualty, disaster, theft sub-types
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
    let disposal_sub_type = RwSignal::new("normal".to_string());
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

    // Sub-type options: (value, i18n_key, icon_svg, color)
    let sub_types: Vec<(&str, &str, &str, &str)> = vec![
        ("normal", "asset.dispose_sub_normal", "M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636", "orange"),
        ("casualty", "asset.dispose_sub_casualty", "M17.657 18.657A8 8 0 016.343 7.343S7 9 9 10c0 .5.5 1 1 1s1.5-.5 2-1 1-2 1-2 2.5 1.5 4 3c.5.5 1.5 1 2.5 1s2-.5 2.5-1", "red"),
        ("disaster", "asset.dispose_sub_disaster", "M13 10V3L4 14h7v7l9-11h-7z", "amber"),
        ("theft", "asset.dispose_sub_theft", "M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z", "violet"),
    ];

    view! {
        <div
            class=move || if show.get() {
                "fixed inset-0 z-50 flex items-end sm:items-center justify-center bg-black/40 backdrop-blur-sm animate-fade-in"
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
            <div class="bg-white w-full max-w-lg rounded-t-2xl sm:rounded-2xl shadow-2xl max-h-[85vh] overflow-y-auto pb-24 animate-scale-in">
                // Header
                <div class="sticky top-0 bg-white border-b border-gray-100 px-4 py-3 flex items-center justify-between z-10">
                    <h3 class="text-base font-bold text-gray-900 flex items-center gap-2">
                        <svg class="w-5 h-5 text-orange-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636"/>
                        </svg>
                        {move || i18n.t("asset.dispose_title")}
                    </h3>
                    <button
                        class="p-1 text-gray-400"
                        on:click=move |_| show.set(false)
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                        </svg>
                    </button>
                </div>

                <div class="px-4 py-4 space-y-4">
                    // Current book value info
                    <div class="bg-gray-50 rounded-lg p-3">
                        <p class="text-xs text-gray-500">{move || i18n.t("asset.disposal_book_value")}</p>
                        <p class="text-xl font-bold text-gray-900 mt-0.5">{format_bv.clone()}</p>
                    </div>

                    // Disposal sub-type selector
                    <div>
                        <label class="block text-xs font-medium text-gray-700 mb-2">
                            {move || i18n.t("asset.dispose_sub_label")}
                        </label>
                        <div class="grid grid-cols-2 gap-2">
                            {sub_types.into_iter().map(|(val, key, icon_path, color)| {
                                let val_str = val.to_string();
                                let val_str2 = val.to_string();
                                let val_for_click = val.to_string();
                                let key = key.to_string();
                                let icon_path = icon_path.to_string();
                                let color = color.to_string();
                                let color2 = color.clone();

                                let btn_class = move || {
                                    let selected = disposal_sub_type.get() == val_str;
                                    if selected {
                                        match color.as_str() {
                                            "red" => "border-2 border-red-400 bg-red-50 text-red-700",
                                            "amber" => "border-2 border-amber-400 bg-amber-50 text-amber-700",
                                            "violet" => "border-2 border-violet-400 bg-violet-50 text-violet-700",
                                            _ => "border-2 border-orange-400 bg-orange-50 text-orange-700",
                                        }
                                    } else {
                                        "border border-gray-200 bg-white text-gray-600"
                                    }
                                };

                                let icon_color = move || {
                                    let selected = disposal_sub_type.get() == val_str2;
                                    if selected {
                                        match color2.as_str() {
                                            "red" => "text-red-500",
                                            "amber" => "text-amber-500",
                                            "violet" => "text-violet-500",
                                            _ => "text-orange-500",
                                        }
                                    } else {
                                        "text-gray-400"
                                    }
                                };

                                view! {
                                    <button
                                        class=move || format!(
                                            "flex items-center gap-2 p-2.5 rounded-lg transition-all text-left text-xs font-medium {}",
                                            btn_class()
                                        )
                                        on:click={
                                            let v = val_for_click.clone();
                                            move |_| disposal_sub_type.set(v.clone())
                                        }
                                    >
                                        <svg class=move || format!("w-4 h-4 shrink-0 {}", icon_color()) fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d=icon_path.clone()/>
                                        </svg>
                                        {move || i18n.t(&key)}
                                    </button>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>

                    // Disposal date
                    <div>
                        <label class="block text-xs font-medium text-gray-700 mb-1">
                            {move || i18n.t("asset.disposal_date")}
                        </label>
                        <input
                            type="date"
                            class="input-field"
                            prop:value=move || disposal_date.get()
                            on:input=move |e| {
                                disposal_date.set(event_target_value(&e));
                            }
                        />
                    </div>

                    // Disposal proceeds
                    <div>
                        <label class="block text-xs font-medium text-gray-700 mb-1">
                            {move || i18n.t("asset.disposal_proceeds")}
                        </label>
                        <input
                            type="number"
                            min="0"
                            step="1"
                            class="input-field"
                            placeholder="0"
                            prop:value=move || disposal_proceeds.get()
                            on:input=move |e| {
                                disposal_proceeds.set(event_target_value(&e));
                            }
                        />
                        <p class="text-xs text-gray-400 mt-1">{move || i18n.t("asset.disposal_proceeds_hint")}</p>
                    </div>

                    // Disposal reason — placeholder changes by sub-type
                    <div>
                        <label class="block text-xs font-medium text-gray-700 mb-1">
                            {move || i18n.t("asset.disposal_reason")}
                        </label>
                        <textarea
                            class="input-field min-h-[60px]"
                            prop:value=move || disposal_reason.get()
                            placeholder=move || {
                                match disposal_sub_type.get().as_str() {
                                    "casualty" => i18n.t("asset.dispose_reason_casualty_hint"),
                                    "disaster" => i18n.t("asset.dispose_reason_disaster_hint"),
                                    "theft" => i18n.t("asset.dispose_reason_theft_hint"),
                                    _ => i18n.t("asset.disposal_reason_placeholder"),
                                }
                            }
                            on:input=move |e| {
                                disposal_reason.set(event_target_value(&e));
                            }
                        ></textarea>
                    </div>

                    // Insurance / recovery amount hint for casualty/disaster/theft
                    {move || {
                        let sub = disposal_sub_type.get();
                        if sub == "casualty" || sub == "disaster" || sub == "theft" {
                            Some(view! {
                                <div class="bg-blue-50 border border-blue-200 rounded-lg p-3 flex items-start gap-2">
                                    <svg class="w-4 h-4 text-blue-500 shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                    </svg>
                                    <p class="text-xs text-blue-700">{move || i18n.t("asset.dispose_insurance_hint")}</p>
                                </div>
                            })
                        } else {
                            None
                        }
                    }}

                    // Buttons
                    <div class="flex gap-3">
                        <button
                            class="flex-1 py-3 text-gray-600 font-medium border border-gray-200 rounded-lg active:bg-gray-50 text-sm"
                            on:click=move |_| show.set(false)
                        >
                            {move || i18n.t("asset.cancel")}
                        </button>
                        <button
                            class="flex-1 py-3 text-white font-medium bg-orange-500 rounded-lg active:bg-orange-600 disabled:opacity-50 text-sm"
                            disabled=move || is_saving.get()
                            on:click={
                                let mut asset = asset.clone();
                                move |_| {
                                    is_saving.set(true);
                                    let date = disposal_date.get();
                                    let proceeds = Decimal::from_str(&disposal_proceeds.get()).unwrap_or(Decimal::ZERO);
                                    let reason = disposal_reason.get();
                                    let sub = disposal_sub_type.get();

                                    asset.status = AssetStatus::Disposed;
                                    asset.disposal_type = Some("disposal".to_string());
                                    asset.disposal_sub_type = Some(sub);
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
        </div>
    }
}
