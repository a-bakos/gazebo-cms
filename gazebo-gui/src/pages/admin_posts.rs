use yew::prelude::*;

use crate::components::admin_bar::AdminBar;
use crate::components::admin_menu::AdminMenu;
use crate::components::table_entries::EntriesTable;

#[function_component(AdminPosts)]
pub fn admin_posts() -> Html {
    html! {
        <main id={"gb-gui-admin-area"}>
            <AdminBar />

            <div class={"gb-admin-panel"}>
                <AdminMenu />

                <div class={"gb-admin-main"}>
                    <h1>{"Posts"}</h1>
                    <EntriesTable />
                </div>
            </div>

        </main>
    }
}
