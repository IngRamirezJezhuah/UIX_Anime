use leptos::prelude::*;

/// Documentation for [`Esq_Img`]
#[component]
pub fn Esq_Img() -> impl IntoView {
    

    view! {
        <div>
        <h1 class="wifi_title">Esquema Img</h1>
            <div /*class="color_Schema"*/>
                <img src="" alt=""/>
                <div>
                    <div class="contenedor">
                        <div>
                            <div class="grid_grnd_1">
                                <img class="img_grnd_1" src="/public/miku_evil.jpg" alt="Hatsune_miku_evil"/>
                            </div>
                            <div class="contenedor"> 
                                <div class="grid_grnd_2">
                                    <img class="img_grnd_2" src="/public/waifucabra.jpg" alt="Hatsune_miku_evil"/>
                                </div>
                            </div>
                        </div>
                            /*<div class="grid_pqñ">cara 2</div>
                            <div class="grid_pqñ">cara 3</div>*/
                    </div>
                    <h1 class="wifi_txt">texto random</h1>
                </div>
            </div>
        </div>
    }
}