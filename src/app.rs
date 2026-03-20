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
        <AppRouter />
    }
}
