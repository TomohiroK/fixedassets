use leptos::prelude::*;
use crate::i18n::use_i18n;
use crate::models::company::{AseanCountry, Currency, CompanySetup};

#[component]
pub fn SetupPage() -> impl IntoView {
    let i18n = use_i18n();
    let selected_country = RwSignal::new(Option::<String>::None);
    let selected_currency = RwSignal::new(Option::<String>::None);
    let company_name = RwSignal::new(String::new());
    let step = RwSignal::new(1u32); // 1=country, 2=currency, 3=company name

    // When country changes, reset currency to local
    let on_country_select = move |code: String| {
        selected_country.set(Some(code.clone()));
        if let Some(country) = AseanCountry::from_code(&code) {
            selected_currency.set(Some(country.local_currency().code().to_string()));
        }
        step.set(2);
    };

    let on_complete = move |_| {
        if let (Some(country_code), Some(currency_code)) = (selected_country.get(), selected_currency.get()) {
            let name = company_name.get().trim().to_string();
            if !name.is_empty() {
                let setup = CompanySetup {
                    company_name: name,
                    country_code,
                    currency_code,
                };
                setup.save();
                crate::stores::asset_store::mark_data_version_current();
                // Go to dashboard
                if let Some(window) = web_sys::window() {
                    let _ = window.location().set_href("/");
                }
            }
        }
    };

    let is_en = move || i18n.current_locale() == "en";

    view! {
        <div class="min-h-screen bg-gray-50">
            // Header
            <div class="bg-white/80 backdrop-blur-lg border-b border-gray-200/60 px-4 py-3">
                <div class="flex items-center justify-between max-w-lg mx-auto">
                    <h1 class="text-lg font-bold text-gray-900">{move || i18n.t("setup.title")}</h1>
                    <button
                        class="text-xs text-gray-500 border border-gray-200 px-2.5 py-1 rounded-full active:bg-gray-100 transition-colors"
                        on:click=move |_| {
                            let next = if i18n.current_locale() == "en" { "ja" } else { "en" };
                            i18n.set_locale(next);
                        }
                    >
                        {move || if is_en() { "日本語" } else { "English" }}
                    </button>
                </div>
            </div>

            // Progress indicator
            <div class="max-w-lg mx-auto px-6 pt-4">
                <div class="flex items-center gap-2 mb-6">
                    {[1u32, 2, 3].into_iter().map(|s| {
                        view! {
                            <div class=move || format!(
                                "flex-1 h-1.5 rounded-full transition-colors {}",
                                if step.get() >= s { "bg-gray-900" } else { "bg-gray-200" }
                            )></div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>

            <div class="max-w-lg mx-auto px-6">
                // Step 1: Country selection
                {move || if step.get() == 1 {
                    Some(view! {
                        <div>
                            <h2 class="text-xl font-bold text-gray-900 mb-1">{move || i18n.t("setup.select_country")}</h2>
                            <p class="text-sm text-gray-500 mb-4">{move || i18n.t("setup.country_hint")}</p>
                            <div class="grid grid-cols-2 gap-2">
                                {AseanCountry::all().into_iter().map(|country| {
                                    let code = country.code().to_string();
                                    let code2 = code.clone();
                                    let flag = country.flag().to_string();
                                    let name_en = country.name_en().to_string();
                                    let name_ja = country.name_ja().to_string();
                                    view! {
                                        <button
                                            class=move || {
                                                let sel = selected_country.get().as_deref() == Some(&code);
                                                format!(
                                                    "flex items-center gap-2 p-3 rounded-xl border-2 text-left transition-all active:scale-95 {}",
                                                    if sel { "border-blue-600 bg-blue-50" } else { "border-gray-200 bg-white" }
                                                )
                                            }
                                            on:click={
                                                let code2 = code2.clone();
                                                move |_| on_country_select(code2.clone())
                                            }
                                        >
                                            <span class="text-2xl">{flag.clone()}</span>
                                            <div>
                                                <p class="text-sm font-semibold text-gray-900">
                                                    {if is_en() { name_en.clone() } else { name_ja.clone() }}
                                                </p>
                                            </div>
                                        </button>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    })
                } else {
                    None
                }}

                // Step 2: Currency selection
                {move || if step.get() == 2 {
                    let country_code = selected_country.get().unwrap_or_default();
                    let country = AseanCountry::from_code(&country_code);
                    let currencies = country.as_ref().map(|c| Currency::available_for(c)).unwrap_or_default();
                    let country_name = country.as_ref().map(|c| {
                        if is_en() { c.name_en().to_string() } else { c.name_ja().to_string() }
                    }).unwrap_or_default();
                    let flag = country.as_ref().map(|c| c.flag().to_string()).unwrap_or_default();

                    Some(view! {
                        <div>
                            <div class="flex items-center gap-2 mb-4">
                                <button
                                    class="text-gray-400 p-1"
                                    on:click=move |_| step.set(1)
                                >
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                                    </svg>
                                </button>
                                <span class="text-xl">{flag}</span>
                                <span class="text-sm text-gray-600 font-medium">{country_name}</span>
                            </div>

                            <h2 class="text-xl font-bold text-gray-900 mb-1">{move || i18n.t("setup.select_currency")}</h2>
                            <p class="text-sm text-gray-500 mb-4">{move || i18n.t("setup.currency_hint")}</p>

                            <div class="space-y-2">
                                {currencies.into_iter().map(|currency| {
                                    let code = currency.code().to_string();
                                    let code_for_class = code.clone();
                                    let code_for_click = code.clone();
                                    let code_for_check = code.clone();
                                    let code_for_display = code.clone();
                                    let symbol = currency.symbol().to_string();
                                    let name = currency.name_en().to_string();
                                    view! {
                                        <button
                                            class=move || {
                                                let sel = selected_currency.get().as_deref() == Some(&code_for_class);
                                                format!(
                                                    "w-full flex items-center justify-between p-4 rounded-xl border-2 transition-all active:scale-[0.98] {}",
                                                    if sel { "border-blue-600 bg-blue-50" } else { "border-gray-200 bg-white" }
                                                )
                                            }
                                            on:click={
                                                let c = code_for_click.clone();
                                                move |_| selected_currency.set(Some(c.clone()))
                                            }
                                        >
                                            <div class="flex items-center gap-3">
                                                <span class="text-lg font-bold text-gray-700 w-10">{symbol.clone()}</span>
                                                <div class="text-left">
                                                    <p class="text-sm font-semibold text-gray-900">{code_for_display}</p>
                                                    <p class="text-xs text-gray-500">{name}</p>
                                                </div>
                                            </div>
                                            {move || {
                                                let sel = selected_currency.get().as_deref() == Some(&code_for_check);
                                                if sel {
                                                    Some(view! {
                                                        <svg class="w-5 h-5 text-blue-600" fill="currentColor" viewBox="0 0 24 24">
                                                            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
                                                        </svg>
                                                    })
                                                } else {
                                                    None
                                                }
                                            }}
                                        </button>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>

                            <button
                                class="btn-primary mt-6"
                                on:click=move |_| {
                                    if selected_currency.get().is_some() {
                                        step.set(3);
                                    }
                                }
                            >
                                {move || i18n.t("common.next")}
                            </button>
                        </div>
                    })
                } else {
                    None
                }}

                // Step 3: Company name
                {move || if step.get() == 3 {
                    Some(view! {
                        <div>
                            <div class="flex items-center gap-2 mb-4">
                                <button
                                    class="text-gray-400 p-1"
                                    on:click=move |_| step.set(2)
                                >
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                                    </svg>
                                </button>
                            </div>

                            <h2 class="text-xl font-bold text-gray-900 mb-1">{move || i18n.t("setup.company_name")}</h2>
                            <p class="text-sm text-gray-500 mb-4">{move || i18n.t("setup.company_hint")}</p>

                            <input
                                type="text"
                                class="input-field text-lg"
                                placeholder=move || i18n.t("setup.company_placeholder")
                                prop:value=move || company_name.get()
                                on:input=move |ev| company_name.set(event_target_value(&ev))
                            />

                            // Summary
                            <div class="mt-6 bg-gray-100 rounded-xl p-4 space-y-2">
                                <div class="flex justify-between text-sm">
                                    <span class="text-gray-500">{move || i18n.t("setup.country")}</span>
                                    <span class="font-medium text-gray-900">{move || {
                                        let code = selected_country.get().unwrap_or_default();
                                        AseanCountry::from_code(&code).map(|c| {
                                            format!("{} {}", c.flag(), if is_en() { c.name_en() } else { c.name_ja() })
                                        }).unwrap_or_default()
                                    }}</span>
                                </div>
                                <div class="flex justify-between text-sm">
                                    <span class="text-gray-500">{move || i18n.t("setup.currency")}</span>
                                    <span class="font-medium text-gray-900">{move || {
                                        let code = selected_currency.get().unwrap_or_default();
                                        Currency::from_code(&code).map(|c| {
                                            format!("{} {}", c.symbol(), c.code())
                                        }).unwrap_or_default()
                                    }}</span>
                                </div>
                            </div>

                            <button
                                class="btn-primary mt-6"
                                prop:disabled=move || company_name.get().trim().is_empty()
                                on:click=on_complete
                            >
                                {move || i18n.t("setup.complete")}
                            </button>
                        </div>
                    })
                } else {
                    None
                }}
            </div>
        </div>
    }
}
