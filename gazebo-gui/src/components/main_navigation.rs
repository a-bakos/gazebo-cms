use yew::prelude::*;

#[function_component(MainNavigation)]
pub fn main_navigation() -> Html {
    html! {
        <nav>
            <ul>
                <li><a title="home" href="/hello">{ "Home" }</a></li>
                <li><a title="login" href="/login">{ "Login" }</a></li>
                <li><a title="about" href="/adduser">{ "Add user" }</a></li>
            </ul>
        </nav>
    }
}
