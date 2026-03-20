use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::i18n::use_i18n;
use crate::auth::use_auth;
use crate::stores::asset_store;
use crate::models::company::CompanySetup;

#[component]
pub fn SettingsPage() -> impl IntoView {
    let i18n = use_i18n();
    let auth = use_auth();
    let navigate = use_navigate();
    let status_message = RwSignal::new(Option::<String>::None);

    let setup = CompanySetup::load();
    let country_display = setup.as_ref().and_then(|s| s.country()).map(|c| {
        format!("{} {}", c.flag(), c.name_en())
    }).unwrap_or_else(|| "—".to_string());
    let currency_display = setup.as_ref().and_then(|s| s.currency()).map(|c| {
        format!("{} {}", c.symbol(), c.code())
    }).unwrap_or_else(|| "—".to_string());
    let company_name_signal = RwSignal::new(
        setup.as_ref().map(|s| s.company_name.clone()).unwrap_or_default()
    );
    let editing_name = RwSignal::new(false);

    view! {
        <div class="page-container">
            <h2 class="page-title">{move || i18n.t("settings.title")}</h2>

            // User info & Logout
            {move || auth.user.get().map(|user| {
                let nav = navigate.clone();
                view! {
                    <div class="card mb-4">
                        <div class="flex items-center justify-between">
                            <div>
                                <p class="font-semibold text-gray-900">{user.name.clone()}</p>
                                <p class="text-sm text-gray-500">{user.email.clone()}</p>
                            </div>
                            <button
                                class="text-sm text-red-600 font-medium px-3 py-2 border border-red-200 rounded-lg active:bg-red-50"
                                on:click=move |_| {
                                    auth.logout();
                                    let nav = nav.clone();
                                    nav("/welcome", Default::default());
                                }
                            >
                                {move || i18n.t("auth.logout")}
                            </button>
                        </div>
                    </div>
                }
            })}

            // Company / Country info
            <div class="card mb-4">
                <h3 class="font-semibold text-gray-900 mb-3">{move || i18n.t("setup.company_info")}</h3>
                <div class="space-y-2">
                    // Company name - editable
                    <div class="flex justify-between items-center text-sm">
                        <span class="text-gray-500">{move || i18n.t("setup.company_name")}</span>
                        {move || if editing_name.get() {
                            view! {
                                <div class="flex items-center gap-1.5">
                                    <input
                                        type="text"
                                        class="text-sm border border-blue-300 rounded-lg px-2 py-1 w-40 text-right"
                                        prop:value=move || company_name_signal.get()
                                        on:input=move |ev| company_name_signal.set(event_target_value(&ev))
                                    />
                                    <button
                                        class="text-blue-600 font-medium text-xs px-2 py-1"
                                        on:click=move |_| {
                                            let new_name = company_name_signal.get().trim().to_string();
                                            if !new_name.is_empty() {
                                                if let Some(mut s) = CompanySetup::load() {
                                                    s.company_name = new_name;
                                                    s.save();
                                                }
                                            }
                                            editing_name.set(false);
                                        }
                                    >{move || i18n.t("asset.save")}</button>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="flex items-center gap-1.5">
                                    <span class="font-medium text-gray-900">{move || company_name_signal.get()}</span>
                                    <button
                                        class="text-gray-400 p-0.5"
                                        on:click=move |_| editing_name.set(true)
                                    >
                                        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"/>
                                        </svg>
                                    </button>
                                </div>
                            }.into_any()
                        }}
                    </div>
                    <div class="flex justify-between text-sm">
                        <span class="text-gray-500">{move || i18n.t("setup.country")}</span>
                        <span class="font-medium text-gray-900">{country_display.clone()}</span>
                    </div>
                    <div class="flex justify-between text-sm">
                        <span class="text-gray-500">{move || i18n.t("setup.currency")}</span>
                        <span class="font-medium text-gray-900">{currency_display.clone()}</span>
                    </div>
                </div>
                <button
                    class="w-full mt-4 py-2.5 text-sm text-orange-600 font-medium border border-orange-200 rounded-lg active:bg-orange-50"
                    on:click=move |_| {
                        let msg = i18n.t("setup.change_country_warning");
                        let window = web_sys::window().unwrap();
                        if window.confirm_with_message(&msg).unwrap_or(false) {
                            // Clear all data
                            leptos::task::spawn_local(async move {
                                let _ = asset_store::clear_all_assets().await;
                                CompanySetup::clear();
                                // Clear auth
                                if let Some(window) = web_sys::window() {
                                    if let Ok(Some(storage)) = window.local_storage() {
                                        let _ = storage.remove_item("fa_user");
                                        let _ = storage.remove_item("fa_users");
                                        let _ = storage.remove_item("fa_user_plans");
                                    }
                                    let _ = window.location().set_href("/setup");
                                }
                            });
                        }
                    }
                >
                    {move || i18n.t("setup.change_country")}
                </button>
            </div>

            // Admin link
            <a href="/admin" class="card mb-4 block active:bg-gray-50 transition-colors">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-3">
                        <div class="w-10 h-10 bg-gray-900 rounded-lg flex items-center justify-center">
                            <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"/>
                            </svg>
                        </div>
                        <span class="font-medium text-gray-900">{move || i18n.t("admin.title")}</span>
                    </div>
                    <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                    </svg>
                </div>
            </a>

            // Language Setting
            <div class="card mb-4">
                <h3 class="font-semibold text-gray-900 mb-3">{move || i18n.t("settings.language")}</h3>
                <div class="grid grid-cols-2 gap-2">
                    <button
                        class=move || if i18n.current_locale() == "en" { "btn-primary" } else { "btn-secondary" }
                        on:click=move |_| i18n.set_locale("en")
                    >
                        "English"
                    </button>
                    <button
                        class=move || if i18n.current_locale() == "ja" { "btn-primary" } else { "btn-secondary" }
                        on:click=move |_| i18n.set_locale("ja")
                    >
                        "日本語"
                    </button>
                </div>
            </div>

            // Data Management
            <div class="card mb-4">
                <h3 class="font-semibold text-gray-900 mb-3">{move || i18n.t("settings.data_management")}</h3>
                <div class="space-y-3">
                    // Export
                    <button
                        class="btn-secondary"
                        on:click=move |_| {
                            leptos::task::spawn_local(async move {
                                match asset_store::export_all_assets().await {
                                    Ok(json) => {
                                        download_json(&json, "fixedassets_export.json");
                                        status_message.set(Some("Export completed".to_string()));
                                    }
                                    Err(e) => {
                                        status_message.set(Some(format!("Export failed: {}", e)));
                                    }
                                }
                            });
                        }
                    >
                        {move || i18n.t("settings.export_data")}
                    </button>

                    // Import
                    <div>
                        <label class="btn-secondary cursor-pointer block">
                            {move || i18n.t("settings.import_data")}
                            <input
                                type="file"
                                accept=".json"
                                class="hidden"
                                on:change=move |ev| {
                                    use wasm_bindgen::JsCast;
                                    let target = ev.target().unwrap();
                                    let input: web_sys::HtmlInputElement = target.unchecked_into();
                                    if let Some(files) = input.files() {
                                        if let Some(file) = files.get(0) {
                                            let reader = web_sys::FileReader::new().unwrap();
                                            let reader_clone = reader.clone();
                                            let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                                                if let Ok(result) = reader_clone.result() {
                                                    if let Some(text) = result.as_string() {
                                                        leptos::task::spawn_local(async move {
                                                            match asset_store::import_assets(&text).await {
                                                                Ok(count) => {
                                                                    status_message.set(Some(format!("Imported {} assets", count)));
                                                                }
                                                                Err(e) => {
                                                                    status_message.set(Some(format!("Import failed: {}", e)));
                                                                }
                                                            }
                                                        });
                                                    }
                                                }
                                            }) as Box<dyn FnMut(_)>);
                                            reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                                            onload.forget();
                                            let _ = reader.read_as_text(&file);
                                        }
                                    }
                                }
                            />
                        </label>
                    </div>

                    // Clear
                    <button
                        class="w-full py-3 text-red-600 font-medium border border-red-200 rounded-lg active:bg-red-50"
                        on:click=move |_| {
                            let msg = i18n.t("settings.clear_confirm");
                            let window = web_sys::window().unwrap();
                            if window.confirm_with_message(&msg).unwrap_or(false) {
                                leptos::task::spawn_local(async move {
                                    match asset_store::clear_all_assets().await {
                                        Ok(()) => {
                                            status_message.set(Some("All data cleared".to_string()));
                                        }
                                        Err(e) => {
                                            status_message.set(Some(format!("Clear failed: {}", e)));
                                        }
                                    }
                                });
                            }
                        }
                    >
                        {move || i18n.t("settings.clear_data")}
                    </button>
                </div>
            </div>

            // Status message
            {move || status_message.get().map(|msg| view! {
                <div class="card bg-blue-50 border-blue-200 text-blue-800 text-sm">
                    {msg}
                </div>
            })}

            // Version
            <div class="text-center text-xs text-gray-400 mt-8">
                <p>{move || i18n.t("settings.version")} " 0.1.0"</p>
            </div>
        </div>
    }
}

fn download_json(json: &str, filename: &str) {
    use wasm_bindgen::JsCast;
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let blob_parts = js_sys::Array::new();
    blob_parts.push(&wasm_bindgen::JsValue::from_str(json));

    let options = web_sys::BlobPropertyBag::new();
    options.set_type("application/json");

    let blob = web_sys::Blob::new_with_str_sequence_and_options(&blob_parts, &options).unwrap();
    let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

    let a: web_sys::HtmlElement = document.create_element("a").unwrap().unchecked_into();
    a.set_attribute("href", &url).unwrap();
    a.set_attribute("download", filename).unwrap();
    a.click();

    let _ = web_sys::Url::revoke_object_url(&url);
}
