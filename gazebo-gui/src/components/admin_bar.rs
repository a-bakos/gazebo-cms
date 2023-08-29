use crate::{app::MainNavigationRoute, components::admin_menu::AdminMenu};
use gazebo_core_common::consts::DEFAULT_APP_NAME;
use yew::prelude::*;
use yew_router::prelude::{Link, Redirect};

#[function_component(AdminBar)]
pub fn admin_bar() -> Html {
    let current_user_ctx = use_context::<crate::context::CurrentUserContext>()
        .expect("Current accounts context missing");
    match &current_user_ctx.user {
        Some(user) => html! {
            <>
                <div class="sticky top-0">
                    <nav class="bg-[#1e1612] text-white w-full">
                        <ul class="flex items-center">
                            // If admin, show front end link
                            <li class="bg-black text-white">
                                <Link<MainNavigationRoute> to={MainNavigationRoute::Home} classes="h-full inline-block p-4">
                                    {"Front"}
                                </Link<MainNavigationRoute>></li>
                            // If frontend, show admin link todo
                            <li class="bg-black text-white">
                                <Link<MainNavigationRoute> to={MainNavigationRoute::Admin}>
                                    {"Admin"}
                                </Link<MainNavigationRoute>>
                            </li>
                            <li class="text-center text-white">
                                <Link<MainNavigationRoute>
                                    to={MainNavigationRoute::AdminProfile}>
                                    <p>{"[USER]"}</p>
                                    <p>{user.username.clone()}</p>
                                    <p>{user.role.clone()}</p>
                                </Link<MainNavigationRoute>>
                            </li>
                            <li class="flex-1 text-center">
                                <p class="text-xl font-bold">{DEFAULT_APP_NAME}</p>
                                <p>{"www.siteaddress.com"}</p>
                            </li>
                            // <li>
                            //     <Link<MainNavigationRoute> to={MainNavigationRoute::EntryEdit}>
                            //         {"Create new entry"}
                            //     </Link<MainNavigationRoute>>
                            // </li>
                            <li class="bg-gray-500 text-white">
                               {"?Help"}
                            </li>
                            <li class="bg-black text-white"><a class="h-full inline-block p-4" title="Logout" href="/logout">{"?Logout"}</a></li>
                        </ul>
                    </nav>
                    <AdminMenu />
                </div>
            </>
        },
        None => html! {
            // Not logged in
            <Redirect<MainNavigationRoute> to={MainNavigationRoute::Home} />
        },
    }
}
