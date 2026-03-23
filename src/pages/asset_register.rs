use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::i18n::use_i18n;
use crate::auth::{use_auth, FREE_ASSET_LIMIT};
use crate::stores::asset_store;
use crate::components::asset_form::AssetForm;
use crate::components::wizard_form::WizardForm;

#[component]
pub fn AssetRegisterPage() -> impl IntoView {
    let i18n = use_i18n();
    let auth = use_auth();
    let navigate = use_navigate();

    // false = standard form, true = wizard/guided mode
    let wizard_mode = RwSignal::new(false);

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
            match asset_store::batch_save_assets(&assets).await {
                Ok(()) => {},
                Err(e) => {
                    log::error!("Failed to save assets: {}", e);
                    return;
                }
            }
            navigate("/assets", Default::default());
        });
    });

    // Need a second callback for wizard (same logic)
    let navigate2 = use_navigate();
    let on_submit_wizard = Callback::new(move |assets: Vec<crate::models::asset::Asset>| {
        let navigate = navigate2.clone();
        leptos::task::spawn_local(async move {
            match asset_store::batch_save_assets(&assets).await {
                Ok(()) => {},
                Err(e) => {
                    log::error!("Failed to save assets: {}", e);
                    return;
                }
            }
            navigate("/assets", Default::default());
        });
    });

    view! {
        <div class="page-container">
            <h2 class="page-title">{move || i18n.t("asset.register")}</h2>

            // Mode toggle
            <div class="mb-4">
                <div class="flex bg-gray-100 rounded-lg p-1">
                    <button
                        type="button"
                        class=move || if !wizard_mode.get() {
                            "flex-1 py-2 px-3 rounded-md text-sm font-semibold bg-white text-gray-900 shadow-sm transition-all"
                        } else {
                            "flex-1 py-2 px-3 rounded-md text-sm font-medium text-gray-500 transition-all"
                        }
                        on:click=move |_| wizard_mode.set(false)
                    >
                        <div class="flex items-center justify-center gap-1.5">
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                            </svg>
                            {move || i18n.t("wizard.mode_standard")}
                        </div>
                    </button>
                    <button
                        type="button"
                        class=move || if wizard_mode.get() {
                            "flex-1 py-2 px-3 rounded-md text-sm font-semibold bg-white text-gray-900 shadow-sm transition-all"
                        } else {
                            "flex-1 py-2 px-3 rounded-md text-sm font-medium text-gray-500 transition-all"
                        }
                        on:click=move |_| wizard_mode.set(true)
                    >
                        <div class="flex items-center justify-center gap-1.5">
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/>
                            </svg>
                            {move || i18n.t("wizard.mode_guided")}
                        </div>
                    </button>
                </div>
            </div>

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
                                    {move || if wizard_mode.get() {
                                        view! {
                                            <WizardForm
                                                on_submit=on_submit_wizard
                                                submit_label=Signal::derive(move || i18n.t("asset.save"))
                                            />
                                        }.into_any()
                                    } else {
                                        view! {
                                            <AssetForm
                                                on_submit=on_submit
                                                submit_label=Signal::derive(move || i18n.t("asset.save"))
                                            />
                                        }.into_any()
                                    }}
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
