use leptos::prelude::*;
use crate::i18n::use_i18n;

#[component]
pub fn LandingPage() -> impl IntoView {
    let i18n = use_i18n();

    // A/B test: randomly pick background image
    let bg_image = if js_sys::Math::random() < 0.5 {
        "images/hero-1.webp"
    } else {
        "images/hero-2.webp"
    };

    let bg_style = format!(
        "background-image: url('{}'); background-size: cover; background-position: center;",
        bg_image
    );

    view! {
        <div class="min-h-screen text-white relative" style=bg_style>
            // Dark overlay for readability
            <div class="absolute inset-0 bg-black/50"></div>

            // Content
            <div class="relative z-10 min-h-screen flex flex-col">
                // Header
                <div class="px-6 pt-6">
                    <div class="flex items-center justify-between max-w-lg mx-auto">
                        <h1 class="text-2xl font-bold tracking-tight drop-shadow-lg">{move || i18n.t("app.title")}</h1>
                        <button
                            class="text-sm bg-white/20 backdrop-blur-sm px-3 py-1.5 rounded-lg"
                            on:click=move |_| {
                                let current = i18n.current_locale();
                                let next = if current == "en" { "ja" } else { "en" };
                                i18n.set_locale(next);
                            }
                        >
                            {move || if i18n.current_locale() == "en" { "日本語" } else { "English" }}
                        </button>
                    </div>
                </div>

                // Spacer to push content down
                <div class="flex-1"></div>

                // Bottom content area
                <div class="px-6 pb-8">
                    <div class="max-w-lg mx-auto">
                        // Headline
                        <h2 class="text-3xl font-bold leading-tight mb-2 drop-shadow-lg">
                            {move || i18n.t("landing.headline")}
                        </h2>
                        <p class="text-white/80 text-sm leading-relaxed mb-6 drop-shadow">
                            {move || i18n.t("landing.subheadline")}
                        </p>

                        // Feature pills - compact horizontal
                        <div class="flex gap-2 mb-6 overflow-x-auto pb-1">
                            <div class="flex items-center gap-1.5 bg-white/15 backdrop-blur-sm rounded-full px-3 py-1.5 shrink-0">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 6v6m0 0v6m0-6h6m-6 0H6"/>
                                </svg>
                                <span class="text-xs font-medium whitespace-nowrap">{move || i18n.t("landing.feature_track")}</span>
                            </div>
                            <div class="flex items-center gap-1.5 bg-white/15 backdrop-blur-sm rounded-full px-3 py-1.5 shrink-0">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z"/>
                                </svg>
                                <span class="text-xs font-medium whitespace-nowrap">{move || i18n.t("landing.feature_depreciation")}</span>
                            </div>
                            <div class="flex items-center gap-1.5 bg-white/15 backdrop-blur-sm rounded-full px-3 py-1.5 shrink-0">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064"/>
                                </svg>
                                <span class="text-xs font-medium whitespace-nowrap">{move || i18n.t("landing.feature_multilang")}</span>
                            </div>
                        </div>

                        // CTA Buttons
                        <div class="space-y-3">
                            <a href="/signup" class="block w-full py-4 bg-blue-600 hover:bg-blue-500 text-white rounded-xl font-bold text-lg text-center shadow-lg active:bg-blue-700 transition-colors">
                                {move || i18n.t("landing.get_started")}
                            </a>
                            <a href="/login" class="block w-full py-4 bg-white/15 backdrop-blur-sm text-white rounded-xl font-bold text-lg text-center active:bg-white/25 transition-colors border border-white/30">
                                {move || i18n.t("landing.sign_in")}
                            </a>
                        </div>

                        // Footer
                        <div class="text-center mt-6 text-white/50 text-xs">
                            <p>{move || i18n.t("landing.footer")}</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
