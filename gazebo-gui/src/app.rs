use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::admin::Admin;
use crate::pages::home::Home;
use crate::pages::login::Login;

use crate::components::admin_bar::AdminBar;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::login_form::LoginForm;
use crate::components::post_rollup::PostRollup;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/admin")]
    Admin,
    #[not_found]
    #[at("/404")]
    NotFound,
}

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
