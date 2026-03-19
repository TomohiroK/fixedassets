use leptos::prelude::*;
use crate::i18n::use_i18n;
use crate::models::asset::Asset;
use crate::components::common::format_currency;

#[component]
pub fn AssetRow(asset: Asset) -> impl IntoView {
    let i18n = use_i18n();
    let id = asset.id.clone();
    let name = asset.name.clone();
    let cost = asset.cost;
    let status_class = asset.status.badge_class().to_string();
    let status_key = asset.status.i18n_key().to_string();

    view! {
        <a href=format!("/assets/{}", id) class="flex items-center justify-between py-3 px-4 active:bg-gray-50 transition-colors border-b border-gray-100">
            <div class="flex-1 min-w-0 mr-3">
                <p class="text-sm font-medium text-gray-900 truncate">{name}</p>
            </div>
            <div class="flex items-center gap-2 shrink-0">
                <span class="text-sm font-medium text-gray-700">{format_currency(&cost)}</span>
                <span class=format!("{} text-[10px] px-1.5 py-0.5", status_class)>{move || i18n.t(&status_key)}</span>
                <svg class="w-4 h-4 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                </svg>
            </div>
        </a>
    }
}
