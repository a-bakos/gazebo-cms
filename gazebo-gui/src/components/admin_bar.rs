use yew::prelude::*;

#[function_component(AdminBar)]
pub fn admin_bar() -> Html {
    html! {
        <>
            <nav class="gb-admin-bar">
                <ul>
                    <li><a title="" href="">{"Hello"}</a></li>
                    <li><a title="Admin area" href="">{"Admin"}</a></li>
                    <li><a title="Front End" href="">{"Front"}</a></li>
                    <li><a title="" href="">{"ID"}</a></li>
                    <li><a title="Add new entry" href="">{"Create new entry"}</a></li>
                    <li><a title="Logout" href="">{"Exit"}</a></li>
                </ul>
            </nav>
        </>
    }
}
