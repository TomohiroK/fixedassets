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
    login::LoginPage,
    signup::SignupPage,
    admin::AdminPage,
    setup::SetupPage,
};

#[component]
pub fn AppRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <div class="page-container">"404 - Page not found"</div> }>
                // Public routes - no setup required
                <Route path=path!("/welcome") view=LandingPage />
                <Route path=path!("/login") view=LoginPage />
                <Route path=path!("/signup") view=SignupPage />
                <Route path=path!("/admin") view=AdminPage />
                <Route path=path!("/setup") view=SetupGuardedSetup />
                // Auth + Setup guarded routes
                <Route path=path!("/") view=GuardedDashboard />
                <Route path=path!("/assets") view=GuardedAssetList />
                <Route path=path!("/register") view=GuardedRegister />
                <Route path=path!("/assets/:id") view=GuardedDetail />
                <Route path=path!("/settings") view=GuardedSettings />
            </Routes>
        </Router>
    }
}

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
