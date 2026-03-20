use leptos::prelude::*;
use crate::i18n::use_i18n;
use crate::stores::asset_store;
use crate::models::asset::{Asset, Category};
use crate::components::common::{LoadingSpinner, EmptyState, SearchBar};
use crate::components::asset_card::AssetProductCard;

#[component]
pub fn AssetListPage() -> impl IntoView {
    let i18n = use_i18n();
    let search_query = RwSignal::new(String::new());
    let selected_category = RwSignal::new(None::<usize>);
    let refresh_trigger = RwSignal::new(0u32);

    let assets = LocalResource::new(move || {
        refresh_trigger.get();
        async move {
            asset_store::get_all_assets().await.unwrap_or_default()
        }
    });

    view! {
        <div class="page-container">
            <h2 class="page-title">{move || i18n.t("asset.title")}</h2>

            // Search bar
            <div class="mb-4">
                <SearchBar
                    value=search_query
                    placeholder=Signal::derive(move || i18n.t("asset.search_placeholder"))
                />
            </div>

            <Suspense fallback=move || view! { <LoadingSpinner /> }>
                {move || {
                    assets.get().map(|data| {
                        let assets_vec: Vec<Asset> = (*data).clone();
                        let query = search_query.get().to_lowercase();
                        let cat_filter = selected_category.get();

                        // Count assets per category (for badges)
                        let category_counts: Vec<(usize, Category, usize)> = Category::all()
                            .into_iter()
                            .enumerate()
                            .map(|(idx, cat)| {
                                let count = assets_vec.iter()
                                    .filter(|a| a.category.to_index() == idx)
                                    .count();
                                (idx, cat, count)
                            })
                            .collect();

                        // Filter assets
                        let filtered: Vec<Asset> = assets_vec.into_iter().filter(|a| {
                            if let Some(cat_idx) = cat_filter {
                                if a.category.to_index() != cat_idx {
                                    return false;
                                }
                            }
                            if !query.is_empty() {
                                let matches = a.name.to_lowercase().contains(&query)
                                    || a.asset_number.to_lowercase().contains(&query)
                                    || a.location.to_lowercase().contains(&query)
                                    || a.description.to_lowercase().contains(&query)
                                    || a.tags.iter().any(|t| t.to_lowercase().contains(&query));
                                if !matches {
                                    return false;
                                }
                            }
                            true
                        }).collect();

                        let count = filtered.len();
                        view! {
                            <div>
                                // CATEGORY VIEW: show when no category is selected
                                <div class=move || if selected_category.get().is_none() && search_query.get().is_empty() {
                                    ""
                                } else {
                                    "hidden"
                                }>
                                    // Category grid (2 columns, photo cards)
                                    <div class="grid grid-cols-2 gap-3">
                                        {category_counts.iter().map(|(idx, cat, cnt)| {
                                            let image_url = cat.image_path().to_string();
                                            let key = cat.i18n_key().to_string();
                                            let count = *cnt;
                                            let idx = *idx;
                                            let bg_style = format!(
                                                "background-image: url('{}'); background-size: cover; background-position: center; min-height: 120px;",
                                                image_url
                                            );
                                            view! {
                                                <button
                                                    class="relative rounded-2xl overflow-hidden text-left active:scale-[0.97] transition-transform shadow-sm"
                                                    style=bg_style
                                                    on:click=move |_| selected_category.set(Some(idx))
                                                >
                                                    // Dark overlay for text readability
                                                    <div class="absolute inset-0 bg-gradient-to-t from-black/70 via-black/20 to-transparent"></div>
                                                    // Content at bottom
                                                    <div class="relative z-10 h-full flex flex-col justify-end p-3">
                                                        <p class="text-white font-semibold text-sm leading-tight drop-shadow-md">{move || i18n.t(&key)}</p>
                                                        <p class="text-white/80 text-xs mt-0.5 drop-shadow-md">
                                                            {count} " " {move || i18n.t("asset.items")}
                                                        </p>
                                                    </div>
                                                </button>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>

                                    // Total count at bottom
                                    <p class="text-center text-xs text-gray-400 mt-4">
                                        {count} " " {move || i18n.t("asset.total_items")}
                                    </p>
                                </div>

                                // PRODUCT LIST VIEW: show when category is selected or search is active
                                <div class=move || if selected_category.get().is_some() || !search_query.get().is_empty() {
                                    ""
                                } else {
                                    "hidden"
                                }>
                                    // Back button + category header
                                    {move || {
                                        if let Some(cat_idx) = selected_category.get() {
                                            let cat = Category::from_index(cat_idx);
                                            let key = cat.i18n_key().to_string();
                                            let emoji = cat.emoji().to_string();
                                            Some(view! {
                                                <div class="flex items-center gap-3 mb-4">
                                                    <button
                                                        class="p-2 -ml-2 rounded-lg active:bg-gray-100"
                                                        on:click=move |_| selected_category.set(None)
                                                    >
                                                        <svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                                                        </svg>
                                                    </button>
                                                    <div class="flex items-center gap-2">
                                                        <span class="text-xl">{emoji}</span>
                                                        <h3 class="font-semibold text-gray-900">{move || i18n.t(&key)}</h3>
                                                    </div>
                                                    <span class="ml-auto text-xs text-gray-400 bg-gray-100 px-2 py-1 rounded-full">
                                                        {count} " " {move || i18n.t("asset.items")}
                                                    </span>
                                                </div>
                                            })
                                        } else {
                                            None
                                        }
                                    }}

                                    // Category filter pills (horizontal scroll) — when in search mode
                                    {move || {
                                        if !search_query.get().is_empty() {
                                            Some(view! {
                                                <div class="-mx-4 px-4 overflow-x-auto mb-3">
                                                    <div class="flex gap-1.5 pb-1" style="min-width: max-content;">
                                                        <button
                                                            class=move || if selected_category.get().is_none() {
                                                                "shrink-0 px-3 py-1.5 rounded-full text-xs font-medium bg-blue-600 text-white"
                                                            } else {
                                                                "shrink-0 px-3 py-1.5 rounded-full text-xs font-medium bg-gray-100 text-gray-600 active:bg-gray-200"
                                                            }
                                                            on:click=move |_| selected_category.set(None)
                                                        >
                                                            {move || i18n.t("asset.filter_all")}
                                                        </button>
                                                        {Category::all().into_iter().enumerate().map(|(idx, cat)| {
                                                            let key = cat.i18n_key().to_string();
                                                            let emoji = cat.emoji().to_string();
                                                            view! {
                                                                <button
                                                                    class=move || if selected_category.get() == Some(idx) {
                                                                        "shrink-0 px-3 py-1.5 rounded-full text-xs font-medium bg-blue-600 text-white"
                                                                    } else {
                                                                        "shrink-0 px-3 py-1.5 rounded-full text-xs font-medium bg-gray-100 text-gray-600 active:bg-gray-200"
                                                                    }
                                                                    on:click=move |_| {
                                                                        if selected_category.get() == Some(idx) {
                                                                            selected_category.set(None);
                                                                        } else {
                                                                            selected_category.set(Some(idx));
                                                                        }
                                                                    }
                                                                >
                                                                    {emoji.clone()} " " {move || i18n.t(&key)}
                                                                </button>
                                                            }
                                                        }).collect::<Vec<_>>()}
                                                    </div>
                                                </div>
                                            })
                                        } else {
                                            None
                                        }
                                    }}

                                    // Product grid (2 columns, EC style)
                                    {if filtered.is_empty() {
                                        view! {
                                            <EmptyState message=i18n.t("asset.no_assets") />
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class="grid grid-cols-2 gap-3">
                                                {filtered.into_iter().map(|asset| {
                                                    view! { <AssetProductCard asset=asset /> }
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        }.into_any()
                                    }}
                                </div>
                            </div>
                        }.into_any()
                    })
                }}
            </Suspense>
        </div>
    }
}
