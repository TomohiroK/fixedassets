use leptos::prelude::*;
use crate::i18n::use_i18n;
use crate::models::asset::Asset;
use crate::stores::photo_store;
use crate::components::common::format_currency;

/// EC-style product card with photo for grid display
#[component]
pub fn AssetProductCard(asset: Asset) -> impl IntoView {
    let i18n = use_i18n();
    let id = asset.id.clone();
    let asset_id_for_photo = asset.id.clone();
    let name = asset.name.clone();
    let cost = asset.cost;
    let category_key = asset.category.i18n_key().to_string();
    let status_class = asset.status.badge_class().to_string();
    let status_key = asset.status.i18n_key().to_string();
    let asset_num = asset.asset_number.clone();
    let has_num = !asset_num.is_empty();
    let emoji = asset.category.emoji().to_string();
    let emoji_for_fallback = emoji.clone();
    let emoji_for_suspense = emoji.clone();

    // Load first photo thumbnail
    let first_photo = LocalResource::new(move || {
        let aid = asset_id_for_photo.clone();
        async move {
            photo_store::get_photos_for_asset(&aid)
                .await
                .unwrap_or_default()
                .into_iter()
                .next()
        }
    });

    view! {
        <a href=format!("/assets/{}", id)
           class="block bg-white rounded-2xl overflow-hidden shadow-sm border border-gray-100 active:scale-[0.98] transition-transform"
        >
            // Photo area (square aspect ratio)
            <div class="aspect-square bg-gray-50 relative overflow-hidden">
                <Suspense fallback=move || view! {
                    <div class="w-full h-full flex items-center justify-center text-4xl opacity-30">
                        {emoji_for_suspense.clone()}
                    </div>
                }>
                    {move || {
                        let emoji_fallback = emoji_for_fallback.clone();
                        first_photo.get().map(move |data| {
                            match (*data).clone() {
                                Some(photo) => {
                                    let thumb = photo.thumbnail_url.clone();
                                    view! {
                                        <img src=thumb class="w-full h-full object-cover" loading="lazy" />
                                    }.into_any()
                                }
                                None => {
                                    view! {
                                        <div class="w-full h-full flex items-center justify-center text-4xl opacity-30">
                                            {emoji_fallback.clone()}
                                        </div>
                                    }.into_any()
                                }
                            }
                        })
                    }}
                </Suspense>
                // Status badge overlay
                <span class=format!("absolute top-2 right-2 {} text-[10px] px-1.5 py-0.5 shadow-sm", status_class)>
                    {move || i18n.t(&status_key)}
                </span>
            </div>
            // Info area
            <div class="p-3">
                {if has_num {
                    Some(view! { <p class="text-[10px] text-gray-400 font-mono leading-tight">{asset_num}</p> })
                } else {
                    None
                }}
                <p class="text-sm font-medium text-gray-900 truncate leading-snug">{name}</p>
                <p class="text-[10px] text-gray-500 mt-0.5">{move || i18n.t(&category_key)}</p>
                <p class="text-sm font-bold text-gray-900 mt-1.5">{format_currency(&cost)}</p>
            </div>
        </a>
    }
}

/// Compact list-style row (kept for potential reuse)
#[component]
pub fn AssetRow(asset: Asset) -> impl IntoView {
    let i18n = use_i18n();
    let id = asset.id.clone();
    let asset_num = asset.asset_number.clone();
    let has_num = !asset_num.is_empty();
    let name = asset.name.clone();
    let cost = asset.cost;
    let status_class = asset.status.badge_class().to_string();
    let status_key = asset.status.i18n_key().to_string();
    let category_key = asset.category.i18n_key().to_string();
    let tags = asset.tags.clone();
    let has_tags = !tags.is_empty();

    view! {
        <a href=format!("/assets/{}", id) class="flex items-center justify-between py-3 px-4 active:bg-gray-50 transition-colors border-b border-gray-100">
            <div class="flex-1 min-w-0 mr-3">
                {if has_num {
                    Some(view! { <p class="text-[10px] text-gray-400 leading-tight">{asset_num}</p> })
                } else {
                    None
                }}
                <p class="text-sm font-medium text-gray-900 truncate">{name}</p>
                <p class="text-[10px] text-gray-500">{move || i18n.t(&category_key)}</p>
                {if has_tags {
                    Some(view! {
                        <div class="flex gap-1 mt-0.5 overflow-hidden">
                            {tags.iter().take(3).map(|t| {
                                let tag = t.clone();
                                view! { <span class="text-[10px] bg-gray-100 text-gray-500 px-1.5 py-0.5 rounded-full truncate max-w-[60px]">{tag}</span> }
                            }).collect::<Vec<_>>()}
                            {if tags.len() > 3 {
                                Some(view! { <span class="text-[10px] text-gray-400">{format!("+{}", tags.len() - 3)}</span> })
                            } else {
                                None
                            }}
                        </div>
                    })
                } else {
                    None
                }}
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
