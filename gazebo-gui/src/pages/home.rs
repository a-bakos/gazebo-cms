// Homepage

use crate::api::post::GB_Post;
use yew::platform::spawn_local;
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

    let row_titles_handle = use_state(|| Vec::<GB_Post>::new());
    let row_titles = row_titles_handle.clone();
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let mut entry_rows: Vec<GB_Post> = vec![];
                let response = crate::api::post::api_get_all_posts().await.unwrap();
                for response_gb_post in response.iter() {
                    entry_rows.push(response_gb_post.clone());
                }
                row_titles_handle.set(entry_rows);
            });
            || ()
        },
        (),
    );

    html! {
        <>
            <main id={"gb-gui-home-page"}>
                <AdminBar />
                <Nav />
                <h1 class={"text-3xl"}>{"Hello, "}{name.clone()}</h1>
                <section class={"mx-auto max-w-6xl"}>
                    {
                        for row_titles.iter().map(|entry_row| html! {
                            <article class={"bg-yellow-100 mb-3"}>
                                <h2 class={"bold text-xl"}>{entry_row.title.clone()}</h2>
                                <p>{"Published: "} {entry_row.date_publish.clone()}</p>
                                <p>{"By: "} {entry_row.id_author}</p>
                                <p>{entry_row.content.clone()}</p>
                            </article>
                        } )
                    }
                </section>
            </main>
            <Footer />
        </>
    }
}
