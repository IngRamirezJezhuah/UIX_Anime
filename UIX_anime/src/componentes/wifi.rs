use leptos::prelude::*;

/// Documentation for [`WIFI`]
#[component]
pub fn Wifi() -> impl IntoView {
    

    view! {
        <div>
            <p class="wifi_title"> Configuraciones de red</p>
            <p class="txt_norm"> administra tus redes</p>
            <div class="contendor">
                <div class="usercard">
                    <p class="Texto_titulos">Wifi</p>
                    <p>redes disponibles</p>
                </div>
                <div class="usercard">papaue</div>
            </div>
        </div>
    }
}