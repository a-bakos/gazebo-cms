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
        <nav>
            <Link<MainNavigationRoute> classes={classes!("testclass")} to={MainNavigationRoute::Home}>
                <h1>{"standalone menu item"}</h1>
            </Link<MainNavigationRoute>>
            <ul>
                {
                    nav_items.iter().map(|nav_item| {
                        html!{
                            <li
                                key={nav_item.id}
                                class={classes!("nav_item", if nav_item.is_active {"active"} else {""} )}>
                                <Link<MainNavigationRoute> to={nav_item.link.clone()}>{nav_item.label.clone()}</Link<MainNavigationRoute>>
                            </li>
                        }
                    }).collect::<Html>()
                }
            </ul>
        </nav>
    }
}
