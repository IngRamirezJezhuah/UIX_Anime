use leptos::prelude::*;
use leptos_router::components::Router;

use crate::app::App;
use crate::other::Other;


/// Documentation for [`Router`]
#[component]
pub fn AppRouter() -> impl IntoView {

    view! {
        <Router>
            <Routes fallback=|| "Pagina no encontrada.">
                <Route path="/" view=App/>
                <Route path="/oher" view=Other/>
            </Routes>
        </Router>
    }
}