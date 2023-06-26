use yew::prelude::*;
use yew_router::prelude::Link;

use crate::app::Route;

pub struct NavItem {
    id: u32,
    label: String,
    is_active: bool,
    link: Route,
}

#[function_component(Nav)]
pub fn nav() -> Html {
    let nav_items = use_state(|| {
        vec![
            NavItem {
                id: 0,
                label: "Home".to_string(),
                is_active: false,
                link: Route::Home,
            },
            NavItem {
                id: 1,
                label: "Admin".to_string(),
                is_active: false,
                link: Route::Admin,
            },
            NavItem {
                id: 2,
                label: "Login".to_string(),
                is_active: false,
                link: Route::Login,
            },
        ]
    });
    html! {
        <nav>
            <Link<Route> classes={classes!("testclass")} to={Route::Home}>
                <h1>{"home"}</h1>
            </Link<Route>>
            <ul>
                {
                    nav_items.iter().map(|nav_item| {
                        html!{
                            <li
                                key={nav_item.id}
                                class={classes!("nav_item", if nav_item.is_active {"active"} else {""} )}>
                                <Link<Route> to={nav_item.link.clone()}>{nav_item.label.clone()}</Link<Route>>
                            </li>
                        }
                    }).collect::<Html>()
                }
            </ul>
        </nav>
    }
}
