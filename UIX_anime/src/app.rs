use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}


#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let navigate = use_navigate();

    view! {
        <main class="container">
            <h1>"Welcome to Tauri + Leptos"</h1>
            <p>"Click on the Tauri and Leptos logos to learn more."</p>
            //<button>navigate("/wifi", Default::default())</button>
            <button on:click=move |_| navigate("/wifi", Default::default())>"Ir a otra pantalla"</button>
            
        </main>
    }
}
