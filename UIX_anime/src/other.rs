use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_navigate;


// Documentation for [`other`]
#[component]
pub fn Other() -> impl IntoView {
    let navigate = use_navigate();

    view! {
        <div>
            //<p>views</p>
            <nav class="barra_seleccion">
                <button on:click={let value = navigate.clone(); move |_| value("/", Default::default())}>"Sistema"</button>
                <button on:click={let value = navigate.clone(); move |_| value("wifi", Default::default())}>"Wifi y Bluethoot"</button>
                <button on:click={let value = navigate.clone(); move |_| value("esquema", Default::default())}>"Apariencia"</button>
                <button>Teclado/mouse</button>
                <button>Hora e idioma</button>
                <button>Seguridad</button>
            </nav>
            <Outlet/>
        </div>
    }
}

