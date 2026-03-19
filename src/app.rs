use leptos::prelude::*;
use crate::i18n::I18n;
use crate::auth::AuthState;
use crate::router::AppRouter;

#[component]
pub fn App() -> impl IntoView {
    let i18n = I18n::new();
    let auth = AuthState::new();
    provide_context(i18n);
    provide_context(auth);

    view! {
        <AppRouter />
    }
}
