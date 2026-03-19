use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::i18n::use_i18n;
use crate::stores::asset_store;
use crate::components::asset_form::AssetForm;

#[component]
pub fn AssetRegisterPage() -> impl IntoView {
    let i18n = use_i18n();
    let navigate = use_navigate();

    let on_submit = Callback::new(move |asset| {
        let navigate = navigate.clone();
        leptos::task::spawn_local(async move {
            match asset_store::save_asset(&asset).await {
                Ok(()) => {
                    navigate("/assets", Default::default());
                }
                Err(e) => {
                    log::error!("Failed to save asset: {}", e);
                }
            }
        });
    });

    view! {
        <div class="page-container">
            <h2 class="page-title">{move || i18n.t("asset.register")}</h2>
            <AssetForm
                on_submit=on_submit
                submit_label=Signal::derive(move || i18n.t("asset.save"))
            />
        </div>
    }
}
