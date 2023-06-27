use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::gazebo_admin_area::GazeboAdminArea;
use crate::pages::home::Home;
use crate::pages::login::Login;

use crate::pages::logout::Logout;

use crate::components::admin_bar::AdminBar;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::login_form::LoginForm;
use crate::components::post_rollup::PostRollup;

#[derive(Clone, PartialEq, Routable)]
pub enum MainNavigationRoute {
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

pub fn main_nav_switch(route: MainNavigationRoute) -> Html {
    match route {
        MainNavigationRoute::Home => html! { <Home /> },
        MainNavigationRoute::Login => html! { <Login /> },
        MainNavigationRoute::Admin => html! { <GazeboAdminArea /> },
        MainNavigationRoute::NotFound => html! { <h1>{ "1/ 404 not found" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    log::info!("YEW APP STARTED");
    html! {
        <BrowserRouter>
            <Switch<MainNavigationRoute> render={main_nav_switch} />
        </BrowserRouter>
    }
}
