// view entry
// /entry/post/:id
// Front end entry view screen

use crate::components::{admin_bar::AdminBar, footer::Footer};
use gazebo_core_common::entry::{gb_entry::GB_EntryCommon, gb_post::GB_Post, status::EntryStatus};
use yew::{platform::spawn_local, prelude::*};

#[derive(Properties, PartialEq)]
pub struct EntryViewProps {
    pub entry_id: AttrValue,
}

#[function_component(EntryView)]
pub fn entry_view(props: &EntryViewProps) -> Html {
    //let current_user_ctx = use_context::<crate::context::CurrentUserContext>()
    //.expect("Current accounts context missing");
    let entry_id = props.entry_id.clone().to_string();
    let single_entry_viewer_handle = use_state(|| GB_Post::new());
    let single_entry = single_entry_viewer_handle.clone();
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let response =
                    crate::api::post::api_get_single_post("post".to_string(), entry_id.clone())
                        .await
                        .unwrap();
                single_entry_viewer_handle.set(response);
            });
            || ()
        },
        (),
    );

    // todo need an extractor method for this
    let entry_status = single_entry.status.clone();
    let status_string = match entry_status {
        EntryStatus::Post(content_status) => content_status.to_string(),
        _ => "".to_string(),
    };

    html! {
        <>
             <main>
                <AdminBar />
                    <section class="bg-white w-full p-4">
                        <article>
                            <h2 class="w-4/6 mx-auto text-center font-black text-3xl my-6 ">{ single_entry.title.clone() }</h2>
                            <header class="w-4/6 mx-auto border-y py-3 mb-6">
                                { "Posted on: " } { single_entry.date_publish.clone() }
                                { "Written by: " } { single_entry.id_author.clone() }
                                { "Category: xxx" }
                            </header>
                            <div class="w-3/6 mx-auto">
                                { single_entry.content.clone() }
                            </div>
                            <footer class="w-4/6 mx-auto p-6 mt-12 bg-gray-100">
                                <a>{ "Edit entry" }</a>
                                <span>{ "Status: " } { status_string.clone() }</span>
                                <span>{ "Last modified: " } { single_entry.date_publish.clone() }</span>
                                <span>{ "Entry ID: " } { single_entry.id.0.to_string() }</span>
                            </footer>
                        </article>
                        <a class="block w-4/6 mx-auto">{ "Back to homepage" }</a>
                    </section>
            </main>
            <Footer />
        </>
    }
}
