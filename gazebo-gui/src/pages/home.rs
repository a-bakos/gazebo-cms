// Homepage

use gazebo_core_common::{
    entry::{
        consts::POST_UNTITLED_DEFAULT_TITLE,
        entry_type::EntryType,
        gb_post::GB_Post,
        status::{ContentStatus, EntryStatus},
    },
    helpers::get_permalink,
};
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::app::MainNavigationRoute;

use crate::components::{admin_bar::AdminBar, footer::Footer, nav::Nav};

#[function_component(Home)]
pub fn home() -> Html {
    let current_user_ctx = use_context::<crate::context::CurrentUserContext>()
        .expect("Current accounts context missing");

    let name = match &current_user_ctx.user {
        Some(user) => format!("TEST {}", &user.username),
        None => "Logged out".to_string(),
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
                        for row_titles.iter().filter(|entry_row|
                            // entry_row.status != EntryStatus::Post(ContentStatus::Draft) &&
                            entry_row.status != EntryStatus::Post(ContentStatus::Trash) //&&
                            // entry_row.status != EntryStatus::Post(ContentStatus::Private)
                        )
                        .map(|entry_row| {
                            let entry_prefix_title = match entry_row.status {
                                EntryStatus::Post(ContentStatus::Draft) => "Draft: ",
                                EntryStatus::Post(ContentStatus::Private) => "Private: ",
                                _ => "",
                            };
                            let entry_title = match entry_row.title.clone() {
                                Some(title) => title,
                                None => POST_UNTITLED_DEFAULT_TITLE.to_string(),
                            };

                            html! {
                                <article class={"border mb-3 p-3"}>
                                    <h2 class={"bold text-xl mb-2"}>
                                        <Link<MainNavigationRoute>
                                            to={ MainNavigationRoute::EntryView { id: entry_row.id.0.to_string() } }
                                            classes={ "hover:underline" }>
                                            {format!{"{}{}", entry_prefix_title, entry_title.clone()}}
                                        </Link<MainNavigationRoute>>
                                    </h2>
                                    <div class={"mb-2"}>
                                        <p>{"Published: "} {entry_row.date_publish.clone()}</p>
                                        <p>{"By: "} {entry_row.id_author.clone()}</p>
                                    </div>
                                    <p class={"pb-2"}>{entry_row.excerpt.clone()}</p>
                                    <p class={"pb-2 border-b"}>{entry_row.content.clone()}</p>

                                    <Link<MainNavigationRoute>
                                        to={ MainNavigationRoute::EntryView { id: entry_row.id.0.to_string() } }
                                        classes={ "inline-block mt-2 p-3 bg-gray-100 underline text-bold hover:text-red-400" }>
                                        {"Read more"}
                                    </Link<MainNavigationRoute>>
                                </article>
                            }
                        } )
                    }
                </section>
            </main>
            <Footer />
        </>
    }
}
