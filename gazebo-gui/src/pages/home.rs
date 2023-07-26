// Homepage

use yew::prelude::*;

use crate::components::{admin_bar::AdminBar, footer::Footer, nav::Nav};

#[function_component(Home)]
pub fn home() -> Html {
    let current_user_ctx = use_context::<crate::context::CurrentUserContext>()
        .expect("Current accounts context missing");

    let name = match &current_user_ctx.user {
        Some(user) => &user.username,
        None => "Logged out",
    };

    html! {
        <>
            <main id={"gb-gui-home-page"}>
                <AdminBar />
                <Nav />
                <h1>{"Home "}{name.clone()}</h1>
            </main>
            <Footer />
        </>
    }
}
