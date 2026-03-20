use leptos::prelude::*;
use crate::i18n::use_i18n;

#[component]
pub fn LoadingSpinner() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <div class="flex flex-col items-center justify-center py-12">
            <div class="w-8 h-8 border-4 border-blue-200 border-t-blue-600 rounded-full animate-spin"></div>
            <p class="mt-3 text-sm text-gray-500">{move || i18n.t("common.loading")}</p>
        </div>
    }
}

#[component]
pub fn EmptyState(
    #[prop(into)] message: String,
    #[prop(optional)] sub_message: Option<String>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center py-16 px-4">
            <svg class="w-16 h-16 text-gray-300 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                    d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4"/>
            </svg>
            <p class="text-gray-500 font-medium">{message}</p>
            {sub_message.map(|msg| view! {
                <p class="text-sm text-gray-400 mt-1">{msg}</p>
            })}
        </div>
    }
}

#[component]
pub fn SearchBar(
    value: RwSignal<String>,
    #[prop(into)] placeholder: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="relative">
            <svg class="w-5 h-5 text-gray-400 absolute left-3 top-1/2 -translate-y-1/2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
            </svg>
            <input
                type="search"
                class="input-field pl-10"
                placeholder=placeholder
                prop:value=move || value.get()
                on:input=move |ev| {
                    use leptos::prelude::*;
                    let val = event_target_value(&ev);
                    value.set(val);
                }
            />
        </div>
    }
}

pub fn format_currency(amount: &rust_decimal::Decimal) -> String {
    use crate::models::company::CompanySetup;

    let symbol = CompanySetup::load()
        .and_then(|s| s.currency())
        .map(|c| c.symbol().to_string())
        .unwrap_or_else(|| "$".to_string());

    let s = format!("{:.2}", amount);
    let parts: Vec<&str> = s.split('.').collect();
    let int_part = parts[0];
    let dec_part = parts.get(1).unwrap_or(&"00");

    let is_negative = int_part.starts_with('-');
    let digits: Vec<char> = int_part.chars().filter(|c| c.is_ascii_digit()).collect();

    let mut formatted = String::new();
    let len = digits.len();
    for (i, c) in digits.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            formatted.push(',');
        }
        formatted.push(*c);
    }

    if is_negative {
        format!("{}-{}.{}", symbol, formatted, dec_part)
    } else {
        format!("{}{}.{}", symbol, formatted, dec_part)
    }
}
