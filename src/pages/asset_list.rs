use leptos::prelude::*;
use crate::i18n::use_i18n;
use crate::stores::asset_store;
use crate::models::asset::Asset;
use crate::components::common::{LoadingSpinner, EmptyState, SearchBar};
use crate::components::asset_card::AssetRow;

#[component]
pub fn AssetListPage() -> impl IntoView {
    let i18n = use_i18n();
    let search_query = RwSignal::new(String::new());
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

                        let filtered: Vec<Asset> = if query.is_empty() {
                            assets_vec
                        } else {
                            assets_vec.into_iter().filter(|a| {
                                a.name.to_lowercase().contains(&query)
                                || a.location.to_lowercase().contains(&query)
                                || a.description.to_lowercase().contains(&query)
                                || a.tags.iter().any(|t| t.to_lowercase().contains(&query))
                            }).collect()
                        };

                        let count = filtered.len();

                        if filtered.is_empty() {
                            view! {
                                <EmptyState message=i18n.t("asset.no_assets") />
                            }.into_any()
                        } else {
                            view! {
                                <div>
                                    <p class="text-xs text-gray-400 mb-2">{count} " items"</p>
                                    <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
                                        {filtered.into_iter().map(|asset| {
                                            view! { <AssetRow asset=asset /> }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            }.into_any()
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
