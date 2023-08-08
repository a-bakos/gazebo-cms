// Admin all posts/entries page

use yew::prelude::*;

use crate::components::{
    admin_bar::AdminBar, admin_menu::AdminMenu, button_add_new_entry::ButtonAddNewEntry,
    entries_table_posts::EntriesTable,
};

#[function_component(AdminPosts)]
pub fn admin_posts() -> Html {
    html! {
        <main id={crate::consts::CSS_ID_ADMIN_AREA}>
            <AdminBar />

            <div class={"flex bg-red-100"}>
                <div class={"w-full"}>
                    <h1>{"Posts"}</h1>
                    <ButtonAddNewEntry />
                    <EntriesTable />
                </div>
            </div>

        </main>
    }
}
