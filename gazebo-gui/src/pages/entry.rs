// view entry
// /entry/post/:id

// Admin post/entry editor screen // move to admin/

use crate::api::post::api_get_single_post;
use crate::components::{admin_bar::AdminBar, entry_editor::EntryEditor};
use gazebo_core_common::account::gb_account::AccountID;
use gazebo_core_common::entry::entry_id::EntryID;
use gazebo_core_common::entry::entry_type::EntryType;
use gazebo_core_common::entry::gb_post::GB_Post;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EntryViewProps {
    pub entry_id: AttrValue,
}

#[function_component(EntryView)]
pub fn entry_view(props: &EntryViewProps) -> Html {
    let current_user_ctx = use_context::<crate::context::CurrentUserContext>()
        .expect("Current accounts context missing");
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

    html! {
         <main>
            <AdminBar />
                <section class="bg-white w-full p-4">
                    <p>{ single_entry.title.clone() }</p>
                </section>
        </main>
    }
}
