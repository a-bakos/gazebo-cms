use crate::app::MainNavigationRoute;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(ButtonAddNewEntry)]
pub fn button_add_new_entry() -> Html {
    html! {
        <Link<MainNavigationRoute> to={MainNavigationRoute::EntryEdit}>
            {"Add New Entry"}
        </Link<MainNavigationRoute>>
    }
}
