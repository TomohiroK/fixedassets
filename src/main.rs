mod app;
mod auth;
mod i18n;
mod models;
mod stores;
mod components;
mod pages;
mod router;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Debug);

    leptos::mount::mount_to_body(App);
}
