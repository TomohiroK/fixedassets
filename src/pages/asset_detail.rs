use leptos::prelude::*;
use leptos_router::hooks::{use_params_map, use_navigate};
use crate::i18n::use_i18n;
use crate::stores::asset_store;
use crate::components::common::LoadingSpinner;
use crate::components::asset_detail::AssetDetailView;
use crate::components::asset_form::AssetForm;

#[component]
pub fn AssetDetailPage() -> impl IntoView {
    let i18n = use_i18n();
    let params = use_params_map();
    let navigate = use_navigate();
    let is_editing = RwSignal::new(false);
    let refresh_trigger = RwSignal::new(0u32);

    let asset = LocalResource::new(move || {
        let id = params.get().get("id").unwrap_or_default();
        refresh_trigger.get();
        async move {
            asset_store::get_asset(&id).await.unwrap_or(None)
        }
    });

    let nav_for_delete = navigate.clone();

    view! {
        <div class="page-container">
            <Suspense fallback=move || view! { <LoadingSpinner /> }>
                {move || {
                    let nav_del = nav_for_delete.clone();
                    asset.get().map(|data| {
                        match (*data).clone() {
                            Some(asset_data) => {
                                let asset_for_detail = asset_data.clone();
                                let asset_for_form = asset_data.clone();
                                let asset_id_del = asset_data.id.clone();

                                view! {
                                    <div>
                                        {move || {
                                            if is_editing.get() {
                                                let on_submit = {
                                                    Callback::new(move |updated_asset| {
                                                        leptos::task::spawn_local(async move {
                                                            match asset_store::save_asset(&updated_asset).await {
                                                                Ok(()) => {
                                                                    is_editing.set(false);
                                                                    refresh_trigger.update(|v| *v += 1);
                                                                }
                                                                Err(e) => log::error!("Save error: {}", e),
                                                            }
                                                        });
                                                    })
                                                };
                                                view! {
                                                    <div>
                                                        <div class="flex items-center justify-between mb-4">
                                                            <h2 class="page-title mb-0">{move || i18n.t("asset.edit")}</h2>
                                                            <button
                                                                class="text-sm text-gray-500"
                                                                on:click=move |_| is_editing.set(false)
                                                            >
                                                                {move || i18n.t("asset.cancel")}
                                                            </button>
                                                        </div>
                                                        <AssetForm
                                                            initial=asset_for_form.clone()
                                                            on_submit=on_submit
                                                            submit_label=Signal::derive(move || i18n.t("asset.save"))
                                                        />
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <div>
                                                        <div class="flex items-center justify-between mb-4">
                                                            <h2 class="page-title mb-0">{move || i18n.t("asset.detail")}</h2>
                                                            <div class="flex gap-2">
                                                                <button
                                                                    class="text-sm text-blue-600 font-medium"
                                                                    on:click=move |_| is_editing.set(true)
                                                                >
                                                                    {move || i18n.t("asset.edit")}
                                                                </button>
                                                            </div>
                                                        </div>
                                                        <AssetDetailView asset=asset_for_detail.clone() />
                                                        <div class="mt-6">
                                                            <button
                                                                class="w-full py-3 text-red-600 font-medium border border-red-200 rounded-lg active:bg-red-50"
                                                                on:click={
                                                                    let id = asset_id_del.clone();
                                                                    let nav = nav_del.clone();
                                                                    move |_| {
                                                                        let id = id.clone();
                                                                        let nav = nav.clone();
                                                                        let msg = i18n.t("asset.confirm_delete");
                                                                        let window = web_sys::window().unwrap();
                                                                        if window.confirm_with_message(&msg).unwrap_or(false) {
                                                                            leptos::task::spawn_local(async move {
                                                                                match asset_store::delete_asset(&id).await {
                                                                                    Ok(()) => {
                                                                                        nav("/assets", Default::default());
                                                                                    }
                                                                                    Err(e) => log::error!("Delete error: {}", e),
                                                                                }
                                                                            });
                                                                        }
                                                                    }
                                                                }
                                                            >
                                                                {move || i18n.t("asset.delete")}
                                                            </button>
                                                        </div>
                                                    </div>
                                                }.into_any()
                                            }
                                        }}
                                    </div>
                                }.into_any()
                            }
                            None => {
                                view! {
                                    <div class="text-center py-12 text-gray-500">
                                        "Asset not found"
                                    </div>
                                }.into_any()
                            }
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
