use crate::app::MainNavigationRoute;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(AdminMenu)]
pub fn admin_menu() -> Html {
    html! {
        <nav class={"gb-admin-menu"}>
            <ul>
                <li>
                    <Link<MainNavigationRoute> classes={classes!("testclass")} to={MainNavigationRoute::Admin}>
                        {"admin"}
                    </Link<MainNavigationRoute>>
                </li>
                <li><a href="">{"Posts"}</a></li>
                <li><a href="">{"Media"}</a></li>
                <li><a href="">{"Settings"}</a></li>
                <li><a href="">{"Users"}</a></li>
            </ul>
        </nav>
    }
}
