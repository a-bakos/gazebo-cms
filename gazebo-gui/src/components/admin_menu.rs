use crate::app::MainNavigationRoute;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(AdminMenu)]
pub fn admin_menu() -> Html {
    html! {
        <nav class={"bg-[#496551] w-full sticky text-white border-t border-[#688a72]"}>
            <ul class="flex justify-center">
                <li class={"border-l border-r border-[#688a72]"}>
                    <Link<MainNavigationRoute> to={MainNavigationRoute::Admin} classes="inline-block p-4 hover:bg-[#688a72] hover:text-white">
                        {"Dashboard"}
                    </Link<MainNavigationRoute>>
                </li>
                <li class={"border-r border-[#688a72]"}>
                    <Link<MainNavigationRoute> to={MainNavigationRoute::AdminPosts} classes="inline-block p-4 hover:bg-[#688a72] hover:text-white">
                        {"Posts"}
                    </Link<MainNavigationRoute>>
                </li>
                <li class={"border-r border-[#688a72]"}>
                    //<Link<MainNavigationRoute> to={MainNavigationRoute::AdminPosts}>
                    <a class="inline-block p-4 hover:bg-[#688a72] hover:text-white">
                        {"Pages"}
                    </a>
                    //</Link<MainNavigationRoute>>
                </li>
                <li class={"border-r border-[#688a72]"}>
                    <Link<MainNavigationRoute> to={MainNavigationRoute::AdminMedia} classes="inline-block p-4 hover:bg-[#688a72] hover:text-white">
                        {"Media"}
                    </Link<MainNavigationRoute>>
                </li>
                <li class={"border-r border-[#688a72]"}>
                    <Link<MainNavigationRoute> to={MainNavigationRoute::AdminAccounts} classes="inline-block p-4 hover:bg-[#688a72] hover:text-white">
                        {"Accounts"}
                    </Link<MainNavigationRoute>>
                </li>
                <li class={"border-r border-[#688a72]"}>
                    // <Link<MainNavigationRoute> to={MainNavigationRoute::AdminAccounts}>
                    <a class="inline-block p-4 hover:bg-[#688a72] hover:text-white">
                        {"Search"}
                    </a>
                    // </Link<MainNavigationRoute>>
                </li>
                <li class={"border-r border-[#688a72]"}>
                    // <Link<MainNavigationRoute> to={MainNavigationRoute::AdminAccounts}>
                    <a class="inline-block p-4 hover:bg-[#688a72] hover:text-white">
                    {"Logs"}
                    </a>
                    // </Link<MainNavigationRoute>>
                </li>
                <li class={"border-r border-[#688a72]"}>
                    <Link<MainNavigationRoute> to={MainNavigationRoute::AdminSettings} classes="inline-block p-4 hover:bg-[#688a72] hover:text-white">
                        {"Settings"}
                    </Link<MainNavigationRoute>>
                </li>
            </ul>
        </nav>
    }
}
