use leptos::prelude::*;
use wasm_bindgen::JsCast;
use crate::i18n::use_i18n;
use crate::auth::use_auth;

#[component]
pub fn ReportPage() -> impl IntoView {
    let i18n = use_i18n();
    let auth = use_auth();

    let back_href = move || {
        if auth.is_logged_in() { "/settings" } else { "/welcome" }
    };

    // Form state
    let category = RwSignal::new("bug".to_string());
    let subject = RwSignal::new(String::new());
    let description = RwSignal::new(String::new());
    let contact_email = RwSignal::new(String::new());
    let submitted = RwSignal::new(false);
    let copied = RwSignal::new(false);

    // Pre-fill email if logged in
    if let Some(user) = auth.user.get() {
        contact_email.set(user.email.clone());
    }

    let is_logged_in = auth.is_logged_in();

    // Build formatted report text
    let build_report = move || -> String {
        let cat_label = match category.get().as_str() {
            "bug" => "Bug Report",
            "feature" => "Feature Request",
            _ => "General Inquiry",
        };
        let email_val = contact_email.get();
        let subj = subject.get();
        let desc = description.get();

        let mut report = format!(
            "--- FixedAssets Report ---\nType: {}\nSubject: {}\n",
            cat_label, subj
        );

        if !email_val.is_empty() {
            report.push_str(&format!("Contact: {}\n", email_val));
        }

        // Add browser info
        if let Some(window) = web_sys::window() {
            let nav = window.navigator();
            if let Ok(ua) = nav.user_agent() {
                report.push_str(&format!("User-Agent: {}\n", ua));
            }
            if let Ok(loc) = window.location().href() {
                report.push_str(&format!("URL: {}\n", loc));
            }
        }

        report.push_str(&format!("\nDescription:\n{}\n", desc));
        report.push_str("--- End Report ---\n");
        report
    };

    // Copy to clipboard action
    let on_copy = move |_| {
        let report = build_report();
        if let Some(window) = web_sys::window() {
            let clipboard = window.navigator().clipboard();
            let _promise: js_sys::Promise = clipboard.write_text(&report);
            copied.set(true);
            // Reset after 3 seconds using setTimeout
            let cb = wasm_bindgen::closure::Closure::once(move || {
                copied.set(false);
            });
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                3000,
            );
            cb.forget();
        }
    };

    // mailto action
    let on_mailto = move |_| {
        let cat_label = match category.get().as_str() {
            "bug" => "Bug Report",
            "feature" => "Feature Request",
            _ => "General Inquiry",
        };
        let subj = subject.get();
        let desc = description.get();
        let email_val = contact_email.get();

        let mailto_subject = js_sys::encode_uri_component(
            &format!("[FixedAssets {}] {}", cat_label, subj)
        );
        let mut body = format!("Type: {}\n\n{}", cat_label, desc);
        if !email_val.is_empty() {
            body.push_str(&format!("\n\nContact: {}", email_val));
        }
        let mailto_body = js_sys::encode_uri_component(&body);

        let mailto_url = format!(
            "mailto:support@fixedassets.app?subject={}&body={}",
            mailto_subject, mailto_body
        );

        if let Some(window) = web_sys::window() {
            let _ = window.location().set_href(&mailto_url);
        }
    };

    // Submit action: show confirmation
    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        submitted.set(true);
    };

    let can_submit = move || {
        !subject.get().trim().is_empty()
            && !description.get().trim().is_empty()
            && (is_logged_in || !contact_email.get().trim().is_empty())
    };

    view! {
        <div class="min-h-screen bg-gray-50">
            // Header
            <div class="bg-white/80 backdrop-blur-lg border-b border-gray-200/60 sticky top-0 z-10">
                <div class="max-w-lg mx-auto px-4 py-3 flex items-center justify-between">
                    <a href=back_href class="text-gray-600 text-sm font-medium flex items-center gap-1">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                        </svg>
                        {move || i18n.t("app.title")}
                    </a>
                    <button
                        class="text-xs text-gray-500 border border-gray-200 px-2.5 py-1 rounded-full active:bg-gray-100"
                        on:click=move |_| {
                            let next = if i18n.current_locale() == "en" { "ja" } else { "en" };
                            i18n.set_locale(next);
                        }
                    >
                        {move || if i18n.current_locale() == "en" { "日本語" } else { "EN" }}
                    </button>
                </div>
            </div>

            <div class="max-w-lg mx-auto px-4 py-6">
                // Title
                <div class="mb-6">
                    <h1 class="text-xl font-bold text-gray-900">{move || i18n.t("report.title")}</h1>
                    <p class="text-sm text-gray-500 mt-1">{move || i18n.t("report.subtitle")}</p>
                </div>

                {move || if submitted.get() {
                    // Confirmation view
                    view! {
                        <div class="space-y-4">
                            <div class="bg-white rounded-xl border border-gray-200 p-6 text-center space-y-4">
                                <div class="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto">
                                    <svg class="w-8 h-8 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                    </svg>
                                </div>
                                <h2 class="text-lg font-bold text-gray-900">{move || i18n.t("report.ready_title")}</h2>
                                <p class="text-sm text-gray-600">{move || i18n.t("report.ready_desc")}</p>
                            </div>

                            // Preview
                            <div class="bg-gray-900 rounded-xl p-4 text-xs font-mono text-gray-300 whitespace-pre-wrap max-h-48 overflow-y-auto">
                                {build_report()}
                            </div>

                            // Actions
                            <div class="space-y-3">
                                <button
                                    class="w-full py-3 bg-gray-900 text-white font-medium rounded-xl active:bg-gray-700 transition-colors flex items-center justify-center gap-2"
                                    on:click=on_mailto
                                >
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>
                                    </svg>
                                    {move || i18n.t("report.send_email")}
                                </button>
                                <button
                                    class=move || if copied.get() {
                                        "w-full py-3 bg-green-600 text-white font-medium rounded-xl transition-colors flex items-center justify-center gap-2"
                                    } else {
                                        "w-full py-3 bg-white text-gray-900 font-medium border border-gray-200 rounded-xl active:bg-gray-50 transition-colors flex items-center justify-center gap-2"
                                    }
                                    on:click=on_copy
                                >
                                    {move || if copied.get() {
                                        view! {
                                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                            </svg>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"/>
                                            </svg>
                                        }.into_any()
                                    }}
                                    {move || if copied.get() {
                                        i18n.t("report.copied")
                                    } else {
                                        i18n.t("report.copy_clipboard")
                                    }}
                                </button>
                                <button
                                    class="w-full py-3 text-gray-500 text-sm font-medium active:text-gray-700"
                                    on:click=move |_| submitted.set(false)
                                >
                                    {move || i18n.t("report.back_to_form")}
                                </button>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    // Form view
                    view! {
                        <form on:submit=on_submit class="space-y-4">
                            // Category
                            <div class="bg-white rounded-xl border border-gray-200 p-4">
                                <label class="text-sm font-medium text-gray-900 mb-2 block">{move || i18n.t("report.category")}</label>
                                <div class="grid grid-cols-3 gap-2">
                                    <button
                                        type="button"
                                        class=move || if category.get() == "bug" {
                                            "py-2.5 text-xs font-medium rounded-lg border-2 border-red-500 bg-red-50 text-red-700 transition-colors"
                                        } else {
                                            "py-2.5 text-xs font-medium rounded-lg border border-gray-200 bg-white text-gray-600 active:bg-gray-50 transition-colors"
                                        }
                                        on:click=move |_| category.set("bug".to_string())
                                    >
                                        <div class="flex flex-col items-center gap-1">
                                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 16.5c-.77.833.192 2.5 1.732 2.5z"/>
                                            </svg>
                                            {move || i18n.t("report.cat_bug")}
                                        </div>
                                    </button>
                                    <button
                                        type="button"
                                        class=move || if category.get() == "feature" {
                                            "py-2.5 text-xs font-medium rounded-lg border-2 border-blue-500 bg-blue-50 text-blue-700 transition-colors"
                                        } else {
                                            "py-2.5 text-xs font-medium rounded-lg border border-gray-200 bg-white text-gray-600 active:bg-gray-50 transition-colors"
                                        }
                                        on:click=move |_| category.set("feature".to_string())
                                    >
                                        <div class="flex flex-col items-center gap-1">
                                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"/>
                                            </svg>
                                            {move || i18n.t("report.cat_feature")}
                                        </div>
                                    </button>
                                    <button
                                        type="button"
                                        class=move || if category.get() == "inquiry" {
                                            "py-2.5 text-xs font-medium rounded-lg border-2 border-teal-500 bg-teal-50 text-teal-700 transition-colors"
                                        } else {
                                            "py-2.5 text-xs font-medium rounded-lg border border-gray-200 bg-white text-gray-600 active:bg-gray-50 transition-colors"
                                        }
                                        on:click=move |_| category.set("inquiry".to_string())
                                    >
                                        <div class="flex flex-col items-center gap-1">
                                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z"/>
                                            </svg>
                                            {move || i18n.t("report.cat_inquiry")}
                                        </div>
                                    </button>
                                </div>
                            </div>

                            // Email (non-logged-in only)
                            {move || if !is_logged_in {
                                Some(view! {
                                    <div class="bg-white rounded-xl border border-gray-200 p-4">
                                        <label class="text-sm font-medium text-gray-900 mb-2 block">{move || i18n.t("report.email")}</label>
                                        <input
                                            type="email"
                                            class="input-field"
                                            required=true
                                            placeholder=move || i18n.t("report.email_placeholder")
                                            prop:value=move || contact_email.get()
                                            on:input=move |ev| {
                                                use wasm_bindgen::JsCast;
                                                let val = ev.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>().value();
                                                contact_email.set(val);
                                            }
                                        />
                                    </div>
                                })
                            } else {
                                None
                            }}

                            // Subject
                            <div class="bg-white rounded-xl border border-gray-200 p-4">
                                <label class="text-sm font-medium text-gray-900 mb-2 block">{move || i18n.t("report.subject")}</label>
                                <input
                                    type="text"
                                    class="input-field"
                                    required=true
                                    placeholder=move || i18n.t("report.subject_placeholder")
                                    prop:value=move || subject.get()
                                    on:input=move |ev| {
                                        use wasm_bindgen::JsCast;
                                        let val = ev.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>().value();
                                        subject.set(val);
                                    }
                                />
                            </div>

                            // Description
                            <div class="bg-white rounded-xl border border-gray-200 p-4">
                                <label class="text-sm font-medium text-gray-900 mb-2 block">{move || i18n.t("report.description")}</label>
                                <textarea
                                    class="input-field min-h-[120px] resize-y"
                                    required=true
                                    rows="5"
                                    placeholder=move || i18n.t("report.description_placeholder")
                                    prop:value=move || description.get()
                                    on:input=move |ev| {
                                        use wasm_bindgen::JsCast;
                                        let val = ev.target().unwrap().unchecked_into::<web_sys::HtmlTextAreaElement>().value();
                                        description.set(val);
                                    }
                                ></textarea>
                                <p class="text-[11px] text-gray-400 mt-1.5">
                                    {move || i18n.t("report.description_hint")}
                                </p>
                            </div>

                            // Submit
                            <button
                                type="submit"
                                class="w-full py-3.5 bg-gray-900 text-white font-semibold rounded-xl active:bg-gray-700 disabled:opacity-40 disabled:cursor-not-allowed transition-colors"
                                disabled=move || !can_submit()
                            >
                                {move || i18n.t("report.submit")}
                            </button>
                        </form>
                    }.into_any()
                }}

                // Footer
                <div class="text-center text-xs text-gray-400 mt-8 mb-4">
                    <p>{move || i18n.t("landing.footer")}</p>
                </div>
            </div>
        </div>
    }
}
