use leptos::prelude::*;

/// Documentation for [`WIFI`]
#[component]
pub fn Wifi() -> impl IntoView {
    

    view! {
        <div>
            <p class="Texto_titulos"> Configuraciones de red</p>
            <p class="txt_norm"> administra tus redes</p>
            <div class="cardcontainer">
                <div class="card">
                    <p class="Texto_titulos">Wifi</p>
                    <p class="txt_norm">redes disponibles</p>
                    <div class="switch-wrapper">
                        <input   type="checkbox" />
                        <ul>
                            <li class="list"> wifi</li>
                            <li class="list"> lista wifi</li>
                            <li class="list"> wifi</li>
                            <li class="list"> lista wifi</li>
                            <li class="list"> lista wifi</li>
                        </ul>
                    </div>
                </div>
                <div class="card">
                    <p class="Texto_titulos">vpn</p>
                    <p class="txt_norm">server</p>
                    <div>
                        <input class="input_style" type="checkbox"/>
                        <ul>
                        <li class="list"><svg viewbox="0 0 24 24"><circle cx="11" cy="11" r="7" /><path d="m20-3.5-3.5"/></svg> server</li>
                        <li class="list"><svg viewbox="0 0 24 24"><circle cx="11" cy="11" r="7" /><path d="m20-3.5-3.5"/></svg> server</li>
                        <li class="list"><svg viewbox="0 0 24 24"><circle cx="11" cy="11" r="7" /><path d="m20-3.5-3.5"/></svg> server</li>
                        <li class="list"><svg viewbox="0 0 24 24"><circle cx="11" cy="11" r="7" /><path d="m20-3.5-3.5"/></svg> server</li>
                        <li class="list"><svg viewbox="0 0 24 24"><circle cx="11" cy="11" r="7" /><path d="m20-3.5-3.5"/></svg> server</li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}