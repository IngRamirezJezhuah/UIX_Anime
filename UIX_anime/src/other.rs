//other es la barra de busqueda
use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_navigate;
use serde::de::value;

#[component]
pub fn Other() -> impl IntoView {
    let navigate = use_navigate();

    view! {
        <div class="contenedor">
            <nav class="barra_seleccion">
                <button class="btn_nav" on:click={let value = navigate.clone(); 
                move |_| value("/", Default::default())} 
                >" 󱄫 Perfil"</button>
                <button class="btn_nav" on:click={let value = navigate.clone();
                move |_| value("/wifi", Default::default())}>
                    "󱚾 Red"
                </button>
                <button class="btn_nav" on:click={let value = navigate.clone();
                move |_| value("/esquema_img", Default::default())}>
                    "󰸌 Apariencia"
                </button>
                <button class="btn_nav" on:click={let value = navigate.clone();
                move |_| value("/perifericos", Default::default())}>
                    "Preifericos"
                </button>
                <button class="btn_nav">
                    "Hora e idioma"
                </button>
                <button class="btn_nav">
                    "󰒃 Seguridad"
                </button>
                
            </nav>
            <Outlet/>
        </div>
    }
}

