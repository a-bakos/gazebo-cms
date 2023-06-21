use yew::prelude::*;

#[function_component(AdminBar)]
pub fn admin_bar() -> Html {
    html! {
        <>
            <nav class="gb-admin-bar">
                <ul>
                    <li><a href="">{"Hello"}</a></li>
                    <li><a href="">{"Admin"}</a></li>
                    <li><a href="">{"Front"}</a></li>
                    <li><a href="">{"ID"}</a></li>
                    <li><a href="">{"Add"}</a></li>
                    <li><a href="">{"Exit"}</a></li>
                </ul>
            </nav>
        </>
    }
}
