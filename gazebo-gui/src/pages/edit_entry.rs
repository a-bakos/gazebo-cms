// This will be the post editor screen

use yew::prelude::*;

use crate::components::admin_bar::AdminBar;
use crate::components::admin_menu::AdminMenu;
use crate::components::entry_editor::EntryEditor;

#[function_component(EntryEdit)]
pub fn entry_edit() -> Html {
    html! {
        <main id={"gb-gui-admin-area"}>
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
