// Admin post/entry editor screen // move to admin/

use yew::prelude::*;

use crate::components::admin_bar::AdminBar;
use crate::components::admin_menu::AdminMenu;
use crate::components::entry_editor::EntryEditor;

#[function_component(EntryEdit)]
pub fn entry_edit() -> Html {
    html! {
         <main id={crate::consts::CSS_ID_ADMIN_AREA}>
            <AdminBar />

            <div class={"gb-admin-panel"}>
                <AdminMenu />

                <div class={"gb-admin-main"}>
                    <EntryEditor />
                </div>
            </div>

        </main>
    }
}
