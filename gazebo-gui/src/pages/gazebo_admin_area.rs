// This will be the admin (management) area

use yew::prelude::*;

use crate::components::admin_bar::AdminBar;
use crate::components::admin_menu::AdminMenu;
use crate::components::nav::Nav;
use crate::components::table_entries::EntriesTable;

use crate::app::MainNavigationRoute;
use yew_router::prelude::Link;

#[function_component(GazeboAdminArea)]
pub fn gazebo_admin_area() -> Html {
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
