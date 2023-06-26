use yew::prelude::*;

use crate::components::nav::Nav;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <main>
            <Nav />
            <h1>{"Home"}</h1>
        </main>
    }
}
