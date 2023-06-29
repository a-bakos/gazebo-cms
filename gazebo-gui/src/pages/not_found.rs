use yew::prelude::*;

use crate::components::admin_bar::AdminBar;
use crate::components::nav::Nav;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <main>
            <AdminBar />
            <Nav />
            <h1>{"404 Not found."}</h1>
        </main>
    }
}
