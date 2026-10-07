use leptos::prelude::*;

/// Documentation for [`WIFI`]
#[component]
pub fn Wifi() -> impl IntoView {
    

    view! {
        <div>
            <p class="wifi_title"> Configuraciones de red</p>
            <p class="txt_norm"> administra tus redes</p>
            <div>
                <div class="card">
                    <p class="Texto_titulos">Wifi</p>
                    <p class="txt_norm">redes disponibles</p>
                    <div class="switch-wrapper">
                        <input  type="checkbox" />
                    </div>
                </div>
                <div class="usercard">papaue</div>
            </div>
        </div>
    }
}