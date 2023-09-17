use crate::app::MainNavigationRoute;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(ButtonAddNewEntry)]
pub fn button_add_new_entry() -> Html {
    html! {
        <Link<MainNavigationRoute> to={MainNavigationRoute::EntryEdit} classes="bg-blue-200 px-4 py-2 mr-px hover:bg-blue-500">
            {"Add New Entry"}
        </Link<MainNavigationRoute>>
    }
}
