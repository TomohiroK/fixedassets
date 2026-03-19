use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::i18n::use_i18n;
use crate::auth::{use_auth, get_all_stored_users, toggle_user_paid};

const ADMIN_EMAIL: &str = "admin@example.com";

#[component]
pub fn AdminPage() -> impl IntoView {
    let i18n = use_i18n();
    let admin_authenticated = RwSignal::new(false);
    let password_input = RwSignal::new(String::new());
    let error_msg = RwSignal::new(Option::<String>::None);

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

            {move || if admin_authenticated.get() {
                view! { <AdminPanel /> }.into_any()
            } else {
                view! {
                    <div class="page-container">
                        <div class="max-w-sm mx-auto mt-12">
                            <div class="card">
                                <div class="text-center mb-6">
                                    <div class="w-16 h-16 bg-gray-900 rounded-2xl mx-auto flex items-center justify-center mb-4">
                                        <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                                        </svg>
                                    </div>
                                    <p class="text-sm text-gray-500">{ADMIN_EMAIL}</p>
                                </div>
                                <form on:submit=move |ev| {
                                    ev.prevent_default();
                                    let pw = password_input.get();
                                    let users = get_all_stored_users();
                                    if let Some(admin) = users.iter().find(|u| u.email == ADMIN_EMAIL) {
                                        if admin.password == pw {
                                            admin_authenticated.set(true);
                                            error_msg.set(None);
                                            return;
                                        }
                                    }
                                    error_msg.set(Some(i18n.t("admin.wrong_password")));
                                }>
                                    <input
                                        type="password"
                                        class="input-field mb-3"
                                        placeholder="Password"
                                        prop:value=move || password_input.get()
                                        on:input=move |ev| {
                                            use wasm_bindgen::JsCast;
                                            let target = ev.target().unwrap();
                                            let input: web_sys::HtmlInputElement = target.unchecked_into();
                                            password_input.set(input.value());
                                        }
                                    />
                                    {move || error_msg.get().map(|msg| view! {
                                        <p class="text-sm text-red-600 mb-3">{msg}</p>
                                    })}
                                    <button type="submit" class="btn-primary w-full">
                                        {move || i18n.t("admin.login_as")}
                                    </button>
                                </form>
                            </div>
                        </div>
                    </div>
                }.into_any()
            }}
        </div>
    }
}

#[component]
fn AdminPanel() -> impl IntoView {
    let i18n = use_i18n();
    let auth = use_auth();
    let navigate = use_navigate();

    let users = RwSignal::new(get_all_stored_users());

    view! {
        <div class="page-container">
            // Current user
            {move || auth.user.get().map(|user| {
                let plan_label = if user.paid {
                    i18n.t("admin.paid")
                } else {
                    i18n.t("admin.free")
                };
                view! {
                    <div class="card mb-4 bg-blue-50 border-blue-200">
                        <p class="text-xs text-blue-600 font-medium mb-1">{move || i18n.t("admin.current_user")}</p>
                        <div class="flex items-center justify-between">
                            <div>
                                <p class="font-semibold text-gray-900">{user.name.clone()}</p>
                                <p class="text-sm text-gray-500">{user.email.clone()}</p>
                            </div>
                            <span class=if user.paid {
                                "text-xs font-medium px-2 py-1 rounded-full bg-amber-100 text-amber-700"
                            } else {
                                "text-xs font-medium px-2 py-1 rounded-full bg-gray-100 text-gray-600"
                            }>{plan_label}</span>
                        </div>
                    </div>
                }
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
                                let paid = user.paid;
                                let email_for_login = user.email.clone();
                                let name_for_login = user.name.clone();
                                let paid_for_login = user.paid;
                                let email_for_toggle = user.email.clone();
                                let nav = navigate.clone();
                                let plan_text = if paid {
                                    i18n.t("admin.paid")
                                } else {
                                    i18n.t("admin.free")
                                };
                                view! {
                                    <div class="card">
                                        <div class="flex items-center justify-between mb-2">
                                            <div class="flex-1 min-w-0 mr-2">
                                                <p class="font-medium text-gray-900">{name}</p>
                                                <p class="text-sm text-gray-500">{email}</p>
                                            </div>
                                            <button
                                                class="text-sm bg-blue-600 text-white px-3 py-2 rounded-lg active:bg-blue-700 transition-colors shrink-0"
                                                on:click=move |_| {
                                                    auth.login(email_for_login.clone(), name_for_login.clone(), paid_for_login);
                                                    let nav = nav.clone();
                                                    nav("/", Default::default());
                                                }
                                            >
                                                {move || i18n.t("admin.login_as")}
                                            </button>
                                        </div>
                                        <div class="flex items-center justify-between pt-2 border-t border-gray-100">
                                            <div class="flex items-center gap-2">
                                                <span class="text-xs text-gray-500">{move || i18n.t("admin.plan")}</span>
                                                <span class=if paid {
                                                    "text-xs font-medium px-2 py-0.5 rounded-full bg-amber-100 text-amber-700"
                                                } else {
                                                    "text-xs font-medium px-2 py-0.5 rounded-full bg-gray-100 text-gray-600"
                                                }>{plan_text}</span>
                                            </div>
                                            <button
                                                class="text-xs text-blue-600 font-medium px-2 py-1 border border-blue-200 rounded-lg active:bg-blue-50"
                                                on:click={
                                                    let email_for_toggle = email_for_toggle.clone();
                                                    move |_| {
                                                        toggle_user_paid(&email_for_toggle);
                                                        users.set(get_all_stored_users());
                                                        if let Some(current) = auth.user.get() {
                                                            if current.email == email_for_toggle {
                                                                let new_users = get_all_stored_users();
                                                                if let Some(u) = new_users.iter().find(|u| u.email == email_for_toggle) {
                                                                    auth.login(u.email.clone(), u.name.clone(), u.paid);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            >
                                                {move || i18n.t("admin.toggle_plan")}
                                            </button>
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
