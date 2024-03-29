use yew::prelude::*;

use crate::components::{admin::profile_editor::ProfileEditor, admin_bar::AdminBar};

#[function_component(AdminProfile)]
pub fn admin_profile() -> Html {
    html! {
        <main id={crate::consts::CSS_ID_ADMIN_AREA}>
            <AdminBar />

            <div class={"flex bg-red-100"}>
                <div class={"w-full"}>
                    <h1>{"Profile editor"}</h1>
                    <ProfileEditor />
                </div>
            </div>
        </main>
    }
}
