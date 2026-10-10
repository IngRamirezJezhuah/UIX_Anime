use leptos::prelude::*;

/// Documentation for [`Perifericos`]
#[component]
pub fn Perifericos() -> impl IntoView {
    

    view! {
        <div>
            <h1 class="Texto_titulos">Perifericos</h1>
            <div>
            // boton de buscar
                <button class="search-btn" popovertarget="palette">
                    <svg></svg>
                    buscar
                </button>

            // buscador
            <section class="palette" id="palette" popover>
                <search class="palette_search">
                    <svg viewbox="0 0 24 24"><circle cx="11" cy="11" r="7" /><path d="m20-3.5-3.5"/></svg>
                    <input type="search" autofocus placeholder="Escribe un comando o busca..."/>
                    <kbd>esc</kbd>
                </search>

                <section>
                    <h2>sugerencias</h2>
                    <ul classs="palette__list">

                    // elemento 1 
                    <li>
                        <button popovertarget="palette" popovertargetaction="hide">
                        <svg viewBox="0 0 24 24"><path d="M12 5v12M5 12h14"/></svg>
                        crear nuevo short
                        <kbd>"⌘N"</kbd>
                        </button>
                    </li>
                    // elemento 2
                    <li>
                        <button popovertarget="palette" popovertargetaction="hide">
                        <svg viewBox="0 0 24 24"><rect x="3" y="3" width="7" height="7" rx="1"/><rect x="14" y="3" width="7" height="7" rx="1"/><rect x="3" y="14" width="7" height="7" rx="1"/><rect x="14" y="14" width="7" height="7" rx="1"/></svg>
                        Abrir biblioteca de componentes
                        </button>
                    </li>
                    // elemento 3
                    <li>
                        <button popovertarget="palette" popovertargetaction="hide">
                        <svg viewBox="0 0 24 24"><circle cx="12" cy="12" r="9"/><path d="M12 3a9 9 0 0 1 0 18z" fill="currentColor"/></svg>
                        Cambiar tema
                        <kbd>"⌘T"</kbd>
                        </button>
                    </li>
                    
                    </ul>
                </section>
            </section>
            </div>
        </div>
    }
}