use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::i18n::use_i18n;
use crate::auth::{use_auth, validate_password_strength};

#[component]
pub fn SignupPage() -> impl IntoView {
    let i18n = use_i18n();
    let auth = use_auth();
    let navigate = use_navigate();

    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let confirm_password = RwSignal::new(String::new());
    let agreed_tos = RwSignal::new(false);
    let error = RwSignal::new(Option::<String>::None);

    view! {
        <div class="min-h-screen bg-gray-50 flex flex-col">
            // Header
            <div class="bg-white/80 backdrop-blur-lg border-b border-gray-200/60 px-4 py-3">
                <div class="flex items-center justify-between max-w-lg mx-auto">
                    <a href="/welcome" class="flex items-center gap-2 text-gray-600">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                        </svg>
                    </a>
                    <h1 class="text-lg font-bold text-gray-900">{move || i18n.t("auth.sign_up")}</h1>
                    <div class="w-5"></div>
                </div>
            </div>

            <div class="flex-1 flex items-start justify-center pt-12 px-4">
                <div class="w-full max-w-sm">
                    // Logo
                    <div class="text-center mb-8">
                        <div class="w-16 h-16 bg-blue-600 rounded-2xl mx-auto flex items-center justify-center mb-4">
                            <svg class="w-10 h-10 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.5">
                                <path stroke-linecap="round" stroke-linejoin="round"
                                    d="M2 7a2 2 0 012-2h16a2 2 0 012 2v10a2 2 0 01-2 2H4a2 2 0 01-2-2V7z"/>
                                <path stroke-linecap="round" stroke-linejoin="round" d="M2 7l10 6 10-6"/>
                            </svg>
                        </div>
                        <h2 class="text-xl font-bold text-gray-900">{move || i18n.t("auth.create_account")}</h2>
                    </div>

                    // Error
                    {move || error.get().map(|msg| view! {
                        <div class="mb-4 p-3 bg-red-50 border border-red-200 rounded-lg text-red-700 text-sm">
                            {msg}
                        </div>
                    })}

                    // Form
                    <form
                        class="space-y-4"
                        on:submit=move |ev| {
                            ev.prevent_default();
                            let nav = navigate.clone();

                            if !agreed_tos.get() {
                                error.set(Some(i18n.t("auth.agree_tos_required")));
                                return;
                            }

                            if password.get() != confirm_password.get() {
                                error.set(Some(i18n.t("auth.password_mismatch")));
                                return;
                            }

                            if let Err(pw_err) = validate_password_strength(&password.get()) {
                                let msg = match pw_err.as_str() {
                                    "password_too_short" => i18n.t("auth.password_too_short"),
                                    "password_weak" => i18n.t("auth.password_weak"),
                                    _ => pw_err,
                                };
                                error.set(Some(msg));
                                return;
                            }

                            match auth.signup(email.get(), name.get(), password.get()) {
                                Ok(()) => {
                                    nav("/", Default::default());
                                }
                                Err(e) => {
                                    error.set(Some(e));
                                }
                            }
                        }
                    >
                        <div>
                            <label class="label">{move || i18n.t("auth.name")}</label>
                            <input
                                type="text"
                                class="input-field"
                                required=true
                                prop:value=move || name.get()
                                on:input=move |ev| name.set(event_target_value(&ev))
                            />
                        </div>

                        <div>
                            <label class="label">{move || i18n.t("auth.email")}</label>
                            <input
                                type="email"
                                class="input-field"
                                required=true
                                placeholder="email@example.com"
                                prop:value=move || email.get()
                                on:input=move |ev| email.set(event_target_value(&ev))
                            />
                        </div>

                        <div>
                            <label class="label">{move || i18n.t("auth.password")}</label>
                            <input
                                type="password"
                                class="input-field"
                                required=true
                                placeholder="••••••••"
                                prop:value=move || password.get()
                                on:input=move |ev| password.set(event_target_value(&ev))
                            />
                        </div>

                        <div>
                            <label class="label">{move || i18n.t("auth.confirm_password")}</label>
                            <input
                                type="password"
                                class="input-field"
                                required=true
                                placeholder="••••••••"
                                prop:value=move || confirm_password.get()
                                on:input=move |ev| confirm_password.set(event_target_value(&ev))
                            />
                        </div>

                        <button type="submit" class="btn-primary text-lg py-4">
                            {move || i18n.t("auth.sign_up")}
                        </button>
                    </form>

                    <p class="mt-6 text-center text-sm text-gray-500">
                        {move || i18n.t("auth.have_account")} " "
                        <a href="/login" class="text-blue-600 font-medium">{move || i18n.t("auth.sign_in")}</a>
                    </p>

                    // Terms checkbox (above submit or below form)
                    <div class="mt-4 flex items-start gap-2">
                        <input
                            type="checkbox"
                            id="tos-checkbox"
                            class="mt-1 h-4 w-4 text-blue-600 border-gray-300 rounded"
                            prop:checked=move || agreed_tos.get()
                            on:change=move |ev| {
                                use wasm_bindgen::JsCast;
                                let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                                agreed_tos.set(target.checked());
                            }
                        />
                        <label for="tos-checkbox" class="text-xs text-gray-500">
                            {move || i18n.t("auth.agree_tos_prefix")} " "
                            <a href="/terms" target="_blank" class="text-blue-500 underline">"Terms of Service"</a>
                        </label>
                    </div>
                </div>
            </div>
        </div>
    }
}
