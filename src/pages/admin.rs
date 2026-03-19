use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use serde::{Deserialize, Serialize};
use crate::i18n::use_i18n;
use crate::auth::use_auth;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StoredUser {
    email: String,
    name: String,
    password: String,
}

fn get_all_users() -> Vec<StoredUser> {
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().ok().flatten().unwrap();
    let json = storage.get_item("fa_users").ok().flatten().unwrap_or_else(|| "[]".to_string());
    serde_json::from_str(&json).unwrap_or_default()
}

#[component]
pub fn AdminPage() -> impl IntoView {
    let i18n = use_i18n();
    let auth = use_auth();
    let navigate = use_navigate();

    let users = RwSignal::new(get_all_users());

    view! {
        <div class="min-h-screen bg-gray-50">
            // Header
            <div class="bg-gray-900 text-white px-4 py-3">
                <div class="flex items-center justify-between max-w-lg mx-auto">
                    <h1 class="text-lg font-bold">{move || i18n.t("admin.title")}</h1>
                    <a href="/" class="text-sm bg-gray-700 px-3 py-1.5 rounded-lg">
                        {move || i18n.t("admin.back_to_app")}
                    </a>
                </div>
            </div>

            <div class="page-container">
                // Current user
                {move || auth.user.get().map(|user| view! {
                    <div class="card mb-4 bg-blue-50 border-blue-200">
                        <p class="text-xs text-blue-600 font-medium mb-1">{move || i18n.t("admin.current_user")}</p>
                        <p class="font-semibold text-gray-900">{user.name.clone()}</p>
                        <p class="text-sm text-gray-500">{user.email.clone()}</p>
                    </div>
                })}

                // All accounts
                <h2 class="font-semibold text-gray-900 mb-3">{move || i18n.t("admin.accounts")}</h2>

                {move || {
                    let user_list = users.get();
                    if user_list.is_empty() {
                        view! {
                            <div class="card text-center text-gray-500 py-8">
                                {move || i18n.t("admin.no_accounts")}
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="space-y-2">
                                {user_list.into_iter().map(|user| {
                                    let email = user.email.clone();
                                    let name = user.name.clone();
                                    let email_for_login = user.email.clone();
                                    let name_for_login = user.name.clone();
                                    let nav = navigate.clone();
                                    view! {
                                        <div class="card flex items-center justify-between">
                                            <div>
                                                <p class="font-medium text-gray-900">{name}</p>
                                                <p class="text-sm text-gray-500">{email}</p>
                                            </div>
                                            <button
                                                class="text-sm bg-blue-600 text-white px-3 py-2 rounded-lg active:bg-blue-700 transition-colors"
                                                on:click=move |_| {
                                                    auth.login(email_for_login.clone(), name_for_login.clone());
                                                    let nav = nav.clone();
                                                    nav("/", Default::default());
                                                }
                                            >
                                                {move || i18n.t("admin.login_as")}
                                            </button>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
