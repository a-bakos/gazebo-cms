use yew::{html::IntoPropValue, prelude::*};
use yew_router::prelude::*;

use crate::pages::{
    admin::{
        admin_accounts::AdminAccounts, admin_dashboard::AdminDashboard, admin_media::AdminMedia,
        admin_posts::AdminPosts, admin_profile::AdminProfile, admin_settings::AdminSettings,
        edit_entry::EntryEditExisting, edit_entry_new::EntryEdit,
    },
    entry::EntryView,
    home::Home,
    login::Login,
    not_found::NotFound,
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
    #[at("/admin-media")]
    AdminMedia,
    #[at("/admin-settings")]
    AdminSettings,
    #[at("/admin-accounts")]
    AdminAccounts,
    #[at("/admin-profile")]
    AdminProfile,
    #[at("/entry-edit")]
    EntryEdit,
    #[at("/entry-edit/:entry_type/:id")]
    EntryEditExisting { entry_type: String, id: String },
    // todo this has to work with permalink too
    #[at("/entry/post/:id")]
    EntryView { id: String },
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

        MainNavigationRoute::EntryView { id } => {
            html! { <EntryView entry_id={id} /> }
        }

        // Admin dashboard
        MainNavigationRoute::Admin => html! { <AdminDashboard /> },
        // Admin all posts
        MainNavigationRoute::AdminPosts => html! { <AdminPosts /> },
        // Admin entry editor
        MainNavigationRoute::EntryEdit => html! { <EntryEdit /> },
        MainNavigationRoute::EntryEditExisting { entry_type, id } => {
            html! { <EntryEditExisting entry_type={entry_type} entry_id={id} /> }
        }
        // Admin media lib
        MainNavigationRoute::AdminMedia => html! { <AdminMedia /> },
        MainNavigationRoute::AdminSettings => html! { <AdminSettings /> },
        MainNavigationRoute::AdminAccounts => html! { <AdminAccounts /> },
        MainNavigationRoute::AdminProfile => html! { <AdminProfile /> },

        // 404
        MainNavigationRoute::NotFound => html! { <NotFound /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    log::info!("Gazebo GUI init");
    html! {
        <BrowserRouter>
            <crate::context::CurrentUserProvider>
                <Switch<MainNavigationRoute> render={main_nav_switch} />
            </crate::context::CurrentUserProvider>
        </BrowserRouter>
    }
}
