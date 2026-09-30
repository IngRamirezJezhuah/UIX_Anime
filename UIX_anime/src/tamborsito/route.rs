use leptos::prelude::*;
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::path;

use crate::app::App;
use crate::componentes::esquema_img::Esq_Img;
use crate::componentes::wifi::Wifi;
use crate::other::Other;

/// Documentation for [`AppRouter`]
#[component]
pub fn AppRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Page not found.">
                <ParentRoute path=path!("/") view=Other>
                    <Route path=path!("") view=App/>
                    <Route path=path!("wifi") view=Wifi/>
                    <Route path=path!("esquema") view=Esq_Img/>
                </ParentRoute>
            </Routes>
        </Router>
        
    }
}

/*
<Router>
            <Routes fallback=|| "Pagina no encontrada.">
                <ParentRoute path=path!("/") view=Other />
                    <Route path=path!("") view=App />
                    <Route path=path!("wifi") view=Wifi />
                    <Route path=path!("esquema") view=Esq_Img />
                </ParentRoute>
            </Routes>
        </Router> */