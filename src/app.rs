use leptos::prelude::*;
use crate::i18n::I18n;
use crate::auth::AuthState;
use crate::router::AppRouter;
use crate::components::common::ConfirmDialogProvider;
use crate::stores::asset_store;
use crate::models::accounting_standard::AccountingStandardSignal;

#[component]
pub fn App() -> impl IntoView {
    let i18n = I18n::new();
    let auth = AuthState::new();
    let standard = AccountingStandardSignal::new();
    provide_context(i18n);
    provide_context(auth);
    provide_context(standard);

    // Weekly cleanup: remove free accounts inactive for 40+ days (runs on Sundays)
    crate::auth::run_inactive_account_cleanup();

    // Check data version — if mismatch, reset all data (acts as "server reset")
    if asset_store::needs_data_reset() {
        leptos::task::spawn_local(async move {
            asset_store::reset_all_data().await;
            // Redirect to setup page after reset
            if let Some(window) = web_sys::window() {
                let _ = window.location().set_href("/setup");
            }
        });
    }

    // Periodic session timeout check (every 60 seconds)
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        let check_fn = Closure::wrap(Box::new(move || {
            auth.check_session_timeout();
        }) as Box<dyn Fn()>);

        if let Some(window) = web_sys::window() {
            let _ = window.set_interval_with_callback_and_timeout_and_arguments_0(
                check_fn.as_ref().unchecked_ref(),
                60_000, // 60 sec
            );
        }
        check_fn.forget();

        // Also touch session on user interaction
        let touch_fn = Closure::wrap(Box::new(move || {
            auth.touch_session();
        }) as Box<dyn Fn()>);

        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                let _ = document.add_event_listener_with_callback(
                    "click",
                    touch_fn.as_ref().unchecked_ref(),
                );
            }
        }
        touch_fn.forget();
    }

    view! {
        <ConfirmDialogProvider>
            <AppRouter />
        </ConfirmDialogProvider>
    }
}
