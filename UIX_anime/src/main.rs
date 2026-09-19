mod app;
mod other;
use app::*;
use other::*;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
            <Other/>
        }
    })
}
