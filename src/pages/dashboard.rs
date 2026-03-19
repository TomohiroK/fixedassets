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
                                <DashboardSummary assets=assets_vec />
                            }.into_any()
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
