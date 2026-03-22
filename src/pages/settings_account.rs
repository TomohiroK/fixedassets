use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::i18n::use_i18n;
use crate::auth::use_auth;

/// Account section: displays user info, logout button, and edit form
#[component]
pub fn AccountSection() -> impl IntoView {
    let i18n = use_i18n();
    let auth = use_auth();
    let navigate = use_navigate();

    let editing = RwSignal::new(false);
    let edit_name = RwSignal::new(String::new());
    let edit_email = RwSignal::new(String::new());
    let edit_new_password = RwSignal::new(String::new());
    let edit_current_password = RwSignal::new(String::new());
    let edit_error = RwSignal::new(Option::<String>::None);
    let edit_success = RwSignal::new(false);

    let start_edit = move |_| {
        if let Some(user) = auth.user.get() {
            edit_name.set(user.name.clone());
            edit_email.set(user.email.clone());
            edit_new_password.set(String::new());
            edit_current_password.set(String::new());
            edit_error.set(None);
            edit_success.set(false);
            editing.set(true);
        }
    };

    let cancel_edit = move |_| {
        editing.set(false);
        edit_error.set(None);
        edit_success.set(false);
    };

    let save_edit = move |_| {
        let name = edit_name.get().trim().to_string();
        let email = edit_email.get().trim().to_string();
        let current_pw = edit_current_password.get();
        let new_pw_raw = edit_new_password.get().trim().to_string();
        let new_pw = if new_pw_raw.is_empty() { None } else { Some(new_pw_raw) };

        if name.is_empty() || email.is_empty() {
            edit_error.set(Some("Name and email are required".to_string()));
            return;
        }
        if current_pw.is_empty() {
            edit_error.set(Some(i18n.t("auth.current_password_required")));
            return;
        }

        match auth.update_account(current_pw, name, email, new_pw) {
            Ok(()) => {
                edit_success.set(true);
                edit_error.set(None);
                editing.set(false);
            }
            Err(e) => {
                let msg = match e.as_str() {
                    "wrong_password" => i18n.t("auth.update_error_wrong_password"),
                    "email_taken" => i18n.t("auth.email_taken"),
                    "password_too_short" => i18n.t("auth.password_too_short"),
                    "password_weak" => i18n.t("auth.password_weak"),
                    other => other.to_string(),
                };
                edit_error.set(Some(msg));
            }
        }
    };

    view! {
        {move || {
            let user = auth.user.get();
            let user = match user {
                Some(u) => u,
                None => return view! { <div></div> }.into_any(),
            };

            if editing.get() {
                // Edit mode
                view! {
                    <div class="card mb-4">
                        <h3 class="font-semibold text-gray-900 mb-3">{move || i18n.t("auth.edit_account")}</h3>

                        {move || edit_error.get().map(|msg| view! {
                            <div class="bg-red-50 border border-red-200 text-red-700 text-sm px-3 py-2 rounded-lg mb-3">{msg}</div>
                        })}

                        <div class="space-y-3">
                            <div>
                                <label class="text-sm font-medium text-gray-700 block mb-1">{move || i18n.t("auth.name")}</label>
                                <input
                                    type="text"
                                    class="input-field"
                                    prop:value=move || edit_name.get()
                                    on:input=move |ev| edit_name.set(event_target_value(&ev))
                                />
                            </div>
                            <div>
                                <label class="text-sm font-medium text-gray-700 block mb-1">{move || i18n.t("auth.email")}</label>
                                <input
                                    type="email"
                                    class="input-field"
                                    prop:value=move || edit_email.get()
                                    on:input=move |ev| edit_email.set(event_target_value(&ev))
                                />
                            </div>
                            <div>
                                <label class="text-sm font-medium text-gray-700 block mb-1">{move || i18n.t("auth.new_password")}</label>
                                <input
                                    type="password"
                                    class="input-field"
                                    placeholder=move || i18n.t("auth.new_password_hint")
                                    prop:value=move || edit_new_password.get()
                                    on:input=move |ev| edit_new_password.set(event_target_value(&ev))
                                />
                            </div>
                            <hr class="border-gray-200" />
                            <div>
                                <label class="text-sm font-medium text-gray-700 block mb-1">{move || i18n.t("auth.current_password")}</label>
                                <input
                                    type="password"
                                    class="input-field"
                                    placeholder=move || i18n.t("auth.current_password_required")
                                    prop:value=move || edit_current_password.get()
                                    on:input=move |ev| edit_current_password.set(event_target_value(&ev))
                                />
                            </div>
                            <div class="flex gap-2 pt-1">
                                <button
                                    class="btn-secondary flex-1"
                                    on:click=cancel_edit
                                >
                                    {move || i18n.t("auth.cancel")}
                                </button>
                                <button
                                    class="btn-primary flex-1"
                                    on:click=save_edit
                                >
                                    {move || i18n.t("auth.save_changes")}
                                </button>
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                // View mode
                let nav = navigate.clone();
                view! {
                    <div class="card mb-4">
                        {move || edit_success.get().then(|| view! {
                            <div class="bg-green-50 border border-green-200 text-green-700 text-sm px-3 py-2 rounded-lg mb-3">
                                {move || i18n.t("auth.update_success")}
                            </div>
                        })}
                        <div class="flex items-center justify-between">
                            <div>
                                <p class="font-semibold text-gray-900">{user.name.clone()}</p>
                                <p class="text-sm text-gray-500">{user.email.clone()}</p>
                            </div>
                            <div class="flex gap-2">
                                <button
                                    class="text-sm text-blue-600 font-medium px-3 py-2 border border-blue-200 rounded-lg active:bg-blue-50"
                                    on:click=start_edit
                                >
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/>
                                    </svg>
                                </button>
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
                    </div>
                }.into_any()
            }
        }}
    }
}
