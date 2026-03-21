use leptos::prelude::*;
use crate::i18n::use_i18n;
use crate::models::asset::{Asset, TransferRecord};
use crate::models::department::Department;
use crate::stores::asset_store;

/// Modal for transferring an asset to a different department
#[component]
pub fn TransferDeptModal(
    show: RwSignal<bool>,
    asset: Asset,
    on_transferred: Callback<()>,
) -> impl IntoView {
    let i18n = use_i18n();

    let departments = Department::load_all();
    let has_departments = !departments.is_empty();

    let current_dept_name = asset.department_id.as_ref()
        .map(|id| Department::display_name(id))
        .unwrap_or_else(|| i18n.t("asset.dept_unassigned"));

    let today_str = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let selected_dept = RwSignal::new(String::new());
    let transfer_date = RwSignal::new(today_str);
    let transfer_reason = RwSignal::new(String::new());
    let is_saving = RwSignal::new(false);

    let current_dept_id_for_valid = asset.department_id.clone();
    let is_valid = move || {
        let dept = selected_dept.get();
        !dept.is_empty()
            && !transfer_date.get().is_empty()
            && current_dept_id_for_valid.as_deref() != Some(&dept)
    };
    let is_valid2 = {
        let current_dept_id_for_valid2 = asset.department_id.clone();
        move || {
            let dept = selected_dept.get();
            !dept.is_empty()
                && !transfer_date.get().is_empty()
                && current_dept_id_for_valid2.as_deref() != Some(&dept)
        }
    };

    let asset_for_save = asset.clone();
    let current_dept_for_list = asset.department_id.clone();
    let departments_for_list = departments.clone();

    view! {
        <div
            class=move || if show.get() {
                "fixed inset-0 z-50 flex items-end justify-center bg-black/40 backdrop-blur-sm animate-fade-in"
            } else {
                "hidden"
            }
            on:click=move |e| {
                use wasm_bindgen::JsCast;
                if let Some(target) = e.target() {
                    if let Some(el) = target.dyn_ref::<web_sys::HtmlElement>() {
                        if el.class_list().contains("fixed") {
                            show.set(false);
                        }
                    }
                }
            }
        >
            <div class="bg-white rounded-t-2xl shadow-2xl w-full max-w-lg max-h-[85vh] overflow-y-auto pb-24 animate-scale-in">
                // Header
                <div class="sticky top-0 bg-white border-b border-gray-100 px-4 py-3 flex items-center justify-between z-10">
                    <h2 class="text-base font-bold text-gray-900 flex items-center gap-2">
                        <svg class="w-5 h-5 text-indigo-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4"/>
                        </svg>
                        {move || i18n.t("asset.transfer_dept_title")}
                    </h2>
                    <button class="text-gray-400 p-1" on:click=move |_| show.set(false)>
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                        </svg>
                    </button>
                </div>

                <div class="px-4 py-4 space-y-4">
                    {if !has_departments {
                        // No departments configured
                        Some(view! {
                            <div class="bg-amber-50 border border-amber-200 rounded-lg p-4 text-center">
                                <svg class="w-8 h-8 text-amber-400 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 16.5c-.77.833.192 2.5 1.732 2.5z"/>
                                </svg>
                                <p class="text-sm text-amber-800 font-medium">{move || i18n.t("asset.dept_none_configured")}</p>
                                <p class="text-xs text-amber-600 mt-1">{move || i18n.t("asset.dept_go_settings")}</p>
                            </div>
                        })
                    } else {
                        None
                    }}

                    {if has_departments {
                        Some(view! {
                            <div class="space-y-4">
                                <p class="text-xs text-gray-500">{move || i18n.t("asset.transfer_dept_desc")}</p>

                                // Current department
                                <div class="bg-gray-50 rounded-lg p-3">
                                    <p class="text-xs text-gray-500">{move || i18n.t("asset.dept_current")}</p>
                                    <p class="text-sm font-bold text-gray-900 mt-0.5">{current_dept_name.clone()}</p>
                                </div>

                                // Target department
                                <div>
                                    <label class="block text-xs font-medium text-gray-700 mb-1">
                                        {move || i18n.t("asset.dept_destination")}
                                    </label>
                                    <select
                                        class="input-field"
                                        on:change=move |ev| {
                                            selected_dept.set(event_target_value(&ev));
                                        }
                                    >
                                        <option value="">{move || i18n.t("asset.dept_select")}</option>
                                        {departments_for_list.clone().into_iter().map(|dept| {
                                            let dept_id = dept.id.clone();
                                            let is_current = current_dept_for_list.as_deref() == Some(&dept_id);
                                            let label = if dept.code.is_empty() {
                                                dept.name.clone()
                                            } else {
                                                format!("{} - {}", dept.code, dept.name)
                                            };
                                            view! {
                                                <option
                                                    value=dept_id
                                                    disabled=is_current
                                                >
                                                    {label}
                                                    {if is_current { " (current)" } else { "" }}
                                                </option>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </select>
                                </div>

                                // Transfer date
                                <div>
                                    <label class="block text-xs font-medium text-gray-700 mb-1">
                                        {move || i18n.t("asset.transfer_dept_date")}
                                    </label>
                                    <input
                                        type="date"
                                        class="input-field"
                                        prop:value=move || transfer_date.get()
                                        on:input=move |ev| transfer_date.set(event_target_value(&ev))
                                    />
                                </div>

                                // Reason
                                <div>
                                    <label class="block text-xs font-medium text-gray-700 mb-1">
                                        {move || i18n.t("asset.transfer_dept_reason")}
                                    </label>
                                    <textarea
                                        class="input-field min-h-[60px]"
                                        placeholder=move || i18n.t("asset.transfer_dept_reason_hint")
                                        prop:value=move || transfer_reason.get()
                                        on:input=move |ev| transfer_reason.set(event_target_value(&ev))
                                    ></textarea>
                                </div>

                                // Execute button
                                <button
                                    class=move || format!(
                                        "w-full py-3 rounded-lg font-medium text-sm flex items-center justify-center gap-2 {}",
                                        if is_valid() && !is_saving.get() {
                                            "bg-indigo-600 text-white active:bg-indigo-700"
                                        } else {
                                            "bg-gray-200 text-gray-400"
                                        }
                                    )
                                    disabled=move || !is_valid2() || is_saving.get()
                                    on:click={
                                        let asset = asset_for_save.clone();
                                        move |_| {
                                            let to_dept = selected_dept.get();
                                            if to_dept.is_empty() { return; }

                                            is_saving.set(true);
                                            let mut updated = asset.clone();

                                            updated.transfers.push(TransferRecord {
                                                date: transfer_date.get_untracked(),
                                                from_department_id: updated.department_id.clone(),
                                                to_department_id: to_dept.clone(),
                                                reason: transfer_reason.get_untracked(),
                                            });
                                            updated.department_id = Some(to_dept);
                                            updated.updated_at = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();

                                            leptos::task::spawn_local(async move {
                                                match asset_store::save_asset(&updated).await {
                                                    Ok(()) => on_transferred.run(()),
                                                    Err(e) => {
                                                        log::error!("Transfer error: {}", e);
                                                        is_saving.set(false);
                                                    }
                                                }
                                            });
                                        }
                                    }
                                >
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4"/>
                                    </svg>
                                    {move || i18n.t("asset.transfer_dept_execute")}
                                </button>
                            </div>
                        })
                    } else {
                        None
                    }}
                </div>
            </div>
        </div>
    }
}
