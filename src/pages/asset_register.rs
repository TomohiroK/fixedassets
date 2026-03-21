use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::i18n::use_i18n;
use crate::auth::{use_auth, FREE_ASSET_LIMIT};
use crate::stores::asset_store;
use crate::components::asset_form::AssetForm;

#[component]
pub fn AssetRegisterPage() -> impl IntoView {
    let i18n = use_i18n();
    let auth = use_auth();
    let navigate = use_navigate();

    let limit_check = LocalResource::new(move || {
        let is_paid = auth.is_paid();
        async move {
            if is_paid {
                return (true, 0);
            }
            let assets = asset_store::get_all_assets().await.unwrap_or_default();
            (assets.len() < FREE_ASSET_LIMIT, assets.len())
        }
    });

    let on_submit = Callback::new(move |assets: Vec<crate::models::asset::Asset>| {
        let navigate = navigate.clone();
        leptos::task::spawn_local(async move {
            for asset in &assets {
                match asset_store::save_asset(asset).await {
                    Ok(()) => {},
                    Err(e) => {
                        log::error!("Failed to save asset: {}", e);
                        return;
                    }
                }
            }
            navigate("/assets", Default::default());
        });
    });

    view! {
        <div class="page-container">
            <h2 class="page-title">{move || i18n.t("asset.register")}</h2>
            <Suspense fallback=move || view! { <p class="text-sm text-gray-500">{move || i18n.t("common.loading")}</p> }>
                {move || {
                    limit_check.get().map(|result| {
                        let (can_register, count) = *result;
                        if can_register {
                            view! {
                                <div>
                                    {if !auth.is_paid() {
                                        Some(view! {
                                            <div class="mb-4 p-3 bg-amber-50 border border-amber-200 rounded-lg text-sm text-amber-700">
                                                {move || i18n.t("plan.free_limit_info")}
                                                " (" {count} "/" {FREE_ASSET_LIMIT} ")"
                                            </div>
                                        })
                                    } else {
                                        None
                                    }}
                                    <AssetForm
                                        on_submit=on_submit
                                        submit_label=Signal::derive(move || i18n.t("asset.save"))
                                    />
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="card text-center py-8">
                                    <div class="w-16 h-16 bg-amber-100 rounded-full mx-auto flex items-center justify-center mb-4">
                                        <svg class="w-8 h-8 text-amber-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m0 0v2m0-2h2m-2 0H10m-4.93-4.364A9 9 0 1121 12a9 9 0 01-15.93-.364z"/>
                                        </svg>
                                    </div>
                                    <p class="font-semibold text-gray-900 mb-2">{move || i18n.t("plan.limit_reached")}</p>
                                    <p class="text-sm text-gray-500 mb-4">
                                        {move || i18n.t("plan.limit_message")}
                                        " (" {FREE_ASSET_LIMIT} " assets)"
                                    </p>
                                    <a href="/settings" class="text-sm text-blue-600 font-medium">
                                        {move || i18n.t("plan.upgrade_link")}
                                    </a>
                                </div>
                            }.into_any()
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
