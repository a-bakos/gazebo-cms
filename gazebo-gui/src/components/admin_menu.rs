use crate::app::MainNavigationRoute;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(AdminMenu)]
pub fn admin_menu() -> Html {
    html! {
        <nav class={"gb-admin-menu"}>
            <ul>
                <li>
                    <Link<MainNavigationRoute> to={MainNavigationRoute::Admin}>
                        {"Dashboard"}
                    </Link<MainNavigationRoute>>
                </li>
                <li>
                    <Link<MainNavigationRoute> to={MainNavigationRoute::AdminPosts}>
                        {"Posts"}
                    </Link<MainNavigationRoute>>
                </li>
                <li>
                    <Link<MainNavigationRoute> to={MainNavigationRoute::AdminMedia}>
                        {"Media"}
                    </Link<MainNavigationRoute>>
                </li>
                <li>
                    <Link<MainNavigationRoute> to={MainNavigationRoute::AdminSettings}>
                        {"Settings"}
                    </Link<MainNavigationRoute>>
                </li>
                <li>
                    <Link<MainNavigationRoute> to={MainNavigationRoute::AdminUsers}>
                        {"Users"}
                    </Link<MainNavigationRoute>>
                </li>
            </ul>
        </nav>
    }
}
