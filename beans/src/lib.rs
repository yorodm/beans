use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod views;

use components::ribbon::Ribbon;
use views::{overview::Overview, add_entry::AddEntry, edit_entry::EditEntry, export::Export};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/beans.css"/>
        <Title text="Beans - Ledger Manager"/>

        <Router>
            <main class="app-container">
                <Ribbon/>
                <div class="content">
                    <Routes>
                        <Route path="/" view=Overview/>
                        <Route path="/overview" view=Overview/>
                        <Route path="/add-entry" view=AddEntry/>
                        <Route path="/edit-entry" view=EditEntry/>
                        <Route path="/export" view=Export/>
                    </Routes>
                </div>
            </main>
        </Router>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}

