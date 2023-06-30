// Admin Dashboard screen

use yew::prelude::*;

use crate::components::{admin_bar::AdminBar, admin_menu::AdminMenu};

#[function_component(AdminDashboard)]
pub fn admin_dashboard() -> Html {
    html! {
         <main id={crate::consts::CSS_ID_ADMIN_AREA}>
            <AdminBar />

            <div class={"gb-admin-panel"}>
                <AdminMenu />

                <div class={"gb-admin-main"}>
                   <h1>{"Dashboard"}</h1>
                </div>
            </div>

        </main>
    }
}
