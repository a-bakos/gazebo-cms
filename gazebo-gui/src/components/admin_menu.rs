use crate::app::MainNavigationRoute;
use yew::prelude::*;
use yew_router::hooks::use_route;
use yew_router::prelude::Link;

fn get_classes_for_route(
    current_route: MainNavigationRoute,
    target_route: MainNavigationRoute,
) -> Classes {
    if current_route == target_route {
        classes!(
            "inline-block",
            "p-4",
            "bg-red-400",
            "hover:bg-red-400",
            "text-white"
        )
    } else {
        classes!("inline-block", "p-4", "hover:bg-red-200")
    }
}

#[function_component(AdminMenu)]
pub fn admin_menu() -> Html {
    let current_route = use_route::<MainNavigationRoute>().expect("No current route defined"); // expect is safe here

    let classes_dashboard =
        get_classes_for_route(current_route.clone(), MainNavigationRoute::Admin);
    let classes_posts =
        get_classes_for_route(current_route.clone(), MainNavigationRoute::AdminPosts);
    //let classes_pages = get_classes_for_route(current_route.clone(), MainNavigationRoute::AdminPages);
    let classes_media =
        get_classes_for_route(current_route.clone(), MainNavigationRoute::AdminMedia);
    let classes_accounts =
        get_classes_for_route(current_route.clone(), MainNavigationRoute::AdminAccounts);
    let classes_settings =
        get_classes_for_route(current_route.clone(), MainNavigationRoute::AdminSettings);

    html! {
        <nav class={"bg-[#496551] select-none w-full sticky text-white border-t border-[#688a72]"}>
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
                    <Link<MainNavigationRoute>
                        to={MainNavigationRoute::AdminMedia}
                        classes="inline-block p-4 hover:bg-[#688a72] hover:text-white">
                        {"Media"}
                    </Link<MainNavigationRoute>>
                </li>
                <li class={"border-r border-[#688a72]"}>
                    <Link<MainNavigationRoute>
                        to={MainNavigationRoute::AdminAccounts}
                        classes="inline-block p-4 hover:bg-[#688a72] hover:text-white">
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
