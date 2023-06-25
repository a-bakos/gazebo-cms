use yew::prelude::*;

#[function_component(MainNavigation)]
pub fn main_navigation() -> Html {
    html! {
        <nav>
            <ul>
                <li><a title="home" href="/hello">{ "Home" }</a></li>
            </ul>
        </nav>
    }
}
