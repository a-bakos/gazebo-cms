// Admin Users screen

use yew::prelude::*;

use crate::components::{admin_bar::AdminBar, admin_menu::AdminMenu};

#[function_component(AdminUsers)]
pub fn admin_users() -> Html {
    html! {
         <main id={crate::consts::CSS_ID_ADMIN_AREA}>
            <AdminBar />

            <div class={"gb-admin-panel"}>
                <AdminMenu />

                <div class={"gb-admin-main"}>
                   <h1>{"Users"}</h1>
                </div>
            </div>

        </main>
    }
}
