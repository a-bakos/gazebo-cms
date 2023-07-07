use yew::prelude::*;

#[function_component(WidgetLoggedInUsers)]
pub fn widget_logged_in_users() -> Html {
    html! {
        <aside class="gb-widget gb-widget__logged-in-users">
            <h3>{ "[WIDGET] Logged in users" }</h3>
            <ul>
                <li>{ "User 1" }</li>
                <li>{ "User 2" }</li>
                <li>{ "User 3" }</li>
                <li>{ "User 4" }</li>
            </ul>
        </aside>
    }
}
