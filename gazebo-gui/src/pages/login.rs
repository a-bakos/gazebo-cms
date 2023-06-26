use yew::prelude::*;

use crate::components::nav::Nav;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <main>
            <Nav />
            <h1>{"Login"}</h1>
        </main>
    }
}
