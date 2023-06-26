use yew::prelude::*;

use crate::components::nav::Nav;

#[function_component(Admin)]
pub fn admin() -> Html {
    html! {
        <main>
            <Nav />
            <h1>{"Admin"}</h1>
        </main>
    }
}
