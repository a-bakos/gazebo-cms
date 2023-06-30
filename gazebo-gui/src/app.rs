use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::{
        admin_bar::AdminBar, footer::Footer, header::Header, login_form::LoginForm,
        post_rollup::PostRollup,
    },
    pages::{
        admin::{admin_dashboard::AdminDashboard, admin_posts::AdminPosts},
        edit_entry::EntryEdit,
        home::Home,
        login::Login,
        logout::Logout,
        not_found::NotFound,
    },
};

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
