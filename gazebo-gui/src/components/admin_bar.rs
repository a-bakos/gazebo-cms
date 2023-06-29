use yew::prelude::*;

// todo only show admin bar if logged in
#[function_component(AdminBar)]
pub fn admin_bar() -> Html {
    html! {
        <>
            <nav class="gb-admin-bar">
                <ul>
                    // If admin, show front end link
                    <li><a title="Front End" href="/">{"Front end"}</a></li>
                    // If frontend, show admin link todo
                    <li><a title="Admin area" href="/admin">{"Admin"}</a></li>
                    <li><a title="Add new entry" href="/entry-edit">{"Create new entry"}</a></li>
                    <li><a title="System info" href="/system">{"System"}</a></li>
                    <li><a title="Logout" href="/logout">{"Logout"}</a></li>
                </ul>
            </nav>
        </>
    }
}
