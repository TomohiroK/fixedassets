use leptos::prelude::*;
use crate::i18n::use_i18n;
use crate::auth::use_auth;
use crate::models::department::Department;

#[component]
pub fn DepartmentMasterSection() -> impl IntoView {
    let i18n = use_i18n();
    let auth = use_auth();
    let refresh = RwSignal::new(0u32);
    let new_code = RwSignal::new(String::new());
    let new_name = RwSignal::new(String::new());
    let show_add_form = RwSignal::new(false);
    let editing_id = RwSignal::new(Option::<String>::None);
    let edit_code = RwSignal::new(String::new());
    let edit_name = RwSignal::new(String::new());

    let is_paid = auth.is_paid();
    let dept_limit_reached = move || {
        if is_paid { return false; }
        refresh.get(); // track changes
        Department::load_all().len() >= 1
    };

    view! {
        <div class="card mb-4">
            <div class="flex items-center justify-between mb-3">
                <h3 class="font-semibold text-gray-900">{move || i18n.t("settings.dept_title")}</h3>
                <button
                    class=move || if dept_limit_reached() {
                        "text-sm text-gray-400 font-medium flex items-center gap-1 cursor-not-allowed"
                    } else {
                        "text-sm text-indigo-600 font-medium flex items-center gap-1"
                    }
                    disabled=dept_limit_reached
                    on:click=move |_| {
                        if !dept_limit_reached() {
                            show_add_form.update(|v| *v = !*v);
                        }
                    }
                >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"/>
                    </svg>
                    {move || i18n.t("settings.dept_add")}
                </button>
            </div>

            // Free plan limit notice
            {move || if dept_limit_reached() {
                view! {
                    <div class="mb-3 p-2 bg-amber-50 border border-amber-200 rounded-lg text-xs text-amber-700 flex items-center gap-1.5">
                        <svg class="w-3.5 h-3.5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                        </svg>
                        {move || i18n.t("settings.dept_free_limit")}
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Add form
            {move || if show_add_form.get() {
                view! {
                    <div class="bg-indigo-50 rounded-lg p-3 mb-3 space-y-2">
                        <div class="grid grid-cols-3 gap-2">
                            <input
                                type="text"
                                class="text-sm border border-indigo-200 rounded-lg px-2.5 py-2 bg-white"
                                placeholder=move || i18n.t("settings.dept_code_placeholder")
                                prop:value=move || new_code.get()
                                on:input=move |ev| new_code.set(event_target_value(&ev))
                            />
                            <input
                                type="text"
                                class="col-span-2 text-sm border border-indigo-200 rounded-lg px-2.5 py-2 bg-white"
                                placeholder=move || i18n.t("settings.dept_name_placeholder")
                                prop:value=move || new_name.get()
                                on:input=move |ev| new_name.set(event_target_value(&ev))
                            />
                        </div>
                        <div class="flex gap-2">
                            <button
                                class="flex-1 py-2 text-sm text-gray-600 border border-gray-200 rounded-lg"
                                on:click=move |_| {
                                    show_add_form.set(false);
                                    new_code.set(String::new());
                                    new_name.set(String::new());
                                }
                            >
                                {move || i18n.t("asset.cancel")}
                            </button>
                            <button
                                class="flex-1 py-2 text-sm text-white bg-indigo-600 rounded-lg disabled:opacity-50"
                                disabled=move || new_name.get().trim().is_empty()
                                on:click=move |_| {
                                    let name = new_name.get().trim().to_string();
                                    if !name.is_empty() {
                                        let code = new_code.get().trim().to_string();
                                        Department::add(Department::new(code, name));
                                        new_code.set(String::new());
                                        new_name.set(String::new());
                                        show_add_form.set(false);
                                        refresh.update(|v| *v += 1);
                                    }
                                }
                            >
                                {move || i18n.t("settings.dept_add")}
                            </button>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}

            // Department list
            {move || {
                refresh.get();
                let depts = Department::load_all();
                if depts.is_empty() {
                    view! {
                        <p class="text-sm text-gray-400 text-center py-4">{move || i18n.t("settings.dept_empty")}</p>
                    }.into_any()
                } else {
                    view! {
                        <div class="space-y-1">
                            {depts.into_iter().map(|dept| {
                                let dept_id = dept.id.clone();
                                let dept_id2 = dept.id.clone();
                                let dept_id3 = dept.id.clone();
                                let dept_code = dept.code.clone();
                                let dept_code2 = dept.code.clone();
                                let dept_name = dept.name.clone();
                                let dept_name2 = dept.name.clone();
                                view! {
                                    <div class="flex items-center justify-between py-2 px-2 rounded-lg hover:bg-gray-50">
                                        {move || {
                                            if editing_id.get().as_deref() == Some(&dept_id) {
                                                view! {
                                                    <div class="flex-1 grid grid-cols-3 gap-1.5 mr-2">
                                                        <input
                                                            type="text"
                                                            class="text-xs border border-indigo-200 rounded px-2 py-1.5"
                                                            prop:value=move || edit_code.get()
                                                            on:input=move |ev| edit_code.set(event_target_value(&ev))
                                                        />
                                                        <input
                                                            type="text"
                                                            class="col-span-2 text-xs border border-indigo-200 rounded px-2 py-1.5"
                                                            prop:value=move || edit_name.get()
                                                            on:input=move |ev| edit_name.set(event_target_value(&ev))
                                                        />
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <div class="flex-1 min-w-0">
                                                        <div class="flex items-center gap-2">
                                                            {if !dept_code.is_empty() {
                                                                let c = dept_code.clone();
                                                                Some(view! {
                                                                    <span class="text-xs font-mono text-indigo-500 bg-indigo-50 px-1.5 py-0.5 rounded">{c}</span>
                                                                })
                                                            } else {
                                                                None
                                                            }}
                                                            <span class="text-sm text-gray-900">{dept_name.clone()}</span>
                                                        </div>
                                                    </div>
                                                }.into_any()
                                            }
                                        }}
                                        <div class="flex items-center gap-1 shrink-0">
                                            {move || {
                                                if editing_id.get().as_deref() == Some(&dept_id2) {
                                                    view! {
                                                        <button
                                                            class="text-xs text-indigo-600 font-medium px-2 py-1"
                                                            on:click={
                                                                let did = dept_id2.clone();
                                                                move |_| {
                                                                    let name = edit_name.get().trim().to_string();
                                                                    if !name.is_empty() {
                                                                        Department::update(&Department {
                                                                            id: did.clone(),
                                                                            code: edit_code.get().trim().to_string(),
                                                                            name,
                                                                        });
                                                                        editing_id.set(None);
                                                                        refresh.update(|v| *v += 1);
                                                                    }
                                                                }
                                                            }
                                                        >
                                                            {move || i18n.t("asset.save")}
                                                        </button>
                                                        <button
                                                            class="text-xs text-gray-400 px-2 py-1"
                                                            on:click=move |_| editing_id.set(None)
                                                        >
                                                            {move || i18n.t("asset.cancel")}
                                                        </button>
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <button
                                                            class="text-gray-400 p-1"
                                                            on:click={
                                                                let did = dept_id2.clone();
                                                                let dc = dept_code2.clone();
                                                                let dn = dept_name2.clone();
                                                                move |_| {
                                                                    editing_id.set(Some(did.clone()));
                                                                    edit_code.set(dc.clone());
                                                                    edit_name.set(dn.clone());
                                                                }
                                                            }
                                                        >
                                                            <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"/>
                                                            </svg>
                                                        </button>
                                                        <button
                                                            class="text-red-400 p-1"
                                                            on:click={
                                                                let did = dept_id3.clone();
                                                                move |_| {
                                                                    Department::remove(&did);
                                                                    refresh.update(|v| *v += 1);
                                                                }
                                                            }
                                                        >
                                                            <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
                                                            </svg>
                                                        </button>
                                                    }.into_any()
                                                }
                                            }}
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
