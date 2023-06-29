// Homepage

use yew::prelude::*;

use crate::components::admin_bar::AdminBar;
use crate::components::footer::Footer;
use crate::components::nav::Nav;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <main id={"gb-gui-home-page"}>
                <AdminBar />
                <Nav />
                <h1>{"Home"}</h1>
            </main>
            <Footer />
        </>
    }
}
