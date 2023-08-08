use crate::app::MainNavigationRoute;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(AdminMenu)]
pub fn admin_menu() -> Html {
    html! {
        <nav class={"bg-gray-200 w-full sticky"}>
            <ul class="flex justify-center">
                <li>
                    <Link<MainNavigationRoute> to={MainNavigationRoute::Admin} classes="inline-block p-4 hover:bg-red-500">
                        {"Dashboard"}
                    </Link<MainNavigationRoute>>
                </li>
                <li>
                    <Link<MainNavigationRoute> to={MainNavigationRoute::AdminPosts} classes="inline-block p-4 hover:bg-red-500">
                        {"Posts"}
                    </Link<MainNavigationRoute>>
                </li>
                <li>
                    //<Link<MainNavigationRoute> to={MainNavigationRoute::AdminPosts}>
                    <a class="inline-block p-4 hover:bg-red-500">
                        {"Pages"}
                    </a>
                    //</Link<MainNavigationRoute>>
                </li>
                <li>
                    <Link<MainNavigationRoute> to={MainNavigationRoute::AdminMedia} classes="inline-block p-4 hover:bg-red-500">
                        {"Media"}
                    </Link<MainNavigationRoute>>
                </li>
                <li>
                    <Link<MainNavigationRoute> to={MainNavigationRoute::AdminAccounts} classes="inline-block p-4 hover:bg-red-500">
                        {"Accounts"}
                    </Link<MainNavigationRoute>>
                </li>
                <li>
                    // <Link<MainNavigationRoute> to={MainNavigationRoute::AdminAccounts}>
                    <a class="inline-block p-4 hover:bg-red-500">
                        {"Search"}
                    </a>
                    // </Link<MainNavigationRoute>>
                </li>
                <li>
                    // <Link<MainNavigationRoute> to={MainNavigationRoute::AdminAccounts}>
                    <a class="inline-block p-4 hover:bg-red-500">
                    {"Logs"}
                    </a>
                    // </Link<MainNavigationRoute>>
                </li>
                <li>
                    <Link<MainNavigationRoute> to={MainNavigationRoute::AdminSettings} classes="inline-block p-4 hover:bg-red-500">
                        {"Settings"}
                    </Link<MainNavigationRoute>>
                </li>
            </ul>
        </nav>
    }
}
