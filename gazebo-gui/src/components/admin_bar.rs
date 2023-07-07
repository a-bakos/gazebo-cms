use crate::app::MainNavigationRoute;
use yew::prelude::*;
use yew_router::prelude::Link;

// todo only show admin bar if logged in
#[function_component(AdminBar)]
pub fn admin_bar() -> Html {
    html! {
        <>
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
                        <Link<MainNavigationRoute> to={MainNavigationRoute::EditEntry}>
                            {"Create new entry"}
                        </Link<MainNavigationRoute>>
                    </li>
                    <li><a title="Logout" href="/logout">{"Logout (will be btn)"}</a></li>
                </ul>
            </nav>
        </>
    }
}
