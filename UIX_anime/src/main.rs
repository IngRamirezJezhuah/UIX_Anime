//use app::*;
use other::*;
use leptos::prelude::*;
use tamborsito::*;

use crate::tamborsito::route::AppRouter;

mod app;
mod other;
mod tamborsito;
mod componentes;



fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <AppRouter/> })
}
