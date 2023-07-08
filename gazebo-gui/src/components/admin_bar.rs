use crate::app::MainNavigationRoute;
use std::fs::read_dir;
use yew::prelude::*;
use yew_router::prelude::{Link, Redirect};

// todo only show admin bar if logged in
#[function_component(AdminBar)]
pub fn admin_bar() -> Html {
    let current_user_ctx =
        use_context::<crate::context::CurrentUserContext>().expect("Current user context missing");
    match &current_user_ctx.user {
        Some(user) => html! {
            <>
                <p>{"Hello, "}{user.username.clone()}</p>
                <nav class="gb-admin-bar">
                    <ul>
                        // If admin, show front end link
                        <li><a title="Front End" href="/">{"Front end"}</a></li>
                        // If frontend, show admin link todo
                        <li>
                            <Link<MainNavigationRoute> to={MainNavigationRoute::Admin}>
                                {"Admin"}
                            </Link<MainNavigationRoute>>
                        </li>
                        <li>
                            <Link<MainNavigationRoute> to={MainNavigationRoute::EntryEdit}>
                                {"Create new entry"}
                            </Link<MainNavigationRoute>>
                        </li>
                        <li><a title="Logout" href="/logout">{"Logout (will be btn)"}</a></li>
                    </ul>
                </nav>
            </>
        },
        None => html! {
            <Redirect<MainNavigationRoute> to={MainNavigationRoute::Login} />
        },
    }
}
