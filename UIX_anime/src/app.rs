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
            <h1 class="Texto_titulos">"User Account"</h1>
            <p class="txt_norm">"Modifica tu perfil y preferencias"</p>
            //<button>navigate("/wifi", Default::default())</button>
            //<button on:click=move |_| navigate("/wifi", Default::default())>"Ir a otra pantalla"</button>
            <div class="container">
                <div class="usercard">
                    <img class="prf" src="/public/miku_evil.jpg" alt="" />
                    
                    <p class="user_txt_card">Djxs4n</p>
                    <p class="txt_card">SuperUser</p>
                    <button>change Avatar</button>
                </div>
            </div>
        </main>
    }
}
