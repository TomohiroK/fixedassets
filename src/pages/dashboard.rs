use leptos::prelude::*;
use crate::i18n::use_i18n;
use crate::stores::asset_store;
use crate::models::asset::Asset;
use crate::components::common::{LoadingSpinner, EmptyState};
use crate::components::dashboard::DashboardSummary;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let i18n = use_i18n();

    let assets = LocalResource::new(move || async move {
        asset_store::get_all_assets().await.unwrap_or_default()
    });

    view! {
        <div class="page-container">
            <h2 class="page-title">{move || i18n.t("dashboard.title")}</h2>
            <Suspense fallback=move || view! { <LoadingSpinner /> }>
                {move || {
                    assets.get().map(|data| {
                        let assets_vec: Vec<Asset> = (*data).clone();
                        if assets_vec.is_empty() {
                            view! {
                                <EmptyState
                                    message=i18n.t("dashboard.no_assets")
                                    sub_message=i18n.t("dashboard.get_started")
                                />
                            }.into_any()
                        } else {
                            view! {
                                <div>
                                    // Depreciation processing link
                                    <a
                                        href="/depreciation"
                                        class="block mb-4 p-4 bg-gradient-to-r from-emerald-500 to-teal-600 rounded-xl text-white active:opacity-90"
                                    >
                                        <div class="flex items-center justify-between">
                                            <div class="flex items-center gap-3">
                                                <div class="w-10 h-10 bg-white/20 rounded-lg flex items-center justify-center">
                                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z"/>
                                                    </svg>
                                                </div>
                                                <div>
                                                    <p class="font-bold text-sm">{move || i18n.t("dep_post.title")}</p>
                                                    <p class="text-xs text-white/80">{move || i18n.t("dep_post.action_process")}</p>
                                                </div>
                                            </div>
                                            <svg class="w-5 h-5 text-white/60" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                                            </svg>
                                        </div>
                                    </a>
                                    <DashboardSummary assets=assets_vec />
                                </div>
                            }.into_any()
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
