use leptos::prelude::*;


/// Documentation for [`other`]
#[component]
pub fn Other() -> impl IntoView {

    view! {
        <div>
            <p>views</p>
            <nav class="barra_seleccion">
                <button>Sistema</button>
                <button>Wifi y Bluethoot</button>
                <button>Apariencia</button>
                <button>Teclado/mouse</button>
                <button>Hora e idioma</button>
                <button>Seguridad</button>
            </nav>
        </div>
    }
}

