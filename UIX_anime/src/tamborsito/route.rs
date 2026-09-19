use leptos::prelude::*;
use leptos_router::components::Router;

mod app;
mod other;

use app::*;
use other::*;


/// Documentation for [`Router`]
#[component]
pub fn Router() -> impl IntoView {

    view! {
        <Router>
            <nav>
                /**/
                <App/>
                <Other/>
            </nav>
            <main>
                /**/
            </main>
        </Router>
    }
}