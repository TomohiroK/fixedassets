use leptos::prelude::*;
use crate::i18n::use_i18n;

#[component]
pub fn LandingPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="min-h-screen bg-gradient-to-br from-blue-600 via-blue-700 to-indigo-800 text-white">
            // Hero section with background
            <div class="relative overflow-hidden">
                // Abstract pattern overlay
                <div class="absolute inset-0 opacity-10">
                    <svg class="w-full h-full" viewBox="0 0 800 600" fill="none">
                        <circle cx="400" cy="300" r="250" stroke="white" stroke-width="1"/>
                        <circle cx="400" cy="300" r="200" stroke="white" stroke-width="1"/>
                        <circle cx="400" cy="300" r="150" stroke="white" stroke-width="1"/>
                        <rect x="150" y="100" width="500" height="400" rx="20" stroke="white" stroke-width="1"/>
                        <line x1="200" y1="200" x2="600" y2="200" stroke="white" stroke-width="1"/>
                        <line x1="200" y1="280" x2="600" y2="280" stroke="white" stroke-width="1"/>
                        <line x1="200" y1="360" x2="600" y2="360" stroke="white" stroke-width="1"/>
                    </svg>
                </div>

                <div class="relative z-10 px-6 pt-8 pb-4">
                    // Header
                    <div class="flex items-center justify-between max-w-lg mx-auto">
                        <h1 class="text-2xl font-bold tracking-tight">{move || i18n.t("app.title")}</h1>
                        <button
                            class="text-sm bg-white/20 backdrop-blur px-3 py-1.5 rounded-lg"
                            on:click=move |_| {
                                let current = i18n.current_locale();
                                let next = if current == "en" { "ja" } else { "en" };
                                i18n.set_locale(next);
                            }
                        >
                            {move || if i18n.current_locale() == "en" { "日本語" } else { "English" }}
                        </button>
                    </div>

                    // Hero content
                    <div class="max-w-lg mx-auto mt-16 text-center">
                        // Asset icon
                        <div class="w-24 h-24 bg-white/20 backdrop-blur rounded-3xl mx-auto flex items-center justify-center mb-8 shadow-2xl">
                            <svg class="w-14 h-14" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.5">
                                <path stroke-linecap="round" stroke-linejoin="round"
                                    d="M2 7a2 2 0 012-2h16a2 2 0 012 2v10a2 2 0 01-2 2H4a2 2 0 01-2-2V7z"/>
                                <path stroke-linecap="round" stroke-linejoin="round" d="M2 7l10 6 10-6"/>
                                <path stroke-linecap="round" stroke-linejoin="round"
                                    d="M9 12l-2 2m0-2l2 2m4-2l2 2m-2 0l-2-2"/>
                            </svg>
                        </div>

                        <h2 class="text-3xl font-bold leading-tight mb-4">
                            {move || i18n.t("landing.headline")}
                        </h2>
                        <p class="text-blue-100 text-lg mb-12 leading-relaxed">
                            {move || i18n.t("landing.subheadline")}
                        </p>

                        // Feature cards
                        <div class="grid grid-cols-3 gap-3 mb-12">
                            <div class="bg-white/10 backdrop-blur rounded-xl p-3 text-center">
                                <svg class="w-8 h-8 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.5">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 6v6m0 0v6m0-6h6m-6 0H6"/>
                                </svg>
                                <p class="text-xs font-medium">{move || i18n.t("landing.feature_track")}</p>
                            </div>
                            <div class="bg-white/10 backdrop-blur rounded-xl p-3 text-center">
                                <svg class="w-8 h-8 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.5">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z"/>
                                </svg>
                                <p class="text-xs font-medium">{move || i18n.t("landing.feature_depreciation")}</p>
                            </div>
                            <div class="bg-white/10 backdrop-blur rounded-xl p-3 text-center">
                                <svg class="w-8 h-8 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.5">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064"/>
                                </svg>
                                <p class="text-xs font-medium">{move || i18n.t("landing.feature_multilang")}</p>
                            </div>
                        </div>

                        // CTA Buttons
                        <div class="space-y-3 max-w-sm mx-auto">
                            <a href="/signup" class="block w-full py-4 bg-white text-blue-700 rounded-xl font-bold text-lg text-center shadow-lg active:bg-gray-100 transition-colors">
                                {move || i18n.t("landing.get_started")}
                            </a>
                            <a href="/login" class="block w-full py-4 bg-white/20 backdrop-blur text-white rounded-xl font-bold text-lg text-center active:bg-white/30 transition-colors">
                                {move || i18n.t("landing.sign_in")}
                            </a>
                        </div>
                    </div>

                    // Footer
                    <div class="text-center mt-12 pb-8 text-blue-200 text-xs">
                        <p>{move || i18n.t("landing.footer")}</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
