//use app::*;
use other::*;
use leptos::prelude::*;
use wifi::*;

mod wifi;
mod app;
mod other;
//mod tamborsito;


fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            //<App/>
            <Other/>
            <Wifi/>
            //<tamborsito::route::AppRouter/>
        }
    })
}
