use leptos::prelude::*;
use crate::i18n::use_i18n;

#[component]
pub fn LoadingSpinner() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <div class="flex flex-col items-center justify-center py-12">
            <div class="w-8 h-8 border-4 border-blue-200 border-t-blue-600 rounded-full animate-spin"></div>
            <p class="mt-3 text-sm text-gray-500">{move || i18n.t("common.loading")}</p>
        </div>
    }
}

#[component]
pub fn EmptyState(
    #[prop(into)] message: String,
    #[prop(optional)] sub_message: Option<String>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center py-16 px-4">
            <svg class="w-16 h-16 text-gray-300 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                    d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4"/>
            </svg>
            <p class="text-gray-500 font-medium">{message}</p>
            {sub_message.map(|msg| view! {
                <p class="text-sm text-gray-400 mt-1">{msg}</p>
            })}
        </div>
    }
}

#[component]
pub fn SearchBar(
    value: RwSignal<String>,
    #[prop(into)] placeholder: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="relative">
            <svg class="w-5 h-5 text-gray-400 absolute left-3 top-1/2 -translate-y-1/2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
            </svg>
            <input
                type="search"
                class="input-field pl-10"
                placeholder=placeholder
                prop:value=move || value.get()
                on:input=move |ev| {
                    use leptos::prelude::*;
                    let val = event_target_value(&ev);
                    value.set(val);
                }
            />
        </div>
    }
}

pub fn format_currency(amount: &rust_decimal::Decimal) -> String {
    use crate::models::company::CompanySetup;

    let symbol = CompanySetup::load()
        .and_then(|s| s.currency())
        .map(|c| c.symbol().to_string())
        .unwrap_or_else(|| "$".to_string());

    let s = format!("{:.2}", amount);
    let parts: Vec<&str> = s.split('.').collect();
    let int_part = parts[0];
    let dec_part = parts.get(1).unwrap_or(&"00");

    let is_negative = int_part.starts_with('-');
    let digits: Vec<char> = int_part.chars().filter(|c| c.is_ascii_digit()).collect();

    let mut formatted = String::new();
    let len = digits.len();
    for (i, c) in digits.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            formatted.push(',');
        }
        formatted.push(*c);
    }

    if is_negative {
        format!("{}-{}.{}", symbol, formatted, dec_part)
    } else {
        format!("{}{}.{}", symbol, formatted, dec_part)
    }
}

// ============================================================
// Custom Confirm Dialog (global context pattern)
// ============================================================

/// Style variant for the confirm dialog
#[derive(Clone, Debug, PartialEq)]
pub enum ConfirmStyle {
    Danger,   // Red - for delete
    Warning,  // Orange - for dispose, clear data
    Info,     // Blue - for undo, general
}

impl ConfirmStyle {
    fn icon_color(&self) -> &str {
        match self {
            ConfirmStyle::Danger => "text-red-500",
            ConfirmStyle::Warning => "text-orange-500",
            ConfirmStyle::Info => "text-blue-500",
        }
    }
    fn icon_bg(&self) -> &str {
        match self {
            ConfirmStyle::Danger => "bg-red-50",
            ConfirmStyle::Warning => "bg-orange-50",
            ConfirmStyle::Info => "bg-blue-50",
        }
    }
    fn confirm_btn(&self) -> &str {
        match self {
            ConfirmStyle::Danger => "bg-red-500 active:bg-red-600 text-white",
            ConfirmStyle::Warning => "bg-orange-500 active:bg-orange-600 text-white",
            ConfirmStyle::Info => "bg-blue-500 active:bg-blue-600 text-white",
        }
    }
}

/// Cloneable handle for triggering confirm dialogs (all signals, WASM-safe)
#[derive(Clone, Copy)]
pub struct ConfirmDialog {
    visible: RwSignal<bool>,
    message: RwSignal<String>,
    style: RwSignal<ConfirmStyle>,
    confirm_label: RwSignal<String>,
    cancel_label: RwSignal<String>,
    /// We store callback as an Option<Fn> wrapped in a signal.
    /// Using RwSignal<Option<...>> with a boxed closure via SendWrapper.
    confirmed: RwSignal<u32>, // bump to trigger
    pending_id: RwSignal<u32>,
}

