use yew::prelude::*;

use crate::components::admin_bar::AdminBar;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::login_form::LoginForm;
use crate::components::post_rollup::PostRollup;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
        <AdminBar />
        <Header />
        <main>
            <LoginForm />
            <PostRollup />
        </main>
        <Footer />
        </>
    }
}
