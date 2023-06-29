use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::admin_bar::AdminBar;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::login_form::LoginForm;
use crate::components::post_rollup::PostRollup;

// pages::admin
use crate::pages::admin::admin_dashboard::AdminDashboard;
use crate::pages::admin::admin_posts::AdminPosts;

use crate::pages::edit_entry::EntryEdit;
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::logout::Logout;

#[derive(Clone, PartialEq, Routable)]
pub enum MainNavigationRoute {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/admin")]
    Admin,
    #[at("/admin-posts")]
    AdminPosts,
    #[at("/entry-edit")]
    EntryEdit,
    #[at("/lost-password")]
    LostPassword,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn main_nav_switch(route: MainNavigationRoute) -> Html {
    match route {
        MainNavigationRoute::Home => html! { <Home /> },
        MainNavigationRoute::Login => html! { <Login /> },

        MainNavigationRoute::Admin => html! { <AdminDashboard /> },
        MainNavigationRoute::AdminPosts => html! { <AdminPosts /> },

        MainNavigationRoute::EntryEdit => html! { <EntryEdit />},

        MainNavigationRoute::LostPassword => html! { <h1>{ "Soon." }</h1> },
        MainNavigationRoute::NotFound => html! { <h1>{ "1/ 404 not found" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    log::info!("Gazebo GUI init");
    html! {
        <BrowserRouter>
            <Switch<MainNavigationRoute> render={main_nav_switch} />
        </BrowserRouter>
    }
}
