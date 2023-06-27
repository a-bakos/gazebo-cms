use yew::prelude::*;

use crate::components::login_form::LoginForm;
use crate::components::nav::Nav;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <main id={"gb-gui-login-page"}>
            <Nav />
            <h1>{"Login"}</h1>
            <LoginForm />
        </main>
    }
}
