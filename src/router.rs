use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use crate::auth::use_auth;
use crate::components::layout::PageShell;
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
};

#[component]
pub fn AppRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <div class="page-container">"404 - Page not found"</div> }>
                <Route path=path!("/welcome") view=LandingPage />
                <Route path=path!("/login") view=LoginPage />
                <Route path=path!("/signup") view=SignupPage />
                <Route path=path!("/admin") view=AdminPage />
                <Route path=path!("/") view=GuardedDashboard />
                <Route path=path!("/assets") view=GuardedAssetList />
                <Route path=path!("/register") view=GuardedRegister />
                <Route path=path!("/assets/:id") view=GuardedDetail />
                <Route path=path!("/settings") view=GuardedSettings />
            </Routes>
        </Router>
    }
}

#[component]
fn GuardedDashboard() -> impl IntoView {
    let auth = use_auth();
    view! {
        {move || if auth.is_logged_in() {
            view! { <PageShell><DashboardPage /></PageShell> }.into_any()
        } else {
            view! { <LandingPage /> }.into_any()
        }}
    }
}

#[component]
fn GuardedAssetList() -> impl IntoView {
    let auth = use_auth();
    view! {
        {move || if auth.is_logged_in() {
            view! { <PageShell><AssetListPage /></PageShell> }.into_any()
        } else {
            view! { <LandingPage /> }.into_any()
        }}
    }
}

#[component]
fn GuardedRegister() -> impl IntoView {
    let auth = use_auth();
    view! {
        {move || if auth.is_logged_in() {
            view! { <PageShell><AssetRegisterPage /></PageShell> }.into_any()
        } else {
            view! { <LandingPage /> }.into_any()
        }}
    }
}

#[component]
fn GuardedDetail() -> impl IntoView {
    let auth = use_auth();
    view! {
        {move || if auth.is_logged_in() {
            view! { <PageShell><AssetDetailPage /></PageShell> }.into_any()
        } else {
            view! { <LandingPage /> }.into_any()
        }}
    }
}

#[component]
fn GuardedSettings() -> impl IntoView {
    let auth = use_auth();
    view! {
        {move || if auth.is_logged_in() {
            view! { <PageShell><SettingsPage /></PageShell> }.into_any()
        } else {
            view! { <LandingPage /> }.into_any()
        }}
    }
}