impl ConfirmDialog {
    pub fn show(
        &self,
        message: &str,
        style: ConfirmStyle,
        confirm_label: &str,
        cancel_label: &str,
        on_confirm: impl FnOnce() + 'static,
    ) {
        self.message.set(message.to_string());
        self.style.set(style);
        self.confirm_label.set(confirm_label.to_string());
        self.cancel_label.set(cancel_label.to_string());

        // Store callback in a JS closure via wasm_bindgen
        let id = self.pending_id.get_untracked() + 1;
        self.pending_id.set(id);

        // Use a shared cell to store the callback (single-threaded WASM)
        let cb: std::rc::Rc<std::cell::RefCell<Option<Box<dyn FnOnce()>>>> =
            std::rc::Rc::new(std::cell::RefCell::new(Some(Box::new(on_confirm))));

        // Store in window as a JS property for retrieval
        if let Some(window) = web_sys::window() {
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;

            let closure = Closure::once(Box::new(move || {
                if let Some(f) = cb.borrow_mut().take() {
                    f();
                }
            }) as Box<dyn FnOnce()>);

            let _ = js_sys::Reflect::set(
                &window,
                &JsValue::from_str("__confirm_cb"),
                closure.as_ref(),
            );
            closure.forget(); // intentionally leak — one-shot
        }

        self.visible.set(true);
    }
}

/// Get the global confirm dialog handle
pub fn use_confirm() -> ConfirmDialog {
    use_context::<ConfirmDialog>().expect("ConfirmDialogProvider must be mounted")
}

/// Mount this at the app root to enable confirm dialogs everywhere
#[component]
pub fn ConfirmDialogProvider(children: Children) -> impl IntoView {
    let dialog = ConfirmDialog {
        visible: RwSignal::new(false),
        message: RwSignal::new(String::new()),
        style: RwSignal::new(ConfirmStyle::Info),
        confirm_label: RwSignal::new("OK".to_string()),
        cancel_label: RwSignal::new("Cancel".to_string()),
        confirmed: RwSignal::new(0u32),
        pending_id: RwSignal::new(0u32),
    };

    provide_context(dialog);

    view! {
        {children()}

        // Overlay
        <div
            class=move || if dialog.visible.get() {
                "fixed inset-0 z-[100] flex items-center justify-center bg-black/40 backdrop-blur-sm animate-fade-in"
            } else {
                "hidden"
            }
            on:click=move |e| {
                use wasm_bindgen::JsCast;
                if let Some(target) = e.target() {
                    if let Some(el) = target.dyn_ref::<web_sys::HtmlElement>() {
                        if el.class_list().contains("fixed") {
                            dialog.visible.set(false);
                        }
                    }
                }
            }
        >
            // Dialog card
            <div class="bg-white rounded-2xl shadow-2xl mx-6 w-full max-w-sm overflow-hidden animate-scale-in">
                // Body
                <div class="px-6 pt-6 pb-4 text-center">
                    // Icon
                    <div class=move || format!("w-12 h-12 mx-auto mb-4 rounded-full flex items-center justify-center {}", dialog.style.get().icon_bg())>
                        {move || {
                            let s = dialog.style.get();
                            let color = s.icon_color();
                            match s {
                                ConfirmStyle::Danger => view! {
                                    <svg class=format!("w-6 h-6 {}", color) fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
                                    </svg>
                                }.into_any(),
                                ConfirmStyle::Warning => view! {
                                    <svg class=format!("w-6 h-6 {}", color) fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4.5c-.77-.833-2.694-.833-3.464 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z"/>
                                    </svg>
                                }.into_any(),
                                ConfirmStyle::Info => view! {
                                    <svg class=format!("w-6 h-6 {}", color) fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                    </svg>
                                }.into_any(),
                            }
                        }}
                    </div>
                    // Message
                    <p class="text-sm text-gray-700 leading-relaxed">{move || dialog.message.get()}</p>
                </div>

                // Buttons
                <div class="flex border-t border-gray-100">
                    <button
                        class="flex-1 py-3.5 text-sm font-medium text-gray-500 active:bg-gray-50 border-r border-gray-100"
                        on:click=move |_| {
                            dialog.visible.set(false);
                        }
                    >
                        {move || dialog.cancel_label.get()}
                    </button>
                    <button
                        class=move || format!("flex-1 py-3.5 text-sm font-semibold rounded-br-2xl {}", dialog.style.get().confirm_btn())
                        on:click=move |_| {
                            dialog.visible.set(false);
                            // Call the stored JS callback
                            if let Some(window) = web_sys::window() {
                                use wasm_bindgen::JsCast;
                                if let Ok(cb) = js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("__confirm_cb")) {
                                    if let Some(func) = cb.dyn_ref::<js_sys::Function>() {
                                        let _ = func.call0(&wasm_bindgen::JsValue::NULL);
                                    }
                                    // Clean up
                                    let _ = js_sys::Reflect::delete_property(&window, &wasm_bindgen::JsValue::from_str("__confirm_cb"));
                                }
                            }
                        }
                    >
                        {move || dialog.confirm_label.get()}
                    </button>
                </div>
            </div>
        </div>
    }
}
