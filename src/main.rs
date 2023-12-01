use leptos::{mount_to_body, view};
mod components;
mod models;
mod pages;

pub fn main() {
    use cvsite::pages::home::*;
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <Home />
        }
    })
}
