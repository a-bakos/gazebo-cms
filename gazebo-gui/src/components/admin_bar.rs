use crate::app::MainNavigationRoute;
use yew::prelude::*;
use yew_router::prelude::{Link, Redirect};

#[function_component(AdminBar)]
pub fn admin_bar() -> Html {
    let current_user_ctx =
        use_context::<crate::context::CurrentUserContext>().expect("Current user context missing");
    match &current_user_ctx.user {
        Some(user) => html! {
            <>
                <nav class="gb-admin-bar">
                    <ul>
                        <li>{"Hello, "}{user.username.clone()}</li>
                        // If admin, show front end link
                        <li>
                            <Link<MainNavigationRoute> to={MainNavigationRoute::Home}>
                                {"Front end"}
                            </Link<MainNavigationRoute>></li>
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
            //<p>{"NOT LOGGED IN"}</p>
            <Redirect<MainNavigationRoute> to={MainNavigationRoute::Home} />
        },
    }
}
