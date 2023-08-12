// Admin Dashboard screen

use yew::prelude::*;

use crate::components::{admin::widget_logged_in_users::WidgetLoggedInUsers, admin_bar::AdminBar};

#[function_component(AdminDashboard)]
pub fn admin_dashboard() -> Html {
    html! {
         <main id={crate::consts::CSS_ID_ADMIN_AREA} class={"gb-green-100 w-screen h-screen"}>
            <AdminBar />

            <div class={"flex bg-red-100"}>
                <div class={"w-full"}>
                   <h1 class={"text-2xl"}>{"Dashboard"}</h1>
                    <WidgetLoggedInUsers />
                </div>
            </div>

        </main>
    }
}
