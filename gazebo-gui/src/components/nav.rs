use yew::prelude::*;
use yew_router::prelude::Link;

use crate::app::MainNavigationRoute;

pub struct NavItem {
    id: u32,
    label: String,
    is_active: bool,
    link: MainNavigationRoute,
}

#[function_component(Nav)]
pub fn nav() -> Html {
    let nav_items = use_state(|| {
        vec![
            NavItem {
                id: 0,
                label: "Home".to_string(),
                is_active: false,
                link: MainNavigationRoute::Home,
            },
            NavItem {
                id: 1,
                label: "Admin".to_string(),
                is_active: false,
                link: MainNavigationRoute::Admin,
            },
            NavItem {
                id: 2,
                label: "Login".to_string(),
                is_active: false,
                link: MainNavigationRoute::Login,
            },
        ]
    });
    html! {
        <nav class={"border-b"}>
            <ul class={"flex justify-center p-3"}>
                {
                    nav_items.iter().map(|nav_item| {
                        html!{
                            <li
                                key={nav_item.id}
                                class={""}>
                                <Link<MainNavigationRoute>
                                    classes={"inline-block px-4 py-1 hover:text-red-400 hover:underline"}
                                    to={nav_item.link.clone()}>
                                    {nav_item.label.clone()}
                                </Link<MainNavigationRoute>>
                            </li>
                        }
                    }).collect::<Html>()
                }
            </ul>
        </nav>
    }
}
