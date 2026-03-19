use leptos::prelude::*;
use leptos_router::hooks::use_location;
use crate::i18n::use_i18n;

#[component]
pub fn Header() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <header class="fixed top-0 left-0 right-0 z-50 bg-blue-700 text-white shadow-md">
            <div class="flex items-center justify-between px-4 py-3 max-w-lg mx-auto">
                <h1 class="text-lg font-bold">{move || i18n.t("app.title")}</h1>
                <button
                    class="text-sm bg-blue-600 px-3 py-1 rounded-md active:bg-blue-800 transition-colors"
                    on:click=move |_| {
                        let current = i18n.current_locale();
                        let next = if current == "en" { "ja" } else { "en" };
                        i18n.set_locale(next);
                    }
                >
                    {move || {
                        if i18n.current_locale() == "en" {
                            "日本語"
                        } else {
                            "English"
                        }
                    }}
                </button>
            </div>
        </header>
    }
}

#[component]
pub fn BottomNav() -> impl IntoView {
    let i18n = use_i18n();
    let location = use_location();

    let is_active = move |path: &str| -> &str {
        let current = location.pathname.get();
        if current == path || (path == "/" && current == "") {
            "text-blue-600"
        } else {
            "text-gray-500"
        }
    };

    view! {
        <nav class="fixed bottom-0 left-0 right-0 z-50 bg-white border-t border-gray-200 safe-area-bottom">
            <div class="flex items-center justify-around max-w-lg mx-auto py-2">
                <a href="/" class=move || format!("flex flex-col items-center gap-0.5 px-3 py-1 {}", is_active("/"))>
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"/>
                    </svg>
                    <span class="text-xs">{move || i18n.t("nav.dashboard")}</span>
                </a>
                <a href="/assets" class=move || format!("flex flex-col items-center gap-0.5 px-3 py-1 {}", is_active("/assets"))>
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>
                    </svg>
                    <span class="text-xs">{move || i18n.t("nav.assets")}</span>
                </a>
                <a href="/register" class="flex flex-col items-center gap-0.5 px-3 py-1">
                    <div class="w-12 h-12 bg-blue-600 rounded-full flex items-center justify-center -mt-6 shadow-lg active:bg-blue-700 transition-colors">
                        <svg class="w-7 h-7 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
                        </svg>
                    </div>
                    <span class="text-xs text-gray-500">{move || i18n.t("nav.register")}</span>
                </a>
                <a href="/settings" class=move || format!("flex flex-col items-center gap-0.5 px-3 py-1 {}", is_active("/settings"))>
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                    </svg>
                    <span class="text-xs">{move || i18n.t("nav.settings")}</span>
                </a>
            </div>
        </nav>
    }
}

#[component]
pub fn PageShell(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50">
            <Header />
            <main class="pt-14 pb-20">
                {children()}
            </main>
            <BottomNav />
        </div>
    }
}
