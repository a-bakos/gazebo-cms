use yew::prelude::*;
use yew_router::prelude::Link;

use crate::app::MainNavigationRoute;

#[function_component(LostPassword)]
pub fn lost_password() -> Html {
    html! {
        <div class={"gb-login-lost-password"}>
            <Link<MainNavigationRoute> to={MainNavigationRoute::LostPassword}>
                {"Lost Password"}
            </Link<MainNavigationRoute>>
        </div>
    }
}
