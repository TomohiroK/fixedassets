use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use crate::auth::use_auth;
use crate::components::layout::PageShell;
use crate::models::company::CompanySetup;
use crate::pages::{
    dashboard::DashboardPage,
    asset_list::AssetListPage,
    asset_register::AssetRegisterPage,
    asset_detail::AssetDetailPage,
    settings::SettingsPage,
    landing::LandingPage,
    country_landing::{CountryLandingPage, GeoRedirectLanding},
    login::LoginPage,
    signup::SignupPage,
    admin::AdminPage,
    setup::SetupPage,
    terms::TermsPage,
    depreciation::DepreciationPage,
};

#[component]
pub fn AppRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <div class="page-container">"404 - Page not found"</div> }>
                // Public routes - no setup required
                <Route path=path!("/welcome") view=GeoRedirectLanding />
                <Route path=path!("/login") view=LoginPage />
                <Route path=path!("/signup") view=SignupPage />
                <Route path=path!("/admin") view=AdminPage />
                <Route path=path!("/terms") view=TermsPage />
                <Route path=path!("/setup") view=SetupGuardedSetup />

                // Country-specific SEO landing pages
                <Route path=path!("/japan") view=CountryJapan />
                <Route path=path!("/singapore") view=CountrySingapore />
                <Route path=path!("/malaysia") view=CountryMalaysia />
                <Route path=path!("/thailand") view=CountryThailand />
                <Route path=path!("/indonesia") view=CountryIndonesia />
                <Route path=path!("/philippines") view=CountryPhilippines />
                <Route path=path!("/vietnam") view=CountryVietnam />
                <Route path=path!("/myanmar") view=CountryMyanmar />
                <Route path=path!("/cambodia") view=CountryCambodia />
                <Route path=path!("/laos") view=CountryLaos />
                <Route path=path!("/brunei") view=CountryBrunei />

                // Auth + Setup guarded routes
                <Route path=path!("/") view=GuardedDashboard />
                <Route path=path!("/assets") view=GuardedAssetList />
                <Route path=path!("/register") view=GuardedRegister />
                <Route path=path!("/assets/:id") view=GuardedDetail />
                <Route path=path!("/settings") view=GuardedSettings />
                <Route path=path!("/depreciation") view=GuardedDepreciation />
            </Routes>
        </Router>
    }
}

// Country landing page wrappers (each passes the slug)
#[component] fn CountryJapan() -> impl IntoView { view! { <CountryLandingPage country_slug="japan".to_string() /> } }
#[component] fn CountrySingapore() -> impl IntoView { view! { <CountryLandingPage country_slug="singapore".to_string() /> } }
#[component] fn CountryMalaysia() -> impl IntoView { view! { <CountryLandingPage country_slug="malaysia".to_string() /> } }
#[component] fn CountryThailand() -> impl IntoView { view! { <CountryLandingPage country_slug="thailand".to_string() /> } }
#[component] fn CountryIndonesia() -> impl IntoView { view! { <CountryLandingPage country_slug="indonesia".to_string() /> } }
#[component] fn CountryPhilippines() -> impl IntoView { view! { <CountryLandingPage country_slug="philippines".to_string() /> } }
#[component] fn CountryVietnam() -> impl IntoView { view! { <CountryLandingPage country_slug="vietnam".to_string() /> } }
#[component] fn CountryMyanmar() -> impl IntoView { view! { <CountryLandingPage country_slug="myanmar".to_string() /> } }
#[component] fn CountryCambodia() -> impl IntoView { view! { <CountryLandingPage country_slug="cambodia".to_string() /> } }
#[component] fn CountryLaos() -> impl IntoView { view! { <CountryLandingPage country_slug="laos".to_string() /> } }
#[component] fn CountryBrunei() -> impl IntoView { view! { <CountryLandingPage country_slug="brunei".to_string() /> } }

fn needs_setup() -> bool {
    CompanySetup::load().is_none()
}

/// Setup page itself requires auth
#[component]
fn SetupGuardedSetup() -> impl IntoView {
    let auth = use_auth();
    view! {
        {move || if auth.is_logged_in() {
            view! { <SetupPage /> }.into_any()
        } else {
            view! { <LandingPage /> }.into_any()
        }}
    }
}

#[component]
fn GuardedDashboard() -> impl IntoView {
    let auth = use_auth();
    view! {
        {move || if !auth.is_logged_in() {
            view! { <LandingPage /> }.into_any()
        } else if needs_setup() {
            view! { <SetupPage /> }.into_any()
        } else {
            view! { <PageShell><DashboardPage /></PageShell> }.into_any()
        }}
    }
}

#[component]
fn GuardedAssetList() -> impl IntoView {
    let auth = use_auth();
    view! {
        {move || if !auth.is_logged_in() {
            view! { <LandingPage /> }.into_any()
        } else if needs_setup() {
            view! { <SetupPage /> }.into_any()
        } else {
            view! { <PageShell><AssetListPage /></PageShell> }.into_any()
        }}
    }
}

#[component]
fn GuardedRegister() -> impl IntoView {
    let auth = use_auth();
    view! {
        {move || if !auth.is_logged_in() {
            view! { <LandingPage /> }.into_any()
        } else if needs_setup() {
            view! { <SetupPage /> }.into_any()
        } else {
            view! { <PageShell><AssetRegisterPage /></PageShell> }.into_any()
        }}
    }
}

#[component]
fn GuardedDetail() -> impl IntoView {
    let auth = use_auth();
    view! {
        {move || if !auth.is_logged_in() {
            view! { <LandingPage /> }.into_any()
        } else if needs_setup() {
            view! { <SetupPage /> }.into_any()
        } else {
            view! { <PageShell><AssetDetailPage /></PageShell> }.into_any()
        }}
    }
}

#[component]
fn GuardedDepreciation() -> impl IntoView {
    let auth = use_auth();
    view! {
        {move || if !auth.is_logged_in() {
            view! { <LandingPage /> }.into_any()
        } else if needs_setup() {
            view! { <SetupPage /> }.into_any()
        } else {
            view! { <PageShell><DepreciationPage /></PageShell> }.into_any()
        }}
    }
}

#[component]
fn GuardedSettings() -> impl IntoView {
    let auth = use_auth();
    view! {
        {move || if !auth.is_logged_in() {
            view! { <LandingPage /> }.into_any()
        } else if needs_setup() {
            view! { <SetupPage /> }.into_any()
        } else {
            view! { <PageShell><SettingsPage /></PageShell> }.into_any()
        }}
    }
}
