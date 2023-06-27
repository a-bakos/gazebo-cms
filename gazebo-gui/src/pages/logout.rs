use yew::prelude::*;

use crate::components::nav::Nav;

#[function_component(Logout)]
pub fn logout() -> Html {
    html! {
        <main>
            // <Nav />
            // <AdminNav />
            <h1>{"LOGOUT"}</h1>
        </main>
    }
}
