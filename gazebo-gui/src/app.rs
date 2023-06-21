use yew::prelude::*;

use crate::components::admin_bar::AdminBar;
use crate::components::header::Header;
use crate::components::login_form::LoginForm;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
        <AdminBar />
        <Header />
        <main>
            <LoginForm />
        </main>
        </>
    }
}
