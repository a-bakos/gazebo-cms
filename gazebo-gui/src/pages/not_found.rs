// 404 Not found page

use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <main>
            <crate::components::admin_bar::AdminBar />
            <crate::components::nav::Nav />
            <h1>{"404 Not found."}</h1>
        </main>
    }
}
