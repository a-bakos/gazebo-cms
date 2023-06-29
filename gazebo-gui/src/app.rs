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
use crate::pages::not_found::NotFound;

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
        // Home page
        MainNavigationRoute::Home => html! { <Home /> },
        // Login page
        MainNavigationRoute::Login => html! { <Login /> },
        // Lost password page
        MainNavigationRoute::LostPassword => html! { <h1>{ "Soon." }</h1> },

        // Admin dashboard
        MainNavigationRoute::Admin => html! { <AdminDashboard /> },
        // Admin all posts
        MainNavigationRoute::AdminPosts => html! { <AdminPosts /> },
        // Admin entry editor
        MainNavigationRoute::EntryEdit => html! { <EntryEdit />},

        // 404
        MainNavigationRoute::NotFound => html! { <NotFound /> },
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
