// Login page

use crate::{app::MainNavigationRoute, components::login_form::LoginForm};
use yew::prelude::*;
use yew_router::prelude::Redirect;

#[function_component(Login)]
pub fn login() -> Html {
    let current_user_ctx = use_context::<crate::context::CurrentUserContext>()
        .expect("Current accounts context missing");
    match &current_user_ctx.user {
        Some(user) => html! {
            <Redirect<MainNavigationRoute> to={MainNavigationRoute::Home} />
        },
        None => html! {
            <main class={"w-screen h-screen bg-yellow-200"} id={"gb-gui-login-page"}>
                <h1 class={"font-black text-center text-3xl"}>{"Hello."}</h1>
                <LoginForm />
            </main>
        },
    }
}
