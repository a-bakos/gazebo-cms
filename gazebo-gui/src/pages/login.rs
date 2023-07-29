// Login page

use crate::components::{login_form::LoginForm, nav::Nav};
use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <main id={"gb-gui-login-page"}>
            <h1>{"Hello."}</h1>
            <LoginForm />
        </main>
    }
}
