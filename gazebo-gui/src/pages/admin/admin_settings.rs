// Admin Settings screen

use yew::prelude::*;

use crate::components::{admin_bar::AdminBar, admin_menu::AdminMenu};

#[function_component(AdminSettings)]
pub fn admin_settings() -> Html {
    html! {
         <main id={crate::consts::CSS_ID_ADMIN_AREA}>
            <AdminBar />

            <div class={"flex bg-red-100"}>
                <AdminMenu />

                <div class={"gb-admin-main"}>
                   <h1>{"Settings"}</h1>
                </div>
            </div>

        </main>
    }
}
