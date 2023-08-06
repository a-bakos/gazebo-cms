// Login page

use crate::components::{login_form::LoginForm, nav::Nav};
use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <main class={"w-screen h-screen bg-yellow-200"} id={"gb-gui-login-page"}>
            <h1 class={"font-black text-center text-3xl"}>{"Hello."}</h1>
            <LoginForm />
        </main>
    }
}
