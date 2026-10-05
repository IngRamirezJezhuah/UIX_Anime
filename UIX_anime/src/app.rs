use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use wasm_bindgen::prelude::*;



#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}



#[component]
pub fn App() -> impl IntoView {
    let navigate = use_navigate();

    view! {
        <main class="container">
            <h1 class="Texto_titulos">"Bienvenidos a las configuraciones Hypr"</h1>
            <p>"Primeras configuraciones"</p>
            //<button>navigate("/wifi", Default::default())</button>
            <button on:click=move |_| navigate("/wifi", Default::default())>"Ir a otra pantalla"</button>
            
        </main>
    }
}
