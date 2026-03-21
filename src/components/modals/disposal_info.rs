use leptos::prelude::*;
use rust_decimal::Decimal;
use crate::i18n::use_i18n;
use crate::models::asset::Asset;
use crate::models::depreciation;
use crate::models::company::CompanySetup;

/// Disposal/Sale information display section
#[component]
pub fn DisposalInfoSection(asset: Asset, is_sale: bool) -> impl IntoView {
    let i18n = use_i18n();

    // Calculate book value at disposal and gain/loss
    let book_value_at_disposal = {
        let years = asset.useful_life + asset.prior_depreciation_years;
        let acc = depreciation::accumulated_depreciation(&asset, years);
        let bv = asset.cost - acc;
        if bv < Decimal::ZERO { Decimal::ZERO } else { bv }
    };

    let proceeds = asset.disposal_proceeds.unwrap_or(Decimal::ZERO);
    let gain_loss = proceeds - book_value_at_disposal;
    let is_gain = gain_loss > Decimal::ZERO;
    let is_loss = gain_loss < Decimal::ZERO;

    let format_amount = |amount: Decimal| -> String {
        let setup = CompanySetup::load();
        let symbol = setup.as_ref()
            .and_then(|s| s.currency().map(|c| c.symbol().to_string()))
            .unwrap_or_else(|| "$".to_string());
        format!("{}{}", symbol, amount.round_dp(0))
    };

    let date_display = asset.disposal_date.clone().unwrap_or_default();
    let reason_display = asset.disposal_reason.clone().unwrap_or_default();
    let bv_str = format_amount(book_value_at_disposal);
    let proceeds_str = format_amount(proceeds);
    let gl_str = format_amount(if gain_loss < Decimal::ZERO { -gain_loss } else { gain_loss });

    // Colors: sale = emerald, disposal = orange
    let bg_class = if is_sale { "bg-emerald-50 border-emerald-200" } else { "bg-orange-50 border-orange-200" };
    let title_color = if is_sale { "text-emerald-800" } else { "text-orange-800" };
    let border_color = if is_sale { "border-emerald-200" } else { "border-orange-200" };

    view! {
        <div class=format!("mt-4 border rounded-xl p-4 {}", bg_class)>
            <h3 class=format!("text-sm font-semibold mb-3 flex items-center gap-2 {}", title_color)>
                {if is_sale {
                    view! {
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                        </svg>
                    }.into_any()
                } else {
                    view! {
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636"/>
                        </svg>
                    }.into_any()
                }}
                {move || if is_sale { i18n.t("asset.sale_info") } else { i18n.t("asset.disposal_info") }}
            </h3>
            <div class="space-y-2 text-sm">
                // Type badge
                <div class="flex justify-between">
                    <span class="text-gray-600">
                        {if is_sale { "Type" } else { "Type" }}
                    </span>
                    <span class=if is_sale {
                        "text-xs font-medium px-2 py-0.5 rounded-full bg-emerald-100 text-emerald-700"
                    } else {
                        "text-xs font-medium px-2 py-0.5 rounded-full bg-orange-100 text-orange-700"
                    }>
                        {move || if is_sale { i18n.t("asset.type_sale") } else { i18n.t("asset.type_disposal") }}
                    </span>
                </div>
                // Date
                <div class="flex justify-between">
                    <span class="text-gray-600">
                        {move || if is_sale { i18n.t("asset.sell_date") } else { i18n.t("asset.disposal_date") }}
                    </span>
                    <span class="font-medium text-gray-900">{date_display.clone()}</span>
                </div>
                // Book value
                <div class="flex justify-between">
                    <span class="text-gray-600">{move || i18n.t("asset.disposal_book_value")}</span>
                    <span class="font-medium text-gray-900">{bv_str.clone()}</span>
                </div>
                // Proceeds / Sale price
                <div class="flex justify-between">
                    <span class="text-gray-600">
                        {move || if is_sale { i18n.t("asset.sale_price_label") } else { i18n.t("asset.disposal_proceeds") }}
                    </span>
                    <span class="font-medium text-gray-900">{proceeds_str.clone()}</span>
                </div>
                // Gain/Loss
                <div class=format!("flex justify-between border-t pt-2 mt-2 {}", border_color)>
                    <span class="text-gray-600 font-medium">
                        {move || if is_sale {
                            if is_gain { i18n.t("asset.sale_gain") } else { i18n.t("asset.sale_loss") }
                        } else {
                            if is_gain { i18n.t("asset.disposal_gain") } else { i18n.t("asset.disposal_loss") }
                        }}
                    </span>
                    <span class=if is_gain {
                        "font-bold text-green-600"
                    } else if is_loss {
                        "font-bold text-red-600"
                    } else {
                        "font-bold text-gray-600"
                    }>
                        {if is_loss { "-" } else { "" }}
                        {gl_str.clone()}
                    </span>
                </div>
                // Reason / Buyer
                {if !reason_display.is_empty() {
                    let label_is_sale = is_sale;
                    Some(view! {
                        <div class=format!("flex justify-between border-t pt-2 mt-2 {}", border_color)>
                            <span class="text-gray-600">
                                {move || if label_is_sale { i18n.t("asset.sale_buyer_label") } else { i18n.t("asset.disposal_reason") }}
                            </span>
                            <span class="font-medium text-gray-900 text-right max-w-[60%]">{reason_display.clone()}</span>
                        </div>
                    })
                } else {
                    None
                }}
            </div>
        </div>
    }
}
