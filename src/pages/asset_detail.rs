use leptos::prelude::*;
use leptos_router::hooks::{use_params_map, use_navigate};
use crate::i18n::use_i18n;
use crate::stores::asset_store;
use crate::models::asset::{AssetStatus, Category};
use crate::components::common::{LoadingSpinner, use_confirm, ConfirmStyle};
use crate::components::asset_detail::AssetDetailView;
use crate::components::asset_form::AssetForm;
use crate::components::modals::{DisposalInfoSection, DisposeModal, SellModal, CipTransferModal, ImpairmentModal, ImpairmentInfoSection, CapExModal, CapExInfoSection};

#[component]
pub fn AssetDetailPage() -> impl IntoView {
    let i18n = use_i18n();
    let params = use_params_map();
    let navigate = use_navigate();
    let is_editing = RwSignal::new(false);
    let show_dispose_modal = RwSignal::new(false);
    let show_sell_modal = RwSignal::new(false);
    let show_transfer_modal = RwSignal::new(false);
    let show_impairment_modal = RwSignal::new(false);
    let show_capex_modal = RwSignal::new(false);
    let refresh_trigger = RwSignal::new(0u32);
    let confirm = use_confirm();

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
                                let asset_for_dispose = asset_data.clone();
                                let asset_for_transfer = asset_data.clone();
                                let asset_for_sell = asset_data.clone();
                                let asset_for_impairment = asset_data.clone();
                                let asset_for_capex = asset_data.clone();
                                let asset_id_del = asset_data.id.clone();
                                let is_disposed = asset_data.status == AssetStatus::Disposed;
                                let is_sale = is_disposed && asset_data.disposal_type.as_deref() == Some("sale");
                                let is_cip = asset_data.category == Category::ConstructionInProgress && !is_disposed;
                                let has_impairments = !asset_data.impairments.is_empty();
                                let impairments_for_info = asset_data.impairments.clone();
                                let total_impairment = asset_data.total_impairment();
                                let has_capex = !asset_data.capex_records.is_empty();
                                let capex_for_info = asset_data.capex_records.clone();
                                let total_capex = asset_data.total_capex();
                                let original_cost = asset_data.cost;

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

                                                        // Disposal/Sale info section (show if disposed)
                                                        {if is_disposed {
                                                            let a = asset_for_dispose.clone();
                                                            Some(view! {
                                                                <DisposalInfoSection asset=a is_sale=is_sale />
                                                            })
                                                        } else {
                                                            None
                                                        }}

                                                        // Impairment info section (show if any impairments recorded)
                                                        {if has_impairments {
                                                            Some(view! {
                                                                <ImpairmentInfoSection
                                                                    impairments=impairments_for_info.clone()
                                                                    total_impairment=total_impairment
                                                                />
                                                            })
                                                        } else {
                                                            None
                                                        }}

                                                        // CapEx info section (show if any CapEx recorded)
                                                        {if has_capex {
                                                            Some(view! {
                                                                <CapExInfoSection
                                                                    capex_records=capex_for_info.clone()
                                                                    total_capex=total_capex
                                                                    original_cost=original_cost
                                                                />
                                                            })
                                                        } else {
                                                            None
                                                        }}

                                                        // Action buttons: CapEx + Transfer + Impairment + Dispose + Delete
                                                        <div class="mt-6 space-y-2">
                                                            // CIP Transfer button (only for ConstructionInProgress)
                                                            {if is_cip {
                                                                Some(view! {
                                                                    <button
                                                                        class="w-full py-3 text-blue-600 font-medium border border-blue-200 rounded-lg active:bg-blue-50 flex items-center justify-center gap-2"
                                                                        on:click=move |_| show_transfer_modal.set(true)
                                                                    >
                                                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4"/>
                                                                        </svg>
                                                                        {move || i18n.t("asset.transfer_cip")}
                                                                    </button>
                                                                }.into_any())
                                                            } else {
                                                                None
                                                            }}

                                                            // CapEx button (only for InUse depreciable assets, not CIP)
                                                            {if !is_disposed && !is_cip {
                                                                Some(view! {
                                                                    <button
                                                                        class="w-full py-3 text-teal-600 font-medium border border-teal-200 rounded-lg active:bg-teal-50 flex items-center justify-center gap-2"
                                                                        on:click=move |_| show_capex_modal.set(true)
                                                                    >
                                                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                                                        </svg>
                                                                        {move || i18n.t("asset.capex")}
                                                                    </button>
                                                                }.into_any())
                                                            } else {
                                                                None
                                                            }}

                                                            // Impairment button (only for InUse assets, not CIP or Land)
                                                            {if !is_disposed && !is_cip && asset_data.category != Category::Land {
                                                                Some(view! {
                                                                    <button
                                                                        class="w-full py-3 text-purple-600 font-medium border border-purple-200 rounded-lg active:bg-purple-50 flex items-center justify-center gap-2"
                                                                        on:click=move |_| show_impairment_modal.set(true)
                                                                    >
                                                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 17h8m0 0V9m0 8l-8-8-4 4-6-6"/>
                                                                        </svg>
                                                                        {move || i18n.t("asset.impairment")}
                                                                    </button>
                                                                }.into_any())
                                                            } else {
                                                                None
                                                            }}

                                                            // Dispose + Sell / Undo buttons
                                                            {if is_disposed {
                                                                // Undo disposal/sale button
                                                                let asset_for_undo = asset_for_dispose.clone();
                                                                Some(view! {
                                                                    <button
                                                                        class="w-full py-3 text-amber-600 font-medium border border-amber-200 rounded-lg active:bg-amber-50 flex items-center justify-center gap-2"
                                                                        on:click={
                                                                            let asset_clone = asset_for_undo.clone();
                                                                            let c = confirm.clone();
                                                                            move |_| {
                                                                                let mut a = asset_clone.clone();
                                                                                let msg = i18n.t("asset.undo_dispose_confirm");
                                                                                let ok_label = i18n.t("asset.undo_dispose");
                                                                                let cancel = i18n.t("asset.cancel");
                                                                                c.show(&msg, ConfirmStyle::Info, &ok_label, &cancel, move || {
                                                                                    a.status = AssetStatus::InUse;
                                                                                    a.disposal_type = None;
                                                                                    a.disposal_sub_type = None;
                                                                                    a.disposal_date = None;
                                                                                    a.disposal_proceeds = None;
                                                                                    a.disposal_reason = None;
                                                                                    a.updated_at = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
                                                                                    leptos::task::spawn_local(async move {
                                                                                        match asset_store::save_asset(&a).await {
                                                                                            Ok(()) => {
                                                                                                refresh_trigger.update(|v| *v += 1);
                                                                                            }
                                                                                            Err(e) => log::error!("Undo dispose error: {}", e),
                                                                                        }
                                                                                    });
                                                                                });
                                                                            }
                                                                        }
                                                                    >
                                                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6"/>
                                                                        </svg>
                                                                        {move || i18n.t("asset.undo_dispose")}
                                                                    </button>
                                                                }.into_any())
                                                            } else {
                                                                // Dispose + Sell buttons side by side
                                                                Some(view! {
                                                                    <div class="grid grid-cols-2 gap-2">
                                                                        <button
                                                                            class="py-3 text-orange-600 font-medium border border-orange-200 rounded-lg active:bg-orange-50 flex items-center justify-center gap-1.5 text-sm"
                                                                            on:click=move |_| show_dispose_modal.set(true)
                                                                        >
                                                                            <svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636"/>
                                                                            </svg>
                                                                            {move || i18n.t("asset.dispose")}
                                                                        </button>
                                                                        <button
                                                                            class="py-3 text-emerald-600 font-medium border border-emerald-200 rounded-lg active:bg-emerald-50 flex items-center justify-center gap-1.5 text-sm"
                                                                            on:click=move |_| show_sell_modal.set(true)
                                                                        >
                                                                            <svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                                                            </svg>
                                                                            {move || i18n.t("asset.sell")}
                                                                        </button>
                                                                    </div>
                                                                }.into_any())
                                                            }}

                                                            // Delete button
                                                            <button
                                                                class="w-full py-3 text-red-600 font-medium border border-red-200 rounded-lg active:bg-red-50 flex items-center justify-center gap-2"
                                                                on:click={
                                                                    let id = asset_id_del.clone();
                                                                    let nav = nav_del.clone();
                                                                    let c = confirm.clone();
                                                                    move |_| {
                                                                        let id = id.clone();
                                                                        let nav = nav.clone();
                                                                        let msg = i18n.t("asset.confirm_delete");
                                                                        let ok_label = i18n.t("asset.delete");
                                                                        let cancel = i18n.t("asset.cancel");
                                                                        c.show(&msg, ConfirmStyle::Danger, &ok_label, &cancel, move || {
                                                                            leptos::task::spawn_local(async move {
                                                                                match asset_store::delete_asset(&id).await {
                                                                                    Ok(()) => {
                                                                                        nav("/assets", Default::default());
                                                                                    }
                                                                                    Err(e) => log::error!("Delete error: {}", e),
                                                                                }
                                                                            });
                                                                        });
                                                                    }
                                                                }
                                                            >
                                                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
                                                                </svg>
                                                                {move || i18n.t("asset.delete")}
                                                            </button>
                                                        </div>

                                                        // Dispose modal overlay
                                                        <DisposeModal
                                                            show=show_dispose_modal
                                                            asset=asset_for_dispose.clone()
                                                            on_disposed=Callback::new(move |_| {
                                                                show_dispose_modal.set(false);
                                                                refresh_trigger.update(|v| *v += 1);
                                                            })
                                                        />

                                                        // Sell modal overlay
                                                        <SellModal
                                                            show=show_sell_modal
                                                            asset=asset_for_sell.clone()
                                                            on_sold=Callback::new(move |_| {
                                                                show_sell_modal.set(false);
                                                                refresh_trigger.update(|v| *v += 1);
                                                            })
                                                        />

                                                        // CIP Transfer modal overlay
                                                        <CipTransferModal
                                                            show=show_transfer_modal
                                                            asset=asset_for_transfer.clone()
                                                            on_transferred=Callback::new(move |_| {
                                                                show_transfer_modal.set(false);
                                                                refresh_trigger.update(|v| *v += 1);
                                                            })
                                                        />

                                                        // Impairment modal overlay
                                                        <ImpairmentModal
                                                            show=show_impairment_modal
                                                            asset=asset_for_impairment.clone()
                                                            on_recorded=Callback::new(move |_| {
                                                                show_impairment_modal.set(false);
                                                                refresh_trigger.update(|v| *v += 1);
                                                            })
                                                        />

                                                        // CapEx modal overlay
                                                        <CapExModal
                                                            show=show_capex_modal
                                                            asset=asset_for_capex.clone()
                                                            on_recorded=Callback::new(move |_| {
                                                                show_capex_modal.set(false);
                                                                refresh_trigger.update(|v| *v += 1);
                                                            })
                                                        />
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
