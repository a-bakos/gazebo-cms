// Admin Dashboard screen

use yew::prelude::*;

use crate::components::{admin_bar::AdminBar};

#[function_component(AdminMedia)]
pub fn admin_media() -> Html {
    html! {
         <main id={crate::consts::CSS_ID_ADMIN_AREA}>
            <AdminBar />

            <div class={"flex bg-red-100"}>
                <div class={"gb-admin-main"}>
                   <h1>{"Media Lib"}</h1>
                </div>
            </div>

        </main>
    }
}
