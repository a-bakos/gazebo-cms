use yew::prelude::*;

use crate::components::header::Header;
use crate::components::login_form::LoginForm;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
        <Header />
        <main>
            <LoginForm />
        </main>
        </>
    }
}
