use leptos::prelude::*;
use crate::i18n::use_i18n;
use crate::auth::use_auth;
use crate::stores::asset_store;
use crate::models::company::CompanySetup;
use crate::components::common::{use_confirm, ConfirmStyle};
use crate::pages::settings_account::AccountSection;
use crate::pages::settings_department::DepartmentMasterSection;

#[component]
pub fn SettingsPage() -> impl IntoView {
    let i18n = use_i18n();
    let confirm = use_confirm();
    let auth = use_auth();
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

            // User info & Logout & Edit
            <AccountSection />

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
                    on:click={
                        let c = confirm.clone();
                        move |_| {
                            let msg = i18n.t("setup.change_country_warning");
                            let ok_label = i18n.t("setup.change_country");
                            let cancel = i18n.t("asset.cancel");
                            c.show(&msg, ConfirmStyle::Warning, &ok_label, &cancel, move || {
                                leptos::task::spawn_local(async move {
                                    asset_store::reset_all_data().await;
                                    if let Some(window) = web_sys::window() {
                                        let _ = window.location().set_href("/setup");
                                    }
                                });
                            });
                        }
                    }
                >
                    {move || i18n.t("setup.change_country")}
                </button>
            </div>

            // Department Master
            <DepartmentMasterSection />

            // Admin link — only visible to admin@example.com
            {move || {
                let is_admin = auth.user.get()
                    .map(|u| u.email == "admin@example.com")
                    .unwrap_or(false);
                if is_admin {
                    Some(view! {
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
                    })
                } else {
                    None
                }
            }}

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

            // Import Template
            <div class="card mb-4">
                <h3 class="font-semibold text-gray-900 mb-3">{move || i18n.t("settings.import_template")}</h3>
                <p class="text-xs text-gray-500 mb-3">{move || i18n.t("settings.template_hint")}</p>
                <div class="grid grid-cols-2 gap-2">
                    <button
                        class="btn-secondary text-sm py-2.5"
                        on:click=move |_| {
                            let csv = asset_store::csv_template();
                            download_file(&csv, "ledgea_template.csv", "text/csv");
                            status_message.set(Some(i18n.t("settings.template_downloaded")));
                        }
                    >
                        <div class="flex items-center justify-center gap-1.5">
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                            </svg>
                            "CSV"
                        </div>
                    </button>
                    <button
                        class="btn-secondary text-sm py-2.5"
                        on:click=move |_| {
                            let json = asset_store::json_template();
                            download_file(&json, "ledgea_template.json", "application/json");
                            status_message.set(Some(i18n.t("settings.template_downloaded")));
                        }
                    >
                        <div class="flex items-center justify-center gap-1.5">
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                            </svg>
                            "JSON"
                        </div>
                    </button>
                </div>

                // Format reference
                <details class="mt-3">
                    <summary class="text-xs text-blue-600 cursor-pointer font-medium">
                        {move || i18n.t("settings.format_reference")}
                    </summary>
                    <div class="mt-2 p-3 bg-gray-50 rounded-lg text-[11px] text-gray-600 space-y-2 overflow-x-auto">
                        <p class="font-semibold text-gray-700">{move || i18n.t("settings.csv_columns")}</p>
                        <table class="w-full text-left">
                            <tbody>
                                <tr class="border-b border-gray-200">
                                    <td class="py-1 font-mono text-blue-700">"asset_number"</td>
                                    <td class="py-1">{move || i18n.t("settings.col_asset_number")}</td>
                                </tr>
                                <tr class="border-b border-gray-200">
                                    <td class="py-1 font-mono text-blue-700">"name"</td>
                                    <td class="py-1">{move || i18n.t("settings.col_name")}</td>
                                </tr>
                                <tr class="border-b border-gray-200">
                                    <td class="py-1 font-mono text-blue-700">"category"</td>
                                    <td class="py-1">{move || i18n.t("settings.col_category")}</td>
                                </tr>
                                <tr class="border-b border-gray-200">
                                    <td class="py-1 font-mono text-blue-700">"acquisition_date"</td>
                                    <td class="py-1">"YYYY-MM-DD"</td>
                                </tr>
                                <tr class="border-b border-gray-200">
                                    <td class="py-1 font-mono text-blue-700">"cost"</td>
                                    <td class="py-1">{move || i18n.t("settings.col_cost")}</td>
                                </tr>
                                <tr class="border-b border-gray-200">
                                    <td class="py-1 font-mono text-blue-700">"salvage_value"</td>
                                    <td class="py-1">{move || i18n.t("settings.col_salvage")}</td>
                                </tr>
                                <tr class="border-b border-gray-200">
                                    <td class="py-1 font-mono text-blue-700">"useful_life"</td>
                                    <td class="py-1">{move || i18n.t("settings.col_life")}</td>
                                </tr>
                                <tr class="border-b border-gray-200">
                                    <td class="py-1 font-mono text-blue-700">"depreciation_method"</td>
                                    <td class="py-1">"SL / DB"</td>
                                </tr>
                                <tr class="border-b border-gray-200">
                                    <td class="py-1 font-mono text-blue-700">"location"</td>
                                    <td class="py-1">{move || i18n.t("settings.col_location")}</td>
                                </tr>
                                <tr class="border-b border-gray-200">
                                    <td class="py-1 font-mono text-blue-700">"description"</td>
                                    <td class="py-1">{move || i18n.t("settings.col_description")}</td>
                                </tr>
                                <tr class="border-b border-gray-200">
                                    <td class="py-1 font-mono text-blue-700">"prior_years"</td>
                                    <td class="py-1">{move || i18n.t("settings.col_prior_years")}</td>
                                </tr>
                                <tr class="border-b border-gray-200">
                                    <td class="py-1 font-mono text-blue-700">"prior_months"</td>
                                    <td class="py-1">{move || i18n.t("settings.col_prior_months")}</td>
                                </tr>
                                <tr class="border-b border-gray-200">
                                    <td class="py-1 font-mono text-blue-700">"status"</td>
                                    <td class="py-1">"InUse / Disposed / Transferred / Maintenance"</td>
                                </tr>
                                <tr class="border-b border-gray-200">
                                    <td class="py-1 font-mono text-blue-700">"tags"</td>
                                    <td class="py-1">{move || i18n.t("settings.col_tags")}</td>
                                </tr>
                                <tr class="border-b border-gray-200">
                                    <td class="py-1 font-mono text-blue-700">"department"</td>
                                    <td class="py-1">{move || i18n.t("settings.col_department")}</td>
                                </tr>
                                <tr class="border-b border-gray-200">
                                    <td class="py-1 font-mono text-blue-700">"quantity"</td>
                                    <td class="py-1">{move || i18n.t("settings.col_quantity")}</td>
                                </tr>
                                <tr class="border-b border-gray-200">
                                    <td class="py-1 font-mono text-blue-700">"ifrs_useful_life"</td>
                                    <td class="py-1">{move || i18n.t("settings.col_ifrs_life")}</td>
                                </tr>
                                <tr class="border-b border-gray-200">
                                    <td class="py-1 font-mono text-blue-700">"ifrs_salvage_value"</td>
                                    <td class="py-1">{move || i18n.t("settings.col_ifrs_salvage")}</td>
                                </tr>
                                <tr>
                                    <td class="py-1 font-mono text-blue-700">"ifrs_method"</td>
                                    <td class="py-1">{move || i18n.t("settings.col_ifrs_method")}</td>
                                </tr>
                            </tbody>
                        </table>

                        <p class="font-semibold text-gray-700 pt-2">{move || i18n.t("settings.category_values")}</p>
                        <p class="font-mono leading-relaxed">
                            "Land, Building, BuildingEquipment, Structures, Machinery, ToolsFixtures, Vehicles, LeasedAssets, ConstructionInProgress, Patents, Trademarks, LeaseholdRights, Software, FacilityRights, Other"
                        </p>
                        <p class="text-gray-500 italic">{move || i18n.t("settings.category_ja_hint")}</p>
                    </div>
                </details>
            </div>

            // Data Management
            <div class="card mb-4">
                <h3 class="font-semibold text-gray-900 mb-3">{move || i18n.t("settings.data_management")}</h3>
                <div class="space-y-3">
                    // Export
                    <div class="grid grid-cols-2 gap-2">
                        <button
                            class="btn-secondary text-sm"
                            on:click=move |_| {
                                leptos::task::spawn_local(async move {
                                    match asset_store::export_all_assets().await {
                                        Ok(json) => {
                                            download_file(&json, "ledgea_export.json", "application/json");
                                            status_message.set(Some("Export (JSON) completed".to_string()));
                                        }
                                        Err(e) => {
                                            status_message.set(Some(format!("Export failed: {}", e)));
                                        }
                                    }
                                });
                            }
                        >
                            {move || format!("{} (JSON)", i18n.t("settings.export_data"))}
                        </button>
                        <button
                            class="btn-secondary text-sm"
                            on:click=move |_| {
                                leptos::task::spawn_local(async move {
                                    match asset_store::export_all_assets_csv().await {
                                        Ok(csv) => {
                                            download_file(&csv, "ledgea_export.csv", "text/csv");
                                            status_message.set(Some("Export (CSV) completed".to_string()));
                                        }
                                        Err(e) => {
                                            status_message.set(Some(format!("Export failed: {}", e)));
                                        }
                                    }
                                });
                            }
                        >
                            {move || format!("{} (CSV)", i18n.t("settings.export_data"))}
                        </button>
                    </div>

                    // Import
                    <div>
                        <label class="btn-secondary cursor-pointer block text-sm">
                            <div class="flex items-center justify-center gap-2">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"/>
                                </svg>
                                {move || i18n.t("settings.import_data")} " (JSON / CSV)"
                            </div>
                            <input
                                type="file"
                                accept=".json,.csv"
                                class="hidden"
                                on:change=move |ev| {
                                    use wasm_bindgen::JsCast;
                                    let target = ev.target().unwrap();
                                    let input: web_sys::HtmlInputElement = target.unchecked_into();
                                    if let Some(files) = input.files() {
                                        if let Some(file) = files.get(0) {
                                            let filename = file.name();
                                            let is_csv = filename.ends_with(".csv");
                                            let reader = web_sys::FileReader::new().unwrap();
                                            let reader_clone = reader.clone();
                                            let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                                                if let Ok(result) = reader_clone.result() {
                                                    if let Some(text) = result.as_string() {
                                                        let is_csv = is_csv;
                                                        leptos::task::spawn_local(async move {
                                                            let result = if is_csv {
                                                                asset_store::import_assets_csv(&text).await
                                                            } else {
                                                                asset_store::import_assets(&text).await
                                                            };
                                                            match result {
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
                        on:click={
                            let c = confirm.clone();
                            move |_| {
                                let msg = i18n.t("settings.clear_confirm");
                                let ok_label = i18n.t("settings.clear_data");
                                let cancel = i18n.t("asset.cancel");
                                c.show(&msg, ConfirmStyle::Danger, &ok_label, &cancel, move || {
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

            // Terms of Service link
            <a href="/terms" class="card mb-4 block active:bg-gray-50 transition-colors">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-3">
                        <div class="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center">
                            <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                            </svg>
                        </div>
                        <span class="font-medium text-gray-900">{move || i18n.t("settings.terms")}</span>
                    </div>
                    <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                    </svg>
                </div>
            </a>

            // Report & Contact link
            <a href="/report" class="card mb-4 block active:bg-gray-50 transition-colors">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-3">
                        <div class="w-10 h-10 bg-orange-100 rounded-lg flex items-center justify-center">
                            <svg class="w-5 h-5 text-orange-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z"/>
                            </svg>
                        </div>
                        <span class="font-medium text-gray-900">{move || i18n.t("report.link")}</span>
                    </div>
                    <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                    </svg>
                </div>
            </a>

            // Version
            <div class="text-center text-xs text-gray-400 mt-8">
                <p>{move || i18n.t("settings.version")} " 0.1.0"</p>
            </div>
        </div>
    }
}

pub fn download_file(content: &str, filename: &str, mime_type: &str) {
    use wasm_bindgen::JsCast;
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Add BOM for CSV to support Excel opening with UTF-8
    let content_with_bom = if mime_type.contains("csv") {
        format!("\u{FEFF}{}", content)
    } else {
        content.to_string()
    };

    let blob_parts = js_sys::Array::new();
    blob_parts.push(&wasm_bindgen::JsValue::from_str(&content_with_bom));

    let options = web_sys::BlobPropertyBag::new();
    options.set_type(mime_type);

    let blob = web_sys::Blob::new_with_str_sequence_and_options(&blob_parts, &options).unwrap();
    let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

    let a: web_sys::HtmlElement = document.create_element("a").unwrap().unchecked_into();
    a.set_attribute("href", &url).unwrap();
    a.set_attribute("download", filename).unwrap();
    a.click();

    let _ = web_sys::Url::revoke_object_url(&url);
}
